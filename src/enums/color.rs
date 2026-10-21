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
    Undefined = 0,
    /// PANTONE。
    Pantone = 1,
    /// CMYK。
    CMYK = 2,
    /// CMY。
    CMY = 3,
    /// RGB。
    RGB = 5,
    /// HSB。
    HSB = 6,
    /// HLS。
    HLS = 7,
    /// 黑白。
    BlackAndWhite = 8,
    /// 灰度。
    Grayscale = 9,
    /// YIQ。
    YIQ = 10,
    /// Lab。
    Lab = 11,
    /// PANTONE Hexachrome。
    PantoneHex = 12,
    /// 专色。
    Spot = 13,
    /// 注册色。
    Registration = 14,
    /// 混合。
    Mixed = 15,
    /// 自定义。
    Custom = 16,
}

impl cdrColorType {
    pub fn from_i32(v: i32) -> Option<Self> {
        match v {
            0 => Some(Self::Undefined),
            1 => Some(Self::Pantone),
            2 => Some(Self::CMYK),
            3 => Some(Self::CMY),
            5 => Some(Self::RGB),
            6 => Some(Self::HSB),
            7 => Some(Self::HLS),
            8 => Some(Self::BlackAndWhite),
            9 => Some(Self::Grayscale),
            10 => Some(Self::YIQ),
            11 => Some(Self::Lab),
            12 => Some(Self::PantoneHex),
            13 => Some(Self::Spot),
            14 => Some(Self::Registration),
            15 => Some(Self::Mixed),
            16 => Some(Self::Custom),
            _ => None,
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
    Custom = 0,

    TRUMATCH = 1,
    PANTONEProcess = 2,
    PANTONECorel8 = 3,
    Uniform = 7,

    FOCOLTONE = 8,

    SpectraMaster = 9,
    TOYO = 10,
    DIC = 11,
    Lab = 13,
    HKS = 23,
    WebSafe = 25,
    PANTONEMetallic = 30,
    PANTONEPastelCoated = 31,
    PANTONEPastelUncoated = 32,
    PANTONEHexCoated = 33,
    PANTONEHexUncoated = 34,
    PANTONEMatte = 35,
    PANTONECoated = 36,
    PANTONEUncoated = 37,
    PANTONEProcessCoatedEURO = 38,
    PANTONESolid2ProcessEURO = 39,
    // 10..39 待补
    SVGPalette = 40,
    UserInks = 16,
}

impl cdrPaletteID {
    pub fn from_i32(v: i32) -> Option<Self> {
        match v {
            0 => Some(Self::Custom),
            1 => Some(Self::TRUMATCH),
            2 => Some(Self::PANTONEProcess),
            3 => Some(Self::PANTONECorel8),
            7 => Some(Self::Uniform),
            8 => Some(Self::FOCOLTONE),
            9 => Some(Self::SpectraMaster),
            10 => Some(Self::TOYO),
            11 => Some(Self::DIC),
            13 => Some(Self::Lab),
            23 => Some(Self::HKS),
            25 => Some(Self::WebSafe),
            30 => Some(Self::PANTONEMetallic),
            31 => Some(Self::PANTONEPastelCoated),
            32 => Some(Self::PANTONEPastelUncoated),
            33 => Some(Self::PANTONEHexCoated),
            34 => Some(Self::PANTONEHexUncoated),
            35 => Some(Self::PANTONEMatte),
            36 => Some(Self::PANTONECoated),
            37 => Some(Self::PANTONEUncoated),
            38 => Some(Self::PANTONEProcessCoatedEURO),
            39 => Some(Self::PANTONESolid2ProcessEURO),
            40 => Some(Self::SVGPalette),
            16 => Some(Self::UserInks),
            _ => None,
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
    Perceptual = 0,
    RelativeColorimetric = 1,
    Saturation = 2,
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
    ColorModelRGB = 0,
    ColorModelCMYK = 1,
    ColorModelGrayscale = 2,
    ColorModelLab = 3,
    ColorModelHexachrome = 4,
}

// =============================================================
// clrDeviceType —— 设备类型
// =============================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum clrDeviceType {
    Monitor = 0,
    CompositePrinter = 1,
    SeparationPrinter = 2,

    Scanner = 3,
    InternalRGB = 4,
    DeviceAll = -1,
}

// =============================================================
// clrColorPolicyAction —— 颜色策略动作
// =============================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum clrColorPolicyAction {
    AssignBaselineProfile = 0,
    ConvertToBaselineProfile = 1,
    ConvertToEmbeddedProfile = 2,
    AssignEmbeddedProfile = 3,
}

// =============================================================
// clrImportColorCorrection / clrExportColorCorrection —— 导入导出颜色校正
// =============================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum clrImportColorCorrection {
    NoImportCorrection = 0,
    ApplyEmbeddedOrDefaultProfile = 1,
    ApplyCustomImportProfile = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum clrExportColorCorrection {
    NoExportCorrection = 0,
    EmbedInternalRGBProfile = 1,
    EmbedCustomExportProfile = 2,
}

// =============================================================
// cdrDuotoneType —— 双色调类型
// =============================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum cdrDuotoneType {
    Monotone = 0,

    Duotone = 1,

    Tritone = 2,

    Quadtone = 3,
}

// =============================================================
// cdrPaletteType —— 调色板类型
// =============================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum cdrPaletteType {
    FixedPalette = 0,
    CustomPalette = 1,
    DocumentPalette = 2,
}

// =============================================================
// cdrPaletteVersion —— 调色板版本
// =============================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum cdrPaletteVersion {
    PaletteVersion9 = 0,
    PaletteVersion12 = 1,
}
