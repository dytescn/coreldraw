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
    Wireframe       = 1,
    /// 草稿。
    Draft           = 2,
    /// 正常。
    Normal          = 3,
    /// 增强。
    Enhanced        = 4,
    // 5 缺失
    /// 像素预览。
    Pixel           = 6,
}

impl cdrViewType {
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
    Normal    = 1,
    Minimized = 2,
    Maximized = 3,
    Restore   = 9,
}

impl cdrWindowState {
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum cdrWindowArrangeStyle {
    Cascade   = 0,
    TileHorz  = 1,
    TileVert  = 2,
    // TODO: 待补
}

// =============================================================
// cdrWindowType —— 窗口类型
// =============================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum cdrWindowType {
    DocumentWindow = 0,
    // TODO: 待补
}