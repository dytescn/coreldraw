//! UI 框架枚举（ICUI* 系列）

// =============================================================
// cuiBarPosition —— 工具栏位置
// =============================================================

/// CorelDRAW 枚举 `cuiBarPosition`：工具栏位置。
///
/// 对应类型库中的 `VGCore::cuiBarPosition`，用 `#[repr(i32)]` 映射实际整数值。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum cuiBarPosition {
    /// 左栏。
    BarLeft = 0,
    /// 顶栏。
    BarTop = 1,
    /// 右栏。
    BarRight = 2,
    /// 底栏。
    BarBottom = 3,
    /// 栏 floating。
    BarFloating = 4,
}

// =============================================================
// cuiBarType —— 工具栏类型
// =============================================================

/// CorelDRAW 枚举 `cuiBarType`：工具栏类型。
///
/// 对应类型库中的 `VGCore::cuiBarType`，用 `#[repr(i32)]` 映射实际整数值。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum cuiBarType {
    /// 栏类型常规。
    BarTypeNormal = 0,
    /// 栏类型菜单栏。
    BarTypeMenuBar = 1,
    /// 栏类型 popup。
    BarTypePopup = 2,
    /// 栏类型状态栏。
    BarTypeStatusBar = 3,
    /// 栏类型属性栏。
    BarTypePropertyBar = 4,
}

// =============================================================
// cuiBarProtection —— 工具栏保护
// =============================================================

/// CorelDRAW 枚举 `cuiBarProtection`：工具栏保护。
///
/// 对应类型库中的 `VGCore::cuiBarProtection`，用 `#[repr(i32)]` 映射实际整数值。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum cuiBarProtection {
    /// 栏无 protection。
    BarNoProtection = 0,
    /// 栏无 customize。
    BarNoCustomize = 1,
    /// 栏无移动。
    BarNoMove = 4,
    /// 栏无更改可见性。
    BarNoChangeVisible = 8,
    /// 栏无更改停靠。
    BarNoChangeDock = 16,
    /// 栏无垂直停靠。
    BarNoVerticalDock = 32,
    /// 栏无水平停靠。
    BarNoHorizontalDock = 64,
}

// =============================================================
// cuiWindowState —— UI 窗口状态
// =============================================================

/// CorelDRAW 枚举 `cuiWindowState`：UI 窗口状态。
///
/// 对应类型库中的 `VGCore::cuiWindowState`，用 `#[repr(i32)]` 映射实际整数值。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum cuiWindowState {
    /// 窗口状态 restored。
    WindowStateRestored = 0,
    /// 窗口状态 maximized。
    WindowStateMaximized = 1,
    /// 窗口状态 minimized。
    WindowStateMinimized = 2,
}

// =============================================================
// cuiDockHostOrientation —— 停靠方向
// =============================================================

/// CorelDRAW 枚举 `cuiDockHostOrientation`：停靠方向。
///
/// 对应类型库中的 `VGCore::cuiDockHostOrientation`，用 `#[repr(i32)]` 映射实际整数值。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum cuiDockHostOrientation {
    /// 水平停靠宿主。
    DockHostHorizontal = 0,
    /// 垂直停靠宿主。
    DockHostVertical = 1,
}

// =============================================================
// cuiDockOperation —— 停靠操作
// =============================================================

/// CorelDRAW 枚举 `cuiDockOperation`：停靠操作。
///
/// 对应类型库中的 `VGCore::cuiDockOperation`，用 `#[repr(i32)]` 映射实际整数值。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum cuiDockOperation {
    /// 停靠 operationinsert。
    DockOperationInsert = 0,
    /// 左顶停靠 operation 拆分。
    DockOperationSplitTopLeft = 1,
    /// 右底停靠 operation 拆分。
    DockOperationSplitBottomRight = 2,
}

// =============================================================
// cuiDockItemType —— 停靠项类型
// =============================================================

/// CorelDRAW 枚举 `cuiDockItemType`：停靠项类型。
///
/// 对应类型库中的 `VGCore::cuiDockItemType`，用 `#[repr(i32)]` 映射实际整数值。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum cuiDockItemType {
    /// 停靠条目停靠宿主。
    DockItemDockHost = 0,
    /// 停靠条目视图宿主。
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
    /// 默认值。
    Default = 0,
    /// 无自动绕排。
    NoAutoWrap = 1,
}

// =============================================================
// cuiTaskPriority —— 任务优先级
// =============================================================

/// CorelDRAW 枚举 `cuiTaskPriority`：任务优先级。
///
/// 对应类型库中的 `VGCore::cuiTaskPriority`，用 `#[repr(i32)]` 映射实际整数值。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum cuiTaskPriority {
    /// low。
    Low = 0,
    /// 常规。
    Normal = 1,
    /// high。
    High = 2,
}
