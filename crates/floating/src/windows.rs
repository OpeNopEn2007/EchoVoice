//! Windows 悬浮胶囊实现
//!
//! 使用 WS_EX_LAYERED + UpdateLayeredWindow + Direct2D 实现
//! 参考微信输入法效果：深色半透明背景，圆角 16px，三个白点跳动动画

use crate::{CapsuleState, CapsuleWindow, FloatingError, CAPSULE_HEIGHT, CAPSULE_WIDTH};
use std::ffi::c_void;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Instant;

use windows::{
    core::*,
    Win32::Foundation::*,
    Win32::Graphics::Direct2D::*,
    Win32::Graphics::Direct2D::Common::*,
    Win32::Graphics::Dxgi::Common::*,
    Win32::Graphics::Gdi::*,
    Win32::System::LibraryLoader::GetModuleHandleW,
    Win32::UI::WindowsAndMessaging::*,
};

const DOT_SIZE: f32 = 4.0;
const DOT_SPACING: f32 = 5.0;
const DOT_BASE_X: f32 = 14.0;
const ANIMATION_DURATION: f32 = 0.6; // 一个完整跳动周期（秒）

/// Windows 悬浮胶囊窗口
pub struct WindowsCapsule {
    hwnd: HWND,
    state: CapsuleState,
    visible: AtomicBool,
    animation_running: AtomicBool,
    animation_start: Instant,
    // Direct2D 资源
    d2d_factory: ID2D1Factory,
    render_target: Option<ID2D1DCRenderTarget>,
    // 字体资源
    text_format: Option<windows::Win32::Graphics::DirectWrite::IDWriteTextFormat>,
}

impl WindowsCapsule {
    pub fn new() -> std::result::Result<Self, FloatingError> {
        unsafe {
            // 创建 Direct2D 工厂
            let d2d_factory = D2D1CreateFactory::<ID2D1Factory>(
                D2D1_FACTORY_TYPE_SINGLE_THREADED,
                None,
            ).map_err(|e| FloatingError::WindowCreationFailed(format!("D2D1CreateFactory failed: {}", e)))?;

            // 注册窗口类
            let instance = GetModuleHandleW(None)
                .map_err(|e| FloatingError::WindowCreationFailed(format!("GetModuleHandle failed: {}", e)))?;

            let class_name = w!("EchoVoiceCapsuleWindow");

            let wc = WNDCLASSEXW {
                cbSize: std::mem::size_of::<WNDCLASSEXW>() as u32,
                lpfnWndProc: Some(window_proc),
                hInstance: instance.into(),
                lpszClassName: class_name,
                hCursor: LoadCursorW(None, IDC_ARROW)
                    .map_err(|e| FloatingError::WindowCreationFailed(format!("LoadCursor failed: {}", e)))?,
                ..Default::default()
            };

            if RegisterClassExW(&wc) == 0 {
                // 可能已注册，检查错误
                let _ = GetLastError();
            }

            // 创建分层窗口
            let hwnd = CreateWindowExW(
                WINDOW_EX_STYLE(WS_EX_LAYERED.0 | WS_EX_TOOLWINDOW.0 | WS_EX_NOACTIVATE.0),
                class_name,
                w!("EchoVoice Capsule"),
                WINDOW_STYLE(WS_POPUP.0),
                0, 0, CAPSULE_WIDTH, CAPSULE_HEIGHT,
                None,
                None,
                instance,
                None,
            );

            if hwnd.0 == 0 {
                return Err(FloatingError::WindowCreationFailed("CreateWindowEx failed".to_string()));
            }

            // 设置窗口透明色（用于 UpdateLayeredWindow）
            let _ = SetLayeredWindowAttributes(hwnd, COLORREF(0), 0, LWA_COLORKEY);

            // 创建字体
            let dwrite_factory = windows::Win32::Graphics::DirectWrite::DWriteCreateFactory::<
                windows::Win32::Graphics::DirectWrite::IDWriteFactory,
            >(windows::Win32::Graphics::DirectWrite::DWRITE_FACTORY_TYPE_SHARED)
            .map_err(|_| FloatingError::WindowCreationFailed("DWriteCreateFactory failed".to_string()))?;

            let text_format = dwrite_factory
                .CreateTextFormat(
                    w!("Segoe UI"),
                    None,
                    windows::Win32::Graphics::DirectWrite::DWRITE_FONT_WEIGHT_NORMAL,
                    windows::Win32::Graphics::DirectWrite::DWRITE_FONT_STYLE_NORMAL,
                    windows::Win32::Graphics::DirectWrite::DWRITE_FONT_STRETCH_NORMAL,
                    13.0,
                    w!("zh-CN"),
                )
                .ok();

            if let Some(ref format) = text_format {
                let _ = format.SetTextAlignment(
                    windows::Win32::Graphics::DirectWrite::DWRITE_TEXT_ALIGNMENT_LEADING,
                );
                let _ = format.SetParagraphAlignment(
                    windows::Win32::Graphics::DirectWrite::DWRITE_PARAGRAPH_ALIGNMENT_CENTER,
                );
            }

            Ok(Self {
                hwnd,
                state: CapsuleState::Idle,
                visible: AtomicBool::new(false),
                animation_running: AtomicBool::new(false),
                animation_start: Instant::now(),
                d2d_factory,
                render_target: None,
                text_format,
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
            let mut bmi = BITMAPINFO {
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

            // 创建 Direct2D 渲染目标
            let props = D2D1_RENDER_TARGET_PROPERTIES {
                r#type: D2D1_RENDER_TARGET_TYPE_DEFAULT,
                pixelFormat: D2D1_PIXEL_FORMAT {
                    format: DXGI_FORMAT_B8G8R8A8_UNORM,
                    alphaMode: D2D1_ALPHA_MODE_PREMULTIPLIED,
                },
                ..Default::default()
            };

            let render_target = self
                .d2d_factory
                .CreateDCRenderTarget(&props)
                .map_err(|e| FloatingError::RenderFailed(format!("CreateDCRenderTarget failed: {}", e)))?;

            render_target.BindDC(mem_dc, &RECT {
                left: 0,
                top: 0,
                right: width,
                bottom: height,
            });

            // 开始绘制
            render_target.BeginDraw();

            // 清空背景（完全透明）
            render_target.Clear(None);

            // 创建圆角矩形路径
            let round_rect = D2D1_ROUNDED_RECT {
                rect: D2D_RECT_F {
                    left: 0.0,
                    top: 0.0,
                    right: width as f32,
                    bottom: height as f32,
                },
                radiusX: 16.0,
                radiusY: 16.0,
            };

            // 创建深色半透明画刷（rgba(40,40,40,0.9)）
            let bg_color = D2D1_COLOR_F {
                r: 0.157,
                g: 0.157,
                b: 0.157,
                a: 0.9,
            };
            let bg_brush = render_target
                .CreateSolidColorBrush(&bg_color, None)
                .map_err(|e| FloatingError::RenderFailed(format!("CreateSolidColorBrush failed: {}", e)))?;

            // 填充圆角矩形背景
            render_target.FillRoundedRectangle(&round_rect, &bg_brush);

            // 绘制三个白点动画
            let white_color = D2D1_COLOR_F {
                r: 1.0,
                g: 1.0,
                b: 1.0,
                a: 1.0,
            };
            let white_brush = render_target
                .CreateSolidColorBrush(&white_color, None)
                .map_err(|e| FloatingError::RenderFailed(format!("CreateSolidColorBrush failed: {}", e)))?;

            let elapsed = if self.animation_running.load(Ordering::SeqCst) {
                self.animation_start.elapsed().as_secs_f32()
            } else {
                0.0
            };

            for i in 0..3 {
                let x = DOT_BASE_X + i as f32 * DOT_SPACING;
                let base_y = (height as f32 - DOT_SIZE) / 2.0;

                // 计算动画偏移（正弦波，带相位差）
                let phase = i as f32 * 0.3; // 相位差
                let t = ((elapsed + phase) % ANIMATION_DURATION) / ANIMATION_DURATION;
                let offset = (t * std::f32::consts::PI * 2.0).sin() * 2.0;

                let y = base_y + offset;

                // 计算透明度
                let alpha = 0.3 + (t * std::f32::consts::PI * 2.0).sin() * 0.35;
                let alpha = alpha.clamp(0.3, 1.0);

                let dot_color = D2D1_COLOR_F {
                    r: 1.0,
                    g: 1.0,
                    b: 1.0,
                    a: alpha,
                };
                let dot_brush = render_target
                    .CreateSolidColorBrush(&dot_color, None)
                    .map_err(|e| FloatingError::RenderFailed(format!("CreateSolidColorBrush failed: {}", e)))?;

                // 绘制圆点
                let ellipse = D2D1_ELLIPSE {
                    point: D2D_POINT_2F { x: x + DOT_SIZE / 2.0, y: y + DOT_SIZE / 2.0 },
                    radiusX: DOT_SIZE / 2.0,
                    radiusY: DOT_SIZE / 2.0,
                };
                render_target.FillEllipse(&ellipse, &dot_brush);
            }

            // 绘制文字
            let text = match self.state {
                CapsuleState::Recording => "正在听",
                CapsuleState::Processing => "思考中",
                CapsuleState::Success => "✓ 已复制",
                CapsuleState::NoAudio => "未检测到声音",
                CapsuleState::Error(_) => "识别失败",
                CapsuleState::Idle => "",
            };

            if !text.is_empty() && self.text_format.is_some() {
                let text_utf16: Vec<u16> = text.encode_utf16().collect();
                let white_color = D2D1_COLOR_F {
                    r: 1.0,
                    g: 1.0,
                    b: 1.0,
                    a: 1.0,
                };
                let text_brush = render_target
                    .CreateSolidColorBrush(&white_color, None)
                    .map_err(|e| FloatingError::RenderFailed(format!("CreateSolidColorBrush failed: {}", e)))?;

                let layout_rect = D2D_RECT_F {
                    left: 35.0,
                    top: 0.0,
                    right: (width - 10) as f32,
                    bottom: height as f32,
                };

                // Use DrawTextW from GDI or create a text layout and draw it
                // For Direct2D, we need to use the ID2D1RenderTarget trait methods
                // DrawText is available via the ID2D1RenderTarget interface
                let dwrite_factory = windows::Win32::Graphics::DirectWrite::DWriteCreateFactory::<
                    windows::Win32::Graphics::DirectWrite::IDWriteFactory,
                >(windows::Win32::Graphics::DirectWrite::DWRITE_FACTORY_TYPE_SHARED)
                .map_err(|_| FloatingError::RenderFailed("DWriteCreateFactory failed".to_string()))?;

                let text_layout = dwrite_factory.CreateTextLayout(
                    &text_utf16,
                    self.text_format.as_ref().unwrap(),
                    layout_rect.right - layout_rect.left,
                    layout_rect.bottom - layout_rect.top,
                ).map_err(|e| FloatingError::RenderFailed(format!("CreateTextLayout failed: {}", e)))?;

                render_target.DrawTextLayout(
                    D2D_POINT_2F { x: layout_rect.left, y: layout_rect.top },
                    &text_layout,
                    &text_brush,
                    D2D1_DRAW_TEXT_OPTIONS_NONE,
                );
            }

            // 结束绘制
            render_target.EndDraw(None, None);

            // 释放 DC（位图仍保留）
            DeleteDC(mem_dc);

            Ok((hbitmap, GetDC(None), width, height))
        }
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
            let mut blend = BLENDFUNCTION {
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
                Some(&pt_src as *const POINT),
                COLORREF(0),
                Some(&blend as *const BLENDFUNCTION),
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
            // 需要重新渲染，但由于无法访问 self，这里简化处理
            // 实际实现可能需要使用 SetWindowLongPtr 存储指针
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
