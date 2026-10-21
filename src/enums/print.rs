//! 打印 / PDF 枚举

// =============================================================
// Prn* —— 打印相关
// =============================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum PrnColorMode {
    BlackAndWhite = 80,
    Colors16 = 81,
    GreyScale = 82,
    Palette = 83,
    RGB = 84,
    CMYK = 85,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum PrnPlaceType {
    PlaceAsInDocument = 0,
    PlaceFitToPage = 1,
    PlaceCenterPage = 2,
    PlaceCenterTop = 3,
    PlaceCenterLeft = 4,
    PlaceCenterRight = 5,
    PlaceCenterBottom = 6,
    PlaceLeftTop = 7,
    PlaceRightTop = 8,
    PlaceLeftBottom = 9,
    PlaceRightBottom = 10,
    PlaceCustom = 11,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum PrnPlateType {
    Cyan = 0,
    Magenta = 1,
    Yellow = 2,
    Black = 3,
    Orange = 4,
    Green = 5,
    Spot = 6,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum PrnPrintRange {
    WholeDocument = 0,
    CurrentPage = 1,

    Selection = 2,
    PageRange = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum PrnFileMode {
    SingleFile = 0,
    SeparatePages = 1,
    SeparatePlates = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum PrnPaperOrientation {
    Portrait = 0,
    Landscape = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum PrnPaperSize {
    PaperSizeLetter = 1,
    PaperSizeLetterSmall = 2,
    PaperSizeTabloid = 3,
    PaperSizeLedger = 4,
    PaperSizeLegal = 5,
    PaperSizeStatement = 6,
    PaperSizeExecutive = 7,
    PaperSizeA3 = 8,
    PaperSizeA4 = 9,
    PaperSizeA4Small = 10,
    PaperSizeA5 = 11,
    PaperSizeB4 = 12,
    PaperSizeB5 = 13,
    PaperSizeFolio = 14,
    PaperSizeQuarto = 15,
    PaperSize10x14 = 16,
    PaperSize11x17 = 17,
    PaperSizeNote = 18,
    PaperSizeEnv9 = 19,
    PaperSizeEnv10 = 20,
    PaperSizeEnv11 = 21,
    PaperSizeEnv12 = 22,
    PaperSizeEnv14 = 23,
    PaperSizeCSheet = 24,
    PaperSizeDSheet = 25,
    PaperSizeESheet = 26,
    PaperSizeEnvDL = 27,
    PaperSizeEnvC5 = 28,
    PaperSizeEnvC3 = 29,
    PaperSizeEnvC4 = 30,
    PaperSizeEnvC6 = 31,
    PaperSizeEnvC65 = 32,
    PaperSizeISOB4 = 33,
    PaperSizeISOB5 = 34,
    PaperSizeISOB6 = 35,
    PaperSizeEnvItaly = 36,
    PaperSizeEnvMonarch = 37,
    PaperSizeEnvPersonal = 38,
    PaperSizeFanfoldUS = 39,
    PaperSizeFanfoldGerman = 40,
    PaperSizeFanfoldLglGerman = 41,
    PaperSizeJapanesePostcard = 43,
    PaperSize9x11 = 44,
    PaperSize10x11 = 45,
    PaperSize15x11 = 46,
    PaperSizeEnvInvite = 47,
    PaperSizeLetterExtra = 50,
    PaperSizeLegalExtra = 51,
    PaperSizeTabloidExtra = 52,
    PaperSizeA4Extra = 53,
    PaperSizeLetterTransverse = 54,
    PaperSizeA4Transverse = 55,
    PaperSizeLetterExtraTransverse = 56,
    PaperSizeAPlus = 57,
    PaperSizeBPlus = 58,
    PaperSizeLetterPlus = 59,
    PaperSizeA4Plus = 60,
    PaperSizeA5Transverse = 61,
    PaperSizeB5Transverse = 62,
    PaperSizeA3Extra = 63,
    PaperSizeA5Extra = 64,
    PaperSizeB5Extra = 65,
    PaperSizeA2 = 66,
    PaperSizeA3Transverse = 67,
    PaperSizeA3ExtraTransverse = 68,
    PaperSizeJapaneseDblPostcard = 69,
    PaperSizeA6 = 70,
    PaperSizeLetterRotated = 75,
    PaperSizeA3Rotated = 76,
    PaperSizeA4Rotated = 77,
    PaperSizeA5Rotated = 78,
    PaperSizeB4JISRotated = 79,
    PaperSizeB5JISRotated = 80,
    PaperSizeJapanesePostcardRotated = 81,
    PaperSizeJapaneseDlbPostcardRotated = 82,
    PaperSizeA6Rotated = 83,
    PaperSizeB6JIS = 88,
    PaperSizeB6JISRotated = 89,
    PaperSize12x11 = 90,
    PaperSizeP16K = 93,
    PaperSizeP32K = 94,
    PaperSizeP32KBig = 95,
    PaperSizePEnv1 = 96,
    PaperSizePEnv2 = 97,
    PaperSizePEnv3 = 98,
    PaperSizePEnv4 = 99,
    PaperSizePEnv5 = 100,
    PaperSizePEnv6 = 101,
    PaperSizePEnv7 = 102,
    PaperSizePEnv8 = 103,
    PaperSizePEnv9 = 104,
    PaperSizePEnv10 = 105,
    PaperSizeP16KRotated = 106,
    PaperSizeP32KRotated = 107,
    PaperSizeP32KBigRotated = 108,
    PaperSizePEnv1Rotated = 109,
    PaperSizePEnv2Rotated = 110,
    PaperSizePEnv3Rotated = 111,
    PaperSizePEnv4Rotated = 112,
    PaperSizePEnv5Rotated = 113,
    PaperSizePEnv6Rotated = 114,
    PaperSizePEnv7Rotated = 115,
    PaperSizePEnv8Rotated = 116,
    PaperSizePEnv9Rotated = 117,
    PaperSizePEnv10Rotated = 118,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum PrnPostScriptLevel {
    PSLevel1 = 1,
    PSLevel2 = 2,
    PSLevel3 = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum PrnPDFStartup {
    PDFFullScreen = 0,
    PDFPageOnly = 1,
    PDFThumbnails = 2,
    PDFOutlines = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum PrnRegistrationStyle {
    Standard = 0,
    Long = 1,
    Square = 2,
    HalfInverted = 3,
    Corel = 4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum PrnTrapType {
    LayerNormal = 0,
    LayerTransparent = 1,
    LayerOpaque = 2,
    LayerOpaqueIgnore = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum PrnImageTrap {
    TrapNormal = 0,
    TrapSpread = 1,
    TrapChoke = 2,
    TrapCenter = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum PrnPageSet {
    PageSetAll = 0,
    PageSetOdd = 1,
    PageSetEven = 2,
    PageSetLeft = 3,
    PageSetRight = 4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum PrnPageMatchingMode {
    PageMatchPrinterDefault = 0,
    PageMatchOrientation = 1,
    PageMatchSizeAndOrientation = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum PrnObjectsColorMode {
    ObjectsCMYK = 0,
    ObjectsRGB = 1,
    ObjectsGrayscale = 2,
    ObjectsNative = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum PrnBitmapColorMode {
    BitmapCMYK = 0,
    BitmapRGB = 1,
    BitmapGrayscale = 2,
    BitmapNative = 3,
}

// =============================================================
// pdf* —— PDF 导出
// =============================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum pdfExportRange {
    WholeDocument = 0,
    CurrentPage = 1,

    Selection = 2,
    PageRange = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum pdfBitmapCompressionType {
    None = 0,
    LZW = 1,

    JPEG = 2,

    ZIP = 3,
    JP2 = 4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum pdfEncodingType {
    ASCII85 = 0,
    Binary = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum pdfColorMode {
    RGB = 0,

    CMYK = 1,

    Grayscale = 2,
    Native = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum pdfColorProfile {
    CompositeProfile = 0,
    SeparationProfile = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum pdfEPSAs {
    Postscript = 0,
    Preview = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum pdfVersion {
    Version12 = 0,
    Version13 = 1,
    VersionPDFX1 = 2,
    Version14 = 3,
    VersionPDFX1a = 4,
    VersionPDFX3 = 5,
    Version15 = 6,
    VersionPDFA1b = 7,
    Version17 = 8,
    Version17_Acrobat9 = 9,
    VersionPDFX4 = 10,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum pdfTextExportMode {
    TextAsUnicode = 0,
    TextAsAscii = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum pdfPrintPermissions {
    PrintPermissionNone = 0,
    PrintPermissionLowResolution = 1,
    PrintPermissionHighResolution = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum pdfEditPermissions {
    EditPermissionNone = 0,
    EditPermissionInsertDeleteRotatePage = 1,
    EditPermissionAny = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum pdfEncryptionType {
    EncryptTypeNone = 0,
    EncryptTypeStandard = 1,
    EncryptTypeAES = 2,
    EncryptTypeAES256 = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum pdfSpotType {
    SpotAsSpot = 0,
    SpotAsRGB = 1,
    SpotAsCMYK = 2,
    SpotAsGray = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum pdfDisplayOnStart {
    PageOnly = 0,
    FullScreen = 1,
    Bookmarks = 2,
    Thumbnails = 3,
}
