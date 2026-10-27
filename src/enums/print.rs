//! 打印 / PDF 枚举

// =============================================================
// Prn* —— 打印相关
// =============================================================

/// CorelDRAW 枚举 `PrnColorMode`（颜色模式）。
///
/// 对应类型库中的 `VGCore::PrnColorMode`，用 `#[repr(i32)]` 映射实际整数值。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum PrnColorMode {
    /// 黑色白色。
    BlackAndWhite = 80,
    /// colors16。
    Colors16 = 81,
    /// 灰度缩放。
    GreyScale = 82,
    /// 调色板。
    Palette = 83,
    /// RGB。
    RGB = 84,
    /// CMYK。
    CMYK = 85,
}

/// CorelDRAW 枚举 `PrnPlaceType`（放置类型）。
///
/// 对应类型库中的 `VGCore::PrnPlaceType`，用 `#[repr(i32)]` 映射实际整数值。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum PrnPlaceType {
    /// 文档中的放置 as。
    PlaceAsInDocument = 0,
    /// 放置 fit 页面。
    PlaceFitToPage = 1,
    /// 放置中心页面。
    PlaceCenterPage = 2,
    /// 顶放置中心。
    PlaceCenterTop = 3,
    /// 左放置中心。
    PlaceCenterLeft = 4,
    /// 右放置中心。
    PlaceCenterRight = 5,
    /// 底放置中心。
    PlaceCenterBottom = 6,
    /// 左顶放置。
    PlaceLeftTop = 7,
    /// 右顶放置。
    PlaceRightTop = 8,
    /// 左底放置。
    PlaceLeftBottom = 9,
    /// 右底放置。
    PlaceRightBottom = 10,
    /// 放置自定义。
    PlaceCustom = 11,
}

/// CorelDRAW 枚举 `PrnPlateType`（印版类型）。
///
/// 对应类型库中的 `VGCore::PrnPlateType`，用 `#[repr(i32)]` 映射实际整数值。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum PrnPlateType {
    /// 青。
    Cyan = 0,
    /// 品红。
    Magenta = 1,
    /// 黄。
    Yellow = 2,
    /// 黑色。
    Black = 3,
    /// orange。
    Orange = 4,
    /// 绿。
    Green = 5,
    /// 专色。
    Spot = 6,
}

/// CorelDRAW 枚举 `PrnPrintRange`（打印范围）。
///
/// 对应类型库中的 `VGCore::PrnPrintRange`，用 `#[repr(i32)]` 映射实际整数值。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum PrnPrintRange {
    /// whole 文档。
    WholeDocument = 0,
    /// current 页面。
    CurrentPage = 1,

    /// 选择。
    Selection = 2,
    /// 页面范围。
    PageRange = 3,
}

/// CorelDRAW 枚举 `PrnFileMode`（文件模式）。
///
/// 对应类型库中的 `VGCore::PrnFileMode`，用 `#[repr(i32)]` 映射实际整数值。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum PrnFileMode {
    /// single 文件。
    SingleFile = 0,
    /// separate 页面集。
    SeparatePages = 1,
    /// separate 印版集。
    SeparatePlates = 2,
}

/// CorelDRAW 枚举 `PrnPaperOrientation`（纸张方向）。
///
/// 对应类型库中的 `VGCore::PrnPaperOrientation`，用 `#[repr(i32)]` 映射实际整数值。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum PrnPaperOrientation {
    /// portrait。
    Portrait = 0,
    /// landscape。
    Landscape = 1,
}

/// CorelDRAW 枚举 `PrnPaperSize`（纸张尺寸）。
///
/// 对应类型库中的 `VGCore::PrnPaperSize`，用 `#[repr(i32)]` 映射实际整数值。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum PrnPaperSize {
    /// 纸张尺寸 letter。
    PaperSizeLetter = 1,
    /// 纸张尺寸 lettersmall。
    PaperSizeLetterSmall = 2,
    /// 纸张尺寸 tabloid。
    PaperSizeTabloid = 3,
    /// 纸张尺寸 ledger。
    PaperSizeLedger = 4,
    /// 纸张尺寸 legal。
    PaperSizeLegal = 5,
    /// 纸张尺寸 statement。
    PaperSizeStatement = 6,
    /// 纸张尺寸 executive。
    PaperSizeExecutive = 7,
    /// 纸张尺寸 a3。
    PaperSizeA3 = 8,
    /// 纸张尺寸 a4。
    PaperSizeA4 = 9,
    /// 纸张尺寸 a4small。
    PaperSizeA4Small = 10,
    /// 纸张尺寸 a5。
    PaperSizeA5 = 11,
    /// 纸张尺寸 b4。
    PaperSizeB4 = 12,
    /// 纸张尺寸 b5。
    PaperSizeB5 = 13,
    /// 纸张尺寸 folio。
    PaperSizeFolio = 14,
    /// 纸张尺寸 quarto。
    PaperSizeQuarto = 15,
    /// 纸张 size10x14。
    PaperSize10x14 = 16,
    /// 纸张 size11x17。
    PaperSize11x17 = 17,
    /// 纸张尺寸备注。
    PaperSizeNote = 18,
    /// 纸张尺寸 env9。
    PaperSizeEnv9 = 19,
    /// 纸张尺寸 env10。
    PaperSizeEnv10 = 20,
    /// 纸张尺寸 env11。
    PaperSizeEnv11 = 21,
    /// 纸张尺寸 env12。
    PaperSizeEnv12 = 22,
    /// 纸张尺寸 env14。
    PaperSizeEnv14 = 23,
    /// 纸张尺寸 csheet。
    PaperSizeCSheet = 24,
    /// 纸张尺寸 dsheet。
    PaperSizeDSheet = 25,
    /// 纸张尺寸 esheet。
    PaperSizeESheet = 26,
    /// 纸张尺寸 envdl。
    PaperSizeEnvDL = 27,
    /// 纸张尺寸 envc5。
    PaperSizeEnvC5 = 28,
    /// 纸张尺寸 envc3。
    PaperSizeEnvC3 = 29,
    /// 纸张尺寸 envc4。
    PaperSizeEnvC4 = 30,
    /// 纸张尺寸 envc6。
    PaperSizeEnvC6 = 31,
    /// 纸张尺寸 envc65。
    PaperSizeEnvC65 = 32,
    /// 纸张尺寸 isob4。
    PaperSizeISOB4 = 33,
    /// 纸张尺寸 isob5。
    PaperSizeISOB5 = 34,
    /// 纸张尺寸 isob6。
    PaperSizeISOB6 = 35,
    /// 纸张尺寸 envitaly。
    PaperSizeEnvItaly = 36,
    /// 纸张尺寸 envmonarch。
    PaperSizeEnvMonarch = 37,
    /// 纸张尺寸 envpersonal。
    PaperSizeEnvPersonal = 38,
    /// 纸张尺寸 fanfoldus。
    PaperSizeFanfoldUS = 39,
    /// 纸张尺寸 fanfoldgerman。
    PaperSizeFanfoldGerman = 40,
    /// 纸张尺寸 fanfoldlglgerman。
    PaperSizeFanfoldLglGerman = 41,
    /// 纸张尺寸 japanesepostcard。
    PaperSizeJapanesePostcard = 43,
    /// 纸张 size9x11。
    PaperSize9x11 = 44,
    /// 纸张 size10x11。
    PaperSize10x11 = 45,
    /// 纸张 size15x11。
    PaperSize15x11 = 46,
    /// 纸张尺寸 envinvite。
    PaperSizeEnvInvite = 47,
    /// 纸张尺寸 letterextra。
    PaperSizeLetterExtra = 50,
    /// 纸张尺寸 legalextra。
    PaperSizeLegalExtra = 51,
    /// 纸张尺寸 tabloidextra。
    PaperSizeTabloidExtra = 52,
    /// 纸张尺寸 a4extra。
    PaperSizeA4Extra = 53,
    /// 纸张尺寸 lettertransverse。
    PaperSizeLetterTransverse = 54,
    /// 纸张尺寸 a4transverse。
    PaperSizeA4Transverse = 55,
    /// 纸张尺寸 letterextratransverse。
    PaperSizeLetterExtraTransverse = 56,
    /// 纸张尺寸 aplus。
    PaperSizeAPlus = 57,
    /// 纸张尺寸 bplus。
    PaperSizeBPlus = 58,
    /// 纸张尺寸 letterplus。
    PaperSizeLetterPlus = 59,
    /// 纸张尺寸 a4plus。
    PaperSizeA4Plus = 60,
    /// 纸张尺寸 a5transverse。
    PaperSizeA5Transverse = 61,
    /// 纸张尺寸 b5transverse。
    PaperSizeB5Transverse = 62,
    /// 纸张尺寸 a3extra。
    PaperSizeA3Extra = 63,
    /// 纸张尺寸 a5extra。
    PaperSizeA5Extra = 64,
    /// 纸张尺寸 b5extra。
    PaperSizeB5Extra = 65,
    /// 纸张尺寸 a2。
    PaperSizeA2 = 66,
    /// 纸张尺寸 a3transverse。
    PaperSizeA3Transverse = 67,
    /// 纸张尺寸 a3extratransverse。
    PaperSizeA3ExtraTransverse = 68,
    /// 纸张尺寸 japanesedblpostcard。
    PaperSizeJapaneseDblPostcard = 69,
    /// 纸张尺寸 a6。
    PaperSizeA6 = 70,
    /// 纸张尺寸 letter 旋转。
    PaperSizeLetterRotated = 75,
    /// 纸张尺寸 a3 旋转。
    PaperSizeA3Rotated = 76,
    /// 纸张尺寸 a4 旋转。
    PaperSizeA4Rotated = 77,
    /// 纸张尺寸 a5 旋转。
    PaperSizeA5Rotated = 78,
    /// 纸张尺寸 b4jis 旋转。
    PaperSizeB4JISRotated = 79,
    /// 纸张尺寸 b5jis 旋转。
    PaperSizeB5JISRotated = 80,
    /// 纸张尺寸 japanesepostcard 旋转。
    PaperSizeJapanesePostcardRotated = 81,
    /// 纸张尺寸 japanesedlbpostcard 旋转。
    PaperSizeJapaneseDlbPostcardRotated = 82,
    /// 纸张尺寸 a6 旋转。
    PaperSizeA6Rotated = 83,
    /// 纸张尺寸 b6jis。
    PaperSizeB6JIS = 88,
    /// 纸张尺寸 b6jis 旋转。
    PaperSizeB6JISRotated = 89,
    /// 纸张 size12x11。
    PaperSize12x11 = 90,
    /// 纸张尺寸 p16k。
    PaperSizeP16K = 93,
    /// 纸张尺寸 p32k。
    PaperSizeP32K = 94,
    /// 纸张尺寸 p32kbig。
    PaperSizeP32KBig = 95,
    /// 纸张尺寸 penv1。
    PaperSizePEnv1 = 96,
    /// 纸张尺寸 penv2。
    PaperSizePEnv2 = 97,
    /// 纸张尺寸 penv3。
    PaperSizePEnv3 = 98,
    /// 纸张尺寸 penv4。
    PaperSizePEnv4 = 99,
    /// 纸张尺寸 penv5。
    PaperSizePEnv5 = 100,
    /// 纸张尺寸 penv6。
    PaperSizePEnv6 = 101,
    /// 纸张尺寸 penv7。
    PaperSizePEnv7 = 102,
    /// 纸张尺寸 penv8。
    PaperSizePEnv8 = 103,
    /// 纸张尺寸 penv9。
    PaperSizePEnv9 = 104,
    /// 纸张尺寸 penv10。
    PaperSizePEnv10 = 105,
    /// 纸张尺寸 p16k 旋转。
    PaperSizeP16KRotated = 106,
    /// 纸张尺寸 p32k 旋转。
    PaperSizeP32KRotated = 107,
    /// 纸张尺寸 p32kbig 旋转。
    PaperSizeP32KBigRotated = 108,
    /// 纸张尺寸 penv1 旋转。
    PaperSizePEnv1Rotated = 109,
    /// 纸张尺寸 penv2 旋转。
    PaperSizePEnv2Rotated = 110,
    /// 纸张尺寸 penv3 旋转。
    PaperSizePEnv3Rotated = 111,
    /// 纸张尺寸 penv4 旋转。
    PaperSizePEnv4Rotated = 112,
    /// 纸张尺寸 penv5 旋转。
    PaperSizePEnv5Rotated = 113,
    /// 纸张尺寸 penv6 旋转。
    PaperSizePEnv6Rotated = 114,
    /// 纸张尺寸 penv7 旋转。
    PaperSizePEnv7Rotated = 115,
    /// 纸张尺寸 penv8 旋转。
    PaperSizePEnv8Rotated = 116,
    /// 纸张尺寸 penv9 旋转。
    PaperSizePEnv9Rotated = 117,
    /// 纸张尺寸 penv10 旋转。
    PaperSizePEnv10Rotated = 118,
}

/// CorelDRAW 枚举 `PrnPostScriptLevel`（post 脚本级别）。
///
/// 对应类型库中的 `VGCore::PrnPostScriptLevel`，用 `#[repr(i32)]` 映射实际整数值。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum PrnPostScriptLevel {
    /// pslevel1。
    PSLevel1 = 1,
    /// pslevel2。
    PSLevel2 = 2,
    /// pslevel3。
    PSLevel3 = 3,
}

/// CorelDRAW 枚举 `PrnPDFStartup`（PDF 启动）。
///
/// 对应类型库中的 `VGCore::PrnPDFStartup`，用 `#[repr(i32)]` 映射实际整数值。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum PrnPDFStartup {
    /// PDF 完整屏幕。
    PDFFullScreen = 0,
    /// PDF 页面仅。
    PDFPageOnly = 1,
    /// PDFthumbnails。
    PDFThumbnails = 2,
    /// PDF 轮廓。
    PDFOutlines = 3,
}

/// CorelDRAW 枚举 `PrnRegistrationStyle`（套准样式）。
///
/// 对应类型库中的 `VGCore::PrnRegistrationStyle`，用 `#[repr(i32)]` 映射实际整数值。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum PrnRegistrationStyle {
    /// 标准。
    Standard = 0,
    /// long。
    Long = 1,
    /// square。
    Square = 2,
    /// halfinverted。
    HalfInverted = 3,
    /// corel。
    Corel = 4,
}

/// CorelDRAW 枚举 `PrnTrapType`（陷印类型）。
///
/// 对应类型库中的 `VGCore::PrnTrapType`，用 `#[repr(i32)]` 映射实际整数值。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum PrnTrapType {
    /// 图层常规。
    LayerNormal = 0,
    /// 图层透明。
    LayerTransparent = 1,
    /// 图层 opaque。
    LayerOpaque = 2,
    /// 图层 opaqueignore。
    LayerOpaqueIgnore = 3,
}

/// CorelDRAW 枚举 `PrnImageTrap`（图像陷印）。
///
/// 对应类型库中的 `VGCore::PrnImageTrap`，用 `#[repr(i32)]` 映射实际整数值。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum PrnImageTrap {
    /// 陷印常规。
    TrapNormal = 0,
    /// 陷印跨页。
    TrapSpread = 1,
    /// 陷印 choke。
    TrapChoke = 2,
    /// 陷印中心。
    TrapCenter = 3,
}

/// CorelDRAW 枚举 `PrnPageSet`（页面设置）。
///
/// 对应类型库中的 `VGCore::PrnPageSet`，用 `#[repr(i32)]` 映射实际整数值。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum PrnPageSet {
    /// 页面设置全部。
    PageSetAll = 0,
    /// 页面设置 odd。
    PageSetOdd = 1,
    /// 页面设置 even。
    PageSetEven = 2,
    /// 左页面设置。
    PageSetLeft = 3,
    /// 右页面设置。
    PageSetRight = 4,
}

/// CorelDRAW 枚举 `PrnPageMatchingMode`（页面匹配模式）。
///
/// 对应类型库中的 `VGCore::PrnPageMatchingMode`，用 `#[repr(i32)]` 映射实际整数值。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum PrnPageMatchingMode {
    /// 页面 match 打印机默认值。
    PageMatchPrinterDefault = 0,
    /// 页面 match 方向。
    PageMatchOrientation = 1,
    /// 页面 match 尺寸方向。
    PageMatchSizeAndOrientation = 2,
}

/// CorelDRAW 枚举 `PrnObjectsColorMode`（对象集颜色模式）。
///
/// 对应类型库中的 `VGCore::PrnObjectsColorMode`，用 `#[repr(i32)]` 映射实际整数值。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum PrnObjectsColorMode {
    /// 对象集 CMYK。
    ObjectsCMYK = 0,
    /// 对象集 RGB。
    ObjectsRGB = 1,
    /// 对象集灰度。
    ObjectsGrayscale = 2,
    /// 对象集 native。
    ObjectsNative = 3,
}

/// CorelDRAW 枚举 `PrnBitmapColorMode`（位图颜色模式）。
///
/// 对应类型库中的 `VGCore::PrnBitmapColorMode`，用 `#[repr(i32)]` 映射实际整数值。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum PrnBitmapColorMode {
    /// 位图 CMYK。
    BitmapCMYK = 0,
    /// 位图 RGB。
    BitmapRGB = 1,
    /// 位图灰度。
    BitmapGrayscale = 2,
    /// 位图 native。
    BitmapNative = 3,
}

// =============================================================
// pdf* —— PDF 导出
// =============================================================

/// CorelDRAW 枚举 `pdfExportRange`（导出范围）。
///
/// 对应类型库中的 `VGCore::pdfExportRange`，用 `#[repr(i32)]` 映射实际整数值。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum pdfExportRange {
    /// whole 文档。
    WholeDocument = 0,
    /// current 页面。
    CurrentPage = 1,

    /// 选择。
    Selection = 2,
    /// 页面范围。
    PageRange = 3,
}

/// CorelDRAW 枚举 `pdfBitmapCompressionType`（位图压缩类型）。
///
/// 对应类型库中的 `VGCore::pdfBitmapCompressionType`，用 `#[repr(i32)]` 映射实际整数值。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum pdfBitmapCompressionType {
    /// 无。
    None = 0,
    /// lzw。
    LZW = 1,

    /// JPEG。
    JPEG = 2,

    /// zip。
    ZIP = 3,
    /// jp2。
    JP2 = 4,
}

/// CorelDRAW 枚举 `pdfEncodingType`（encoding 类型）。
///
/// 对应类型库中的 `VGCore::pdfEncodingType`，用 `#[repr(i32)]` 映射实际整数值。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum pdfEncodingType {
    /// ascii85。
    ASCII85 = 0,
    /// binary。
    Binary = 1,
}

/// CorelDRAW 枚举 `pdfColorMode`（颜色模式）。
///
/// 对应类型库中的 `VGCore::pdfColorMode`，用 `#[repr(i32)]` 映射实际整数值。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum pdfColorMode {
    /// RGB。
    RGB = 0,

    /// CMYK。
    CMYK = 1,

    /// 灰度。
    Grayscale = 2,
    /// native。
    Native = 3,
}

/// CorelDRAW 枚举 `pdfColorProfile`（颜色配置文件）。
///
/// 对应类型库中的 `VGCore::pdfColorProfile`，用 `#[repr(i32)]` 映射实际整数值。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum pdfColorProfile {
    /// 合成配置文件。
    CompositeProfile = 0,
    /// 分色配置文件。
    SeparationProfile = 1,
}

/// CorelDRAW 枚举 `pdfEPSAs`（EPSas）。
///
/// 对应类型库中的 `VGCore::pdfEPSAs`，用 `#[repr(i32)]` 映射实际整数值。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum pdfEPSAs {
    /// PostScript。
    Postscript = 0,
    /// preview。
    Preview = 1,
}

/// CorelDRAW 枚举 `pdfVersion`（版本）。
///
/// 对应类型库中的 `VGCore::pdfVersion`，用 `#[repr(i32)]` 映射实际整数值。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum pdfVersion {
    /// version12。
    Version12 = 0,
    /// version13。
    Version13 = 1,
    /// 版本 pdfx1。
    VersionPDFX1 = 2,
    /// version14。
    Version14 = 3,
    /// 版本 pdfx1a。
    VersionPDFX1a = 4,
    /// 版本 pdfx3。
    VersionPDFX3 = 5,
    /// version15。
    Version15 = 6,
    /// 版本 pdfa1b。
    VersionPDFA1b = 7,
    /// version17。
    Version17 = 8,
    /// version17acrobat9。
    Version17_Acrobat9 = 9,
    /// 版本 pdfx4。
    VersionPDFX4 = 10,
}

/// CorelDRAW 枚举 `pdfTextExportMode`（文本导出模式）。
///
/// 对应类型库中的 `VGCore::pdfTextExportMode`，用 `#[repr(i32)]` 映射实际整数值。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum pdfTextExportMode {
    /// 文本 asunicode。
    TextAsUnicode = 0,
    /// 文本 asascii。
    TextAsAscii = 1,
}

/// CorelDRAW 枚举 `pdfPrintPermissions`（打印权限）。
///
/// 对应类型库中的 `VGCore::pdfPrintPermissions`，用 `#[repr(i32)]` 映射实际整数值。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum pdfPrintPermissions {
    /// 打印 permission 无。
    PrintPermissionNone = 0,
    /// 打印 permissionlow 分辨率。
    PrintPermissionLowResolution = 1,
    /// 打印 permissionhigh 分辨率。
    PrintPermissionHighResolution = 2,
}

/// CorelDRAW 枚举 `pdfEditPermissions`（编辑权限）。
///
/// 对应类型库中的 `VGCore::pdfEditPermissions`，用 `#[repr(i32)]` 映射实际整数值。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum pdfEditPermissions {
    /// 编辑 permission 无。
    EditPermissionNone = 0,
    /// 编辑 permissioninsert 删除 rotate 页面。
    EditPermissionInsertDeleteRotatePage = 1,
    /// 编辑 permissionany。
    EditPermissionAny = 2,
}

/// CorelDRAW 枚举 `pdfEncryptionType`（encryption 类型）。
///
/// 对应类型库中的 `VGCore::pdfEncryptionType`，用 `#[repr(i32)]` 映射实际整数值。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum pdfEncryptionType {
    /// encrypt 类型无。
    EncryptTypeNone = 0,
    /// encrypt 类型标准。
    EncryptTypeStandard = 1,
    /// encrypt 类型 aes。
    EncryptTypeAES = 2,
    /// encrypt 类型 aes256。
    EncryptTypeAES256 = 3,
}

/// CorelDRAW 枚举 `pdfSpotType`（专色类型）。
///
/// 对应类型库中的 `VGCore::pdfSpotType`，用 `#[repr(i32)]` 映射实际整数值。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum pdfSpotType {
    /// 专色 as 专色。
    SpotAsSpot = 0,
    /// 专色 asRGB。
    SpotAsRGB = 1,
    /// 专色 asCMYK。
    SpotAsCMYK = 2,
    /// 专色 as 灰度。
    SpotAsGray = 3,
}

/// CorelDRAW 枚举 `pdfDisplayOnStart`（开始上的显示）。
///
/// 对应类型库中的 `VGCore::pdfDisplayOnStart`，用 `#[repr(i32)]` 映射实际整数值。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum pdfDisplayOnStart {
    /// 页面仅。
    PageOnly = 0,
    /// 完整屏幕。
    FullScreen = 1,
    /// 书签。
    Bookmarks = 2,
    /// thumbnails。
    Thumbnails = 3,
}
