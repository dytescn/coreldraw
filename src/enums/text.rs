//! 文本枚举

// =============================================================
// cdrAlignment —— 文本对齐
// =============================================================

/// 来源：`VGCore::cdrAlignment`
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum cdrAlignment {
    NoAlignment = 0,
    LeftAlignment = 1,
    RightAlignment = 2,
    CenterAlignment = 3,
    FullJustifyAlignment = 4,
    ForceJustifyAlignment = 5,
    MixedAlignment = 6,
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
    NormalFontStyle = 0,
    BoldFontStyle = 1,
    ItalicFontStyle = 2,
    BoldItalicFontStyle = 3,
    ThinFontStyle = 4,
    ThinItalicFontStyle = 5,
    ExtraLightFontStyle = 6,
    ExtraLightItalicFontStyle = 7,
    MediumFontStyle = 8,
    MediumItalicFontStyle = 9,
    SemiBoldFontStyle = 10,
    SemiBoldItalicFontStyle = 11,
    ExtraBoldFontStyle = 12,
    ExtraBoldItalicFontStyle = 13,
    HeavyFontStyle = 14,
    HeavyItalicFontStyle = 15,
    MixedFontStyle = 16,
    LightFontStyle = 17,
    LightItalicFontStyle = 18,
}

impl cdrFontStyle {
    pub fn from_i32(v: i32) -> Option<Self> {
        match v {
            0 => Some(Self::NormalFontStyle),
            1 => Some(Self::BoldFontStyle),
            2 => Some(Self::ItalicFontStyle),
            3 => Some(Self::BoldItalicFontStyle),
            4 => Some(Self::ThinFontStyle),
            5 => Some(Self::ThinItalicFontStyle),
            6 => Some(Self::ExtraLightFontStyle),
            7 => Some(Self::ExtraLightItalicFontStyle),
            8 => Some(Self::MediumFontStyle),
            9 => Some(Self::MediumItalicFontStyle),
            10 => Some(Self::SemiBoldFontStyle),
            11 => Some(Self::SemiBoldItalicFontStyle),
            12 => Some(Self::ExtraBoldFontStyle),
            13 => Some(Self::ExtraBoldItalicFontStyle),
            14 => Some(Self::HeavyFontStyle),
            15 => Some(Self::HeavyItalicFontStyle),
            16 => Some(Self::MixedFontStyle),
            17 => Some(Self::LightFontStyle),
            18 => Some(Self::LightItalicFontStyle),
            _ => None,
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
    NoFontLine = 0,
    SingleThinFontLine = 1,
    SingleThinWordFontLine = 2,
    SingleThickFontLine = 3,
    SingleThickWordFontLine = 4,
    DoubleThinFontLine = 5,
    DoubleThinWordFontLine = 6,

    MixedFontLine = 7,
}

// =============================================================
// cdrFontCase —— 大小写
// =============================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum cdrFontCase {
    Normal = 0,
    SmallCaps = 1,
    AllCaps = 2,
}

// =============================================================
// cdrFontPosition —— 字体位置（上标 / 下标）
// =============================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum cdrFontPosition {
    Normal = 0,
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
    Artistic = 0,
    Paragraph = 1,
    ArtisticFitted = 2,
    ParagraphFitted = 3,
}

impl cdrTextType {
    pub fn from_i32(v: i32) -> Option<Self> {
        match v {
            0 => Some(Self::Artistic),
            1 => Some(Self::Paragraph),
            2 => Some(Self::ArtisticFitted),
            3 => Some(Self::ParagraphFitted),
            _ => None,
        }
    }
}

// =============================================================
// cdrTextFrames —— 文本框范围
// =============================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum cdrTextFrames {
    ThisFrameOnly = 0,
    StartAtThisFrame = 1,
    AllFrames = 2,
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

    WordIndexing = 1,

    ParagraphIndexing = 2,
}

// =============================================================
// cdrTextCharSet —— 字符集
// =============================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum cdrTextCharSet {
    /// 混合。
    CharSetMixed = -1,
    CharSetANSI = 0,
    /// 默认字符集。
    CharSetDefault = 1,
    CharSetSymbol = 2,
    CharSetShiftJIS = 128,
    CharSetHangul = 129,
    CharSetChineseBig5 = 136,
    CharSetOEM = 255,
    CharSetJohab = 130,
    CharSetHebrew = 177,
    CharSetArabic = 178,
    CharSetGreek = 161,
    CharSetTurkish = 162,
    CharSetVietnamese = 163,
    CharSetThai = 222,
    CharSetEastEurope = 238,
    CharSetRussian = 204,
    CharSetMac = 77,
    CharSetBaltic = 186,
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
    LanguageMixed = -1,
    /// 无语言。
    LanguageNone = 0,
    Afrikaans = 1078,
    Albanian = 1052,
    ArabicAlgeria = 5121,
    ArabicBahrain = 15361,
    ArabicEgypt = 3073,
    ArabicIraq = 2049,
    ArabicJordan = 11265,
    ArabicKuwait = 13313,
    ArabicLebanon = 12289,
    ArabicLibya = 4097,
    ArabicMorocco = 6145,
    ArabicOman = 8193,
    ArabicQatar = 16385,
    Arabic = 1025,
    ArabicSyria = 10241,
    ArabicTunisia = 7169,
    ArabicUAE = 14337,
    ArabicYemen = 9217,
    Armenian = 1067,
    Assamese = 1101,
    AzeriCyrillic = 2092,
    AzeriLatin = 1068,
    Basque = 1069,
    Byelorussian = 1059,
    Bengali = 1093,
    Bulgarian = 1026,
    Burmese = 1109,
    Catalan = 1027,
    ChineseHongKong = 3076,
    ChineseMacao = 5124,
    SimplifiedChinese = 2052,
    /// 中文（新加坡）。
    ChineseSingapore = 4100,
    TraditionalChinese = 1028,
    Croatian = 1050,
    Czech = 1029,
    Danish = 1030,
    BelgianDutch = 2067,
    Dutch = 1043,
    EnglishAUS = 3081,
    EnglishBelize = 10249,
    EnglishCanadian = 4105,
    EnglishCaribbean = 9225,
    EnglishIreland = 6153,
    EnglishJamaica = 8201,
    EnglishNewZealand = 5129,
    EnglishPhilippines = 13321,
    EnglishSouthAfrica = 7177,
    EnglishTrinidad = 11273,
    /// 英语（英国）。
    EnglishUK = 2057,
    /// 英语（美国）。
    EnglishUS = 1033,
    EnglishZimbabwe = 12297,
    Estonian = 1061,
    Faeroese = 1080,
    Farsi = 1065,
    Finnish = 1035,
    BelgianFrench = 2060,
    FrenchCameroon = 11276,
    FrenchCanadian = 3084,
    FrenchCotedIvoire = 12300,
    /// 法语。
    French = 1036,
    FrenchLuxembourg = 5132,
    FrenchMali = 13324,
    FrenchMonaco = 6156,
    FrenchReunion = 8204,
    FrenchSenegal = 10252,
    SwissFrench = 4108,
    FrenchWestIndies = 7180,
    FrenchZaire = 9228,
    FrisianNetherlands = 1122,
    GaelicIreland = 2108,
    GaelicScotland = 1084,
    Galician = 1110,
    Georgian = 1079,
    GermanAustria = 3079,
    /// 德语。
    German = 1031,
    GermanLiechtenstein = 5127,
    GermanLuxembourg = 4103,
    SwissGerman = 2055,
    Greek = 1032,
    Gujarati = 1095,
    Hebrew = 1037,
    Hindi = 1081,
    Hungarian = 1038,
    Icelandic = 1039,
    Indonesian = 1057,
    /// 意大利语。
    Italian = 1040,
    SwissItalian = 2064,
    /// 日语。
    Japanese = 1041,
    Kannada = 1099,
    Kashmiri = 1120,
    Kazakh = 1087,
    Khmer = 1107,
    Kirghiz = 1088,
    Konkani = 1111,
    /// 韩语。
    Korean = 1042,
    Lao = 1108,
    Latvian = 1062,
    Lithuanian = 1063,
    Macedonian = 1071,
    Malaysian = 1086,
    MalayBruneiDarussalam = 2110,
    Malayalam = 1100,
    Maltese = 1082,
    Manipuri = 1112,
    Marathi = 1102,
    Mongolian = 1104,
    Nepali = 1121,
    NorwegianBokmol = 1044,
    NorwegianNynorsk = 2068,
    Oriya = 1096,
    Polish = 1045,
    BrazilianPortuguese = 1046,
    Portuguese = 2070,
    Punjabi = 1094,
    RhaetoRomanic = 1047,
    RomanianMoldova = 2072,
    Romanian = 1048,
    RussianMoldova = 2073,
    /// 俄语。
    Russian = 1049,
    SamiLappish = 1083,
    Sanskrit = 1103,
    SerbianCyrillic = 3098,
    SerbianLatin = 2074,
    Sindhi = 1113,
    Slovak = 1051,
    Slovenian = 1060,
    Sorbian = 1070,
    SpanishArgentina = 11274,
    SpanishBolivia = 16394,
    SpanishChile = 13322,
    SpanishColombia = 9226,
    SpanishCostaRica = 5130,
    SpanishDominicanRepublic = 7178,
    SpanishEcuador = 12298,
    SpanishElSalvador = 17418,
    SpanishGuatemala = 4106,
    SpanishHonduras = 18442,
    MexicanSpanish = 2058,
    SpanishNicaragua = 19466,
    SpanishPanama = 6154,
    SpanishParaguay = 15370,
    SpanishPeru = 10250,
    SpanishPuertoRico = 20490,
    SpanishModernSort = 3082,
    /// 西班牙语。
    Spanish = 1034,
    SpanishUruguay = 14346,
    SpanishVenezuela = 8202,
    Sesotho = 1072,
    Swahili = 1089,
    SwedishFinland = 2077,
    Swedish = 1053,
    Tajik = 1064,
    Tamil = 1097,
    Tatar = 1092,
    Telugu = 1098,
    Thai = 1054,
    Tibetan = 1105,
    Tsonga = 1073,
    Tswana = 1074,
    Turkish = 1055,
    Turkmen = 1090,
    Ukrainian = 1058,
    Urdu = 1056,
    UzbekCyrillic = 2115,
    UzbekLatin = 1091,
    Venda = 1075,
    Vietnamese = 1066,
    Welsh = 1106,
    Xhosa = 1076,
    Zulu = 1077,
}

impl cdrTextLanguage {
    pub const Sutu: Self = Self::Sesotho;
    pub fn from_i32(v: i32) -> Option<Self> {
        match v {
            -1 => Some(Self::LanguageMixed),
            0 => Some(Self::LanguageNone),
            1078 => Some(Self::Afrikaans),
            1052 => Some(Self::Albanian),
            5121 => Some(Self::ArabicAlgeria),
            15361 => Some(Self::ArabicBahrain),
            3073 => Some(Self::ArabicEgypt),
            2049 => Some(Self::ArabicIraq),
            11265 => Some(Self::ArabicJordan),
            13313 => Some(Self::ArabicKuwait),
            12289 => Some(Self::ArabicLebanon),
            4097 => Some(Self::ArabicLibya),
            6145 => Some(Self::ArabicMorocco),
            8193 => Some(Self::ArabicOman),
            16385 => Some(Self::ArabicQatar),
            1025 => Some(Self::Arabic),
            10241 => Some(Self::ArabicSyria),
            7169 => Some(Self::ArabicTunisia),
            14337 => Some(Self::ArabicUAE),
            9217 => Some(Self::ArabicYemen),
            1067 => Some(Self::Armenian),
            1101 => Some(Self::Assamese),
            2092 => Some(Self::AzeriCyrillic),
            1068 => Some(Self::AzeriLatin),
            1069 => Some(Self::Basque),
            1059 => Some(Self::Byelorussian),
            1093 => Some(Self::Bengali),
            1026 => Some(Self::Bulgarian),
            1109 => Some(Self::Burmese),
            1027 => Some(Self::Catalan),
            3076 => Some(Self::ChineseHongKong),
            5124 => Some(Self::ChineseMacao),
            2052 => Some(Self::SimplifiedChinese),
            4100 => Some(Self::ChineseSingapore),
            1028 => Some(Self::TraditionalChinese),
            1050 => Some(Self::Croatian),
            1029 => Some(Self::Czech),
            1030 => Some(Self::Danish),
            2067 => Some(Self::BelgianDutch),
            1043 => Some(Self::Dutch),
            3081 => Some(Self::EnglishAUS),
            10249 => Some(Self::EnglishBelize),
            4105 => Some(Self::EnglishCanadian),
            9225 => Some(Self::EnglishCaribbean),
            6153 => Some(Self::EnglishIreland),
            8201 => Some(Self::EnglishJamaica),
            5129 => Some(Self::EnglishNewZealand),
            13321 => Some(Self::EnglishPhilippines),
            7177 => Some(Self::EnglishSouthAfrica),
            11273 => Some(Self::EnglishTrinidad),
            2057 => Some(Self::EnglishUK),
            1033 => Some(Self::EnglishUS),
            12297 => Some(Self::EnglishZimbabwe),
            1061 => Some(Self::Estonian),
            1080 => Some(Self::Faeroese),
            1065 => Some(Self::Farsi),
            1035 => Some(Self::Finnish),
            2060 => Some(Self::BelgianFrench),
            11276 => Some(Self::FrenchCameroon),
            3084 => Some(Self::FrenchCanadian),
            12300 => Some(Self::FrenchCotedIvoire),
            1036 => Some(Self::French),
            5132 => Some(Self::FrenchLuxembourg),
            13324 => Some(Self::FrenchMali),
            6156 => Some(Self::FrenchMonaco),
            8204 => Some(Self::FrenchReunion),
            10252 => Some(Self::FrenchSenegal),
            4108 => Some(Self::SwissFrench),
            7180 => Some(Self::FrenchWestIndies),
            9228 => Some(Self::FrenchZaire),
            1122 => Some(Self::FrisianNetherlands),
            2108 => Some(Self::GaelicIreland),
            1084 => Some(Self::GaelicScotland),
            1110 => Some(Self::Galician),
            1079 => Some(Self::Georgian),
            3079 => Some(Self::GermanAustria),
            1031 => Some(Self::German),
            5127 => Some(Self::GermanLiechtenstein),
            4103 => Some(Self::GermanLuxembourg),
            2055 => Some(Self::SwissGerman),
            1032 => Some(Self::Greek),
            1095 => Some(Self::Gujarati),
            1037 => Some(Self::Hebrew),
            1081 => Some(Self::Hindi),
            1038 => Some(Self::Hungarian),
            1039 => Some(Self::Icelandic),
            1057 => Some(Self::Indonesian),
            1040 => Some(Self::Italian),
            2064 => Some(Self::SwissItalian),
            1041 => Some(Self::Japanese),
            1099 => Some(Self::Kannada),
            1120 => Some(Self::Kashmiri),
            1087 => Some(Self::Kazakh),
            1107 => Some(Self::Khmer),
            1088 => Some(Self::Kirghiz),
            1111 => Some(Self::Konkani),
            1042 => Some(Self::Korean),
            1108 => Some(Self::Lao),
            1062 => Some(Self::Latvian),
            1063 => Some(Self::Lithuanian),
            1071 => Some(Self::Macedonian),
            1086 => Some(Self::Malaysian),
            2110 => Some(Self::MalayBruneiDarussalam),
            1100 => Some(Self::Malayalam),
            1082 => Some(Self::Maltese),
            1112 => Some(Self::Manipuri),
            1102 => Some(Self::Marathi),
            1104 => Some(Self::Mongolian),
            1121 => Some(Self::Nepali),
            1044 => Some(Self::NorwegianBokmol),
            2068 => Some(Self::NorwegianNynorsk),
            1096 => Some(Self::Oriya),
            1045 => Some(Self::Polish),
            1046 => Some(Self::BrazilianPortuguese),
            2070 => Some(Self::Portuguese),
            1094 => Some(Self::Punjabi),
            1047 => Some(Self::RhaetoRomanic),
            2072 => Some(Self::RomanianMoldova),
            1048 => Some(Self::Romanian),
            2073 => Some(Self::RussianMoldova),
            1049 => Some(Self::Russian),
            1083 => Some(Self::SamiLappish),
            1103 => Some(Self::Sanskrit),
            3098 => Some(Self::SerbianCyrillic),
            2074 => Some(Self::SerbianLatin),
            1113 => Some(Self::Sindhi),
            1051 => Some(Self::Slovak),
            1060 => Some(Self::Slovenian),
            1070 => Some(Self::Sorbian),
            11274 => Some(Self::SpanishArgentina),
            16394 => Some(Self::SpanishBolivia),
            13322 => Some(Self::SpanishChile),
            9226 => Some(Self::SpanishColombia),
            5130 => Some(Self::SpanishCostaRica),
            7178 => Some(Self::SpanishDominicanRepublic),
            12298 => Some(Self::SpanishEcuador),
            17418 => Some(Self::SpanishElSalvador),
            4106 => Some(Self::SpanishGuatemala),
            18442 => Some(Self::SpanishHonduras),
            2058 => Some(Self::MexicanSpanish),
            19466 => Some(Self::SpanishNicaragua),
            6154 => Some(Self::SpanishPanama),
            15370 => Some(Self::SpanishParaguay),
            10250 => Some(Self::SpanishPeru),
            20490 => Some(Self::SpanishPuertoRico),
            3082 => Some(Self::SpanishModernSort),
            1034 => Some(Self::Spanish),
            14346 => Some(Self::SpanishUruguay),
            8202 => Some(Self::SpanishVenezuela),
            1072 => Some(Self::Sesotho),
            1089 => Some(Self::Swahili),
            2077 => Some(Self::SwedishFinland),
            1053 => Some(Self::Swedish),
            1064 => Some(Self::Tajik),
            1097 => Some(Self::Tamil),
            1092 => Some(Self::Tatar),
            1098 => Some(Self::Telugu),
            1054 => Some(Self::Thai),
            1105 => Some(Self::Tibetan),
            1073 => Some(Self::Tsonga),
            1074 => Some(Self::Tswana),
            1055 => Some(Self::Turkish),
            1090 => Some(Self::Turkmen),
            1058 => Some(Self::Ukrainian),
            1056 => Some(Self::Urdu),
            2115 => Some(Self::UzbekCyrillic),
            1091 => Some(Self::UzbekLatin),
            1075 => Some(Self::Venda),
            1066 => Some(Self::Vietnamese),
            1106 => Some(Self::Welsh),
            1076 => Some(Self::Xhosa),
            1077 => Some(Self::Zulu),
            _ => None,
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
    None = 0,
    Bullet = 1,
    DropCap = 2,
}

// =============================================================
// cdrTextChangeCase —— 文本大小写转换
// =============================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum cdrTextChangeCase {
    Uppercase = 0,
    Lowercase = 1,
    TitleCase = 2,
    SentenceCase = 3,
    ToggleCase = 4,
}

// =============================================================
// cdrTextTabAlignment —— 制表位对齐
// =============================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum cdrTextTabAlignment {
    Left = 0,
    Right = 1,
    Center = 2,
    Decimal = 3,
}

// =============================================================
// cdrVerticalAlignment —— 垂直对齐
// =============================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum cdrVerticalAlignment {
    Top = 0,
    Center = 1,
    Bottom = 2,
    Full = 3,
}

// =============================================================
// cdrLineSpacingType —— 行距类型
// =============================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum cdrLineSpacingType {
    PercentOfCharacterHeightLineSpacing = 0,
    PointLineSpacing = 1,
    PercentOfPointSizeLineSpacing = 2,
    MixedLineSpacing = 3,
}

// =============================================================
// cdrTextPropertySet —— 文本属性集（用于 EnumRanges）
// =============================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum cdrTextPropertySet {
    TextPropertyAll = -1,
    TextPropertyFill = 1,
    TextPropertyOutline = 2,
    TextPropertyFont = 4,
    TextPropertySize = 8,
    TextPropertyStyle = 16,
}
