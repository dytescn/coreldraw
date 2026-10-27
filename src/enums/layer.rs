//! 图层枚举

// =============================================================
// cdrLayerVisibility —— 图层可见性
// =============================================================

/// CorelDRAW 枚举 `cdrLayerVisibility`：图层可见性。
///
/// 对应类型库中的 `VGCore::cdrLayerVisibility`，用 `#[repr(i32)]` 映射实际整数值。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum cdrLayerVisibility {
    /// 可见性。
    Visible = 0,
    /// hidden。
    Hidden = 1,
    // TODO: 待补
}

// =============================================================
// cdrLayerType —— 图层类型
// =============================================================

/// CorelDRAW 枚举 `cdrLayerType`：图层类型。
///
/// 对应类型库中的 `VGCore::cdrLayerType`，用 `#[repr(i32)]` 映射实际整数值。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum cdrLayerType {
    /// 常规。
    Normal = 0,
    /// 参考线集。
    Guides = 1,
    /// 桌面。
    Desktop = 2,
    /// 网格。
    Grid = 3,
    /// 主。
    Master = 4,
}
