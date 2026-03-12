# Floating Capsule 模块设计文档

## 模块职责

提供录音时的可视化反馈，在任务栏上方显示胶囊形悬浮窗，参考微信输入法的设计风格。

---

## 设计目标

1. **极简视觉** - 深色细长胶囊，不抢占注意力
2. **位置固定** - 任务栏正上方，水平居中
3. **即时反馈** - 按下显示，释放消失
4. **状态清晰** - 动画 + 文字双重指示

---

## 视觉设计（参考微信输入法）

### 胶囊形态

```
    ┌───────────────────┐
    │ •••  正在听       │  ← 高度: 32px，圆角: 16px
    └───────────────────┘
      ↑ 三个点跳动动画
```

```
    ┌───────────────────┐
    │ •••  思考中       │  ← 处理中状态
    └───────────────────┘
```

### 位置

- **水平居中**：屏幕中央
- **垂直位置**：任务栏正上方（紧贴，无间隙）
- **层级**：置顶，但不获取焦点

### 配色（参考微信输入法）

| 元素 | 值 |
|------|-----|
| 背景 | `rgba(40, 40, 40, 0.9)` 深灰半透明 |
| 文字 | `#FFFFFF` 纯白 |
| 动画点 | `#FFFFFF` 纯白 |

### 尺寸

| 属性 | 值 |
|------|-----|
| 宽度 | 110-130px（根据文字自适应） |
| 高度 | 32px |
| 圆角 | 16px（半圆角） |
| 字体 | 13px，系统字体（-apple-system, Segoe UI） |
| 内边距 | 0 14px |

### 动画效果

**录音中 / 正在听**：
- 三个白点水平排列
- 依次淡入淡出，形成波浪效果
- 动画周期：1.2s

**处理中 / 思考中**：
- 三个白点依次出现（• → •• → ••• → 循环）
- 或旋转加载动画

---

## 状态流转

```
┌──────────┐    按下热键    ┌──────────────┐
│          │  ──────────▶   │              │
│   隐藏   │                 │ ••• 正在听   │
│          │  ◀──────────   │   (显示)     │
└──────────┘    释放热键    └──────────────┘
                                     │
                                     ▼ 停止录音
                              ┌──────────────┐
                              │ ••• 思考中   │
                              │   (显示)     │
                              └──────────────┘
                                     │
                                     ▼ 识别完成
                              ┌──────────────┐
                              │  ✓ 已复制    │ ──▶ 1.5秒后
                              │   (显示)     │     隐藏
                              └──────────────┘
```

---

## 功能需求

### 1. 显示/隐藏

| 操作 | 行为 |
|------|------|
| 按下热键 | 立即显示，显示"正在听" |
| 释放热键 | 切换为"思考中"，识别完成后切换为"已复制" |
| 识别完成 | 显示"✓ 已复制"，1.5秒后淡出隐藏 |
| 无音频 | 显示"未检测到声音"（红色背景），2秒后隐藏 |

### 2. 动画

**三个点跳动动画**：
```css
@keyframes dot-pulse {
  0%, 100% { opacity: 0.3; transform: scale(0.8); }
  50% { opacity: 1; transform: scale(1); }
}

.dot:nth-child(1) { animation: dot-pulse 1.2s 0s infinite; }
.dot:nth-child(2) { animation: dot-pulse 1.2s 0.2s infinite; }
.dot:nth-child(3) { animation: dot-pulse 1.2s 0.4s infinite; }
```

### 3. 状态文字

| 状态 | 显示 |
|------|------|
| 录音中 | `••• 正在听` |
| 处理中 | `••• 思考中` |
| 成功 | `✓ 已复制` |
| 无音频 | `未检测到声音`（红色背景） |
| 失败 | `识别失败`（红色背景） |

---

## 技术实现

### HTML 结构

```html
<div class="capsule">
  <div class="dots">
    <span class="dot"></span>
    <span class="dot"></span>
    <span class="dot"></span>
  </div>
  <span class="text">正在听</span>
</div>
```

### CSS 样式

```css
.capsule {
  width: auto;
  min-width: 100px;
  height: 32px;
  padding: 0 14px;
  border-radius: 16px;
  background: rgba(40, 40, 40, 0.9);
  backdrop-filter: blur(10px);

  display: flex;
  align-items: center;
  justify-content: center;
  gap: 8px;

  /* 禁止任何交互 */
  pointer-events: none;
  user-select: none;
  -webkit-app-region: no-drag;
}

.dots {
  display: flex;
  gap: 3px;
  align-items: center;
}

.dot {
  width: 4px;
  height: 4px;
  border-radius: 50%;
  background: white;
}

.text {
  color: white;
  font-size: 13px;
  font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", sans-serif;
}
```

### 窗口配置

```rust
let window = WebviewWindowBuilder::new(
    app,
    "capsule",
    tauri::WebviewUrl::App("capsule.html".into())
)
.title("")              // 空标题
.decorations(false)     // 无边框
.transparent(true)      // 透明背景
.always_on_top(true)    // 置顶
.skip_taskbar(true)     // 不显示在任务栏
.resizable(false)       // 不可调整
.visible(false)         // 初始隐藏
.focused(false)         // 不获取焦点
.no_user_saved_state(true)
.inner_size(120.0, 32.0)
.build()?;
```

### 位置计算（紧贴任务栏上方）

```rust
fn get_capsule_position() -> (i32, i32) {
    let monitor = primary_monitor()?;
    let work_area = monitor.work_area();  // 排除任务栏的工作区

    // 胶囊宽度自适应，但固定高度 32px
    let capsule_width = 120;  // 或根据内容计算
    let capsule_height = 32;

    let x = (work_area.width - capsule_width) / 2;
    // 紧贴任务栏顶部（work_area 底部就是任务栏顶部）
    let y = work_area.y + work_area.height - capsule_height;

    (x, y)
}
```

---

## 依赖

```toml
[dependencies]
tauri = { version = "2", features = ["window-all"] }
```

---

## 关联文档

- [Hotkey 模块](../hotkey/design.md) - 触发控制
- [Audio 模块](../audio/design.md) - 音频数据获取
- [接口定义](./interface.md)

---

*文档版本: 0.2.0*
*参考: 微信输入法悬浮胶囊设计*
*最后更新: 2026-03-12*