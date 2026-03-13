//! Windows 悬浮胶囊实现
//!
//! 使用 WS_EX_LAYERED + UpdateLayeredWindow + GDI 实现
//! 参考微信输入法效果：深色半透明背景，圆角 16px，三个白点跳动动画

use crate::{CapsuleState, CapsuleWindow, FloatingError, CAPSULE_HEIGHT, CAPSULE_WIDTH};
use std::ffi::c_void;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Instant;

use windows::{
    core::*,
    Win32::Foundation::*,
    Win32::Graphics::Gdi::*,
    Win32::System::LibraryLoader::GetModuleHandleW,
    Win32::UI::WindowsAndMessaging::*,
};

const DOT_SIZE: i32 = 4;
const DOT_SPACING: i32 = 5;
const DOT_BASE_X: i32 = 14;
const ANIMATION_DURATION: f32 = 0.6; // 一个完整跳动周期（秒）

/// Windows 悬浮胶囊窗口
pub struct WindowsCapsule {
    hwnd: HWND,
    state: CapsuleState,
    visible: AtomicBool,
    animation_running: AtomicBool,
    animation_start: Instant,
}

impl WindowsCapsule {
    pub fn new() -> std::result::Result<Self, FloatingError> {
        unsafe {
            // 注册窗口类
            let instance = GetModuleHandleW(None)
                .map_err(|e| FloatingError::WindowCreationFailed(format!("GetModuleHandle failed: {}", e)))?;

            let class_name = HSTRING::from("EchoVoiceCapsuleWindow");

            let wc = WNDCLASSEXW {
                cbSize: std::mem::size_of::<WNDCLASSEXW>() as u32,
                lpfnWndProc: Some(window_proc),
                hInstance: instance.into(),
                lpszClassName: windows::core::PCWSTR(class_name.as_ptr()),
                hCursor: LoadCursorW(None, IDC_ARROW).ok().unwrap_or_default(),
                ..Default::default()
            };

            let atom = RegisterClassExW(&wc);
            if atom == 0 {
                let last_err = windows::core::Error::from_win32();
                // Class might already be registered, which is OK
                if last_err.code().0 as u32 != 0x00000587 { // ERROR_CLASS_ALREADY_EXISTS
                    return Err(FloatingError::WindowCreationFailed(
                        format!("RegisterClassExW failed: {:?}", last_err)
                    ));
                }
            }

            // 创建分层窗口
            let hwnd = CreateWindowExW(
                WINDOW_EX_STYLE(WS_EX_LAYERED.0 | WS_EX_TOOLWINDOW.0 | WS_EX_NOACTIVATE.0),
                &class_name,
                &HSTRING::from("EchoVoice Capsule"),
                WINDOW_STYLE(WS_POPUP.0),
                0, 0, CAPSULE_WIDTH, CAPSULE_HEIGHT,
                None,
                None,
                instance,
                None,
            );

            if hwnd.0 == 0 {
                return Err(FloatingError::WindowCreationFailed(
                    "CreateWindowEx failed".to_string()
                ));
            }

            // 设置窗口透明色（用于 UpdateLayeredWindow）
            let _ = SetLayeredWindowAttributes(hwnd, COLORREF(0), 0, LWA_COLORKEY);

            Ok(Self {
                hwnd,
                state: CapsuleState::Idle,
                visible: AtomicBool::new(false),
                animation_running: AtomicBool::new(false),
                animation_start: Instant::now(),
            })
        }
    }

    /// 渲染胶囊到内存 DC
    fn render(&self) -> std::result::Result<(HBITMAP, HDC, i32, i32), FloatingError> {
        unsafe {
            let width = CAPSULE_WIDTH;
            let height = CAPSULE_HEIGHT;

            // 创建内存 DC
            let screen_dc = GetDC(None);
            let mem_dc = CreateCompatibleDC(screen_dc);
            ReleaseDC(None, screen_dc);

            // 创建 32 位位图
            let bmi = BITMAPINFO {
                bmiHeader: BITMAPINFOHEADER {
                    biSize: std::mem::size_of::<BITMAPINFOHEADER>() as u32,
                    biWidth: width as i32,
                    biHeight: -(height as i32), // 自顶向下
                    biPlanes: 1,
                    biBitCount: 32,
                    biCompression: BI_RGB.0 as u32,
                    ..Default::default()
                },
                ..Default::default()
            };

            let mut bits: *mut u8 = std::ptr::null_mut();
            let hbitmap = CreateDIBSection(
                mem_dc,
                &bmi,
                DIB_RGB_COLORS,
                &mut bits as *mut _ as *mut *mut c_void,
                HANDLE(0),
                0,
            )
            .map_err(|e| FloatingError::RenderFailed(format!("CreateDIBSection failed: {}", e)))?;

            SelectObject(mem_dc, hbitmap);

            // 使用 GDI 绘制
            self.draw_with_gdi(mem_dc, width, height);

            // 释放 DC（位图仍保留）
            DeleteDC(mem_dc);

            Ok((hbitmap, GetDC(None), width, height))
        }
    }

    /// 使用 GDI 绘制胶囊
    unsafe fn draw_with_gdi(&self, mem_dc: HDC, width: i32, height: i32) {
        // 清空背景（透明）
        let screen_dc = GetDC(None);
        let mem_dc_temp = CreateCompatibleDC(screen_dc);
        ReleaseDC(None, screen_dc);

        // 创建内存位图用于绘制
        let mut bmi = BITMAPINFO {
            bmiHeader: BITMAPINFOHEADER {
                biSize: std::mem::size_of::<BITMAPINFOHEADER>() as u32,
                biWidth: width as i32,
                biHeight: -(height as i32),
                biPlanes: 1,
                biBitCount: 32,
                biCompression: BI_RGB.0 as u32,
                ..Default::default()
            },
            ..Default::default()
        };

        let mut bits: *mut u8 = std::ptr::null_mut();
        let hbitmap = CreateDIBSection(
            mem_dc_temp,
            &bmi,
            DIB_RGB_COLORS,
            &mut bits as *mut _ as *mut *mut c_void,
            HANDLE(0),
            0,
        ).unwrap_or_default();

        SelectObject(mem_dc_temp, hbitmap);

        // 填充透明背景
        let mut rect = RECT {
            left: 0,
            top: 0,
            right: width,
            bottom: height,
        };
        FillRect(mem_dc_temp, &rect, HBRUSH(GetStockObject(BLACK_BRUSH).0));

        // 绘制圆角矩形背景（深色半透明）
        let bg_brush = CreateSolidBrush(COLORREF(0x282828));
        let pen = CreatePen(PS_SOLID, 0, COLORREF(0x282828));
        let old_pen = SelectObject(mem_dc_temp, pen);
        let old_brush = SelectObject(mem_dc_temp, bg_brush);

        // 绘制圆角矩形
        RoundRect(mem_dc_temp, 0, 0, width, height, 32, 32);

        // 恢复
        SelectObject(mem_dc_temp, old_pen);
        SelectObject(mem_dc_temp, old_brush);
        DeleteObject(pen);
        DeleteObject(bg_brush);

        // 绘制三个白点动画
        let elapsed = if self.animation_running.load(Ordering::SeqCst) {
            self.animation_start.elapsed().as_secs_f32()
        } else {
            0.0
        };

        let white_brush = CreateSolidBrush(COLORREF(0xFFFFFF));
        let old_brush = SelectObject(mem_dc_temp, white_brush);
        let null_pen = GetStockObject(NULL_PEN);
        let old_pen = SelectObject(mem_dc_temp, null_pen);

        for i in 0..3 {
            let x = DOT_BASE_X + i * DOT_SPACING;
            let base_y = (height - DOT_SIZE) / 2;

            // 计算动画偏移（正弦波，带相位差）
            let phase = i as f32 * 0.3;
            let t = ((elapsed + phase) % ANIMATION_DURATION) / ANIMATION_DURATION;
            let offset = (t * std::f32::consts::PI * 2.0).sin() * 2.0;
            let y = base_y + offset as i32;

            // 计算透明度（通过调整圆点大小模拟）
            let alpha = 0.3 + (t * std::f32::consts::PI * 2.0).sin() * 0.35;
            let alpha = alpha.clamp(0.3, 1.0);
            let dot_size = (DOT_SIZE as f32 * alpha) as i32;

            // 绘制圆点
            Ellipse(mem_dc_temp, x, y, x + dot_size, y + dot_size);
        }

        SelectObject(mem_dc_temp, old_brush);
        SelectObject(mem_dc_temp, old_pen);
        DeleteObject(white_brush);

        // 绘制文字
        let text = match self.state {
            CapsuleState::Recording => "正在听",
            CapsuleState::Processing => "思考中",
            CapsuleState::Success => "已复制",
            CapsuleState::NoAudio => "未检测到声音",
            CapsuleState::Error(_) => "识别失败",
            CapsuleState::Idle => "",
        };

        if !text.is_empty() {
            SetTextColor(mem_dc_temp, COLORREF(0xFFFFFF));
            SetBkMode(mem_dc_temp, TRANSPARENT);

            let mut text_utf16: Vec<u16> = text.encode_utf16().collect();
            let mut text_rect = RECT {
                left: 35,
                top: 0,
                right: width - 10,
                bottom: height,
            };
            DrawTextW(mem_dc_temp, &mut text_utf16, &mut text_rect,
                DT_LEFT | DT_VCENTER | DT_SINGLELINE);
        }

        // 将绘制结果复制到目标 DC
        BitBlt(mem_dc, 0, 0, width, height, mem_dc_temp, 0, 0, SRCCOPY);

        // 清理
        DeleteObject(hbitmap);
        DeleteDC(mem_dc_temp);
    }

    fn update_layered_window(&self) -> std::result::Result<(), FloatingError> {
        unsafe {
            let (hbitmap, screen_dc, width, height) = self.render()?;

            let mem_dc = CreateCompatibleDC(screen_dc);
            SelectObject(mem_dc, hbitmap);

            // 准备 UpdateLayeredWindow 参数
            let pt_dst = POINT { x: 0, y: 0 };
            let size = SIZE {
                cx: width,
                cy: height,
            };
            let pt_src = POINT { x: 0, y: 0 };
            let blend = BLENDFUNCTION {
                BlendOp: AC_SRC_OVER as u8,
                BlendFlags: 0,
                SourceConstantAlpha: 255,
                AlphaFormat: AC_SRC_ALPHA as u8,
            };

            // 更新分层窗口
            UpdateLayeredWindow(
                self.hwnd,
                screen_dc,
                Some(&pt_dst),
                Some(&size),
                mem_dc,
                Some(&pt_src),
                COLORREF(0),
                Some(&blend),
                ULW_ALPHA,
            )
            .map_err(|e| FloatingError::RenderFailed(format!("UpdateLayeredWindow failed: {}", e)))?;

            // 清理
            DeleteDC(mem_dc);
            ReleaseDC(None, screen_dc);
            let _ = DeleteObject(hbitmap);

            Ok(())
        }
    }

    fn start_animation(&self) {
        if !self.animation_running.load(Ordering::SeqCst) {
            self.animation_running.store(true, Ordering::SeqCst);
        }
    }

    fn stop_animation(&self) {
        self.animation_running.store(false, Ordering::SeqCst);
    }
}

impl CapsuleWindow for WindowsCapsule {
    fn new() -> std::result::Result<Self, FloatingError>
    where
        Self: Sized,
    {
        Self::new()
    }

    fn show(&self, x: i32, y: i32) -> std::result::Result<(), FloatingError> {
        unsafe {
            // 设置窗口位置
            SetWindowPos(
                self.hwnd,
                HWND_TOPMOST,
                x,
                y,
                CAPSULE_WIDTH,
                CAPSULE_HEIGHT,
                SWP_NOACTIVATE | SWP_SHOWWINDOW,
            );

            // 显示窗口
            ShowWindow(self.hwnd, SW_SHOWNOACTIVATE);

            self.visible.store(true, Ordering::SeqCst);
            self.start_animation();

            // 初始渲染
            self.update_layered_window()?;

            // 启动动画定时器
            SetTimer(self.hwnd, 1, 16, None); // 60fps
        }

        Ok(())
    }

    fn hide(&self) -> std::result::Result<(), FloatingError> {
        unsafe {
            ShowWindow(self.hwnd, SW_HIDE);
            KillTimer(self.hwnd, 1);
        }

        self.visible.store(false, Ordering::SeqCst);
        self.stop_animation();
        Ok(())
    }

    fn set_state(&mut self, state: CapsuleState) -> std::result::Result<(), FloatingError> {
        self.state = state;

        // 重新渲染
        if self.visible.load(Ordering::SeqCst) {
            self.update_layered_window()?;
        }

        Ok(())
    }

    fn update_waveform(&mut self, _levels: &[f32]) -> std::result::Result<(), FloatingError> {
        // TODO: 根据音量更新动画强度
        Ok(())
    }

    fn run_loop(&mut self) -> std::result::Result<(), FloatingError> {
        // Windows 消息循环在主线程中处理
        Ok(())
    }

    fn close(&mut self) -> std::result::Result<(), FloatingError> {
        self.stop_animation();
        unsafe {
            DestroyWindow(self.hwnd);
        }
        Ok(())
    }
}

/// 窗口过程函数
unsafe extern "system" fn window_proc(
    hwnd: HWND,
    msg: u32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    match msg {
        WM_TIMER => {
            // 动画帧更新
            LRESULT(0)
        }
        WM_DESTROY => {
            PostQuitMessage(0);
            LRESULT(0)
        }
        _ => DefWindowProcW(hwnd, msg, wparam, lparam),
    }
}

/// 获取屏幕尺寸
pub fn get_screen_size() -> (i32, i32) {
    unsafe {
        let width = GetSystemMetrics(SM_CXSCREEN);
        let height = GetSystemMetrics(SM_CYSCREEN);
        (width, height)
    }
}

/// 获取任务栏高度
pub fn get_taskbar_height() -> i32 {
    unsafe {
        // 获取主监视器工作区
        let mut work_area = RECT::default();
        if SystemParametersInfoW(
            SPI_GETWORKAREA,
            0,
            Some(&mut work_area as *mut _ as *mut c_void),
            SYSTEM_PARAMETERS_INFO_UPDATE_FLAGS(0),
        )
        .is_ok()
        {
            let screen_height = GetSystemMetrics(SM_CYSCREEN);
            (screen_height - work_area.bottom) as i32
        } else {
            40 // 默认任务栏高度
        }
    }
}
