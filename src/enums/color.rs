//! 颜色枚举

// =============================================================
// cdrColorType —— 颜色模型类型
// =============================================================

/// `Color.Type` 的取值。
///
/// 来源：`VGCore::cdrColorType`
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum cdrColorType {
    /// 未定义。
    Undefined   = 0,
    /// PANTONE。
    Pantone     = 1,
    /// CMYK。
    CMYK        = 2,
    /// CMY。
    CMY         = 3,
    /// RGB。
    RGB         = 5,
    /// HSB。
    HSB         = 6,
    /// HLS。
    HLS         = 7,
    /// 黑白。
    BlackAndWhite = 8,
    /// 灰度。
    Grayscale   = 9,
    /// YIQ。
    YIQ         = 10,
    /// Lab。
    Lab         = 11,
    /// PANTONE Hexachrome。
    PantoneHex  = 12,
    /// 专色。
    Spot        = 13,
    /// 注册色。
    Registration = 14,
    /// 混合。
    Mixed       = 15,
    /// 自定义。
    Custom      = 16,
}

impl cdrColorType {
    pub fn from_i32(v: i32) -> Option<Self> {
        match v {
            0  => Some(Self::Undefined),
            1  => Some(Self::Pantone),
            2  => Some(Self::CMYK),
            3  => Some(Self::CMY),
            5  => Some(Self::RGB),
            6  => Some(Self::HSB),
            7  => Some(Self::HLS),
            8  => Some(Self::BlackAndWhite),
            9  => Some(Self::Grayscale),
            10 => Some(Self::YIQ),
            11 => Some(Self::Lab),
            12 => Some(Self::PantoneHex),
            13 => Some(Self::Spot),
            14 => Some(Self::Registration),
            15 => Some(Self::Mixed),
            16 => Some(Self::Custom),
            _  => None,
        }
    }
}

// =============================================================
// cdrPaletteID —— 调色板 ID
// =============================================================

/// 固定调色板标识。
///
/// 来源：`VGCore::cdrPaletteID`
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum cdrPaletteID {
    Custom           = 0,
    TRUMATCH         = 1,
    PantoneProcess   = 2,
    PantoneCorel8    = 3,
    // 4 缺失
    PantoneHexachrome = 5,
    PantoneMetallic  = 6,
    PantonePastel    = 7,
    FOCOLTONE        = 8,
    SpectraMaster    = 9,
    // 10..39 待补
    SVGPalette       = 40,
    // TODO: 完整列表待补
}

impl cdrPaletteID {
    pub fn from_i32(v: i32) -> Option<Self> {
        match v {
            0  => Some(Self::Custom),
            1  => Some(Self::TRUMATCH),
            2  => Some(Self::PantoneProcess),
            3  => Some(Self::PantoneCorel8),
            5  => Some(Self::PantoneHexachrome),
            6  => Some(Self::PantoneMetallic),
            7  => Some(Self::PantonePastel),
            8  => Some(Self::FOCOLTONE),
            9  => Some(Self::SpectraMaster),
            40 => Some(Self::SVGPalette),
            _  => None,
        }
    }
}

// =============================================================
// clrRenderingIntent —— 渲染意图
// =============================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum clrRenderingIntent {
    Perceptual           = 0,
    RelativeColorimetric = 1,
    Saturation           = 2,
    AbsoluteColorimetric = 3,
}

impl clrRenderingIntent {
    pub fn from_i32(v: i32) -> Option<Self> {
        match v {
            0 => Some(Self::Perceptual),
            1 => Some(Self::RelativeColorimetric),
            2 => Some(Self::Saturation),
            3 => Some(Self::AbsoluteColorimetric),
            _ => None,
        }
    }
}

// =============================================================
// clrColorModel —— 颜色模型
// =============================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum clrColorModel {
    RGB       = 0,
    CMYK      = 1,
    Grayscale = 2,
    // TODO: 待补
}

// =============================================================
// clrDeviceType —— 设备类型
// =============================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum clrDeviceType {
    Monitor   = 0,
    Scanner   = 1,
    Printer   = 2,
    // TODO: 待补
}

// =============================================================
// clrColorPolicyAction —— 颜色策略动作
// =============================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum clrColorPolicyAction {
    // TODO: 待补
    Ignore = 0,
    // ...
}

// =============================================================
// clrImportColorCorrection / clrExportColorCorrection —— 导入导出颜色校正
// =============================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum clrImportColorCorrection {
    // TODO: 待补
    None = 0,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum clrExportColorCorrection {
    // TODO: 待补
    None = 0,
}

// =============================================================
// cdrDuotoneType —— 双色调类型
// =============================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum cdrDuotoneType {
    Monotone  = 0,
    Duotone   = 1,
    Tritone   = 2,
    Quadtone  = 3,
    // TODO: 待补
}

// =============================================================
// cdrPaletteType —— 调色板类型
// =============================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum cdrPaletteType {
    Fixed     = 0,
    Custom    = 1,
    Document  = 2,
    // TODO: 待补
}

// =============================================================
// cdrPaletteVersion —— 调色板版本
// =============================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum cdrPaletteVersion {
    // TODO: 待补
    V1 = 0,
}