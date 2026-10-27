//! 页面枚举

// =============================================================
// cdrPageOrientation —— 页面方向
// =============================================================

/// CorelDRAW 枚举 `cdrPageOrientation`：页面方向。
///
/// 对应类型库中的 `VGCore::cdrPageOrientation`，用 `#[repr(i32)]` 映射实际整数值。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum cdrPageOrientation {
    /// portrait。
    Portrait = 0,
    /// landscape。
    Landscape = 1,
}

impl cdrPageOrientation {
    /// 由整数值构造枚举。
    pub fn from_i32(v: i32) -> Option<Self> {
        match v {
            0 => Some(Self::Portrait),
            1 => Some(Self::Landscape),
            _ => None,
        }
    }
}

// =============================================================
// cdrPageBackground —— 页面背景
// =============================================================

/// CorelDRAW 枚举 `cdrPageBackground`：页面背景。
///
/// 对应类型库中的 `VGCore::cdrPageBackground`，用 `#[repr(i32)]` 映射实际整数值。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum cdrPageBackground {
    /// 页面背景无。
    PageBackgroundNone = 0,
    /// 页面背景 solid。
    PageBackgroundSolid = 1,
    /// 页面背景位图。
    PageBackgroundBitmap = 2,
}

// =============================================================
// cdrUnit —— 单位
// =============================================================

/// 文档单位。`ActiveDocument.Unit` 的取值。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum cdrUnit {
    /// tenthmicron。
    TenthMicron = 0,
    /// 英寸。
    Inch = 1,
    /// 英尺。
    Foot = 2,
    /// 毫米。
    Millimeter = 3,
    /// 厘米。
    Centimeter = 4,
    /// 像素。
    Pixel = 5,
    /// 英里。
    Mile = 6,
    /// 米。
    Meter = 7,
    /// 千米。
    Kilometer = 8,
    /// didots。
    Didots = 9,
    /// 阿加特（agate）。
    Agate = 10,
    /// 码。
    Yard = 11,
    /// 点（pica）。
    Pica = 12,
    /// 西塞罗（cicero）。
    Cicero = 13,
    /// 磅（point）。
    Point = 14,
    /// 单位 q。
    UnitQ = 15,
    /// 单位高。
    UnitH = 16,
}

impl cdrUnit {
    /// 由整数值构造枚举。
    pub fn from_i32(v: i32) -> Option<Self> {
        match v {
            0 => Some(Self::TenthMicron),
            1 => Some(Self::Inch),
            2 => Some(Self::Foot),
            3 => Some(Self::Millimeter),
            4 => Some(Self::Centimeter),
            5 => Some(Self::Pixel),
            6 => Some(Self::Mile),
            7 => Some(Self::Meter),
            8 => Some(Self::Kilometer),
            9 => Some(Self::Didots),
            10 => Some(Self::Agate),
            11 => Some(Self::Yard),
            12 => Some(Self::Pica),
            13 => Some(Self::Cicero),
            14 => Some(Self::Point),
            15 => Some(Self::UnitQ),
            16 => Some(Self::UnitH),
            _ => None,
        }
    }
}

// =============================================================
// cdrGuideType —— 参考线类型
// =============================================================

/// CorelDRAW 枚举 `cdrGuideType`：参考线类型。
///
/// 对应类型库中的 `VGCore::cdrGuideType`，用 `#[repr(i32)]` 映射实际整数值。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum cdrGuideType {
    /// 全部参考线集。
    AllGuides = -1,
    /// 水平参考线。
    HorizontalGuide = 0,
    /// 垂直参考线。
    VerticalGuide = 1,
    /// slanted 参考线。
    SlantedGuide = 2,
}

impl cdrGuideType {
    /// 由整数值构造枚举。
    pub fn from_i32(v: i32) -> Option<Self> {
        match v {
            -1 => Some(Self::AllGuides),
            0 => Some(Self::HorizontalGuide),
            1 => Some(Self::VerticalGuide),
            2 => Some(Self::SlantedGuide),
            _ => None,
        }
    }
}

// =============================================================
// cdrGridType —— 网格类型
// =============================================================

/// CorelDRAW 枚举 `cdrGridType`：网格类型。
///
/// 对应类型库中的 `VGCore::cdrGridType`，用 `#[repr(i32)]` 映射实际整数值。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum cdrGridType {
    /// 网格点。
    GridDot = 0,
    /// 网格线。
    GridLine = 1,
}

// =============================================================
// cdrSpreadType —— 跨页类型
// =============================================================

/// CorelDRAW 枚举 `cdrSpreadType`：跨页类型。
///
/// 对应类型库中的 `VGCore::cdrSpreadType`，用 `#[repr(i32)]` 映射实际整数值。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum cdrSpreadType {
    // TODO: 待补
    /// 左。
    Left = 0,
    /// 右。
    Right = 1,
    /// 完整。
    Full = 2,
}
