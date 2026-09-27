//! 打印 / PDF 枚举

// =============================================================
// Prn* —— 打印相关
// =============================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum PrnColorMode {
    BlackAndWhite = 80,
    Colors16      = 81,
    GreyScale     = 82,
    Palette       = 83,
    RGB           = 84,
    CMYK          = 85,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum PrnPlaceType {
    LeftTop = 0,
    // TODO: 待补
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum PrnPlateType {
    /// 未知 / 未定义。
    Unknown = -1,
    // TODO: 完整值待补
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum PrnPrintRange {
    All       = 0,
    Current   = 1,
    Selection = 2,
    Pages     = 3,
    // TODO: 待补
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum PrnFileMode {
    /// 未知 / 未定义。
    Unknown = -1,
    // TODO: 完整值待补
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum PrnPaperOrientation {
    Portrait  = 0,
    Landscape = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum PrnPaperSize {
    Default = 0,
    // TODO: 完整值待补
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum PrnPostScriptLevel {
    Level1 = 0,
    Level2 = 1,
    Level3 = 2,
    // TODO: 待补
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum PrnPDFStartup {
    /// 未知 / 未定义。
    Unknown = -1,
    // TODO: 完整值待补
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum PrnRegistrationStyle {
    /// 未知 / 未定义。
    Unknown = -1,
    // TODO: 完整值待补
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum PrnTrapType {
    /// 未知 / 未定义。
    Unknown = -1,
    // TODO: 完整值待补
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum PrnImageTrap {
    /// 未知 / 未定义。
    Unknown = -1,
    // TODO: 完整值待补
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum PrnPageSet {
    All = 0,
    // TODO: 完整值待补
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum PrnPageMatchingMode {
    /// 未知 / 未定义。
    Unknown = -1,
    // TODO: 完整值待补
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum PrnObjectsColorMode {
    /// 未知 / 未定义。
    Unknown = -1,
    // TODO: 完整值待补
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum PrnBitmapColorMode {
    /// 未知 / 未定义。
    Unknown = -1,
    // TODO: 完整值待补
}

// =============================================================
// pdf* —— PDF 导出
// =============================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum pdfExportRange {
    All       = 0,
    Current   = 1,
    Selection = 2,
    Pages     = 3,
    // TODO: 待补
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum pdfBitmapCompressionType {
    None = 0,
    JPEG = 1,
    ZIP  = 2,
    // TODO: 待补
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum pdfEncodingType {
    /// 未知 / 未定义。
    Unknown = -1,
    // TODO: 完整值待补
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum pdfColorMode {
    RGB       = 0,
    CMYK      = 1,
    Grayscale = 2,
    // TODO: 待补
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum pdfColorProfile {
    /// 未知 / 未定义。
    Unknown = -1,
    // TODO: 完整值待补
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum pdfEPSAs {
    /// 未知 / 未定义。
    Unknown = -1,
    // TODO: 完整值待补
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum pdfVersion {
    V1_3 = 0,
    V1_4 = 1,
    // TODO: 完整值待补
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum pdfTextExportMode {
    Text   = 0,
    Curves = 1,
    // TODO: 待补
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum pdfPrintPermissions {
    /// 未知 / 未定义。
    Unknown = -1,
    // TODO: 完整值待补
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum pdfEditPermissions {
    /// 未知 / 未定义。
    Unknown = -1,
    // TODO: 完整值待补
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum pdfEncryptionType {
    /// 未知 / 未定义。
    Unknown = -1,
    // TODO: 完整值待补
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum pdfSpotType {
    /// 未知 / 未定义。
    Unknown = -1,
    // TODO: 完整值待补
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum pdfDisplayOnStart {
    /// 未知 / 未定义。
    Unknown = -1,
    // TODO: 完整值待补
}