//! macOS 悬浮胶囊实现
//!
//! 使用 NSWindow + NSVisualEffectView 实现毛玻璃效果

use crate::{CapsuleState, CapsuleWindow, FloatingError, CAPSULE_HEIGHT, CAPSULE_WIDTH};
use cocoa::base::{id, nil, NO, YES};
use cocoa::foundation::{NSAutoreleasePool, NSPoint, NSRect, NSSize, NSString};
use objc::runtime::Class;

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

const DOT_SIZE: f64 = 4.0;
const DOT_SPACING: f64 = 5.0;
const DOT_BASE_X: f64 = 14.0;

pub struct MacOSCapsule {
    window: id,
    dot_views: Vec<id>,
    text_label: id,
    state: CapsuleState,
    animation_running: Arc<AtomicBool>,
}

impl MacOSCapsule {
    pub fn new() -> Result<Self, FloatingError> {
        let pool = unsafe { NSAutoreleasePool::new(nil) };

        unsafe {
            // 获取 NSPanel 类
            let panel_class = Class::get("NSPanel").unwrap_or(Class::get("NSWindow").unwrap());

            // 创建窗口
            let frame = NSRect::new(
                NSPoint::new(0.0, 0.0),
                NSSize::new(CAPSULE_WIDTH as f64, CAPSULE_HEIGHT as f64),
            );

            // 创建窗口（使用 NSNonactivatingPanelMask = 1 << 7 = 128）
            let window: id = msg_send![
                panel_class,
                alloc
            ];

            let window: id = msg_send![
                window,
                initWithContentRect:frame
                styleMask:128u64 // NSNonactivatingPanelMask
                backing:2u64 // NSBackingStoreBuffered
                defer:NO
            ];

            if window == nil {
                return Err(FloatingError::WindowCreationFailed(
                    "Failed to create NSWindow".to_string(),
                ));
            }

            // 窗口配置
            let empty_string = NSString::alloc(nil).init_str("");
            let _: () = msg_send![window, setTitle:empty_string];
            let _: () = msg_send![window, setLevel:8i64]; // NSFloatingWindowLevel
            let _: () = msg_send![window, setHasShadow:NO];
            let _: () = msg_send![window, setOpaque:NO];
            let _: () = msg_send![window, setMovable:NO];
            let _: () = msg_send![window, setCollectionBehavior:1u64 << 0u64 | 1u64 << 3u64 | 1u64 << 4u64];

            // 创建 Visual Effect View
            let visual_effect_class = Class::get("NSVisualEffectView").unwrap();
            let visual_effect: id = msg_send![visual_effect_class, alloc];
            let _: () = msg_send![visual_effect, initWithFrame:frame];
            let _: () = msg_send![visual_effect, setMaterial:3u64]; // HUDWindow
            let _: () = msg_send![visual_effect, setBlendingMode:1u64]; // BehindWindow
            let _: () = msg_send![visual_effect, setState:1u64]; // Active
            let _: () = msg_send![visual_effect, setWantsLayer:YES];

            // 创建内容视图
            let view_class = Class::get("NSView").unwrap();
            let content_view: id = msg_send![view_class, alloc];
            let _: () = msg_send![content_view, initWithFrame:frame];
            let _: () = msg_send![content_view, setWantsLayer:YES];

            // 设置圆角
            let layer: id = msg_send![content_view, layer];
            let _: () = msg_send![layer, setCornerRadius:16.0];
            let _: () = msg_send![layer, setMasksToBounds:YES];

            // 设置深色背景
            let color_class = Class::get("NSColor").unwrap();
            let bg_color: id = msg_send![
                color_class,
                colorWithCalibratedRed:0.157
                green:0.157
                blue:0.157
                alpha:0.9
            ];
            let _: () = msg_send![content_view, setBackgroundColor:bg_color];

            // 添加内容视图
            let _: () = msg_send![visual_effect, addSubview:content_view];
            let _: () = msg_send![window, setContentView:visual_effect];

            // 创建 UI
            let (dot_views, text_label) = Self::setup_ui(content_view)?;

            pool.drain();

            Ok(Self {
                window,
                dot_views,
                text_label,
                state: CapsuleState::Idle,
                animation_running: Arc::new(AtomicBool::new(false)),
            })
        }
    }

    fn setup_ui(content_view: id) -> Result<(Vec<id>, id), FloatingError> {
        unsafe {
            let mut dot_views = Vec::new();
            let color_class = Class::get("NSColor").unwrap();

            // 创建三个白点
            for i in 0..3 {
                let dot_class = Class::get("NSView").unwrap();
                let dot: id = msg_send![dot_class, alloc];

                let x = DOT_BASE_X + i as f64 * DOT_SPACING;
                let y = (CAPSULE_HEIGHT as f64 - DOT_SIZE) / 2.0;
                let frame = NSRect::new(
                    NSPoint::new(x, y),
                    NSSize::new(DOT_SIZE, DOT_SIZE),
                );

                let _: () = msg_send![dot, initWithFrame:frame];
                let _: () = msg_send![dot, setWantsLayer:YES];

                let layer: id = msg_send![dot, layer];
                let _: () = msg_send![layer, setCornerRadius:DOT_SIZE / 2.0];
                let _: () = msg_send![layer, setMasksToBounds:YES];

                // 白色背景
                let white_color: id = msg_send![color_class, whiteColor];
                let _: () = msg_send![layer, setBackgroundColor:white_color];
                let _: () = msg_send![layer, setOpacity:0.3f32];

                let _: () = msg_send![content_view, addSubview:dot];
                dot_views.push(dot);
            }

            // 创建文字标签
            let label_class = Class::get("NSTextField").unwrap();
            let label: id = msg_send![label_class, alloc];

            let x = 35.0;
            let y = (CAPSULE_HEIGHT as f64 - 20.0) / 2.0; // 垂直居中
            let width = CAPSULE_WIDTH as f64 - x - 10.0;
            let height = 20.0;

            let frame = NSRect::new(
                NSPoint::new(x, y),
                NSSize::new(width, height),
            );

            let label: id = msg_send![label, initWithFrame:frame];

            let initial_text = NSString::alloc(nil).init_str("正在听");
            let _: () = msg_send![label, setStringValue:initial_text];

            let white_color: id = msg_send![color_class, whiteColor];
            let _: () = msg_send![label, setTextColor:white_color];

            let font_class = Class::get("NSFont").unwrap();
            let font: id = msg_send![font_class, systemFontOfSize:13.0];
            let _: () = msg_send![label, setFont:font];

            let clear_color: id = msg_send![color_class, clearColor];
            let _: () = msg_send![label, setBackgroundColor:clear_color];
            let _: () = msg_send![label, setBordered:NO];
            let _: () = msg_send![label, setEditable:NO];
            let _: () = msg_send![label, setSelectable:NO];
            let _: () = msg_send![label, setAlignment:1u64]; // Center

            let _: () = msg_send![content_view, addSubview:label];

            Ok((dot_views, label))
        }
    }

    fn start_animation(&self) {
        if self.animation_running.load(Ordering::SeqCst) {
            return;
        }
        self.animation_running.store(true, Ordering::SeqCst);

        // TODO: 使用 NSTimer 或 CADisplayLink 实现线程安全的动画
        // 目前暂时使用简单实现
    }

    fn stop_animation(&self) {
        self.animation_running.store(false, Ordering::SeqCst);
    }
}

impl CapsuleWindow for MacOSCapsule {
    fn new() -> Result<Self, FloatingError>
    where
        Self: Sized,
    {
        Self::new()
    }

    fn show(&self, x: i32, y: i32) -> Result<(), FloatingError> {
        unsafe {
            let frame = NSRect::new(
                NSPoint::new(x as f64, y as f64),
                NSSize::new(CAPSULE_WIDTH as f64, CAPSULE_HEIGHT as f64),
            );

            let _: () = msg_send![self.window, setFrame:frame display:YES];
            let _: () = msg_send![self.window, makeKeyAndOrderFront:nil];

            // 启动动画
            self.start_animation();
        }

        Ok(())
    }

    fn hide(&self) -> Result<(), FloatingError> {
        unsafe {
            let _: () = msg_send![self.window, orderOut:nil];
        }

        self.stop_animation();
        Ok(())
    }

    fn set_state(&mut self, state: CapsuleState) -> Result<(), FloatingError> {
        let text = match state {
            CapsuleState::Recording => "正在听",
            CapsuleState::Processing => "思考中",
            CapsuleState::Success => "✓ 已复制",
            CapsuleState::NoAudio => "未检测到声音",
            CapsuleState::Error(_) => "识别失败",
            CapsuleState::Idle => "",
        };

        unsafe {
            let ns_text = NSString::alloc(nil).init_str(text);
            let _: () = msg_send![self.text_label, setStringValue:ns_text];
        }

        self.state = state;
        Ok(())
    }

    fn update_waveform(&mut self, _levels: &[f32]) -> Result<(), FloatingError> {
        Ok(())
    }

    fn run_loop(&mut self) -> Result<(), FloatingError> {
        Ok(())
    }

    fn close(&mut self) -> Result<(), FloatingError> {
        self.stop_animation();
        unsafe {
            let _: () = msg_send![self.window, close];
        }
        Ok(())
    }
}

/// 获取屏幕尺寸
pub fn get_screen_size() -> (i32, i32) {
    unsafe {
        let screen_class = Class::get("NSScreen").unwrap();
        let screen: id = msg_send![screen_class, mainScreen];
        let frame: NSRect = msg_send![screen, frame];
        (frame.size.width as i32, frame.size.height as i32)
    }
}

/// 获取菜单栏高度
pub fn get_menu_bar_height() -> i32 {
    unsafe {
        let screen_class = Class::get("NSScreen").unwrap();
        let screen: id = msg_send![screen_class, mainScreen];
        let frame: NSRect = msg_send![screen, frame];
        let visible_frame: NSRect = msg_send![screen, visibleFrame];
        (frame.size.height - visible_frame.size.height) as i32
    }
}
