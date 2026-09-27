//! UI 框架枚举（ICUI* 系列）

// =============================================================
// cuiBarPosition —— 工具栏位置
// =============================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum cuiBarPosition {
    Floating = 0,
    Top      = 1,
    Bottom   = 2,
    Left     = 3,
    Right    = 4,
    // TODO: 待补
}

// =============================================================
// cuiBarType —— 工具栏类型
// =============================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum cuiBarType {
    MenuBar   = 0,
    ToolBar   = 1,
    StatusBar = 2,
    // TODO: 待补
}

// =============================================================
// cuiBarProtection —— 工具栏保护
// =============================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum cuiBarProtection {
    None        = 0,
    NoCustomize = 1,
    NoMove      = 2,
    NoResize    = 3,
    // TODO: 待补
}

// =============================================================
// cuiWindowState —— UI 窗口状态
// =============================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum cuiWindowState {
    Normal    = 0,
    Minimized = 1,
    Maximized = 2,
    // TODO: 待补
}

// =============================================================
// cuiDockHostOrientation —— 停靠方向
// =============================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum cuiDockHostOrientation {
    Horizontal = 0,
    Vertical   = 1,
    // TODO: 待补
}

// =============================================================
// cuiDockOperation —— 停靠操作
// =============================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum cuiDockOperation {
    /// 未知 / 未定义。
    Unknown = -1,
    // TODO: 完整值待补
}

// =============================================================
// cuiDockItemType —— 停靠项类型
// =============================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum cuiDockItemType {
    /// 未知 / 未定义。
    Unknown = -1,
    // TODO: 完整值待补
}

// =============================================================
// cuiMessageBoxFlags —— 消息框标志
// =============================================================

/// ⚠️ 这是**位标志**（可以用 `|` 组合），不是普通枚举。
/// 目前以占位变体让编译通过；等确定实际值后再细分。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum cuiMessageBoxFlags {
    /// 无标志（默认）。
    None    = 0,
    /// 未知 / 未定义（占位）。
    Unknown = -1,
    // TODO: 完整标志待补（如 MB_OK / MB_YESNO / MB_ICONWARNING ...）
}

// =============================================================
// cuiTaskPriority —— 任务优先级
// =============================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]

#[allow(nonstandard_style)]
pub enum cuiTaskPriority {
    Low    = 0,
    Normal = 1,
    High   = 2,
}