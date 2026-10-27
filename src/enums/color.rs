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
    /// 由整数值构造枚举。
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
    /// 自定义。
    Custom = 0,

    /// trumatch。
    TRUMATCH = 1,
    /// pantone 印刷色。
    PANTONEProcess = 2,
    /// pantonecorel8。
    PANTONECorel8 = 3,
    /// 均匀。
    Uniform = 7,

    /// focoltone。
    FOCOLTONE = 8,

    /// spectra 主。
    SpectraMaster = 9,
    /// toyo。
    TOYO = 10,
    /// dic。
    DIC = 11,
    /// Lab。
    Lab = 13,
    /// hks。
    HKS = 23,
    /// Websafe。
    WebSafe = 25,
    /// pantonemetallic。
    PANTONEMetallic = 30,
    /// pantonepastelcoated。
    PANTONEPastelCoated = 31,
    /// pantonepasteluncoated。
    PANTONEPastelUncoated = 32,
    /// pantonehexcoated。
    PANTONEHexCoated = 33,
    /// pantonehexuncoated。
    PANTONEHexUncoated = 34,
    /// pantone 蒙版。
    PANTONEMatte = 35,
    /// pantonecoated。
    PANTONECoated = 36,
    /// pantoneuncoated。
    PANTONEUncoated = 37,
    /// pantone 印刷色 coatedeuro。
    PANTONEProcessCoatedEURO = 38,
    /// pantonesolid2 印刷色 euro。
    PANTONESolid2ProcessEURO = 39,
    // 10..39 待补
    /// svg 调色板。
    SVGPalette = 40,
    /// 用户 inks。
    UserInks = 16,
}

impl cdrPaletteID {
    /// 由整数值构造枚举。
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

/// CorelDRAW 枚举 `clrRenderingIntent`：渲染意图。
///
/// 对应类型库中的 `VGCore::clrRenderingIntent`，用 `#[repr(i32)]` 映射实际整数值。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum clrRenderingIntent {
    /// perceptual。
    Perceptual = 0,
    /// 相对 colorimetric。
    RelativeColorimetric = 1,
    /// 饱和度。
    Saturation = 2,
    /// 绝对 colorimetric。
    AbsoluteColorimetric = 3,
}

impl clrRenderingIntent {
    /// 由整数值构造枚举。
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

/// CorelDRAW 枚举 `clrColorModel`：颜色模型。
///
/// 对应类型库中的 `VGCore::clrColorModel`，用 `#[repr(i32)]` 映射实际整数值。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum clrColorModel {
    /// 颜色模型 RGB。
    ColorModelRGB = 0,
    /// 颜色模型 CMYK。
    ColorModelCMYK = 1,
    /// 颜色模型灰度。
    ColorModelGrayscale = 2,
    /// 颜色模型 Lab。
    ColorModelLab = 3,
    /// 颜色模型 hexachrome。
    ColorModelHexachrome = 4,
}

// =============================================================
// clrDeviceType —— 设备类型
// =============================================================

/// CorelDRAW 枚举 `clrDeviceType`：设备类型。
///
/// 对应类型库中的 `VGCore::clrDeviceType`，用 `#[repr(i32)]` 映射实际整数值。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum clrDeviceType {
    /// 监视器。
    Monitor = 0,
    /// 合成打印机。
    CompositePrinter = 1,
    /// 分色打印机。
    SeparationPrinter = 2,

    /// scanner。
    Scanner = 3,
    /// 内部 RGB。
    InternalRGB = 4,
    /// 设备全部。
    DeviceAll = -1,
}

// =============================================================
// clrColorPolicyAction —— 颜色策略动作
// =============================================================

/// CorelDRAW 枚举 `clrColorPolicyAction`：颜色策略动作。
///
/// 对应类型库中的 `VGCore::clrColorPolicyAction`，用 `#[repr(i32)]` 映射实际整数值。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum clrColorPolicyAction {
    /// assignbaseline 配置文件。
    AssignBaselineProfile = 0,
    /// 转换 baseline 配置文件。
    ConvertToBaselineProfile = 1,
    /// 转换 embedded 配置文件。
    ConvertToEmbeddedProfile = 2,
    /// assignembedded 配置文件。
    AssignEmbeddedProfile = 3,
}

// =============================================================
// clrImportColorCorrection / clrExportColorCorrection —— 导入导出颜色校正
// =============================================================

/// CorelDRAW 枚举 `clrImportColorCorrection`（导入颜色校正）。
///
/// 对应类型库中的 `VGCore::clrImportColorCorrection`，用 `#[repr(i32)]` 映射实际整数值。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum clrImportColorCorrection {
    /// 无导入校正。
    NoImportCorrection = 0,
    /// 应用 embedded 默认值配置文件。
    ApplyEmbeddedOrDefaultProfile = 1,
    /// 应用自定义导入配置文件。
    ApplyCustomImportProfile = 2,
}

/// CorelDRAW 枚举 `clrExportColorCorrection`（导出颜色校正）。
///
/// 对应类型库中的 `VGCore::clrExportColorCorrection`，用 `#[repr(i32)]` 映射实际整数值。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum clrExportColorCorrection {
    /// 无导出校正。
    NoExportCorrection = 0,
    /// 嵌入内部 RGB 配置文件。
    EmbedInternalRGBProfile = 1,
    /// 嵌入自定义导出配置文件。
    EmbedCustomExportProfile = 2,
}

// =============================================================
// cdrDuotoneType —— 双色调类型
// =============================================================

/// CorelDRAW 枚举 `cdrDuotoneType`：双色调类型。
///
/// 对应类型库中的 `VGCore::cdrDuotoneType`，用 `#[repr(i32)]` 映射实际整数值。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum cdrDuotoneType {
    /// monotone。
    Monotone = 0,

    /// 双色。
    Duotone = 1,

    /// tritone。
    Tritone = 2,

    /// quadtone。
    Quadtone = 3,
}

// =============================================================
// cdrPaletteType —— 调色板类型
// =============================================================

/// CorelDRAW 枚举 `cdrPaletteType`：调色板类型。
///
/// 对应类型库中的 `VGCore::cdrPaletteType`，用 `#[repr(i32)]` 映射实际整数值。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum cdrPaletteType {
    /// 固定调色板。
    FixedPalette = 0,
    /// 自定义调色板。
    CustomPalette = 1,
    /// 文档调色板。
    DocumentPalette = 2,
}

// =============================================================
// cdrPaletteVersion —— 调色板版本
// =============================================================

/// CorelDRAW 枚举 `cdrPaletteVersion`：调色板版本。
///
/// 对应类型库中的 `VGCore::cdrPaletteVersion`，用 `#[repr(i32)]` 映射实际整数值。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum cdrPaletteVersion {
    /// 调色板 version9。
    PaletteVersion9 = 0,
    /// 调色板 version12。
    PaletteVersion12 = 1,
}
