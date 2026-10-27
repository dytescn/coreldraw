//! 图像枚举

// =============================================================
// cdrImageType —— 图像色彩模式
// =============================================================

/// `Bitmap.Mode` 的取值。
///
/// 来源：`VGCore::cdrImageType`
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum cdrImageType {
    /// 黑白（1 位）。
    BlackAndWhite = 0,
    /// 16 色。
    Colors16 = 1,
    /// 灰度。
    Grayscale = 2,
    /// 调色板。
    Paletted = 3,
    /// RGB 彩色。
    RGBColor = 4,
    /// CMYK 彩色。
    CMYKColor = 5,
    /// 双色调。
    Duotone = 6,
}

impl cdrImageType {
    /// 由整数值构造枚举。
    pub fn from_i32(v: i32) -> Option<Self> {
        match v {
            0 => Some(Self::BlackAndWhite),
            1 => Some(Self::Colors16),
            2 => Some(Self::Grayscale),
            3 => Some(Self::Paletted),
            4 => Some(Self::RGBColor),
            5 => Some(Self::CMYKColor),
            6 => Some(Self::Duotone),
            _ => None,
        }
    }
}

// =============================================================
// cdrAntiAliasingType —— 抗锯齿类型
// =============================================================

/// CorelDRAW 枚举 `cdrAntiAliasingType`：抗锯齿类型。
///
/// 对应类型库中的 `VGCore::cdrAntiAliasingType`，用 `#[repr(i32)]` 映射实际整数值。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum cdrAntiAliasingType {
    /// 无 antialiasing。
    NoAntiAliasing = 0,
    /// 常规 antialiasing。
    NormalAntiAliasing = 1,
    /// supersampling。
    Supersampling = 2,
}

impl cdrAntiAliasingType {
    /// 由整数值构造枚举。
    pub fn from_i32(v: i32) -> Option<Self> {
        match v {
            0 => Some(Self::NoAntiAliasing),
            1 => Some(Self::NormalAntiAliasing),
            2 => Some(Self::Supersampling),
            _ => None,
        }
    }
}

// =============================================================
// cdrRenderType —— 黑白渲染类型
// =============================================================

/// CorelDRAW 枚举 `cdrRenderType`：黑白渲染类型。
///
/// 对应类型库中的 `VGCore::cdrRenderType`，用 `#[repr(i32)]` 映射实际整数值。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum cdrRenderType {
    /// render 线 art。
    RenderLineArt = 0,
    /// renderordered。
    RenderOrdered = 1,
    /// renderjarvis。
    RenderJarvis = 2,
    /// renderstucki。
    RenderStucki = 3,
    /// renderfloyd。
    RenderFloyd = 4,
    /// renderhalftone。
    RenderHalftone = 5,
    /// rendercardinality。
    RenderCardinality = 6,
}

// =============================================================
// cdrHalftoneType —— 半色调类型
// =============================================================

/// CorelDRAW 枚举 `cdrHalftoneType`：半色调类型。
///
/// 对应类型库中的 `VGCore::cdrHalftoneType`，用 `#[repr(i32)]` 映射实际整数值。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum cdrHalftoneType {
    /// halftonesquare。
    HalftoneSquare = 0,
    /// halftoneround。
    HalftoneRound = 1,
    /// halftone 线。
    HalftoneLine = 2,
    /// halftone 交叉。
    HalftoneCross = 3,
    /// halftonefixed4x4。
    HalftoneFixed4x4 = 4,
    /// halftonefixed8x8。
    HalftoneFixed8x8 = 5,
}

// =============================================================
// cdrDitherType —— 抖动类型
// =============================================================

/// CorelDRAW 枚举 `cdrDitherType`：抖动类型。
///
/// 对应类型库中的 `VGCore::cdrDitherType`，用 `#[repr(i32)]` 映射实际整数值。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum cdrDitherType {
    /// 抖动无。
    DitherNone = 0,
    /// 抖动 ordered。
    DitherOrdered = 1,
    /// 抖动 jarvis。
    DitherJarvis = 2,
    /// 抖动 stucki。
    DitherStucki = 3,
    /// 抖动 floyd。
    DitherFloyd = 4,
}

// =============================================================
// cdrImagePaletteType —— 调色板类型
// =============================================================

/// CorelDRAW 枚举 `cdrImagePaletteType`：调色板类型。
///
/// 对应类型库中的 `VGCore::cdrImagePaletteType`，用 `#[repr(i32)]` 映射实际整数值。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum cdrImagePaletteType {
    /// 调色板均匀。
    PaletteUniform = 0,
    /// 调色板 stdvga。
    PaletteStdVGA = 1,
    /// 调色板 adaptive。
    PaletteAdaptive = 2,
    /// 调色板 optimized。
    PaletteOptimized = 3,
    /// 调色板黑色 body。
    PaletteBlackBody = 4,
    /// 调色板灰度。
    PaletteGrayscale = 5,
    /// 调色板系统。
    PaletteSystem = 6,
    /// 调色板 ie。
    PaletteIE = 7,
    /// 调色板 netscape。
    PaletteNetscape = 8,
    /// 调色板自定义。
    PaletteCustom = 9,
}

// =============================================================
// cdrTraceType —— 描摹类型
// =============================================================

/// CorelDRAW 枚举 `cdrTraceType`：描摹类型。
///
/// 对应类型库中的 `VGCore::cdrTraceType`，用 `#[repr(i32)]` 映射实际整数值。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum cdrTraceType {
    /// 描摹线 art。
    TraceLineArt = 1,
    /// 描摹 logo。
    TraceLogo = 2,
    /// 描摹 detailedlogo。
    TraceDetailedLogo = 3,
    /// 描摹 clipart。
    TraceClipart = 4,
    /// 描摹 low 质量图像。
    TraceLowQualityImage = 5,
    /// 描摹 high 质量图像。
    TraceHighQualityImage = 6,
    /// 描摹 technical。
    TraceTechnical = 7,
    /// 描摹线 drawing。
    TraceLineDrawing = 8,
}

// =============================================================
// cdrTraceBackgroundMode —— 描摹背景模式
// =============================================================

/// CorelDRAW 枚举 `cdrTraceBackgroundMode`：描摹背景模式。
///
/// 对应类型库中的 `VGCore::cdrTraceBackgroundMode`，用 `#[repr(i32)]` 映射实际整数值。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
#[allow(nonstandard_style)]
pub enum cdrTraceBackgroundMode {
    /// 描摹背景 automatic。
    TraceBackgroundAutomatic = 1,
    /// 描摹背景手动。
    TraceBackgroundManual = 2,
}
