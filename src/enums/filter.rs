//! 文件过滤 / 格式枚举
//!
//! ⚠️ `cdrFilter` 有 **100+ 个值**，此处列常用项。
//! 完整列表请用 `oleview.exe` 打开 `VGCore.tlb` 导出。

// =============================================================
// cdrFilter —— 文件格式过滤器
// =============================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum cdrFilter {
    /// 自动检测（仅导入）。
    AutoSense = 0,
    V_3DMF = 1559,

    AI = 1305,
    AI9 = 1333,
    AT1 = 1303,
    AVI = 1536,
    // ---- 位图格式 ----
    BMP = 769,
    CAL = 800,
    // ---- CorelDRAW 格式 ----
    CDR = 1795,
    CDT = 1800,

    CDX = 1796,
    CGM = 1280,
    CLK = 1802,
    CMX5 = 1794,
    CMX6 = 1793,
    CPT = 1808,
    CPT8 = 1799,
    CPX = 1797,
    CUR = 785,
    DCS = 801,
    DOC = 2068,
    DRW = 1339,

    DWG = 1328,

    DXF = 1296,

    EMF = 1300,

    EPS = 1289,
    EPSPhotoPaint = 804,
    EXE = 786,
    FH = 1344,
    FMV = 1329,
    FPX = 806,
    GEM = 1284,

    GIF = 773,
    GIFAnimation = 1558,
    HPGL = 1281,
    HTML = 1557,
    ICO = 784,
    IMG = 787,
    JP2 = 820,
    JPC = 821,

    JPEG = 774,
    JPG2000Import = 818,
    MAC = 791,
    MacWord5 = 2052,
    MET = 1291,
    MIF = 2165,
    NAP = 1292,
    OS2BMP = 792,
    PAT = 1801,
    PCD = 775,

    PCX = 770,
    PIC = 1334,
    PICT = 1293,
    // 775..789 待补
    PNG = 802,
    PP4 = 789,
    PP5 = 803,
    PPF = 819,
    PPT = 1548,

    PSD = 788,
    PSInterpreted = 1290,
    QTM = 1542,
    QTVR = 1560,
    QuattroPro = 2067,
    RIFF = 807,
    RTF = 2053,
    SAM = 2063,
    SCT = 776,
    SHW = 1806,
    // ---- 矢量格式 ----
    SVG = 1345,
    SVGZ = 1347,
    SWF = 1343,

    TGA = 771,

    TIFF = 772,
    TTF = 1302,
    TXT = 2048,
    VSD = 1315,
    WI = 793,
    WKS = 2066,

    WMF = 1294,
    Word2 = 2050,
    Word55 = 2051,
    Word95 = 2049,
    WP4 = 2058,
    WP50 = 2057,
    WP51 = 2056,
    WP9 = 2055,
    WPG = 1287,
    WPM = 2166,
    WS4 = 2164,
    WS6 = 2163,
    WS7 = 2060,
    WS2000 = 2061,
    WSW = 2059,
    XCF = 816,
    XLS = 2064,
    XPM = 809,
    XY = 2062,
    CSL = 1810,
    DES = 1805,
    PSP = 822,
    RAW = 823,
    CGZ = 2315,
    CMX64 = 1811,
    PUB = 1349,
    HEIF = 824,
    WEBP = 825,
}

impl cdrFilter {
    pub const CPT10: Self = Self::CPT;
    pub const CPT11: Self = Self::CPT;
    pub const CPT9: Self = Self::CPT;
    pub const DSF: Self = Self::DRW;
    pub const PDF: Self = Self::AI9;
    pub const PLT: Self = Self::HPGL;
    pub const PSEncapsulated: Self = Self::EPS;
    pub const Word2000: Self = Self::DOC;
    pub const WPD: Self = Self::WP9;
    pub const WSD: Self = Self::WS2000;
    pub const WVL: Self = Self::WI;
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum cdrFileVersion {
    CurrentVersion = 0,
    // TODO: 待补
    Version5 = 5,

    Version6 = 6,

    Version7 = 7,

    Version8 = 8,

    Version9 = 9,

    Version10 = 10,

    Version11 = 11,

    Version12 = 12,

    Version13 = 13,
    Version1 = 1,
    Version2 = 2,
    Version3 = 3,
    Version4 = 4,
    Version105 = 105,

    Version14 = 14,

    Version15 = 15,

    Version16 = 16,

    Version17 = 17,

    Version18 = 18,

    Version19 = 19,

    Version20 = 20,

    Version21 = 21,

    Version22 = 22,

    Version23 = 23,

    Version24 = 24,
    Version25 = 25,
    Version26 = 26,
}

// =============================================================
// cdrExportRange —— 导出范围
// =============================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum cdrExportRange {
    AllPages = 0,

    CurrentPage = 1,

    Selection = 2,
}

// =============================================================
// cdrThumbnailSize —— 缩略图大小
// =============================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum cdrThumbnailSize {
    NoThumbnail = 0,
    V_1KMonoThumbnail = 1,
    V_5KColorThumbnail = 2,
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
    CompressionNone = 0,
    CompressionLZW = 1,
    CompressionPackBits = 2,
    CompressionHuffman = 3,
    CompressionCCITT3_1d = 4,
    CompressionCCITT3_2d = 5,
    CompressionCCITT4 = 6,
    CompressionRLE_LW = 7,
    CompressionJPEG = 8,
}

impl cdrCompressionType {
    pub const CompressionZIP: Self = Self::CompressionRLE_LW;
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum cdrDataFormatType {
    FormatGeneral = 0,
    FormatDateTime = 1,
    FormatLinearAngular = 2,
    FormatNumeric = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum cdrDataType {
    DataTypeString = 0,
    DataTypeNumber = 1,
    DataTypeEvent = 2,
    DataTypeAction = 3,
}
