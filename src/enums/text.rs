//! 文本枚举

// =============================================================
// cdrAlignment —— 文本对齐
// =============================================================

/// 来源：`VGCore::cdrAlignment`
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum cdrAlignment {
    NoAlignment        = 0,
    LeftAlignment      = 1,
    RightAlignment     = 2,
    CenterAlignment    = 3,
    FullJustifyAlignment = 4,
    ForceJustifyAlignment = 5,
    MixedAlignment     = 6,
}

impl cdrAlignment {
    pub fn from_i32(v: i32) -> Option<Self> {
        match v {
            0 => Some(Self::NoAlignment),
            1 => Some(Self::LeftAlignment),
            2 => Some(Self::RightAlignment),
            3 => Some(Self::CenterAlignment),
            4 => Some(Self::FullJustifyAlignment),
            5 => Some(Self::ForceJustifyAlignment),
            6 => Some(Self::MixedAlignment),
            _ => None,
        }
    }
}

// =============================================================
// cdrFontStyle —— 字体样式
// =============================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum cdrFontStyle {
    Normal          = 0,
    Bold            = 1,
    Italic          = 2,
    BoldItalic      = 3,
    Thin            = 4,
    ThinItalic      = 5,
    ExtraLight      = 6,
    // TODO: 6..12 待补
    Medium          = 13,
    SemiBold        = 14,
    // TODO: 完整列表待补
}

impl cdrFontStyle {
    pub fn from_i32(v: i32) -> Option<Self> {
        match v {
            0  => Some(Self::Normal),
            1  => Some(Self::Bold),
            2  => Some(Self::Italic),
            3  => Some(Self::BoldItalic),
            4  => Some(Self::Thin),
            5  => Some(Self::ThinItalic),
            6  => Some(Self::ExtraLight),
            13 => Some(Self::Medium),
            14 => Some(Self::SemiBold),
            _  => None,
        }
    }
}

// =============================================================
// cdrFontLine —— 字体线条（下划线 / 删除线等）
// =============================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum cdrFontLine {
    None         = 0,
    SingleThin   = 1,
    SingleThick  = 2,
    DoubleThin   = 3,
    DoubleThick  = 4,
    MixedFontLine = 5,
    // TODO: 待补
}

// =============================================================
// cdrFontCase —— 大小写
// =============================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum cdrFontCase {
    Normal    = 0,
    SmallCaps = 1,
    AllCaps   = 2,
}

// =============================================================
// cdrFontPosition —— 字体位置（上标 / 下标）
// =============================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum cdrFontPosition {
    Normal    = 0,
    Subscript = 1,
    Superscript = 2,
}

// =============================================================
// cdrTextType —— 文本类型
// =============================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum cdrTextType {
    Artistic  = 1,
    Paragraph = 2,
}

// =============================================================
// cdrTextFrames —— 文本框范围
// =============================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum cdrTextFrames {
    ThisFrameOnly     = 0,
    StartAtThisFrame  = 1,
    AllFrames         = 2,
}

impl cdrTextFrames {
    pub fn from_i32(v: i32) -> Option<Self> {
        match v {
            0 => Some(Self::ThisFrameOnly),
            1 => Some(Self::StartAtThisFrame),
            2 => Some(Self::AllFrames),
            _ => None,
        }
    }
}

// =============================================================
// cdrTextIndexingType —— 文本索引类型
// =============================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum cdrTextIndexingType {
    CharacterIndexing = 0,
    WordIndexing      = 1,
    LineIndexing      = 2,
    ParagraphIndexing = 3,
    // TODO: 待补
}

// =============================================================
// cdrTextCharSet —— 字符集
// =============================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum cdrTextCharSet {
    /// 默认字符集。
    CharSetDefault = 0,
    /// ASCII。
    CharSetASCII   = 1,
    /// Mac Roman。
    CharSetMacRoman = 2,
    /// Unicode。
    CharSetUnicode = 3,
    /// 混合。
    CharSetMixed   = 4,
    // TODO: 待补
}

// =============================================================
// cdrTextLanguage —— 文本语言
// =============================================================

/// 文本语言码。
///
/// ⚠️ 该枚举在 VGCore 类型库里有 **200+ 个值**，此处只列常见项，
/// 完整列表请用 `oleview.exe` 打开 `VGCore.tlb` 导出。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum cdrTextLanguage {
    /// 无语言。
    LanguageNone     = 0,
    /// 英语（美国）。
    EnglishUS        = 1033,
    /// 英语（英国）。
    EnglishUK        = 2057,
    /// 简体中文。
    ChineseSimplified = 2052,
    /// 繁体中文。
    ChineseTraditional = 1028,
    /// 中文（新加坡）。
    ChineseSingapore = 4100,
    /// 日语。
    Japanese         = 1041,
    /// 韩语。
    Korean           = 1042,
    /// 德语。
    German           = 1031,
    /// 法语。
    French           = 1036,
    /// 西班牙语。
    Spanish          = 1034,
    /// 意大利语。
    Italian          = 1040,
    /// 葡萄牙语（巴西）。
    PortugueseBrazil = 1046,
    /// 俄语。
    Russian          = 1049,
    // TODO: 完整列表待补
}

impl cdrTextLanguage {
    pub fn from_i32(v: i32) -> Option<Self> {
        match v {
            0    => Some(Self::LanguageNone),
            1033 => Some(Self::EnglishUS),
            2057 => Some(Self::EnglishUK),
            2052 => Some(Self::ChineseSimplified),
            1028 => Some(Self::ChineseTraditional),
            4100 => Some(Self::ChineseSingapore),
            1041 => Some(Self::Japanese),
            1042 => Some(Self::Korean),
            1031 => Some(Self::German),
            1036 => Some(Self::French),
            1034 => Some(Self::Spanish),
            1040 => Some(Self::Italian),
            1046 => Some(Self::PortugueseBrazil),
            1049 => Some(Self::Russian),
            _    => None,
        }
    }
}

// =============================================================
// cdrTextEffect —— 文本效果
// =============================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum cdrTextEffect {
    None     = 0,
    Bullet   = 1,
    DropCap  = 2,
}

// =============================================================
// cdrTextChangeCase —— 文本大小写转换
// =============================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum cdrTextChangeCase {
    Uppercase    = 0,
    Lowercase    = 1,
    TitleCase    = 2,
    SentenceCase = 3,
    ToggleCase   = 4,
}

// =============================================================
// cdrTextTabAlignment —— 制表位对齐
// =============================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum cdrTextTabAlignment {
    Left    = 0,
    Right   = 1,
    Center  = 2,
    Decimal = 3,
}

// =============================================================
// cdrVerticalAlignment —— 垂直对齐
// =============================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum cdrVerticalAlignment {
    Top     = 0,
    Center  = 1,
    Bottom  = 2,
    Full    = 3,
}

// =============================================================
// cdrLineSpacingType —— 行距类型
// =============================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum cdrLineSpacingType {
    PercentOfCharacterHeight = 0,
    Points                   = 1,
    PercentOfPointSize       = 2,
    // TODO: 待补
}

// =============================================================
// cdrTextPropertySet —— 文本属性集（用于 EnumRanges）
// =============================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum cdrTextPropertySet {
    // TODO: 待补
    All = 0,
}