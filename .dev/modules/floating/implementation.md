# Floating Capsule 原生实现方案

> Windows + macOS 双平台原生实现，100% 复刻微信输入法效果

## 参考图片

> **注意**: 以下路径为开发机器上的参考效果图，用于视觉还原

| 状态 | 图片路径 | 说明 |
|------|----------|------|
| 录音中 | `/Users/openopen/Pictures/正在听.png` | 三个白点跳动 + "正在听" |
| 处理中 | `/Users/openopen/Pictures/思考中.png` | 三个白点跳动 + "思考中" |

**目标效果**:
- 深色半透明胶囊：深灰背景 rgba(40, 40, 40, 0.9)
- 尺寸：120px × 32px，圆角 16px
- 位置：任务栏正上方，水平居中
- 动画：三个白点依次淡入淡出

---

## 架构概览

```
┌─────────────────────────────────────────────────────────────┐
│                    Rust 抽象层 (floating crate)              │
│  ┌─────────────────┐              ┌─────────────────────┐  │
│  │  CapsuleWindow  │◄────────────►│    CapsuleState     │  │
│  │    (Trait)      │              │    (Enum)           │  │
│  └─────────────────┘              └─────────────────────┘  │
│           │                                                  │
│           ▼ 平台分发                                          │
├──────────────────────────────┬──────────────────────────────┤
│      Windows 实现            │         macOS 实现           │
│  ┌──────────────────────┐   │   ┌──────────────────────┐   │
│  │   Win32 Layered      │   │   │   NSWindow +         │   │
│  │   Window + GDI+      │   │   │   NSVisualEffectView │   │
│  └──────────────────────┘   │   └──────────────────────┘   │
│           │                  │            │                 │
│           ▼                  │            ▼                 │
│  ┌──────────────────────┐   │   ┌──────────────────────┐   │
│  │  UpdateLayeredWindow │   │   │   Core Animation     │   │
│  │  Direct2D / GDI+     │   │   │   CATextLayer        │   │
│  └──────────────────────┘   │   └──────────────────────┘   │
└──────────────────────────────┴──────────────────────────────┘
```

---

## 一、Windows 原生实现

### 核心技术

- **WS_EX_LAYERED** - 分层窗口，支持透明
- **UpdateLayeredWindow** - 直接更新位图，无子窗口
- **Direct2D** - 硬件加速绘制，抗锯齿
- **Windows Animation Manager (WAM)** - 系统动画

### 窗口创建

```rust
// crates/floating/src/platform/windows.rs

use windows::Win32::Foundation::{HWND, LPARAM, LRESULT, WPARAM, HINSTANCE};
use windows::Win32::Graphics::Direct2D::*;
use windows::Win32::Graphics::DirectWrite::*;
use windows::Win32::Graphics::Gdi::{
    CreateCompatibleDC, DeleteDC, SelectObject, DeleteObject,
    BITMAPINFO, BITMAPINFOHEADER, BI_RGB, DIB_RGB_COLORS,
    UpdateLayeredWindow, BLENDFUNCTION, AC_SRC_OVER, AC_SRC_ALPHA,
    GetDC, ReleaseDC, ScreenToClient,
};
use windows::Win32::UI::WindowsAndMessaging::*;
use windows::Win32::System::LibraryLoader::GetModuleHandleW;

pub struct WindowsCapsule {
    hwnd: HWND,
    d2d_factory: ID2D1Factory,
    render_target: Option<ID2D1DCRenderTarget>,
    write_factory: IDWriteFactory,
    text_format: IDWriteTextFormat,

    // 动画状态
    dot_animation: DotAnimation,
    current_state: CapsuleState,
}

struct DotAnimation {
    phase: f32,  // 0.0 ~ 1.0
    timer: u32,  // 动画定时器 ID
}

impl WindowsCapsule {
    pub fn new() -> anyhow::Result<Self> {
        // 注册窗口类
        let class_name = encode_utf16("EchoVoiceCapsule");

        let wnd_class = WNDCLASSW {
            lpfnWndProc: Some(Self::window_proc),
            hInstance: GetModuleHandleW(None)?.into(),
            lpszClassName: PCWSTR(class_name.as_ptr()),
            hbrBackground: HBRUSH(std::ptr::null_mut()), // 不擦除背景
            style: CS_HREDRAW | CS_VREDRAW | CS_DBLCLKS,
            ..Default::default()
        };

        unsafe { RegisterClassW(&wnd_class) };

        // 创建分层窗口
        let hwnd = unsafe {
            CreateWindowExW(
                WS_EX_LAYERED |           // 分层窗口（透明）
                WS_EX_TRANSPARENT |       // 鼠标穿透
                WS_EX_TOOLWINDOW |        // 不在 Alt+Tab 显示
                WS_EX_NOACTIVATE,         // 不获取焦点
                PCWSTR(class_name.as_ptr()),
                PCWSTR::null(),
                WS_POPUP,                 // 无标题栏无边框
                CW_USEDEFAULT,
                CW_USEDEFAULT,
                CAPSULE_WIDTH,
                CAPSULE_HEIGHT,
                HWND(std::ptr::null_mut()),
                HMENU(std::ptr::null_mut()),
                GetModuleHandleW(None)?,
                std::ptr::null_mut(),
            )?
        };

        // 初始化 Direct2D
        let d2d_factory = unsafe {
            D2D1CreateFactory::<ID2D1Factory>(
                D2D1_FACTORY_TYPE_SINGLE_THREADED,
                None,
            )?
        };

        // 初始化 DirectWrite
        let write_factory = unsafe {
            DWriteCreateInstance::<IDWriteFactory>(
                DWRITE_FACTORY_TYPE_SHARED,
            )?
        };

        // 创建文本格式
        let text_format = unsafe {
            write_factory.CreateTextFormat(
                w!("Segoe UI"),  // 系统字体
                None,
                DWRITE_FONT_WEIGHT_NORMAL,
                DWRITE_FONT_STYLE_NORMAL,
                DWRITE_FONT_STRETCH_NORMAL,
                13.0,  // 字号
                w!("zh-cn"),
            )?
        };
        unsafe { text_format.SetTextAlignment(DWRITE_TEXT_ALIGNMENT_CENTER)?; }
        unsafe { text_format.SetParagraphAlignment(DWRITE_PARAGRAPH_ALIGNMENT_CENTER)?; }

        let mut capsule = Self {
            hwnd,
            d2d_factory,
            render_target: None,
            write_factory,
            text_format,
            dot_animation: DotAnimation { phase: 0.0, timer: 0 },
            current_state: CapsuleState::Idle,
        };

        // 设置定时器（60fps 动画）
        capsule.start_animation_timer()?;

        Ok(capsule)
    }

    fn create_render_target(&mut self) -> anyhow::Result<()> {
        let props = D2D1_RENDER_TARGET_PROPERTIES {
            r#type: D2D1_RENDER_TARGET_TYPE_DEFAULT,
            pixelFormat: D2D1_PIXEL_FORMAT {
                format: DXGI_FORMAT_B8G8R8A8_UNORM,
                alphaMode: D2D1_ALPHA_MODE_PREMULTIPLIED,
            },
            dpiX: 0.0,
            dpiY: 0.0,
            usage: D2D1_RENDER_TARGET_USAGE_NONE,
            minLevel: D2D1_FEATURE_LEVEL_DEFAULT,
        };

        let dc = unsafe { GetDC(self.hwnd) };
        let render_target = unsafe {
            self.d2d_factory.CreateDCRenderTarget(&props)?
        };
        unsafe { render_target.BindDC(dc, &RECT { left: 0, top: 0, right: CAPSULE_WIDTH, bottom: CAPSULE_HEIGHT })?; }
        unsafe { ReleaseDC(self.hwnd, dc) };

        self.render_target = Some(render_target);
        Ok(())
    }

    pub fn show(&self, x: i32, y: i32) -> anyhow::Result<()> {
        unsafe {
            SetWindowPos(
                self.hwnd,
                HWND_TOPMOST,
                x, y,
                CAPSULE_WIDTH, CAPSULE_HEIGHT,
                SWP_FRAMECHANGED | SWP_SHOWWINDOW,
            )?;
            ShowWindow(self.hwnd, SW_SHOWNA)?; // SW_SHOWNA = 显示但不激活
        }
        Ok(())
    }

    pub fn hide(&self) -> anyhow::Result<()> {
        unsafe {
            ShowWindow(self.hwnd, SW_HIDE)?;
        }
        Ok(())
    }

    fn render(&mut self) -> anyhow::Result<()> {
        let rt = self.render_target.as_ref().ok_or_else(|| anyhow::anyhow!("Render target not created"))?;

        unsafe {
            rt.BeginDraw();

            // 清空背景（完全透明）
            rt.Clear(Some(&D2D1_COLOR_F { r: 0.0, g: 0.0, b: 0.0, a: 0.0 }));

            // 创建画刷
            let bg_color = D2D1_COLOR_F { r: 0.157, g: 0.157, b: 0.157, a: 0.9 }; // #282828
            let bg_brush = rt.CreateSolidColorBrush(&bg_color, None)?;

            let text_color = D2D1_COLOR_F { r: 1.0, g: 1.0, b: 1.0, a: 1.0 };
            let text_brush = rt.CreateSolidColorBrush(&text_color, None)?;

            // 绘制圆角矩形背景
            let rect = D2D1_ROUNDED_RECT {
                rect: D2D1_RECT_F {
                    left: 0.0, top: 0.0,
                    right: CAPSULE_WIDTH as f32, bottom: CAPSULE_HEIGHT as f32,
                },
                radiusX: 16.0,
                radiusY: 16.0,
            };
            rt.FillRoundedRectangle(&rect, &bg_brush);

            // 绘制三个点（带动画）
            self.draw_animated_dots(rt, &text_brush)?;

            // 绘制文字
            let text = self.get_state_text();
            let text_rect = D2D1_RECT_F {
                left: 40.0, top: 0.0,  // 留出点动画的空间
                right: CAPSULE_WIDTH as f32, bottom: CAPSULE_HEIGHT as f32,
            };
            rt.DrawText(
                &encode_utf16(&text),
                &self.text_format,
                &text_rect,
                &text_brush,
                D2D1_DRAW_TEXT_OPTIONS_NONE,
                DWRITE_MEASURING_MODE_NATURAL,
            );

            rt.EndDraw(None, None)?;
        }

        // 更新分层窗口
        self.update_layered_window()?;

        Ok(())
    }

    fn draw_animated_dots(&self, rt: &ID2D1DCRenderTarget, brush: &ID2D1SolidColorBrush) -> anyhow::Result<()> {
        let base_x = 14.0;
        let center_y = CAPSULE_HEIGHT as f32 / 2.0;
        let dot_radius = 2.0;
        let dot_spacing = 5.0;

        for i in 0..3 {
            let phase = (self.dot_animation.phase + i as f32 * 0.33) % 1.0;
            let alpha = if phase < 0.5 {
                0.3 + phase * 1.4  // 0.3 -> 1.0
            } else {
                1.0 - (phase - 0.5) * 1.4  // 1.0 -> 0.3
            };
            let scale = 0.6 + alpha * 0.4;

            let x = base_x + i as f32 * dot_spacing;
            let ellipse = D2D1_ELLIPSE {
                point: D2D_POINT_2F { x, y: center_y },
                radiusX: dot_radius * scale,
                radiusY: dot_radius * scale,
            };

            let dot_brush = rt.CreateSolidColorBrush(
                &D2D1_COLOR_F { r: 1.0, g: 1.0, b: 1.0, a: alpha },
                None,
            )?;
            rt.FillEllipse(&ellipse, &dot_brush);
        }

        Ok(())
    }

    fn update_layered_window(&self) -> anyhow::Result<()> {
        // 获取 DC 的位图并更新分层窗口
        let hdc_screen = unsafe { GetDC(HWND(std::ptr::null_mut())) };
        let hdc_mem = unsafe { CreateCompatibleDC(hdc_screen) };

        // 创建 32-bit DIB
        let mut bmi = BITMAPINFO::default();
        bmi.bmiHeader.biSize = std::mem::size_of::<BITMAPINFOHEADER>() as u32;
        bmi.bmiHeader.biWidth = CAPSULE_WIDTH;
        bmi.bmiHeader.biHeight = -CAPSULE_HEIGHT; // 自顶向下
        bmi.bmiHeader.biPlanes = 1;
        bmi.bmiHeader.biBitCount = 32;
        bmi.bmiHeader.biCompression = BI_RGB as u32;

        let mut bits: *mut std::ffi::c_void = std::ptr::null_mut();
        let hbitmap = unsafe {
            CreateDIBSection(
                hdc_mem,
                &bmi,
                DIB_RGB_COLORS,
                &mut bits,
                None,
                0,
            )?
        };

        unsafe { SelectObject(hdc_mem, hbitmap) };

        // 将 DC 内容复制到位图（需要实现）
        // ...

        // 更新分层窗口
        let pt_src = POINT { x: 0, y: 0 };
        let size = SIZE { cx: CAPSULE_WIDTH, cy: CAPSULE_HEIGHT };
        let pt_dst = POINT { x: 0, y: 0 }; // 相对于窗口客户区

        let blend = BLENDFUNCTION {
            BlendOp: AC_SRC_OVER as u8,
            BlendFlags: 0,
            SourceConstantAlpha: 255, // 使用位图 alpha
            AlphaFormat: AC_SRC_ALPHA as u8,
        };

        unsafe {
            UpdateLayeredWindow(
                self.hwnd,
                hdc_screen,
                &pt_dst,
                &size,
                hdc_mem,
                &pt_src,
                0,
                &blend,
                ULW_ALPHA,
            )?;
        }

        unsafe {
            DeleteObject(hbitmap.into())?;
            DeleteDC(hdc_mem)?;
            ReleaseDC(HWND(std::ptr::null_mut()), hdc_screen)?;
        }

        Ok(())
    }

    fn start_animation_timer(&mut self) -> anyhow::Result<()> {
        unsafe {
            SetTimer(self.hwnd, ANIMATION_TIMER_ID, 16, None); // 60fps
        }
        Ok(())
    }

    fn stop_animation_timer(&self) -> anyhow::Result<()> {
        unsafe {
            KillTimer(self.hwnd, ANIMATION_TIMER_ID)?;
        }
        Ok(())
    }

    extern "system" fn window_proc(
        hwnd: HWND,
        msg: u32,
        wparam: WPARAM,
        lparam: LPARAM,
    ) -> LRESULT {
        match msg {
            WM_TIMER => {
                // 更新动画相位并重绘
                LRESULT(0)
            }
            WM_DESTROY => {
                unsafe { PostQuitMessage(0) };
                LRESULT(0)
            }
            _ => unsafe { DefWindowProcW(hwnd, msg, wparam, lparam) },
        }
    }
}
```

---

## 二、macOS 原生实现

### 核心技术

- **NSWindow** + **NSVisualEffectView** - 系统级毛玻璃效果
- **CALayer** - Core Animation 硬件加速
- **CATextLayer** - 高质量文字渲染
- **CABasicAnimation** - 流畅动画

### 实现代码

```rust
// crates/floating/src/platform/macos.rs

use cocoa::appkit::{
    NSWindow, NSWindowStyleMask, NSWindowLevel, NSBackingStoreBuffered,
    NSColor, NSView, NSVisualEffectView, NSVisualEffectMaterial,
    NSVisualEffectBlendingMode, NSVisualEffectState,
};
use cocoa::base::{id, nil, YES, NO};
use cocoa::foundation::{NSString, NSRect, NSPoint, NSSize, NSAutoreleasePool};
use core_graphics::geometry::{CGRect, CGPoint, CGSize};
use core_animation::{CALayer, CATextLayer, CABasicAnimation};
use objc::runtime::{Object, Sel};
use objc::{msg_send, sel, sel_impl};

pub struct MacOSCapsule {
    window: id,           // NSWindow
    visual_effect: id,    // NSVisualEffectView
    content_view: id,     // 自定义 NSView
    dot_layers: Vec<id>,  // CATextLayer for dots
    text_layer: id,       // CATextLayer for text
}

const CAPSULE_WIDTH: f64 = 120.0;
const CAPSULE_HEIGHT: f64 = 32.0;

impl MacOSCapsule {
    pub fn new() -> anyhow::Result<Self> {
        let pool = NSAutoreleasePool::new(nil);

        // 创建窗口
        let frame = NSRect::new(
            NSPoint::new(0.0, 0.0),
            NSSize::new(CAPSULE_WIDTH, CAPSULE_HEIGHT),
        );

        let window_style = NSWindowStyleMask::NSBorderlessWindowMask |
                          NSWindowStyleMask::NSNonactivatingPanelMask;

        let window: id = unsafe {
            msg_send![
                NSWindow::alloc(nil),
                initWithContentRect:frame
                styleMask:window_style
                backing:NSBackingStoreBuffered
                defer:NO
            ]
        };

        // 窗口配置
        unsafe {
            window.setLevel_(NSWindowLevel::NSFloatingWindowLevel as i64);
            window.setBackgroundColor_(NSColor::clearColor(nil));
            window.setOpaque_(NO);
            window.setHasShadow_(NO);  // 无阴影
            window.setIgnoresMouseEvents_(YES);  // 鼠标穿透
            window.setCollectionBehavior_(
                1 << 0 | // NSWindowCollectionBehaviorCanJoinAllSpaces
                1 << 3 | // NSWindowCollectionBehaviorStationary
                1 << 4   // NSWindowCollectionBehaviorIgnoresCycles
            );
        }

        // 创建 Visual Effect View（毛玻璃效果）
        let visual_effect: id = unsafe {
            let view: id = msg_send![NSVisualEffectView::alloc(nil), initWithFrame:frame];
            msg_send![view, setMaterial:NSVisualEffectMaterial::NSVisualEffectMaterialHUDWindow];
            msg_send![view, setBlendingMode:NSVisualEffectBlendingMode::NSVisualEffectBlendingModeBehindWindow];
            msg_send![view, setState:NSVisualEffectState::NSVisualEffectStateActive];
            msg_send![view, setWantsLayer:YES];
            view
        };

        // 创建内容视图
        let content_view = Self::create_content_view()?;
        unsafe {
            visual_effect.addSubview_(content_view);
            window.setContentView_(visual_effect);
        }

        let mut capsule = Self {
            window,
            visual_effect,
            content_view,
            dot_layers: Vec::new(),
            text_layer: nil,
        };

        // 创建点和文字图层
        capsule.setup_layers()?;

        unsafe { pool.drain() };

        Ok(capsule)
    }

    fn create_content_view() -> anyhow::Result<id> {
        let frame = NSRect::new(
            NSPoint::new(0.0, 0.0),
            NSSize::new(CAPSULE_WIDTH, CAPSULE_HEIGHT),
        );

        let view: id = unsafe {
            let view: id = msg_send![NSView::alloc(nil), initWithFrame:frame];
            msg_send![view, setWantsLayer:YES];

            // 设置圆角
            let layer: id = msg_send![view, layer];
            msg_send![layer, setCornerRadius:16.0];
            msg_send![layer, setMasksToBounds:YES];

            // 半透明黑色背景
            let background_color: id = msg_send![
                NSColor::class(nil),
                colorWithCalibratedRed:0.157
                green:0.157
                blue:0.157
                alpha:0.9
            ];
            msg_send![view, setBackgroundColor:background_color];

            view
        };

        Ok(view)
    }

    fn setup_layers(&mut self) -> anyhow::Result<()> {
        let layer: id = unsafe { msg_send![self.content_view, layer] };

        // 创建三个点的图层
        for i in 0..3 {
            let dot_layer: id = unsafe {
                let layer: id = msg_send![CATextLayer::class(), layer];
                let text = NSString::alloc(nil).init_str("●");
                msg_send![layer, setString:text];
                msg_send![layer, setFontSize:4.0];
                msg_send![layer, setForegroundColor:CGColorCreateGenericRGB(1.0, 1.0, 1.0, 1.0)];

                let x = 14.0 + i as f64 * 5.0;
                let frame = CGRect::new(
                    &CGPoint::new(x, CAPSULE_HEIGHT / 2.0 - 2.0),
                    &CGSize::new(4.0, 4.0),
                );
                msg_send![layer, setFrame:frame];
                msg_send![layer, setContentsScale:2.0]; // Retina 支持

                layer
            };

            self.dot_layers.push(dot_layer);
            unsafe { msg_send![layer, addSublayer:dot_layer] };
        }

        // 创建文字图层
        self.text_layer = unsafe {
            let text_layer: id = msg_send![CATextLayer::class(), layer];
            let text = NSString::alloc(nil).init_str("正在听");
            msg_send![text_layer, setString:text];
            msg_send![text_layer, setFontSize:13.0];
            msg_send![text_layer, setForegroundColor:CGColorCreateGenericRGB(1.0, 1.0, 1.0, 1.0)];

            let frame = CGRect::new(
                &CGPoint::new(35.0, 0.0),
                &CGSize::new(CAPSULE_WIDTH - 40.0, CAPSULE_HEIGHT),
            );
            msg_send![text_layer, setFrame:frame];
            msg_send![text_layer, setAlignmentMode:kCAAlignmentLeft];
            msg_send![text_layer, setContentsScale:2.0];

            text_layer
        };

        unsafe { msg_send![layer, addSublayer:self.text_layer] };

        // 启动点动画
        self.start_dot_animation()?;

        Ok(())
    }

    fn start_dot_animation(&self) -> anyhow::Result<()> {
        for (i, dot_layer) in self.dot_layers.iter().enumerate() {
            unsafe {
                // 创建透明度动画
                let opacity_anim: id = msg_send![CABasicAnimation::class(), animationWithKeyPath:
                    NSString::alloc(nil).init_str("opacity")];

                msg_send![opacity_anim, setFromValue:0.3_f64];
                msg_send![opacity_anim, setToValue:1.0_f64];
                msg_send![opacity_anim, setDuration:0.6_f64];
                msg_send![opacity_anim, setAutoreverses:YES];
                msg_send![opacity_anim, setRepeatCount:f64::INFINITY];

                // 错开动画相位
                let delay = i as f64 * 0.2;
                msg_send![opacity_anim, setBeginTime:CACurrentMediaTime() + delay];

                // 创建缩放动画
                let scale_anim: id = msg_send![CABasicAnimation::class(), animationWithKeyPath:
                    NSString::alloc(nil).init_str("transform.scale")];
                msg_send![scale_anim, setFromValue:0.8_f64];
                msg_send![scale_anim, setToValue:1.0_f64];
                msg_send![scale_anim, setDuration:0.6_f64];
                msg_send![scale_anim, setAutoreverses:YES];
                msg_send![scale_anim, setRepeatCount:f64::INFINITY];
                msg_send![scale_anim, setBeginTime:CACurrentMediaTime() + delay];

                msg_send![*dot_layer, addAnimation:opacity_anim forKey:@"opacity"];
                msg_send![*dot_layer, addAnimation:scale_anim forKey:@"scale"];
            }
        }

        Ok(())
    }

    pub fn show(&self, x: f64, y: f64) -> anyhow::Result<()> {
        let frame = NSRect::new(
            NSPoint::new(x, y),
            NSSize::new(CAPSULE_WIDTH, CAPSULE_HEIGHT),
        );

        unsafe {
            self.window.setFrame_display_(frame, YES);
            self.window.makeKeyAndOrderFront_(nil);
        }

        Ok(())
    }

    pub fn hide(&self) -> anyhow::Result<()> {
        unsafe {
            self.window.orderOut_(nil);
        }
        Ok(())
    }

    pub fn set_state(&mut self, state: CapsuleState) -> anyhow::Result<()> {
        let text = match state {
            CapsuleState::Recording => "正在听",
            CapsuleState::Processing => "思考中",
            CapsuleState::Success => "✓ 已复制",
            CapsuleState::NoAudio => "未检测到声音",
            CapsuleState::Error(_) => "识别失败",
        };

        unsafe {
            let ns_text = NSString::alloc(nil).init_str(text);
            msg_send![self.text_layer, setString:ns_text];
        }

        Ok(())
    }

    fn get_screen_size() -> (f64, f64) {
        unsafe {
            let screen: id = msg_send![NSScreen::class(), mainScreen];
            let frame: NSRect = msg_send![screen, frame];
            (frame.size.width, frame.size.height)
        }
    }

    fn get_menu_bar_height() -> f64 {
        unsafe {
            let screen: id = msg_send![NSScreen::class(), mainScreen];
            let frame: NSRect = msg_send![screen, frame];
            let visible_frame: NSRect = msg_send![screen, visibleFrame];
            frame.size.height - visible_frame.size.height
        }
    }
}
```

---

## 三、统一 Rust 接口

```rust
// crates/floating/src/lib.rs

pub enum CapsuleState {
    Idle,
    Recording,
    Processing,
    Success,
    NoAudio,
    Error(String),
}

pub trait CapsuleWindow {
    fn new() -> anyhow::Result<Self>;
    fn show(&self, x: i32, y: i32) -> anyhow::Result<()>;
    fn hide(&self) -> anyhow::Result<()>;
    fn set_state(&mut self, state: CapsuleState) -> anyhow::Result<()>;
    fn set_position(&self, x: i32, y: i32) -> anyhow::Result<()>;
}

// 平台特定导出
#[cfg(target_os = "windows")]
pub use platform::windows::WindowsCapsule as NativeCapsule;

#[cfg(target_os = "macos")]
pub use platform::macos::MacOSCapsule as NativeCapsule;

// 使用示例
pub fn create_capsule() -> anyhow::Result<NativeCapsule> {
    NativeCapsule::new()
}
```

---

## 四、Cargo.toml 配置

```toml
[package]
name = "echovoice-floating"
version = "0.1.0"
edition = "2021"

[dependencies]
anyhow = "1.0"

[target.'cfg(target_os = "windows")'.dependencies]
windows = { version = "0.52", features = [
    "Win32_Foundation",
    "Win32_Graphics_Direct2D",
    "Win32_Graphics_DirectWrite",
    "Win32_Graphics_Gdi",
    "Win32_UI_WindowsAndMessaging",
    "Win32_System_LibraryLoader",
    "Win32_System_Threading",
] }

[target.'cfg(target_os = "macos")'.dependencies]
cocoa = "0.25"
core-graphics = "0.23"
objc = "0.2"
core-animation = "0.1"
```

---

## 依赖安装

### Windows
需要 Windows SDK 和 DirectX SDK（通常已包含在 Windows SDK 中）

### macOS
```bash
# 安装 Xcode Command Line Tools
xcode-select --install

# 确保有 Cocoa 框架
# macOS 系统自带
```

---

*文档版本: 1.0.0*
*技术方案: Windows Win32 + Direct2D, macOS Cocoa + Core Animation*
*目标: 100% 复刻微信输入法胶囊效果*