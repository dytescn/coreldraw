//! 应用层枚举

// =============================================================
// cdrTriState —— 三态布尔
// =============================================================

/// 三态布尔值，用于 API 中"未定义 / 真 / 假"的场景。
///
/// 来源：`VGCore::cdrTriState`
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum cdrTriState {
    Undefined = -2,
    True = -1,
    False = 0,
}

impl cdrTriState {
    pub fn from_i32(v: i32) -> Option<Self> {
        match v {
            -2 => Some(Self::Undefined),
            -1 => Some(Self::True),
            0 => Some(Self::False),
            _ => None,
        }
    }

    pub fn to_bool(self) -> Option<bool> {
        match self {
            Self::True => Some(true),
            Self::False => Some(false),
            Self::Undefined => None,
        }
    }
}

// =============================================================
// cdrApplicationID —— 应用 ID
// =============================================================

/// 区分当前运行的 CorelDRAW 产品。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum cdrApplicationID {
    CorelDRAW = 0,
    CorelDESIGNER = 1,
}

impl cdrApplicationID {
    pub fn from_i32(v: i32) -> Option<Self> {
        match v {
            0 => Some(Self::CorelDRAW),
            1 => Some(Self::CorelDESIGNER),
            _ => None,
        }
    }
}

// =============================================================
// cdrApplicationClass —— 应用类别
// =============================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum cdrApplicationClass {
    CreativeGraphics = 0,
    TechnicalGraphics = 1,
}

impl cdrApplicationClass {
    pub fn from_i32(v: i32) -> Option<Self> {
        match v {
            0 => Some(Self::CreativeGraphics),
            1 => Some(Self::TechnicalGraphics),
            _ => None,
        }
    }
}

// =============================================================
// cdrAppStartupMode —— 应用启动模式
// =============================================================

/// `Application.StartupMode` 的取值。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum cdrAppStartupMode {
    StartupWelcomeScreen = 0,
    StartupNewDocument = 1,
    StartupOpenDocument = 2,
    StartupLastEditedDocument = 3,
    StartupNewFromTemplate = 4,
    StartupTutorial = 5,
    /// 启动时什么都不做。
    StartupDoNothing = 6,
}

impl cdrAppStartupMode {
    pub fn from_i32(v: i32) -> Option<Self> {
        match v {
            0 => Some(Self::StartupWelcomeScreen),
            1 => Some(Self::StartupNewDocument),
            2 => Some(Self::StartupOpenDocument),
            3 => Some(Self::StartupLastEditedDocument),
            4 => Some(Self::StartupNewFromTemplate),
            5 => Some(Self::StartupTutorial),
            6 => Some(Self::StartupDoNothing),
            _ => None,
        }
    }
}

// =============================================================
// cdrAppStatus —— 应用状态
// =============================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum cdrAppStatus {
    // TODO: 待补
    Unknown = -1,
}

impl cdrAppStatus {
    pub fn from_i32(v: i32) -> Option<Self> {
        match v {
            -1 => Some(Self::Unknown),
            _ => None,
        }
    }
}

// =============================================================
// cdrCommandCheckState —— 插件命令勾选状态
// =============================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum cdrCommandCheckState {
    CommandUnchecked = 0,
    CommandChecked = 1,
}

impl cdrCommandCheckState {
    pub fn from_i32(v: i32) -> Option<Self> {
        match v {
            0 => Some(Self::CommandUnchecked),
            1 => Some(Self::CommandChecked),
            _ => None,
        }
    }
}
