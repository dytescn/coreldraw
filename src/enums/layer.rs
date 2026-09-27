//! 图层枚举

// =============================================================
// cdrLayerVisibility —— 图层可见性
// =============================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum cdrLayerVisibility {
    Visible   = 0,
    Hidden    = 1,
    // TODO: 待补
}

// =============================================================
// cdrLayerType —— 图层类型
// =============================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum cdrLayerType {
    Normal    = 0,
    Guides    = 1,
    Desktop   = 2,
    Grid      = 3,
    Master    = 4,
}