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

    // ---- 位图格式 ----
    BMP  = 769,
    PCX  = 770,
    TGA  = 771,
    TIFF = 772,
    GIF  = 773,
    JPEG = 774,
    // 775..789 待补
    PNG  = 790,
    PSD  = 791,
    // ...

    // ---- CorelDRAW 格式 ----
    CDR  = 1795,
    CDX  = 1796,
    CMX  = 1797,
    // ...

    // ---- 矢量格式 ----
    SVG  = 1900,
    PDF  = 1901,
    AI   = 1902,
    EPS  = 1903,
    WMF  = 1904,
    EMF  = 1905,
    DXF  = 1906,
    DWG  = 1907,
    // ...

    // TODO: 完整列表待补
}

impl cdrFilter {
    pub fn from_i32(v: i32) -> Option<Self> {
        match v {
            0    => Some(Self::AutoSense),
            769  => Some(Self::BMP),
            770  => Some(Self::PCX),
            771  => Some(Self::TGA),
            772  => Some(Self::TIFF),
            773  => Some(Self::GIF),
            774  => Some(Self::JPEG),
            790  => Some(Self::PNG),
            791  => Some(Self::PSD),
            1795 => Some(Self::CDR),
            1796 => Some(Self::CDX),
            1797 => Some(Self::CMX),
            1900 => Some(Self::SVG),
            1901 => Some(Self::PDF),
            1902 => Some(Self::AI),
            1903 => Some(Self::EPS),
            1904 => Some(Self::WMF),
            1905 => Some(Self::EMF),
            1906 => Some(Self::DXF),
            1907 => Some(Self::DWG),
            _    => None,
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
    // TODO: 待补
    Version5  = 5,
    Version6  = 6,
    Version7  = 7,
    Version8  = 8,
    Version9  = 9,
    Version10 = 10,
    Version11 = 11,
    Version12 = 12,
    Version13 = 13,
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
    // ...
}

// =============================================================
// cdrExportRange —— 导出范围
// =============================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum cdrExportRange {
    CurrentPage  = 0,
    AllPages     = 1,
    Selection    = 2,
    // TODO: 待补
}

// =============================================================
// cdrThumbnailSize —— 缩略图大小
// =============================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum cdrThumbnailSize {
    NoThumbnail        = 0,
    Size1K             = 1,
    Size5K             = 2,
    Size10K            = 3,
    Size10KColor       = 4,
    Size5KColor        = 5,
    // TODO: 待补
}

// =============================================================
// cdrCompressionType —— 压缩类型
// =============================================================

/// 来源：`PHOTOPAINT::cdrCompressionType`
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum cdrCompressionType {
    None          = 0,
    LZW           = 1,
    PackBits      = 2,
    Huffman       = 3,
    CCITT3_1d     = 4,
    // 5..7 待补
    JPEG          = 8,
    // TODO: 待补
}

impl cdrCompressionType {
    pub fn from_i32(v: i32) -> Option<Self> {
        match v {
            0 => Some(Self::None),
            1 => Some(Self::LZW),
            2 => Some(Self::PackBits),
            3 => Some(Self::Huffman),
            4 => Some(Self::CCITT3_1d),
            8 => Some(Self::JPEG),
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
    // TODO: 待补
    Default = 0,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum cdrDataType {
    // TODO: 待补
    String = 0,
    Number = 1,
}