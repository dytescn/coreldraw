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
    /// cusp。
    Cusp = 0,
    /// smooth。
    Smooth = 1,
    /// symmetrical。
    Symmetrical = 2,
    /// mixed。
    Mixed = 3,
}

impl cdrNodeType {
    /// 由整数值构造枚举。
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

/// CorelDRAW 枚举 `cdrSegmentType`：线段类型。
///
/// 对应类型库中的 `VGCore::cdrSegmentType`，用 `#[repr(i32)]` 映射实际整数值。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum cdrSegmentType {
    /// 线线段。
    LineSegment = 0,
    /// 曲线线段。
    CurveSegment = 1,
    /// mixed 线段集。
    MixedSegments = 2,
}

// =============================================================
// cdrSegmentOffsetType —— 线段偏移类型
// =============================================================

/// 用于 `GetPointAt` / `AddNodeAt` 等方法的 OffsetType 参数。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum cdrSegmentOffsetType {
    /// 绝对。
    Absolute = 0,
    /// 相对。
    Relative = 1,
}

// =============================================================
// cdrWeldMethod —— 焊接方法
// =============================================================

/// CorelDRAW 枚举 `cdrWeldMethod`：焊接方法。
///
/// 对应类型库中的 `VGCore::cdrWeldMethod`，用 `#[repr(i32)]` 映射实际整数值。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum cdrWeldMethod {
    /// weld 方法 union。
    WeldMethodUnion = 0,
    /// weld 方法 intersect。
    WeldMethodIntersect = 1,
    /// weld 方法 subtract。
    WeldMethodSubtract = 2,
    /// weld 方法 xor。
    WeldMethodXor = 3,
}
