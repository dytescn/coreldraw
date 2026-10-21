//! UI 框架枚举（ICUI* 系列）

// =============================================================
// cuiBarPosition —— 工具栏位置
// =============================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum cuiBarPosition {
    BarLeft = 0,
    BarTop = 1,
    BarRight = 2,
    BarBottom = 3,
    BarFloating = 4,
}

// =============================================================
// cuiBarType —— 工具栏类型
// =============================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum cuiBarType {
    BarTypeNormal = 0,
    BarTypeMenuBar = 1,
    BarTypePopup = 2,
    BarTypeStatusBar = 3,
    BarTypePropertyBar = 4,
}

// =============================================================
// cuiBarProtection —— 工具栏保护
// =============================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum cuiBarProtection {
    BarNoProtection = 0,
    BarNoCustomize = 1,
    BarNoMove = 4,
    BarNoChangeVisible = 8,
    BarNoChangeDock = 16,
    BarNoVerticalDock = 32,
    BarNoHorizontalDock = 64,
}

// =============================================================
// cuiWindowState —— UI 窗口状态
// =============================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum cuiWindowState {
    WindowStateRestored = 0,
    WindowStateMaximized = 1,
    WindowStateMinimized = 2,
}

// =============================================================
// cuiDockHostOrientation —— 停靠方向
// =============================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum cuiDockHostOrientation {
    DockHostHorizontal = 0,
    DockHostVertical = 1,
}

// =============================================================
// cuiDockOperation —— 停靠操作
// =============================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum cuiDockOperation {
    DockOperationInsert = 0,
    DockOperationSplitTopLeft = 1,
    DockOperationSplitBottomRight = 2,
}

// =============================================================
// cuiDockItemType —— 停靠项类型
// =============================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum cuiDockItemType {
    DockItemDockHost = 0,
    DockItemViewHost = 1,
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
    Default = 0,
    NoAutoWrap = 1,
}

// =============================================================
// cuiTaskPriority —— 任务优先级
// =============================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum cuiTaskPriority {
    Low = 0,
    Normal = 1,
    High = 2,
}
