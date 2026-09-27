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
    True      = -1,
    False     = 0,
}

impl cdrTriState {
    pub fn from_i32(v: i32) -> Option<Self> {
        match v {
            -2 => Some(Self::Undefined),
            -1 => Some(Self::True),
            0  => Some(Self::False),
            _  => None,
        }
    }

    pub fn to_bool(self) -> Option<bool> {
        match self {
            Self::True  => Some(true),
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
    Draw       = 0,
    PhotoPaint = 1,
    Designer   = 2,
    // TODO: 完整值待补
}

impl cdrApplicationID {
    pub fn from_i32(v: i32) -> Option<Self> {
        match v {
            0 => Some(Self::Draw),
            1 => Some(Self::PhotoPaint),
            2 => Some(Self::Designer),
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
    // TODO: 完整值待补
    Unknown = -1,
}

impl cdrApplicationClass {
    pub fn from_i32(v: i32) -> Option<Self> {
        match v {
            -1 => Some(Self::Unknown),
            _  => None,
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
    /// 启动时什么都不做。
    StartupDoNothing = 0,
    /// 启动时新建文档。
    NewDocumentMode  = 1,
    // TODO: 启动时打开模板 —— 具体名称和值待核
}

impl cdrAppStartupMode {
    pub fn from_i32(v: i32) -> Option<Self> {
        match v {
            0 => Some(Self::StartupDoNothing),
            1 => Some(Self::NewDocumentMode),
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
            _  => None,
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
    Unchecked = 0,
    Checked   = 1,
    // TODO: 待补
}

impl cdrCommandCheckState {
    pub fn from_i32(v: i32) -> Option<Self> {
        match v {
            0 => Some(Self::Unchecked),
            1 => Some(Self::Checked),
            _ => None,
        }
    }
}