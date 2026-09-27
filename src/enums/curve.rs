//! 曲线枚举

// =============================================================
// cdrNodeType —— 节点类型
// =============================================================

/// `Node.Type` 的取值。
///
/// 来源：`VGCore::cdrNodeType`
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum cdrNodeType {
    Cusp        = 0,
    Smooth      = 1,
    Symmetrical = 2,
    Mixed       = 3,
}

impl cdrNodeType {
    pub fn from_i32(v: i32) -> Option<Self> {
        match v {
            0 => Some(Self::Cusp),
            1 => Some(Self::Smooth),
            2 => Some(Self::Symmetrical),
            3 => Some(Self::Mixed),
            _ => None,
        }
    }
}

// =============================================================
// cdrSegmentType —— 线段类型
// =============================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum cdrSegmentType {
    Line   = 1,
    Curve  = 2,
    // TODO: 待补
}

// =============================================================
// cdrSegmentOffsetType —— 线段偏移类型
// =============================================================

/// 用于 `GetPointAt` / `AddNodeAt` 等方法的 OffsetType 参数。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum cdrSegmentOffsetType {
    Absolute = 0,
    Relative = 1,
}

// =============================================================
// cdrWeldMethod —— 焊接方法
// =============================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum cdrWeldMethod {
    // TODO: 待补
    Union = 0,
}