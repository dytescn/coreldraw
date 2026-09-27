//! 轮廓枚举

// =============================================================
// cdrOutlineLineCaps —— 线条端点样式
// =============================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum cdrOutlineLineCaps {
    /// 未定义。
    UndefinedLineCaps = -1,
    /// 平头（butt）。
    Flat              = 0,
    /// 圆头。
    Round             = 1,
    /// 方头（square）。
    Square            = 2,
}

impl cdrOutlineLineCaps {
    pub fn from_i32(v: i32) -> Option<Self> {
        match v {
            -1 => Some(Self::UndefinedLineCaps),
            0  => Some(Self::Flat),
            1  => Some(Self::Round),
            2  => Some(Self::Square),
            _  => None,
        }
    }
}

// =============================================================
// cdrOutlineLineJoin —— 线条连接样式
// =============================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum cdrOutlineLineJoin {
    /// 未定义。
    UndefinedLineJoin = -1,
    /// 斜接（miter）。
    Miter             = 0,
    /// 圆角（round）。
    Round             = 1,
    /// 斜角（bevel）。
    Bevel             = 2,
}

impl cdrOutlineLineJoin {
    pub fn from_i32(v: i32) -> Option<Self> {
        match v {
            -1 => Some(Self::UndefinedLineJoin),
            0  => Some(Self::Miter),
            1  => Some(Self::Round),
            2  => Some(Self::Bevel),
            _  => None,
        }
    }
}

// =============================================================
// cdrOutlineJustification —— 轮廓对齐方式
// =============================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum cdrOutlineJustification {
    /// 中线对齐。
    Middle = 0,
    /// 内侧对齐。
    Inside = 1,
    /// 外侧对齐。
    Outside = 2,
}

// =============================================================
// cdrOutlineDashAdjust —— 虚线调整方式
// =============================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum cdrOutlineDashAdjust {
    // TODO: 待补
    None = 0,
    // ...
}

// =============================================================
// cdrArrowHeadType —— 箭头类型
// =============================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum cdrArrowHeadType {
    // TODO: 待补
    None = 0,
    // ...
}