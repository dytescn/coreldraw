//! 视图枚举

// =============================================================
// cdrViewType —— 视图模式
// =============================================================

/// `ActiveView.Type` 的取值。
///
/// 来源：`VGCore::cdrViewType`
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum cdrViewType {
    /// 简单线框。
    SimpleWireframe = 0,
    /// 线框。
    Wireframe = 1,
    /// 草稿。
    Draft = 2,
    /// 正常。
    Normal = 3,
    /// 增强。
    Enhanced = 4,
    // 5 缺失
    /// 像素预览。
    Pixel = 6,
}

impl cdrViewType {
    /// 由整数值构造枚举。
    pub fn from_i32(v: i32) -> Option<Self> {
        match v {
            0 => Some(Self::SimpleWireframe),
            1 => Some(Self::Wireframe),
            2 => Some(Self::Draft),
            3 => Some(Self::Normal),
            4 => Some(Self::Enhanced),
            6 => Some(Self::Pixel),
            _ => None,
        }
    }
}

// =============================================================
// cdrWindowState —— 窗口状态
// =============================================================

/// 来源：`VGCore::cdrWindowState`
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum cdrWindowState {
    /// 常规。
    Normal = 1,
    /// minimized。
    Minimized = 2,
    /// maximized。
    Maximized = 3,
    /// restore。
    Restore = 9,
}

impl cdrWindowState {
    /// 由整数值构造枚举。
    pub fn from_i32(v: i32) -> Option<Self> {
        match v {
            1 => Some(Self::Normal),
            2 => Some(Self::Minimized),
            3 => Some(Self::Maximized),
            9 => Some(Self::Restore),
            _ => None,
        }
    }
}

// =============================================================
// cdrWindowArrangeStyle —— 窗口排列方式
// =============================================================

/// CorelDRAW 枚举 `cdrWindowArrangeStyle`：窗口排列方式。
///
/// 对应类型库中的 `VGCore::cdrWindowArrangeStyle`，用 `#[repr(i32)]` 映射实际整数值。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum cdrWindowArrangeStyle {
    /// 平铺 horizontally。
    TileHorizontally = 0,
    /// 平铺 vertically。
    TileVertically = 1,

    /// cascade。
    Cascade = 2,
}

// =============================================================
// cdrWindowType —— 窗口类型
// =============================================================

/// CorelDRAW 枚举 `cdrWindowType`：窗口类型。
///
/// 对应类型库中的 `VGCore::cdrWindowType`，用 `#[repr(i32)]` 映射实际整数值。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum cdrWindowType {
    /// 文档窗口。
    DocumentWindow = 0,
    // TODO: 待补
}
