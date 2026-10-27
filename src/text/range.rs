//! `IVGTextRange` / `IVGTextRanges` —— 文本范围

use wincom::{ComObject, Variant};
use windows::Win32::System::Com::IDispatch;

use crate::curve::IvgCurve;
use crate::fill::IvgFill;
use crate::outline::IvgOutline;
use crate::text::{
    IvgTextCharacters, IvgTextColumns, IvgTextFrames, IvgTextLines,
    IvgTextParagraphs, IvgTextTabPositions, IvgTextWords,
};

// =============================================================
// IvgTextRange
// =============================================================

/// CorelDRAW `IVGTextRange` 对象的后期绑定包装（文本范围）。
///
/// 所有属性与方法通过 `IDispatch::Invoke` 按名称调用，调用失败时一般返回 `None` 或 `false`。
pub struct IvgTextRange {
    disp: ComObject,
}

impl IvgTextRange {
    /// 由 COM `IDispatch` 构造包装对象。
    pub fn new(disp: IDispatch) -> Self {
        Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) }
    }

    /// 从 COM `IDispatch` 构造包装对象，等价于 `new`。
    pub fn from_disp(disp: IDispatch) -> Self {
        Self::new(disp)
    }

    /// 把底层 COM 对象包装成 `VARIANT`，用于传给其它 COM 调用。
    pub fn as_variant(&self) -> Variant {
        self.disp.as_variant()
    }

    /// 获取底层 `ComObject` 的引用。
    pub fn raw(&self) -> &ComObject {
        &self.disp
    }

    // ---------------------------------------------------------
    // 通用工具
    // ---------------------------------------------------------

    fn prop_i64(&self, name: &str) -> Option<i64> {
        self.disp.get_property(name).ok()?.to_i64().ok()
    }

    fn prop_f32(&self, name: &str) -> Option<f32> {
        self.disp.get_property(name).ok()?.to_f64().ok().map(|v| v as f32)
    }

    fn prop_f64(&self, name: &str) -> Option<f64> {
        self.disp.get_property(name).ok()?.to_f64().ok()
    }

    fn prop_bool(&self, name: &str) -> Option<bool> {
        self.disp.get_property(name).ok()?.to_bool().ok()
    }

    fn prop_string(&self, name: &str) -> Option<String> {
        self.disp.get_property(name).ok()?.to_string().ok()
    }

    fn prop_dispatch(&self, name: &str) -> Option<IDispatch> {
        self.disp.get_property(name).ok()?.to_idispatch().ok()
    }

    fn put_i64(&self, name: &str, v: i64) -> bool {
        let arg = Variant::from_i64(v);
        self.disp.set_property(name, vec![arg]).is_ok()
    }

    fn put_f32(&self, name: &str, v: f32) -> bool {
        let arg = Variant::from_f64(v as f64);
        self.disp.set_property(name, vec![arg]).is_ok()
    }

    fn put_f64(&self, name: &str, v: f64) -> bool {
        let arg = Variant::from_f64(v);
        self.disp.set_property(name, vec![arg]).is_ok()
    }

    fn put_bool(&self, name: &str, v: bool) -> bool {
        let arg = Variant::from_bool(v);
        self.disp.set_property(name, vec![arg]).is_ok()
    }

    fn put_string(&self, name: &str, v: impl Into<String>) -> bool {
        let arg = Variant::from_str(v.into());
        self.disp.set_property(name, vec![arg]).is_ok()
    }

    // fn put_dispatch(&self, name: &str, v: Variant) -> bool {
    //     self.disp.set_property(name, vec![v]).is_ok()
    // }

    // ---------------------------------------------------------
    // 内容 / 边界
    // ---------------------------------------------------------

    /// 读取文本（`Text`）。
    pub fn text(&self) -> Option<String> { self.prop_string("Text") }
    /// 设置文本（`Text`）。
    pub fn set_text(&self, v: impl Into<String>) -> bool { self.put_string("Text", v) }

    /// 读取宽文本（`WideText`）。
    pub fn wide_text(&self) -> Option<String> { self.prop_string("WideText") }
    /// 设置宽文本（`WideText`）。
    pub fn set_wide_text(&self, v: impl Into<String>) -> bool {
        self.put_string("WideText", v)
    }

    /// 读取开始（`Start`）。
    pub fn start(&self) -> Option<i64> { self.prop_i64("Start") }
    /// 设置开始（`Start`）。
    pub fn set_start(&self, v: i32) -> bool { self.put_i64("Start", v as i64) }

    /// 读取 end（`End`）。
    pub fn end(&self) -> Option<i64> { self.prop_i64("End") }
    /// 设置 end（`End`）。
    pub fn set_end(&self, v: i32) -> bool { self.put_i64("End", v as i64) }

    /// 读取长度（`Length`）。
    pub fn length(&self) -> Option<i64> { self.prop_i64("Length") }
    /// 设置长度（`Length`）。
    pub fn set_length(&self, v: i32) -> bool { self.put_i64("Length", v as i64) }

    /// 是否空（`IsEmpty`）。
    pub fn is_empty(&self) -> Option<bool> { self.prop_bool("IsEmpty") }

    /// 设置范围（`SetRange`）。
    pub fn set_range(&self, start: i32, end: i32) -> bool {
        let args = vec![
            Variant::from_i64(start as i64),
            Variant::from_i64(end as i64),
        ];
        self.disp.invoke_method("SetRange", args).is_ok()
    }

    // ---------------------------------------------------------
    // 子视图
    // ---------------------------------------------------------

    /// 读取字符集（`Characters`）。
    pub fn characters(&self) -> Option<IvgTextCharacters> {
        self.prop_dispatch("Characters").map(IvgTextCharacters::new)
    }

    /// 读取词集（`Words`）。
    pub fn words(&self) -> Option<IvgTextWords> {
        self.prop_dispatch("Words").map(IvgTextWords::new)
    }

    /// 读取线（`Lines`）。
    pub fn lines(&self) -> Option<IvgTextLines> {
        self.prop_dispatch("Lines").map(IvgTextLines::new)
    }

    /// 读取段落集（`Paragraphs`）。
    pub fn paragraphs(&self) -> Option<IvgTextParagraphs> {
        self.prop_dispatch("Paragraphs").map(IvgTextParagraphs::new)
    }

    /// 读取分栏集（`Columns`）。
    pub fn columns(&self) -> Option<IvgTextColumns> {
        self.prop_dispatch("Columns").map(IvgTextColumns::new)
    }

    /// 读取文本框集（`Frames`）。
    pub fn frames(&self) -> Option<IvgTextFrames> {
        self.prop_dispatch("Frames").map(IvgTextFrames::new)
    }

    /// 读取制表位集（`Tabs`）。
    pub fn tabs(&self) -> Option<IvgTextTabPositions> {
        self.prop_dispatch("Tabs").map(IvgTextTabPositions::new)
    }

    // ---------------------------------------------------------
    // 字体
    // ---------------------------------------------------------

    /// `cdrFontStyle`
    pub fn style(&self) -> Option<i64> { self.prop_i64("Style") }
    /// 设置样式（`Style`）。
    pub fn set_style(&self, v: i32) -> bool { self.put_i64("Style", v as i64) }

    /// 读取加粗（`Bold`）。
    pub fn bold(&self) -> Option<bool> { self.prop_bool("Bold") }
    /// 设置加粗（`Bold`）。
    pub fn set_bold(&self, v: bool) -> bool { self.put_bool("Bold", v) }

    /// 读取斜体（`Italic`）。
    pub fn italic(&self) -> Option<bool> { self.prop_bool("Italic") }
    /// 设置斜体（`Italic`）。
    pub fn set_italic(&self, v: bool) -> bool { self.put_bool("Italic", v) }

    /// `cdrFontLine`
    pub fn underline(&self) -> Option<i64> { self.prop_i64("Underline") }
    /// 设置下划线（`Underline`）。
    pub fn set_underline(&self, v: i32) -> bool { self.put_i64("Underline", v as i64) }

    /// 读取删除线（`Strikethru`）。
    pub fn strikethru(&self) -> Option<i64> { self.prop_i64("Strikethru") }
    /// 设置删除线（`Strikethru`）。
    pub fn set_strikethru(&self, v: i32) -> bool { self.put_i64("Strikethru", v as i64) }

    /// 读取上划线（`Overscore`）。
    pub fn overscore(&self) -> Option<i64> { self.prop_i64("Overscore") }
    /// 设置上划线（`Overscore`）。
    pub fn set_overscore(&self, v: i32) -> bool { self.put_i64("Overscore", v as i64) }

    /// 读取字体（`Font`）。
    pub fn font(&self) -> Option<String> { self.prop_string("Font") }
    /// 设置字体（`Font`）。
    pub fn set_font(&self, v: impl Into<String>) -> bool { self.put_string("Font", v) }

    /// 读取尺寸（`Size`）。
    pub fn size(&self) -> Option<f64> { self.prop_f64("Size") }

    /// 设置尺寸（`Size`）。
    pub fn set_size(&self, v: f32) -> bool { self.put_f32("Size", v) }

    /// `cdrFontPosition`
    pub fn position(&self) -> Option<i64> { self.prop_i64("Position") }
    /// 设置位置（`Position`）。
    pub fn set_position(&self, v: i32) -> bool { self.put_i64("Position", v as i64) }

    /// `cdrFontCase`
    pub fn case(&self) -> Option<i64> { self.prop_i64("Case") }
    /// 设置 COM 属性 `Case`。
    pub fn set_case(&self, v: i32) -> bool { self.put_i64("Case", v as i64) }

    /// 读取字符角度（`CharAngle`）。
    pub fn char_angle(&self) -> Option<f32> { self.prop_f32("CharAngle") }
    /// 设置字符角度（`CharAngle`）。
    pub fn set_char_angle(&self, v: f32) -> bool { self.put_f32("CharAngle", v) }

    /// `cdrTextLanguage`
    pub fn language_id(&self) -> Option<i64> { self.prop_i64("LanguageID") }
    /// 设置语言标识（`LanguageID`）。
    pub fn set_language_id(&self, v: i32) -> bool { self.put_i64("LanguageID", v as i64) }

    /// `cdrTextCharSet`
    pub fn char_set(&self) -> Option<i64> { self.prop_i64("CharSet") }
    /// 设置字符设置（`CharSet`）。
    pub fn set_char_set(&self, v: i32) -> bool { self.put_i64("CharSet", v as i64) }

    /// 读取范围字距调整（`RangeKerning`）。
    pub fn range_kerning(&self) -> Option<i64> { self.prop_i64("RangeKerning") }
    /// 设置范围字距调整（`RangeKerning`）。
    pub fn set_range_kerning(&self, v: i32) -> bool {
        self.put_i64("RangeKerning", v as i64)
    }

    // ---------------------------------------------------------
    // 段落
    // ---------------------------------------------------------

    /// `cdrAlignment`
    pub fn alignment(&self) -> Option<i64> { self.prop_i64("Alignment") }
    /// 设置对齐（`Alignment`）。
    pub fn set_alignment(&self, v: i32) -> bool { self.put_i64("Alignment", v as i64) }

    /// 读取第一个线缩进（`FirstLineIndent`）。
    pub fn first_line_indent(&self) -> Option<f64> { self.prop_f64("FirstLineIndent") }
    /// 设置第一个线缩进（`FirstLineIndent`）。
    pub fn set_first_line_indent(&self, v: f64) -> bool {
        self.put_f64("FirstLineIndent", v)
    }

    /// 读取左缩进（`LeftIndent`）。
    pub fn left_indent(&self) -> Option<f64> { self.prop_f64("LeftIndent") }
    /// 设置左缩进（`LeftIndent`）。
    pub fn set_left_indent(&self, v: f64) -> bool { self.put_f64("LeftIndent", v) }

    /// 读取右缩进（`RightIndent`）。
    pub fn right_indent(&self) -> Option<f64> { self.prop_f64("RightIndent") }
    /// 设置右缩进（`RightIndent`）。
    pub fn set_right_indent(&self, v: f64) -> bool { self.put_f64("RightIndent", v) }

    /// 读取最小词间距（`MinWordSpacing`）。
    pub fn min_word_spacing(&self) -> Option<f32> { self.prop_f32("MinWordSpacing") }
    /// 设置最小词间距（`MinWordSpacing`）。
    pub fn set_min_word_spacing(&self, v: f32) -> bool {
        self.put_f32("MinWordSpacing", v)
    }

    /// 读取最大词间距（`MaxWordSpacing`）。
    pub fn max_word_spacing(&self) -> Option<f32> { self.prop_f32("MaxWordSpacing") }
    /// 设置最大词间距（`MaxWordSpacing`）。
    pub fn set_max_word_spacing(&self, v: f32) -> bool {
        self.put_f32("MaxWordSpacing", v)
    }

    /// 读取最大字符间距（`MaxCharSpacing`）。
    pub fn max_char_spacing(&self) -> Option<f32> { self.prop_f32("MaxCharSpacing") }
    /// 设置最大字符间距（`MaxCharSpacing`）。
    pub fn set_max_char_spacing(&self, v: f32) -> bool {
        self.put_f32("MaxCharSpacing", v)
    }

    /// 读取字符间距（`CharSpacing`）。
    pub fn char_spacing(&self) -> Option<f32> { self.prop_f32("CharSpacing") }
    /// 设置字符间距（`CharSpacing`）。
    pub fn set_char_spacing(&self, v: f32) -> bool { self.put_f32("CharSpacing", v) }

    /// 读取词间距（`WordSpacing`）。
    pub fn word_spacing(&self) -> Option<f32> { self.prop_f32("WordSpacing") }
    /// 设置词间距（`WordSpacing`）。
    pub fn set_word_spacing(&self, v: f32) -> bool { self.put_f32("WordSpacing", v) }

    /// 读取线间距（`LineSpacing`）。
    pub fn line_spacing(&self) -> Option<f32> { self.prop_f32("LineSpacing") }
    /// 设置线间距（`LineSpacing`）。
    pub fn set_line_spacing(&self, v: f32) -> bool { self.put_f32("LineSpacing", v) }

    /// `cdrLineSpacingType`
    pub fn line_spacing_type(&self) -> Option<i64> { self.prop_i64("LineSpacingType") }

    /// 读取段落间距之前（`ParaSpacingBefore`）。
    pub fn para_spacing_before(&self) -> Option<f32> { self.prop_f32("ParaSpacingBefore") }
    /// 设置段落间距之前（`ParaSpacingBefore`）。
    pub fn set_para_spacing_before(&self, v: f32) -> bool {
        self.put_f32("ParaSpacingBefore", v)
    }

    /// 读取段落间距之后（`ParaSpacingAfter`）。
    pub fn para_spacing_after(&self) -> Option<f32> { self.prop_f32("ParaSpacingAfter") }
    /// 设置段落间距之后（`ParaSpacingAfter`）。
    pub fn set_para_spacing_after(&self, v: f32) -> bool {
        self.put_f32("ParaSpacingAfter", v)
    }

    // ---------------------------------------------------------
    // 位移
    // ---------------------------------------------------------

    /// 读取水平偏移（`HorizShift`）。
    pub fn horiz_shift(&self) -> Option<i64> { self.prop_i64("HorizShift") }
    /// 设置水平偏移（`HorizShift`）。
    pub fn set_horiz_shift(&self, v: i32) -> bool { self.put_i64("HorizShift", v as i64) }

    /// 读取垂直偏移（`VertShift`）。
    pub fn vert_shift(&self) -> Option<i64> { self.prop_i64("VertShift") }
    /// 设置垂直偏移（`VertShift`）。
    pub fn set_vert_shift(&self, v: i32) -> bool { self.put_i64("VertShift", v as i64) }

    // ---------------------------------------------------------
    // 断字
    // ---------------------------------------------------------

    /// `cdrTriState`
    pub fn auto_hyphenate(&self) -> Option<i64> { self.prop_i64("AutoHyphenate") }
    /// 设置自动断字（`AutoHyphenate`）。
    pub fn set_auto_hyphenate(&self, v: i32) -> bool { self.put_i64("AutoHyphenate", v as i64) }

    /// 读取连字符热区域（`HyphenHotZone`）。
    pub fn hyphen_hot_zone(&self) -> Option<f64> { self.prop_f64("HyphenHotZone") }
    /// 设置连字符热区域（`HyphenHotZone`）。
    pub fn set_hyphen_hot_zone(&self, v: f64) -> bool { self.put_f64("HyphenHotZone", v) }

    /// 读取连字符最小字符之前（`HyphenMinCharsBefore`）。
    pub fn hyphen_min_chars_before(&self) -> Option<i64> { self.prop_i64("HyphenMinCharsBefore") }
    /// 设置连字符最小字符之前（`HyphenMinCharsBefore`）。
    pub fn set_hyphen_min_chars_before(&self, v: i32) -> bool {
        self.put_i64("HyphenMinCharsBefore", v as i64)
    }

    /// 读取连字符最小字符之后（`HyphenMinCharsAfter`）。
    pub fn hyphen_min_chars_after(&self) -> Option<i64> { self.prop_i64("HyphenMinCharsAfter") }
    /// 设置连字符最小字符之后（`HyphenMinCharsAfter`）。
    pub fn set_hyphen_min_chars_after(&self, v: i32) -> bool {
        self.put_i64("HyphenMinCharsAfter", v as i64)
    }

    /// 读取连字符最小词长度（`HyphenMinWordLength`）。
    pub fn hyphen_min_word_length(&self) -> Option<i64> { self.prop_i64("HyphenMinWordLength") }
    /// 设置连字符最小词长度（`HyphenMinWordLength`）。
    pub fn set_hyphen_min_word_length(&self, v: i32) -> bool {
        self.put_i64("HyphenMinWordLength", v as i64)
    }

    /// 读取 COM 属性 `HyphenateCapitals`。
    pub fn hyphenate_capitals(&self) -> Option<bool> { self.prop_bool("HyphenateCapitals") }
    /// 设置 COM 属性 `HyphenateCapitals`。
    pub fn set_hyphenate_capitals(&self, v: bool) -> bool {
        self.put_bool("HyphenateCapitals", v)
    }

    /// 读取断字全部端点词集（`HyphenateAllCapWords`）。
    pub fn hyphenate_all_cap_words(&self) -> Option<bool> {
        self.prop_bool("HyphenateAllCapWords")
    }
    /// 设置断字全部端点词集（`HyphenateAllCapWords`）。
    pub fn set_hyphenate_all_cap_words(&self, v: bool) -> bool {
        self.put_bool("HyphenateAllCapWords", v)
    }

    // ---------------------------------------------------------
    // 填充 / 轮廓
    // ---------------------------------------------------------

    /// 读取填充（`Fill`）。
    pub fn fill(&self) -> Option<IvgFill> {
        self.prop_dispatch("Fill").map(IvgFill::new)
    }

    /// 读取轮廓（`Outline`）。
    pub fn outline(&self) -> Option<IvgOutline> {
        self.prop_dispatch("Outline").map(IvgOutline::new)
    }

    /// 读取字符后填充（`CharBackFill`）。
    pub fn char_back_fill(&self) -> Option<IvgFill> {
        self.prop_dispatch("CharBackFill").map(IvgFill::new)
    }

    // ---------------------------------------------------------
    // 效果
    // ---------------------------------------------------------

    /// `cdrTextEffect`
    pub fn effect(&self) -> Option<i64> { self.prop_i64("Effect") }

    /// 应用无效果（COM 方法 `ApplyNoEffect`）。
    pub fn apply_no_effect(&self) -> bool {
        self.disp.invoke_method("ApplyNoEffect", vec![]).is_ok()
    }

    /// 应用项目符号效果（COM 方法 `ApplyBulletEffect`）。
    pub fn apply_bullet_effect(
        &self,
        symbol: impl Into<String>,
        font: impl Into<String>,
        size: f32,
        baseline_shift: f32,
        horizontal_position: f64,
        hanging_indent: bool,
    ) -> bool {
        let args = vec![
            Variant::from_str(symbol.into()),
            Variant::from_str(font.into()),
            Variant::from_f64(size as f64),
            Variant::from_f64(baseline_shift as f64),
            Variant::from_f64(horizontal_position),
            Variant::from_bool(hanging_indent),
        ];
        self.disp.invoke_method("ApplyBulletEffect", args).is_ok()
    }

    /// 应用投影端点效果（COM 方法 `ApplyDropCapEffect`）。
    pub fn apply_drop_cap_effect(
        &self,
        lines_dropped: i32,
        distance_from_text: f64,
        hanging_indent: bool,
    ) -> bool {
        let args = vec![
            Variant::from_i64(lines_dropped as i64),
            Variant::from_f64(distance_from_text),
            Variant::from_bool(hanging_indent),
        ];
        self.disp.invoke_method("ApplyDropCapEffect", args).is_ok()
    }

    // ---------------------------------------------------------
    // 首字下沉 / 项目符号属性
    // ---------------------------------------------------------

    /// 读取 COM 属性 `DropCapLinesDropped`。
    pub fn drop_cap_lines_dropped(&self) -> Option<i64> {
        self.prop_i64("DropCapLinesDropped")
    }
    /// 设置 COM 属性 `DropCapLinesDropped`。
    pub fn set_drop_cap_lines_dropped(&self, v: i32) -> bool {
        self.put_i64("DropCapLinesDropped", v as i64)
    }

    /// 读取投影端点距离文本（`DropCapDistanceFromText`）。
    pub fn drop_cap_distance_from_text(&self) -> Option<f64> {
        self.prop_f64("DropCapDistanceFromText")
    }
    /// 设置投影端点距离文本（`DropCapDistanceFromText`）。
    pub fn set_drop_cap_distance_from_text(&self, v: f64) -> bool {
        self.put_f64("DropCapDistanceFromText", v)
    }

    /// 读取投影端点悬挂缩进（`DropCapHangingIndent`）。
    pub fn drop_cap_hanging_indent(&self) -> Option<bool> {
        self.prop_bool("DropCapHangingIndent")
    }
    /// 设置投影端点悬挂缩进（`DropCapHangingIndent`）。
    pub fn set_drop_cap_hanging_indent(&self, v: bool) -> bool {
        self.put_bool("DropCapHangingIndent", v)
    }

    /// 读取项目符号字体（`BulletFont`）。
    pub fn bullet_font(&self) -> Option<String> { self.prop_string("BulletFont") }
    /// 设置项目符号字体（`BulletFont`）。
    pub fn set_bullet_font(&self, v: impl Into<String>) -> bool {
        self.put_string("BulletFont", v)
    }

    /// 读取项目符号符号（`BulletSymbol`）。
    pub fn bullet_symbol(&self) -> Option<String> { self.prop_string("BulletSymbol") }
    /// 设置项目符号符号（`BulletSymbol`）。
    pub fn set_bullet_symbol(&self, v: impl Into<String>) -> bool {
        self.put_string("BulletSymbol", v)
    }

    /// 读取项目符号尺寸（`BulletSize`）。
    pub fn bullet_size(&self) -> Option<f32> { self.prop_f32("BulletSize") }
    /// 设置项目符号尺寸（`BulletSize`）。
    pub fn set_bullet_size(&self, v: f32) -> bool { self.put_f32("BulletSize", v) }

    /// 读取 COM 属性 `BulletBaselineShift`。
    pub fn bullet_baseline_shift(&self) -> Option<f32> { self.prop_f32("BulletBaselineShift") }
    /// 设置 COM 属性 `BulletBaselineShift`。
    pub fn set_bullet_baseline_shift(&self, v: f32) -> bool {
        self.put_f32("BulletBaselineShift", v)
    }

    /// 读取项目符号水平位置（`BulletHorizontalPosition`）。
    pub fn bullet_horizontal_position(&self) -> Option<f64> {
        self.prop_f64("BulletHorizontalPosition")
    }
    /// 设置项目符号水平位置（`BulletHorizontalPosition`）。
    pub fn set_bullet_horizontal_position(&self, v: f64) -> bool {
        self.put_f64("BulletHorizontalPosition", v)
    }

    /// 读取项目符号悬挂缩进（`BulletHangingIndent`）。
    pub fn bullet_hanging_indent(&self) -> Option<bool> {
        self.prop_bool("BulletHangingIndent")
    }
    /// 设置项目符号悬挂缩进（`BulletHangingIndent`）。
    pub fn set_bullet_hanging_indent(&self, v: bool) -> bool {
        self.put_bool("BulletHangingIndent", v)
    }

    // ---------------------------------------------------------
    // 编辑
    // ---------------------------------------------------------

    /// 删除（COM 方法 `Delete`）。
    pub fn delete(&self) -> bool { self.disp.invoke_method("Delete", vec![]).is_ok() }
    /// 选择（COM 方法 `Select`）。
    pub fn select(&self) -> bool { self.disp.invoke_method("Select", vec![]).is_ok() }
    /// 复制（COM 方法 `Copy`）。
    pub fn copy(&self) -> bool { self.disp.invoke_method("Copy", vec![]).is_ok() }

    /// paste（COM 方法 `Paste`）。
    pub fn paste(&self) -> Option<IvgTextRange> {
        self.disp
            .invoke_method("Paste", vec![])
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgTextRange::new)
    }

    /// 调用 COM 方法 `Collapse`。
    pub fn collapse(&self, to_end: bool) -> bool {
        let args = vec![Variant::from_bool(to_end)];
        self.disp.invoke_method("Collapse", args).is_ok()
    }

    /// combine（COM 方法 `Combine`）。
    pub fn combine(&self, range: &IvgTextRange) -> bool {
        let args = vec![range.as_variant()];
        self.disp.invoke_method("Combine", args).is_ok()
    }

    /// 在范围（COM 方法 `InRange`）。
    pub fn in_range(&self, range: &IvgTextRange) -> Option<bool> {
        let args = vec![range.as_variant()];
        self.disp.invoke_method("InRange", args).ok()?.to_bool().ok()
    }

    /// 是否相同（COM 方法 `IsSame`）。
    pub fn is_same(&self, range: &IvgTextRange) -> Option<bool> {
        let args = vec![range.as_variant()];
        self.disp.invoke_method("IsSame", args).ok()?.to_bool().ok()
    }

    /// `cdrTextChangeCase`
    pub fn change_case(&self, case: i32) -> bool {
        let args = vec![Variant::from_i64(case as i64)];
        self.disp.invoke_method("ChangeCase", args).is_ok()
    }

    /// duplicate（COM 方法 `Duplicate`）。
    pub fn duplicate(&self) -> Option<IvgTextRange> {
        self.disp
            .invoke_method("Duplicate", vec![])
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgTextRange::new)
    }

    // ---------------------------------------------------------
    // 插入
    // ---------------------------------------------------------

    /// 插入之前（COM 方法 `InsertBefore`）。
    #[allow(clippy::too_many_arguments)]
    pub fn insert_before(
        &self,
        text: impl Into<String>,
        language_id: i32,
        char_set: i32,
        font: impl Into<String>,
    ) -> Option<IvgTextRange> {
        let args = vec![
            Variant::from_str(text.into()),
            Variant::from_i64(language_id as i64),
            Variant::from_i64(char_set as i64),
            Variant::from_str(font.into()),
        ];
        self.disp
            .invoke_method("InsertBefore", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgTextRange::new)
    }

    /// 插入之后（COM 方法 `InsertAfter`）。
    #[allow(clippy::too_many_arguments)]
    pub fn insert_after(
        &self,
        text: impl Into<String>,
        language_id: i32,
        char_set: i32,
        font: impl Into<String>,
    ) -> Option<IvgTextRange> {
        let args = vec![
            Variant::from_str(text.into()),
            Variant::from_i64(language_id as i64),
            Variant::from_i64(char_set as i64),
            Variant::from_str(font.into()),
        ];
        self.disp
            .invoke_method("InsertAfter", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgTextRange::new)
    }

    /// 子范围。
    pub fn range(&self, start: i32, end: i32) -> Option<IvgTextRange> {
        let args = vec![
            Variant::from_i64(start as i64),
            Variant::from_i64(end as i64),
        ];
        self.disp
            .invoke_method("Range", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgTextRange::new)
    }

    // ---------------------------------------------------------
    // 行距 / 属性/ 样式
    // ---------------------------------------------------------

    /// 设置线间距扩展版（`SetLineSpacing`）。
    pub fn set_line_spacing_ex(
        &self,
        line_spacing_type: i32,
        line_spacing: f32,
        para_before: f32,
        para_after: f32,
    ) -> bool {
        let args = vec![
            Variant::from_i64(line_spacing_type as i64),
            Variant::from_f64(line_spacing as f64),
            Variant::from_f64(para_before as f64),
            Variant::from_f64(para_after as f64),
        ];
        self.disp.invoke_method("SetLineSpacing", args).is_ok()
    }

    /// 调用 COM 方法 `CopyAttributes`。
    pub fn copy_attributes(&self, source: &IvgTextRange) -> bool {
        let args = vec![source.as_variant()];
        self.disp.invoke_method("CopyAttributes", args).is_ok()
    }

    /// 应用样式。
    pub fn apply_style(&self, style_name: impl Into<String>) -> bool {
        let args = vec![Variant::from_str(style_name.into())];
        self.disp.invoke_method("ApplyStyle", args).is_ok()
    }

    /// 读取对象样式（`ObjectStyle`）。
    pub fn object_style(&self) -> Option<crate::style::IvgStyle> {
        self.prop_dispatch("ObjectStyle").map(crate::style::IvgStyle::new)
    }

    // ---------------------------------------------------------
    // 基线 / 直线北
    // ---------------------------------------------------------

    /// 读取 COM 属性 `Baselines`。
    pub fn baselines(&self) -> Option<IvgCurve> {
        self.prop_dispatch("Baselines").map(IvgCurve::new)
    }

    /// 读取 COM 属性 `TextLineRects`。
    pub fn text_line_rects(&self) -> Option<IvgCurve> {
        self.prop_dispatch("TextLineRects").map(IvgCurve::new)
    }

    /// 调用 COM 方法 `Straighten`。
    pub fn straighten(&self) -> bool {
        self.disp.invoke_method("Straighten", vec![]).is_ok()
    }

    /// 调用 COM 方法 `AlignToBaseline`。
    pub fn align_to_baseline(&self) -> bool {
        self.disp.invoke_method("AlignToBaseline", vec![]).is_ok()
    }

    // ---------------------------------------------------------
    // OpenType / 文本格式化
    // ---------------------------------------------------------

    /// 读取 COM 属性 `GetOpenTypeFeature`。
    pub fn get_open_type_feature(&self, feature: impl Into<String>) -> Option<i64> {
        let args = vec![Variant::from_str(feature.into())];
        self.disp
            .invoke_method("GetOpenTypeFeature", args)
            .ok()?
            .to_i64()
            .ok()
    }

    /// 设置 COM 属性 `SetOpenTypeFeature`。
    pub fn set_open_type_feature(
        &self,
        feature: impl Into<String>,
        state: i32,
    ) -> bool {
        let args = vec![
            Variant::from_str(feature.into()),
            Variant::from_i64(state as i64),
        ];
        self.disp.invoke_method("SetOpenTypeFeature", args).is_ok()
    }

    /// 读取 COM 属性 `TextFormatter`。
    pub fn text_formatter(&self) -> Option<i64> { self.prop_i64("TextFormatter") }
    /// 设置 COM 属性 `TextFormatter`。
    pub fn set_text_formatter(&self, v: i32) -> bool {
        self.put_i64("TextFormatter", v as i64)
    }

    // ---------------------------------------------------------
    // 枚举子范围
    // ---------------------------------------------------------

    /// `cdrTextPropertySet` 属性集。
    pub fn enum_ranges(&self, property_filter: i32) -> Option<IvgTextRanges> {
        let args = vec![Variant::from_i64(property_filter as i64)];
        self.disp
            .invoke_method("EnumRanges", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgTextRanges::new)
    }

    /// 查找范围集（COM 方法 `FindRanges`）。
    pub fn find_ranges(&self, query: impl Into<String>) -> Option<IvgTextRanges> {
        let args = vec![Variant::from_str(query.into())];
        self.disp
            .invoke_method("FindRanges", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgTextRanges::new)
    }

    /// evaluate（COM 方法 `Evaluate`）。
    pub fn evaluate(&self, expr: impl Into<String>) -> Option<Variant> {
        let args = vec![Variant::from_str(expr.into())];
        self.disp.invoke_method("Evaluate", args).ok()
    }
}

// =============================================================
// IvgTextRanges
// =============================================================

/// CorelDRAW `IVGTextRange` 对象的后期绑定包装（文本范围集）。
///
/// 所有属性与方法通过 `IDispatch::Invoke` 按名称调用，调用失败时一般返回 `None` 或 `false`。
pub struct IvgTextRanges {
    disp: ComObject,
}

impl IvgTextRanges {
    /// 由 COM `IDispatch` 构造包装对象。
    pub fn new(disp: IDispatch) -> Self {
        Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) }
    }

    /// 从 COM `IDispatch` 构造包装对象，等价于 `new`。
    pub fn from_disp(disp: IDispatch) -> Self {
        Self::new(disp)
    }

    fn prop_i64(&self, name: &str) -> Option<i64> {
        self.disp.get_property(name).ok()?.to_i64().ok()
    }

    fn prop_dispatch(&self, name: &str) -> Option<IDispatch> {
        self.disp.get_property(name).ok()?.to_idispatch().ok()
    }

    /// 读取返回条目数量（`Count`）。
    pub fn count(&self) -> Option<i64> { self.prop_i64("Count") }

    /// 按索引或名称取条目（COM 方法 `Item`）。
    pub fn item(&self, index: i32) -> Option<IvgTextRange> {
        let args = vec![Variant::from_i64(index as i64)];
        self.disp
            .invoke_method("Item", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgTextRange::new)
    }

    /// 读取返回第一个条目（`First`）。
    pub fn first(&self) -> Option<IvgTextRange> {
        self.prop_dispatch("First").map(IvgTextRange::new)
    }

    /// 读取返回最后一个条目（`Last`）。
    pub fn last(&self) -> Option<IvgTextRange> {
        self.prop_dispatch("Last").map(IvgTextRange::new)
    }

    /// reverse（COM 方法 `Reverse`）。
    pub fn reverse(&self) -> Option<IvgTextRanges> {
        self.disp
            .invoke_method("Reverse", vec![])
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgTextRanges::new)
    }
}