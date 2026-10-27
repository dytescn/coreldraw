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
    /// undefined。
    Undefined = -2,
    /// 真。
    True = -1,
    /// 假。
    False = 0,
}

impl cdrTriState {
    /// 由整数值构造枚举。
    pub fn from_i32(v: i32) -> Option<Self> {
        match v {
            -2 => Some(Self::Undefined),
            -1 => Some(Self::True),
            0 => Some(Self::False),
            _ => None,
        }
    }

    /// 转换为布尔值。
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
    /// coreldraw。
    CorelDRAW = 0,
    /// corel 设计器。
    CorelDESIGNER = 1,
}

impl cdrApplicationID {
    /// 由整数值构造枚举。
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

/// CorelDRAW 枚举 `cdrApplicationClass`：应用类别。
///
/// 对应类型库中的 `VGCore::cdrApplicationClass`，用 `#[repr(i32)]` 映射实际整数值。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum cdrApplicationClass {
    /// creativegraphics。
    CreativeGraphics = 0,
    /// technicalgraphics。
    TechnicalGraphics = 1,
}

impl cdrApplicationClass {
    /// 由整数值构造枚举。
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
    /// 启动 welcome 屏幕。
    StartupWelcomeScreen = 0,
    /// 启动 new 文档。
    StartupNewDocument = 1,
    /// 启动打开文档。
    StartupOpenDocument = 2,
    /// 启动最后一个 edited 文档。
    StartupLastEditedDocument = 3,
    /// 启动 new 模板。
    StartupNewFromTemplate = 4,
    /// 启动 tutorial。
    StartupTutorial = 5,
    /// 启动时什么都不做。
    StartupDoNothing = 6,
}

impl cdrAppStartupMode {
    /// 由整数值构造枚举。
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

/// CorelDRAW 枚举 `cdrAppStatus`：应用状态。
///
/// 对应类型库中的 `VGCore::cdrAppStatus`，用 `#[repr(i32)]` 映射实际整数值。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum cdrAppStatus {
    // TODO: 待补
    /// 未知。
    Unknown = -1,
}

impl cdrAppStatus {
    /// 由整数值构造枚举。
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

/// CorelDRAW 枚举 `cdrCommandCheckState`：插件命令勾选状态。
///
/// 对应类型库中的 `VGCore::cdrCommandCheckState`，用 `#[repr(i32)]` 映射实际整数值。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum cdrCommandCheckState {
    /// 命令 unchecked。
    CommandUnchecked = 0,
    /// 命令 checked。
    CommandChecked = 1,
}

impl cdrCommandCheckState {
    /// 由整数值构造枚举。
    pub fn from_i32(v: i32) -> Option<Self> {
        match v {
            0 => Some(Self::CommandUnchecked),
            1 => Some(Self::CommandChecked),
            _ => None,
        }
    }
}
