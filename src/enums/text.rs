//! 文本枚举

// =============================================================
// cdrAlignment —— 文本对齐
// =============================================================

/// 来源：`VGCore::cdrAlignment`
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum cdrAlignment {
    /// 无对齐。
    NoAlignment = 0,
    /// 左对齐。
    LeftAlignment = 1,
    /// 右对齐。
    RightAlignment = 2,
    /// 中心对齐。
    CenterAlignment = 3,
    /// 完整 justify 对齐。
    FullJustifyAlignment = 4,
    /// forcejustify 对齐。
    ForceJustifyAlignment = 5,
    /// mixed 对齐。
    MixedAlignment = 6,
}

impl cdrAlignment {
    /// 由整数值构造枚举。
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

/// CorelDRAW 枚举 `cdrFontStyle`：字体样式。
///
/// 对应类型库中的 `VGCore::cdrFontStyle`，用 `#[repr(i32)]` 映射实际整数值。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum cdrFontStyle {
    /// 常规字体样式。
    NormalFontStyle = 0,
    /// 加粗字体样式。
    BoldFontStyle = 1,
    /// 斜体字体样式。
    ItalicFontStyle = 2,
    /// 加粗斜体字体样式。
    BoldItalicFontStyle = 3,
    /// thin 字体样式。
    ThinFontStyle = 4,
    /// thin 斜体字体样式。
    ThinItalicFontStyle = 5,
    /// extra 光源字体样式。
    ExtraLightFontStyle = 6,
    /// extra 光源斜体字体样式。
    ExtraLightItalicFontStyle = 7,
    /// medium 字体样式。
    MediumFontStyle = 8,
    /// medium 斜体字体样式。
    MediumItalicFontStyle = 9,
    /// semi 加粗字体样式。
    SemiBoldFontStyle = 10,
    /// semi 加粗斜体字体样式。
    SemiBoldItalicFontStyle = 11,
    /// extra 加粗字体样式。
    ExtraBoldFontStyle = 12,
    /// extra 加粗斜体字体样式。
    ExtraBoldItalicFontStyle = 13,
    /// heavy 字体样式。
    HeavyFontStyle = 14,
    /// heavy 斜体字体样式。
    HeavyItalicFontStyle = 15,
    /// mixed 字体样式。
    MixedFontStyle = 16,
    /// 光源字体样式。
    LightFontStyle = 17,
    /// 光源斜体字体样式。
    LightItalicFontStyle = 18,
}

impl cdrFontStyle {
    /// 由整数值构造枚举。
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

/// CorelDRAW 枚举 `cdrFontLine`：字体线条（下划线 / 删除线等）。
///
/// 对应类型库中的 `VGCore::cdrFontLine`，用 `#[repr(i32)]` 映射实际整数值。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum cdrFontLine {
    /// 无字体线。
    NoFontLine = 0,
    /// singlethin 字体线。
    SingleThinFontLine = 1,
    /// singlethin 词字体线。
    SingleThinWordFontLine = 2,
    /// singlethick 字体线。
    SingleThickFontLine = 3,
    /// singlethick 词字体线。
    SingleThickWordFontLine = 4,
    /// doublethin 字体线。
    DoubleThinFontLine = 5,
    /// doublethin 词字体线。
    DoubleThinWordFontLine = 6,

    /// mixed 字体线。
    MixedFontLine = 7,
}

// =============================================================
// cdrFontCase —— 大小写
// =============================================================

/// CorelDRAW 枚举 `cdrFontCase`：大小写。
///
/// 对应类型库中的 `VGCore::cdrFontCase`，用 `#[repr(i32)]` 映射实际整数值。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum cdrFontCase {
    /// 常规。
    Normal = 0,
    /// small 端点。
    SmallCaps = 1,
    /// 全部端点。
    AllCaps = 2,
}

// =============================================================
// cdrFontPosition —— 字体位置（上标 / 下标）
// =============================================================

/// CorelDRAW 枚举 `cdrFontPosition`：字体位置（上标 / 下标）。
///
/// 对应类型库中的 `VGCore::cdrFontPosition`，用 `#[repr(i32)]` 映射实际整数值。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum cdrFontPosition {
    /// 常规。
    Normal = 0,
    /// subscript。
    Subscript = 1,
    /// superscript。
    Superscript = 2,
}

// =============================================================
// cdrTextType —— 文本类型
// =============================================================

/// CorelDRAW 枚举 `cdrTextType`：文本类型。
///
/// 对应类型库中的 `VGCore::cdrTextType`，用 `#[repr(i32)]` 映射实际整数值。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum cdrTextType {
    /// 艺术。
    Artistic = 0,
    /// 段落。
    Paragraph = 1,
    /// 艺术已适配。
    ArtisticFitted = 2,
    /// 段落已适配。
    ParagraphFitted = 3,
}

impl cdrTextType {
    /// 由整数值构造枚举。
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

/// CorelDRAW 枚举 `cdrTextFrames`：文本框范围。
///
/// 对应类型库中的 `VGCore::cdrTextFrames`，用 `#[repr(i32)]` 映射实际整数值。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum cdrTextFrames {
    /// this 文本框仅。
    ThisFrameOnly = 0,
    /// this 文本框处的开始。
    StartAtThisFrame = 1,
    /// 全部文本框集。
    AllFrames = 2,
}

impl cdrTextFrames {
    /// 由整数值构造枚举。
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

/// CorelDRAW 枚举 `cdrTextIndexingType`：文本索引类型。
///
/// 对应类型库中的 `VGCore::cdrTextIndexingType`，用 `#[repr(i32)]` 映射实际整数值。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum cdrTextIndexingType {
    /// 字符 indexing。
    CharacterIndexing = 0,

    /// 词 indexing。
    WordIndexing = 1,

    /// 段落 indexing。
    ParagraphIndexing = 2,
}

// =============================================================
// cdrTextCharSet —— 字符集
// =============================================================

/// CorelDRAW 枚举 `cdrTextCharSet`：字符集。
///
/// 对应类型库中的 `VGCore::cdrTextCharSet`，用 `#[repr(i32)]` 映射实际整数值。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum cdrTextCharSet {
    /// 混合。
    CharSetMixed = -1,
    /// 字符设置 ansi。
    CharSetANSI = 0,
    /// 默认字符集。
    CharSetDefault = 1,
    /// 字符设置符号。
    CharSetSymbol = 2,
    /// 字符设置偏移 jis。
    CharSetShiftJIS = 128,
    /// 字符设置 hangul。
    CharSetHangul = 129,
    /// 字符设置 chinesebig5。
    CharSetChineseBig5 = 136,
    /// 字符设置 oem。
    CharSetOEM = 255,
    /// 字符设置 johab。
    CharSetJohab = 130,
    /// 字符设置 hebrew。
    CharSetHebrew = 177,
    /// 字符设置 arabic。
    CharSetArabic = 178,
    /// 字符设置 greek。
    CharSetGreek = 161,
    /// 字符设置 turkish。
    CharSetTurkish = 162,
    /// 字符设置 vietnamese。
    CharSetVietnamese = 163,
    /// 字符设置 thai。
    CharSetThai = 222,
    /// 字符设置 easteurope。
    CharSetEastEurope = 238,
    /// 字符设置 russian。
    CharSetRussian = 204,
    /// 字符设置 mac。
    CharSetMac = 77,
    /// 字符设置 baltic。
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
    /// 语言 mixed。
    LanguageMixed = -1,
    /// 无语言。
    LanguageNone = 0,
    /// afrikaans。
    Afrikaans = 1078,
    /// albanian。
    Albanian = 1052,
    /// arabicalgeria。
    ArabicAlgeria = 5121,
    /// arabicbahrain。
    ArabicBahrain = 15361,
    /// arabicegypt。
    ArabicEgypt = 3073,
    /// arabiciraq。
    ArabicIraq = 2049,
    /// arabicjordan。
    ArabicJordan = 11265,
    /// arabickuwait。
    ArabicKuwait = 13313,
    /// arabiclebanon。
    ArabicLebanon = 12289,
    /// arabiclibya。
    ArabicLibya = 4097,
    /// arabicmorocco。
    ArabicMorocco = 6145,
    /// arabicoman。
    ArabicOman = 8193,
    /// arabicqatar。
    ArabicQatar = 16385,
    /// arabic。
    Arabic = 1025,
    /// arabicsyria。
    ArabicSyria = 10241,
    /// arabictunisia。
    ArabicTunisia = 7169,
    /// arabicuae。
    ArabicUAE = 14337,
    /// arabicyemen。
    ArabicYemen = 9217,
    /// armenian。
    Armenian = 1067,
    /// assamese。
    Assamese = 1101,
    /// azericyrillic。
    AzeriCyrillic = 2092,
    /// azerilatin。
    AzeriLatin = 1068,
    /// basque。
    Basque = 1069,
    /// byelorussian。
    Byelorussian = 1059,
    /// bengali。
    Bengali = 1093,
    /// bulgarian。
    Bulgarian = 1026,
    /// burmese。
    Burmese = 1109,
    /// catalan。
    Catalan = 1027,
    /// chinesehongkong。
    ChineseHongKong = 3076,
    /// chinesemacao。
    ChineseMacao = 5124,
    /// simplifiedchinese。
    SimplifiedChinese = 2052,
    /// 中文（新加坡）。
    ChineseSingapore = 4100,
    /// traditionalchinese。
    TraditionalChinese = 1028,
    /// croatian。
    Croatian = 1050,
    /// czech。
    Czech = 1029,
    /// danish。
    Danish = 1030,
    /// belgiandutch。
    BelgianDutch = 2067,
    /// dutch。
    Dutch = 1043,
    /// englishaus。
    EnglishAUS = 3081,
    /// englishbelize。
    EnglishBelize = 10249,
    /// englishcanadian。
    EnglishCanadian = 4105,
    /// englishcaribbean。
    EnglishCaribbean = 9225,
    /// englishireland。
    EnglishIreland = 6153,
    /// englishjamaica。
    EnglishJamaica = 8201,
    /// englishnewzealand。
    EnglishNewZealand = 5129,
    /// englishphilippines。
    EnglishPhilippines = 13321,
    /// englishsouthafrica。
    EnglishSouthAfrica = 7177,
    /// englishtrinidad。
    EnglishTrinidad = 11273,
    /// 英语（英国）。
    EnglishUK = 2057,
    /// 英语（美国）。
    EnglishUS = 1033,
    /// englishzimbabwe。
    EnglishZimbabwe = 12297,
    /// estonian。
    Estonian = 1061,
    /// faeroese。
    Faeroese = 1080,
    /// farsi。
    Farsi = 1065,
    /// finnish。
    Finnish = 1035,
    /// belgianfrench。
    BelgianFrench = 2060,
    /// frenchcameroon。
    FrenchCameroon = 11276,
    /// frenchcanadian。
    FrenchCanadian = 3084,
    /// frenchcotedivoire。
    FrenchCotedIvoire = 12300,
    /// 法语。
    French = 1036,
    /// frenchluxembourg。
    FrenchLuxembourg = 5132,
    /// frenchmali。
    FrenchMali = 13324,
    /// frenchmonaco。
    FrenchMonaco = 6156,
    /// frenchreunion。
    FrenchReunion = 8204,
    /// frenchsenegal。
    FrenchSenegal = 10252,
    /// swissfrench。
    SwissFrench = 4108,
    /// frenchwestindies。
    FrenchWestIndies = 7180,
    /// frenchzaire。
    FrenchZaire = 9228,
    /// frisiannetherlands。
    FrisianNetherlands = 1122,
    /// gaelicireland。
    GaelicIreland = 2108,
    /// gaelicscotland。
    GaelicScotland = 1084,
    /// galician。
    Galician = 1110,
    /// georgian。
    Georgian = 1079,
    /// germanaustria。
    GermanAustria = 3079,
    /// 德语。
    German = 1031,
    /// germanliechtenstein。
    GermanLiechtenstein = 5127,
    /// germanluxembourg。
    GermanLuxembourg = 4103,
    /// swissgerman。
    SwissGerman = 2055,
    /// greek。
    Greek = 1032,
    /// gujarati。
    Gujarati = 1095,
    /// hebrew。
    Hebrew = 1037,
    /// hindi。
    Hindi = 1081,
    /// hungarian。
    Hungarian = 1038,
    /// icelandic。
    Icelandic = 1039,
    /// indonesian。
    Indonesian = 1057,
    /// 意大利语。
    Italian = 1040,
    /// swissitalian。
    SwissItalian = 2064,
    /// 日语。
    Japanese = 1041,
    /// kannada。
    Kannada = 1099,
    /// kashmiri。
    Kashmiri = 1120,
    /// kazakh。
    Kazakh = 1087,
    /// khmer。
    Khmer = 1107,
    /// kirghiz。
    Kirghiz = 1088,
    /// konkani。
    Konkani = 1111,
    /// 韩语。
    Korean = 1042,
    /// lao。
    Lao = 1108,
    /// latvian。
    Latvian = 1062,
    /// lithuanian。
    Lithuanian = 1063,
    /// macedonian。
    Macedonian = 1071,
    /// malaysian。
    Malaysian = 1086,
    /// malaybruneidarussalam。
    MalayBruneiDarussalam = 2110,
    /// malayalam。
    Malayalam = 1100,
    /// maltese。
    Maltese = 1082,
    /// manipuri。
    Manipuri = 1112,
    /// marathi。
    Marathi = 1102,
    /// mongolian。
    Mongolian = 1104,
    /// nepali。
    Nepali = 1121,
    /// norwegianbokmol。
    NorwegianBokmol = 1044,
    /// norwegiannynorsk。
    NorwegianNynorsk = 2068,
    /// oriya。
    Oriya = 1096,
    /// polish。
    Polish = 1045,
    /// brazilianportuguese。
    BrazilianPortuguese = 1046,
    /// portuguese。
    Portuguese = 2070,
    /// punjabi。
    Punjabi = 1094,
    /// rhaetoromanic。
    RhaetoRomanic = 1047,
    /// romanianmoldova。
    RomanianMoldova = 2072,
    /// romanian。
    Romanian = 1048,
    /// russianmoldova。
    RussianMoldova = 2073,
    /// 俄语。
    Russian = 1049,
    /// samilappish。
    SamiLappish = 1083,
    /// sanskrit。
    Sanskrit = 1103,
    /// serbiancyrillic。
    SerbianCyrillic = 3098,
    /// serbianlatin。
    SerbianLatin = 2074,
    /// sindhi。
    Sindhi = 1113,
    /// slovak。
    Slovak = 1051,
    /// slovenian。
    Slovenian = 1060,
    /// sorbian。
    Sorbian = 1070,
    /// spanishargentina。
    SpanishArgentina = 11274,
    /// spanishbolivia。
    SpanishBolivia = 16394,
    /// spanishchile。
    SpanishChile = 13322,
    /// spanishcolombia。
    SpanishColombia = 9226,
    /// spanishcostarica。
    SpanishCostaRica = 5130,
    /// spanishdominicanrepublic。
    SpanishDominicanRepublic = 7178,
    /// spanishecuador。
    SpanishEcuador = 12298,
    /// spanishelsalvador。
    SpanishElSalvador = 17418,
    /// spanishguatemala。
    SpanishGuatemala = 4106,
    /// spanishhonduras。
    SpanishHonduras = 18442,
    /// mexicanspanish。
    MexicanSpanish = 2058,
    /// spanishnicaragua。
    SpanishNicaragua = 19466,
    /// spanishpanama。
    SpanishPanama = 6154,
    /// spanishparaguay。
    SpanishParaguay = 15370,
    /// spanishperu。
    SpanishPeru = 10250,
    /// spanishpuertorico。
    SpanishPuertoRico = 20490,
    /// spanishmodernsort。
    SpanishModernSort = 3082,
    /// 西班牙语。
    Spanish = 1034,
    /// spanishuruguay。
    SpanishUruguay = 14346,
    /// spanishvenezuela。
    SpanishVenezuela = 8202,
    /// sesotho。
    Sesotho = 1072,
    /// swahili。
    Swahili = 1089,
    /// swedishfinland。
    SwedishFinland = 2077,
    /// swedish。
    Swedish = 1053,
    /// tajik。
    Tajik = 1064,
    /// tamil。
    Tamil = 1097,
    /// tatar。
    Tatar = 1092,
    /// telugu。
    Telugu = 1098,
    /// thai。
    Thai = 1054,
    /// tibetan。
    Tibetan = 1105,
    /// tsonga。
    Tsonga = 1073,
    /// tswana。
    Tswana = 1074,
    /// turkish。
    Turkish = 1055,
    /// turkmen。
    Turkmen = 1090,
    /// ukrainian。
    Ukrainian = 1058,
    /// urdu。
    Urdu = 1056,
    /// uzbekcyrillic。
    UzbekCyrillic = 2115,
    /// uzbeklatin。
    UzbekLatin = 1091,
    /// venda。
    Venda = 1075,
    /// vietnamese。
    Vietnamese = 1066,
    /// welsh。
    Welsh = 1106,
    /// xhosa。
    Xhosa = 1076,
    /// zulu。
    Zulu = 1077,
}

impl cdrTextLanguage {
    /// sutu（`Sutu`）。
    pub const Sutu: Self = Self::Sesotho;
    /// 由整数值构造枚举。
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

/// CorelDRAW 枚举 `cdrTextEffect`：文本效果。
///
/// 对应类型库中的 `VGCore::cdrTextEffect`，用 `#[repr(i32)]` 映射实际整数值。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum cdrTextEffect {
    /// 无。
    None = 0,
    /// 项目符号。
    Bullet = 1,
    /// 投影端点。
    DropCap = 2,
}

// =============================================================
// cdrTextChangeCase —— 文本大小写转换
// =============================================================

/// CorelDRAW 枚举 `cdrTextChangeCase`：文本大小写转换。
///
/// 对应类型库中的 `VGCore::cdrTextChangeCase`，用 `#[repr(i32)]` 映射实际整数值。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum cdrTextChangeCase {
    /// uppercase。
    Uppercase = 0,
    /// lowercase。
    Lowercase = 1,
    /// 标题 case。
    TitleCase = 2,
    /// sentencecase。
    SentenceCase = 3,
    /// togglecase。
    ToggleCase = 4,
}

// =============================================================
// cdrTextTabAlignment —— 制表位对齐
// =============================================================

/// CorelDRAW 枚举 `cdrTextTabAlignment`：制表位对齐。
///
/// 对应类型库中的 `VGCore::cdrTextTabAlignment`，用 `#[repr(i32)]` 映射实际整数值。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum cdrTextTabAlignment {
    /// 左。
    Left = 0,
    /// 右。
    Right = 1,
    /// 中心。
    Center = 2,
    /// decimal。
    Decimal = 3,
}

// =============================================================
// cdrVerticalAlignment —— 垂直对齐
// =============================================================

/// CorelDRAW 枚举 `cdrVerticalAlignment`：垂直对齐。
///
/// 对应类型库中的 `VGCore::cdrVerticalAlignment`，用 `#[repr(i32)]` 映射实际整数值。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum cdrVerticalAlignment {
    /// 顶。
    Top = 0,
    /// 中心。
    Center = 1,
    /// 底。
    Bottom = 2,
    /// 完整。
    Full = 3,
}

// =============================================================
// cdrLineSpacingType —— 行距类型
// =============================================================

/// CorelDRAW 枚举 `cdrLineSpacingType`：行距类型。
///
/// 对应类型库中的 `VGCore::cdrLineSpacingType`，用 `#[repr(i32)]` 映射实际整数值。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum cdrLineSpacingType {
    /// 百分比字符高度线间距。
    PercentOfCharacterHeightLineSpacing = 0,
    /// 点线间距。
    PointLineSpacing = 1,
    /// 百分比点尺寸线间距。
    PercentOfPointSizeLineSpacing = 2,
    /// mixed 线间距。
    MixedLineSpacing = 3,
}

// =============================================================
// cdrTextPropertySet —— 文本属性集（用于 EnumRanges）
// =============================================================

/// CorelDRAW 枚举 `cdrTextPropertySet`：文本属性集（用于 EnumRanges）。
///
/// 对应类型库中的 `VGCore::cdrTextPropertySet`，用 `#[repr(i32)]` 映射实际整数值。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum cdrTextPropertySet {
    /// 文本属性全部。
    TextPropertyAll = -1,
    /// 文本属性填充。
    TextPropertyFill = 1,
    /// 文本属性轮廓。
    TextPropertyOutline = 2,
    /// 文本属性字体。
    TextPropertyFont = 4,
    /// 文本属性尺寸。
    TextPropertySize = 8,
    /// 文本属性样式。
    TextPropertyStyle = 16,
}
