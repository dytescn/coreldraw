//! 页面枚举

// =============================================================
// cdrPageOrientation —— 页面方向
// =============================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum cdrPageOrientation {
    Portrait  = 0,
    Landscape = 1,
}

impl cdrPageOrientation {
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum cdrPageBackground {
    NoBackground        = 0,
    SolidBackground     = 1,
    BitmapBackground    = 2,
    // TODO: 待补
}

// =============================================================
// cdrUnit —— 单位
// =============================================================

/// 文档单位。`ActiveDocument.Unit` 的取值。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum cdrUnit {
    /// 英寸。
    Inch       = 0,
    /// 毫米。
    Millimeter = 1,
    /// 点（pica）。
    Pica       = 2,
    /// 磅（point）。
    Point      = 3,
    /// 厘米。
    Centimeter = 4,
    /// 像素。
    Pixel      = 5,
    /// 西塞罗（cicero）。
    Cicero     = 6,
    /// 迪多点（didot）。
    Didot      = 7,
    /// 英尺。
    Foot       = 8,
    /// 码。
    Yard       = 9,
    /// 米。
    Meter      = 10,
    /// 千米。
    Kilometer  = 11,
    /// 英里。
    Mile       = 12,
    /// 派卡（pica，与 Pica 区分）。
    Pica2      = 13,
    /// 阿加特（agate）。
    Agate      = 14,
    // TODO: 待补
}

impl cdrUnit {
    pub fn from_i32(v: i32) -> Option<Self> {
        match v {
            0  => Some(Self::Inch),
            1  => Some(Self::Millimeter),
            2  => Some(Self::Pica),
            3  => Some(Self::Point),
            4  => Some(Self::Centimeter),
            5  => Some(Self::Pixel),
            6  => Some(Self::Cicero),
            7  => Some(Self::Didot),
            8  => Some(Self::Foot),
            9  => Some(Self::Yard),
            10 => Some(Self::Meter),
            11 => Some(Self::Kilometer),
            12 => Some(Self::Mile),
            13 => Some(Self::Pica2),
            14 => Some(Self::Agate),
            _  => None,
        }
    }
}

// =============================================================
// cdrGuideType —— 参考线类型
// =============================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum cdrGuideType {
    AllGuides       = -1,
    HorizontalGuide = 0,
    VerticalGuide   = 1,
    SlantedGuide    = 2,
}

impl cdrGuideType {
    pub fn from_i32(v: i32) -> Option<Self> {
        match v {
            -1 => Some(Self::AllGuides),
            0  => Some(Self::HorizontalGuide),
            1  => Some(Self::VerticalGuide),
            2  => Some(Self::SlantedGuide),
            _  => None,
        }
    }
}

// =============================================================
// cdrGridType —— 网格类型
// =============================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum cdrGridType {
    Rectangular = 0,
    Polar       = 1,
    // TODO: 待补
}

// =============================================================
// cdrSpreadType —— 跨页类型
// =============================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum cdrSpreadType {
    // TODO: 待补
    Left  = 0,
    Right = 1,
    Full  = 2,
}