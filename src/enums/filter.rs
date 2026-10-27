//! 文件过滤 / 格式枚举
//!
//! ⚠️ `cdrFilter` 有 **100+ 个值**，此处列常用项。
//! 完整列表请用 `oleview.exe` 打开 `VGCore.tlb` 导出。

// =============================================================
// cdrFilter —— 文件格式过滤器
// =============================================================

/// CorelDRAW 枚举 `cdrFilter`：文件格式过滤器。
///
/// 对应类型库中的 `VGCore::cdrFilter`，用 `#[repr(i32)]` 映射实际整数值。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum cdrFilter {
    /// 自动检测（仅导入）。
    AutoSense = 0,
    /// 3dmf。
    V_3DMF = 1559,

    /// ai。
    AI = 1305,
    /// ai9。
    AI9 = 1333,
    /// 在 1。
    AT1 = 1303,
    /// avi。
    AVI = 1536,
    // ---- 位图格式 ----
    /// bmp。
    BMP = 769,
    /// cal。
    CAL = 800,
    // ---- CorelDRAW 格式 ----
    /// cdr。
    CDR = 1795,
    /// cdt。
    CDT = 1800,

    /// cdx。
    CDX = 1796,
    /// cgm。
    CGM = 1280,
    /// clk。
    CLK = 1802,
    /// cmx5。
    CMX5 = 1794,
    /// cmx6。
    CMX6 = 1793,
    /// cpt。
    CPT = 1808,
    /// cpt8。
    CPT8 = 1799,
    /// cpx。
    CPX = 1797,
    /// cur。
    CUR = 785,
    /// dcs。
    DCS = 801,
    /// 文档。
    DOC = 2068,
    /// drw。
    DRW = 1339,

    /// dwg。
    DWG = 1328,

    /// dxf。
    DXF = 1296,

    /// emf。
    EMF = 1300,

    /// EPS。
    EPS = 1289,
    /// EPSphotopaint。
    EPSPhotoPaint = 804,
    /// exe。
    EXE = 786,
    /// fh。
    FH = 1344,
    /// fmv。
    FMV = 1329,
    /// fpx。
    FPX = 806,
    /// gem。
    GEM = 1284,

    /// gif。
    GIF = 773,
    /// gifanimation。
    GIFAnimation = 1558,
    /// hpgl。
    HPGL = 1281,
    /// HTML。
    HTML = 1557,
    /// ico。
    ICO = 784,
    /// img。
    IMG = 787,
    /// jp2。
    JP2 = 820,
    /// jpc。
    JPC = 821,

    /// JPEG。
    JPEG = 774,
    /// jpg2000 导入。
    JPG2000Import = 818,
    /// mac。
    MAC = 791,
    /// macword5。
    MacWord5 = 2052,
    /// met。
    MET = 1291,
    /// mif。
    MIF = 2165,
    /// nap。
    NAP = 1292,
    /// os2bmp。
    OS2BMP = 792,
    /// pat。
    PAT = 1801,
    /// pcd。
    PCD = 775,

    /// pcx。
    PCX = 770,
    /// pic。
    PIC = 1334,
    /// pict。
    PICT = 1293,
    // 775..789 待补
    /// png。
    PNG = 802,
    /// pp4。
    PP4 = 789,
    /// pp5。
    PP5 = 803,
    /// ppf。
    PPF = 819,
    /// ppt。
    PPT = 1548,

    /// psd。
    PSD = 788,
    /// psinterpreted。
    PSInterpreted = 1290,
    /// qtm。
    QTM = 1542,
    /// qtvr。
    QTVR = 1560,
    /// quattropro。
    QuattroPro = 2067,
    /// riff。
    RIFF = 807,
    /// rtf。
    RTF = 2053,
    /// sam。
    SAM = 2063,
    /// sct。
    SCT = 776,
    /// shw。
    SHW = 1806,
    // ---- 矢量格式 ----
    /// svg。
    SVG = 1345,
    /// svgz。
    SVGZ = 1347,
    /// swf。
    SWF = 1343,

    /// tga。
    TGA = 771,

    /// tiff。
    TIFF = 772,
    /// ttf。
    TTF = 1302,
    /// txt。
    TXT = 2048,
    /// vsd。
    VSD = 1315,
    /// wi。
    WI = 793,
    /// wks。
    WKS = 2066,

    /// wmf。
    WMF = 1294,
    /// word2。
    Word2 = 2050,
    /// word55。
    Word55 = 2051,
    /// word95。
    Word95 = 2049,
    /// wp4。
    WP4 = 2058,
    /// wp50。
    WP50 = 2057,
    /// wp51。
    WP51 = 2056,
    /// wp9。
    WP9 = 2055,
    /// wpg。
    WPG = 1287,
    /// wpm。
    WPM = 2166,
    /// ws4。
    WS4 = 2164,
    /// ws6。
    WS6 = 2163,
    /// ws7。
    WS7 = 2060,
    /// ws2000。
    WS2000 = 2061,
    /// wsw。
    WSW = 2059,
    /// xcf。
    XCF = 816,
    /// xls。
    XLS = 2064,
    /// xpm。
    XPM = 809,
    /// XY。
    XY = 2062,
    /// csl。
    CSL = 1810,
    /// des。
    DES = 1805,
    /// psp。
    PSP = 822,
    /// 原始。
    RAW = 823,
    /// cgz。
    CGZ = 2315,
    /// cmx64。
    CMX64 = 1811,
    /// pub。
    PUB = 1349,
    /// heif。
    HEIF = 824,
    /// webp。
    WEBP = 825,
}

impl cdrFilter {
    /// cpt10（`CPT10`）。
    pub const CPT10: Self = Self::CPT;
    /// cpt11（`CPT11`）。
    pub const CPT11: Self = Self::CPT;
    /// cpt9（`CPT9`）。
    pub const CPT9: Self = Self::CPT;
    /// dsf（`DSF`）。
    pub const DSF: Self = Self::DRW;
    /// PDF（`PDF`）。
    pub const PDF: Self = Self::AI9;
    /// plt（`PLT`）。
    pub const PLT: Self = Self::HPGL;
    /// psencapsulated（`PSEncapsulated`）。
    pub const PSEncapsulated: Self = Self::EPS;
    /// word2000（`Word2000`）。
    pub const Word2000: Self = Self::DOC;
    /// wpd（`WPD`）。
    pub const WPD: Self = Self::WP9;
    /// wsd（`WSD`）。
    pub const WSD: Self = Self::WS2000;
    /// wvl（`WVL`）。
    pub const WVL: Self = Self::WI;
    /// 由整数值构造枚举。
    pub fn from_i32(v: i32) -> Option<Self> {
        match v {
            0 => Some(Self::AutoSense),
            1559 => Some(Self::V_3DMF),
            1305 => Some(Self::AI),
            1333 => Some(Self::AI9),
            1303 => Some(Self::AT1),
            1536 => Some(Self::AVI),
            769 => Some(Self::BMP),
            800 => Some(Self::CAL),
            1795 => Some(Self::CDR),
            1800 => Some(Self::CDT),
            1796 => Some(Self::CDX),
            1280 => Some(Self::CGM),
            1802 => Some(Self::CLK),
            1794 => Some(Self::CMX5),
            1793 => Some(Self::CMX6),
            1808 => Some(Self::CPT),
            1799 => Some(Self::CPT8),
            1797 => Some(Self::CPX),
            785 => Some(Self::CUR),
            801 => Some(Self::DCS),
            2068 => Some(Self::DOC),
            1339 => Some(Self::DRW),
            1328 => Some(Self::DWG),
            1296 => Some(Self::DXF),
            1300 => Some(Self::EMF),
            1289 => Some(Self::EPS),
            804 => Some(Self::EPSPhotoPaint),
            786 => Some(Self::EXE),
            1344 => Some(Self::FH),
            1329 => Some(Self::FMV),
            806 => Some(Self::FPX),
            1284 => Some(Self::GEM),
            773 => Some(Self::GIF),
            1558 => Some(Self::GIFAnimation),
            1281 => Some(Self::HPGL),
            1557 => Some(Self::HTML),
            784 => Some(Self::ICO),
            787 => Some(Self::IMG),
            820 => Some(Self::JP2),
            821 => Some(Self::JPC),
            774 => Some(Self::JPEG),
            818 => Some(Self::JPG2000Import),
            791 => Some(Self::MAC),
            2052 => Some(Self::MacWord5),
            1291 => Some(Self::MET),
            2165 => Some(Self::MIF),
            1292 => Some(Self::NAP),
            792 => Some(Self::OS2BMP),
            1801 => Some(Self::PAT),
            775 => Some(Self::PCD),
            770 => Some(Self::PCX),
            1334 => Some(Self::PIC),
            1293 => Some(Self::PICT),
            802 => Some(Self::PNG),
            789 => Some(Self::PP4),
            803 => Some(Self::PP5),
            819 => Some(Self::PPF),
            1548 => Some(Self::PPT),
            788 => Some(Self::PSD),
            1290 => Some(Self::PSInterpreted),
            1542 => Some(Self::QTM),
            1560 => Some(Self::QTVR),
            2067 => Some(Self::QuattroPro),
            807 => Some(Self::RIFF),
            2053 => Some(Self::RTF),
            2063 => Some(Self::SAM),
            776 => Some(Self::SCT),
            1806 => Some(Self::SHW),
            1345 => Some(Self::SVG),
            1347 => Some(Self::SVGZ),
            1343 => Some(Self::SWF),
            771 => Some(Self::TGA),
            772 => Some(Self::TIFF),
            1302 => Some(Self::TTF),
            2048 => Some(Self::TXT),
            1315 => Some(Self::VSD),
            793 => Some(Self::WI),
            2066 => Some(Self::WKS),
            1294 => Some(Self::WMF),
            2050 => Some(Self::Word2),
            2051 => Some(Self::Word55),
            2049 => Some(Self::Word95),
            2058 => Some(Self::WP4),
            2057 => Some(Self::WP50),
            2056 => Some(Self::WP51),
            2055 => Some(Self::WP9),
            1287 => Some(Self::WPG),
            2166 => Some(Self::WPM),
            2164 => Some(Self::WS4),
            2163 => Some(Self::WS6),
            2060 => Some(Self::WS7),
            2061 => Some(Self::WS2000),
            2059 => Some(Self::WSW),
            816 => Some(Self::XCF),
            2064 => Some(Self::XLS),
            809 => Some(Self::XPM),
            2062 => Some(Self::XY),
            1810 => Some(Self::CSL),
            1805 => Some(Self::DES),
            822 => Some(Self::PSP),
            823 => Some(Self::RAW),
            2315 => Some(Self::CGZ),
            1811 => Some(Self::CMX64),
            1349 => Some(Self::PUB),
            824 => Some(Self::HEIF),
            825 => Some(Self::WEBP),
            _ => None,
        }
    }
}

// =============================================================
// cdrFileVersion —— 文件版本
// =============================================================

/// CorelDRAW 枚举 `cdrFileVersion`：文件版本。
///
/// 对应类型库中的 `VGCore::cdrFileVersion`，用 `#[repr(i32)]` 映射实际整数值。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum cdrFileVersion {
    /// current 版本。
    CurrentVersion = 0,
    // TODO: 待补
    /// version5。
    Version5 = 5,

    /// version6。
    Version6 = 6,

    /// version7。
    Version7 = 7,

    /// version8。
    Version8 = 8,

    /// version9。
    Version9 = 9,

    /// version10。
    Version10 = 10,

    /// version11。
    Version11 = 11,

    /// version12。
    Version12 = 12,

    /// version13。
    Version13 = 13,
    /// version1。
    Version1 = 1,
    /// version2。
    Version2 = 2,
    /// version3。
    Version3 = 3,
    /// version4。
    Version4 = 4,
    /// version105。
    Version105 = 105,

    /// version14。
    Version14 = 14,

    /// version15。
    Version15 = 15,

    /// version16。
    Version16 = 16,

    /// version17。
    Version17 = 17,

    /// version18。
    Version18 = 18,

    /// version19。
    Version19 = 19,

    /// version20。
    Version20 = 20,

    /// version21。
    Version21 = 21,

    /// version22。
    Version22 = 22,

    /// version23。
    Version23 = 23,

    /// version24。
    Version24 = 24,
    /// version25。
    Version25 = 25,
    /// version26。
    Version26 = 26,
}

// =============================================================
// cdrExportRange —— 导出范围
// =============================================================

/// CorelDRAW 枚举 `cdrExportRange`：导出范围。
///
/// 对应类型库中的 `VGCore::cdrExportRange`，用 `#[repr(i32)]` 映射实际整数值。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum cdrExportRange {
    /// 全部页面集。
    AllPages = 0,

    /// current 页面。
    CurrentPage = 1,

    /// 选择。
    Selection = 2,
}

// =============================================================
// cdrThumbnailSize —— 缩略图大小
// =============================================================

/// CorelDRAW 枚举 `cdrThumbnailSize`：缩略图大小。
///
/// 对应类型库中的 `VGCore::cdrThumbnailSize`，用 `#[repr(i32)]` 映射实际整数值。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum cdrThumbnailSize {
    /// 无 thumbnail。
    NoThumbnail = 0,
    /// 1k 单色 thumbnail。
    V_1KMonoThumbnail = 1,
    /// 5k 颜色 thumbnail。
    V_5KColorThumbnail = 2,
    /// 10k 颜色 thumbnail。
    V_10KColorThumbnail = 3,
}

// =============================================================
// cdrCompressionType —— 压缩类型
// =============================================================

/// 来源：`PHOTOPAINT::cdrCompressionType`
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum cdrCompressionType {
    /// 压缩无。
    CompressionNone = 0,
    /// 压缩 lzw。
    CompressionLZW = 1,
    /// 压缩 packbits。
    CompressionPackBits = 2,
    /// 压缩 huffman。
    CompressionHuffman = 3,
    /// 压缩 ccitt31d。
    CompressionCCITT3_1d = 4,
    /// 压缩 ccitt32d。
    CompressionCCITT3_2d = 5,
    /// 压缩 ccitt4。
    CompressionCCITT4 = 6,
    /// 压缩 rlelw。
    CompressionRLE_LW = 7,
    /// 压缩 JPEG。
    CompressionJPEG = 8,
}

impl cdrCompressionType {
    /// 压缩 zip（`CompressionZIP`）。
    pub const CompressionZIP: Self = Self::CompressionRLE_LW;
    /// 由整数值构造枚举。
    pub fn from_i32(v: i32) -> Option<Self> {
        match v {
            0 => Some(Self::CompressionNone),
            1 => Some(Self::CompressionLZW),
            2 => Some(Self::CompressionPackBits),
            3 => Some(Self::CompressionHuffman),
            4 => Some(Self::CompressionCCITT3_1d),
            5 => Some(Self::CompressionCCITT3_2d),
            6 => Some(Self::CompressionCCITT4),
            7 => Some(Self::CompressionRLE_LW),
            8 => Some(Self::CompressionJPEG),
            _ => None,
        }
    }
}

// =============================================================
// cdrDataFormatType / cdrDataType —— 数据字段类型
// =============================================================

/// CorelDRAW 枚举 `cdrDataFormatType`（数据格式类型）。
///
/// 对应类型库中的 `VGCore::cdrDataFormatType`，用 `#[repr(i32)]` 映射实际整数值。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum cdrDataFormatType {
    /// 格式 general。
    FormatGeneral = 0,
    /// 格式日期 time。
    FormatDateTime = 1,
    /// 格式线性 angular。
    FormatLinearAngular = 2,
    /// 格式 numeric。
    FormatNumeric = 3,
}

/// CorelDRAW 枚举 `cdrDataType`（数据类型）。
///
/// 对应类型库中的 `VGCore::cdrDataType`，用 `#[repr(i32)]` 映射实际整数值。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum cdrDataType {
    /// 数据类型字符串。
    DataTypeString = 0,
    /// 数据类型 number。
    DataTypeNumber = 1,
    /// 数据类型 event。
    DataTypeEvent = 2,
    /// 数据类型动作。
    DataTypeAction = 3,
}
