//! `IVGTextRange` / `IVGTextRanges` 鈥斺€?鏂囨湰鑼冨洿

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

pub struct IvgTextRange {
    disp: ComObject,
}

impl IvgTextRange {
    pub fn new(disp: IDispatch) -> Self {
        Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) }
    }

    pub fn from_disp(disp: IDispatch) -> Self {
        Self::new(disp)
    }

    pub fn as_variant(&self) -> Variant {
        self.disp.as_variant()
    }

    pub fn raw(&self) -> &ComObject {
        &self.disp
    }

    // ---------------------------------------------------------
    // 閫氱敤宸ュ叿
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
    // 鍐呭 / 杈圭晫
    // ---------------------------------------------------------

    pub fn text(&self) -> Option<String> { self.prop_string("Text") }
    pub fn set_text(&self, v: impl Into<String>) -> bool { self.put_string("Text", v) }

    pub fn wide_text(&self) -> Option<String> { self.prop_string("WideText") }
    pub fn set_wide_text(&self, v: impl Into<String>) -> bool {
        self.put_string("WideText", v)
    }

    pub fn start(&self) -> Option<i64> { self.prop_i64("Start") }
    pub fn set_start(&self, v: i32) -> bool { self.put_i64("Start", v as i64) }

    pub fn end(&self) -> Option<i64> { self.prop_i64("End") }
    pub fn set_end(&self, v: i32) -> bool { self.put_i64("End", v as i64) }

    pub fn length(&self) -> Option<i64> { self.prop_i64("Length") }
    pub fn set_length(&self, v: i32) -> bool { self.put_i64("Length", v as i64) }

    pub fn is_empty(&self) -> Option<bool> { self.prop_bool("IsEmpty") }

    pub fn set_range(&self, start: i32, end: i32) -> bool {
        let args = vec![
            Variant::from_i64(start as i64),
            Variant::from_i64(end as i64),
        ];
        self.disp.invoke_method("SetRange", args).is_ok()
    }

    // ---------------------------------------------------------
    // 瀛愯鍥?
    // ---------------------------------------------------------

    pub fn characters(&self) -> Option<IvgTextCharacters> {
        self.prop_dispatch("Characters").map(IvgTextCharacters::new)
    }

    pub fn words(&self) -> Option<IvgTextWords> {
        self.prop_dispatch("Words").map(IvgTextWords::new)
    }

    pub fn lines(&self) -> Option<IvgTextLines> {
        self.prop_dispatch("Lines").map(IvgTextLines::new)
    }

    pub fn paragraphs(&self) -> Option<IvgTextParagraphs> {
        self.prop_dispatch("Paragraphs").map(IvgTextParagraphs::new)
    }

    pub fn columns(&self) -> Option<IvgTextColumns> {
        self.prop_dispatch("Columns").map(IvgTextColumns::new)
    }

    pub fn frames(&self) -> Option<IvgTextFrames> {
        self.prop_dispatch("Frames").map(IvgTextFrames::new)
    }

    pub fn tabs(&self) -> Option<IvgTextTabPositions> {
        self.prop_dispatch("Tabs").map(IvgTextTabPositions::new)
    }

    // ---------------------------------------------------------
    // 瀛椾綋
    // ---------------------------------------------------------

    /// `cdrFontStyle`
    pub fn style(&self) -> Option<i64> { self.prop_i64("Style") }
    pub fn set_style(&self, v: i32) -> bool { self.put_i64("Style", v as i64) }

    pub fn bold(&self) -> Option<bool> { self.prop_bool("Bold") }
    pub fn set_bold(&self, v: bool) -> bool { self.put_bool("Bold", v) }

    pub fn italic(&self) -> Option<bool> { self.prop_bool("Italic") }
    pub fn set_italic(&self, v: bool) -> bool { self.put_bool("Italic", v) }

    /// `cdrFontLine`
    pub fn underline(&self) -> Option<i64> { self.prop_i64("Underline") }
    pub fn set_underline(&self, v: i32) -> bool { self.put_i64("Underline", v as i64) }

    pub fn strikethru(&self) -> Option<i64> { self.prop_i64("Strikethru") }
    pub fn set_strikethru(&self, v: i32) -> bool { self.put_i64("Strikethru", v as i64) }

    pub fn overscore(&self) -> Option<i64> { self.prop_i64("Overscore") }
    pub fn set_overscore(&self, v: i32) -> bool { self.put_i64("Overscore", v as i64) }

    pub fn font(&self) -> Option<String> { self.prop_string("Font") }
    pub fn set_font(&self, v: impl Into<String>) -> bool { self.put_string("Font", v) }

    pub fn size(&self) -> Option<f64> { self.prop_f64("Size") }

    pub fn set_size(&self, v: f32) -> bool { self.put_f32("Size", v) }

    /// `cdrFontPosition`
    pub fn position(&self) -> Option<i64> { self.prop_i64("Position") }
    pub fn set_position(&self, v: i32) -> bool { self.put_i64("Position", v as i64) }

    /// `cdrFontCase`
    pub fn case(&self) -> Option<i64> { self.prop_i64("Case") }
    pub fn set_case(&self, v: i32) -> bool { self.put_i64("Case", v as i64) }

    pub fn char_angle(&self) -> Option<f32> { self.prop_f32("CharAngle") }
    pub fn set_char_angle(&self, v: f32) -> bool { self.put_f32("CharAngle", v) }

    /// `cdrTextLanguage`
    pub fn language_id(&self) -> Option<i64> { self.prop_i64("LanguageID") }
    pub fn set_language_id(&self, v: i32) -> bool { self.put_i64("LanguageID", v as i64) }

    /// `cdrTextCharSet`
    pub fn char_set(&self) -> Option<i64> { self.prop_i64("CharSet") }
    pub fn set_char_set(&self, v: i32) -> bool { self.put_i64("CharSet", v as i64) }

    pub fn range_kerning(&self) -> Option<i64> { self.prop_i64("RangeKerning") }
    pub fn set_range_kerning(&self, v: i32) -> bool {
        self.put_i64("RangeKerning", v as i64)
    }

    // ---------------------------------------------------------
    // 娈佃惤
    // ---------------------------------------------------------

    /// `cdrAlignment`
    pub fn alignment(&self) -> Option<i64> { self.prop_i64("Alignment") }
    pub fn set_alignment(&self, v: i32) -> bool { self.put_i64("Alignment", v as i64) }

    pub fn first_line_indent(&self) -> Option<f64> { self.prop_f64("FirstLineIndent") }
    pub fn set_first_line_indent(&self, v: f64) -> bool {
        self.put_f64("FirstLineIndent", v)
    }

    pub fn left_indent(&self) -> Option<f64> { self.prop_f64("LeftIndent") }
    pub fn set_left_indent(&self, v: f64) -> bool { self.put_f64("LeftIndent", v) }

    pub fn right_indent(&self) -> Option<f64> { self.prop_f64("RightIndent") }
    pub fn set_right_indent(&self, v: f64) -> bool { self.put_f64("RightIndent", v) }

    pub fn min_word_spacing(&self) -> Option<f32> { self.prop_f32("MinWordSpacing") }
    pub fn set_min_word_spacing(&self, v: f32) -> bool {
        self.put_f32("MinWordSpacing", v)
    }

    pub fn max_word_spacing(&self) -> Option<f32> { self.prop_f32("MaxWordSpacing") }
    pub fn set_max_word_spacing(&self, v: f32) -> bool {
        self.put_f32("MaxWordSpacing", v)
    }

    pub fn max_char_spacing(&self) -> Option<f32> { self.prop_f32("MaxCharSpacing") }
    pub fn set_max_char_spacing(&self, v: f32) -> bool {
        self.put_f32("MaxCharSpacing", v)
    }

    pub fn char_spacing(&self) -> Option<f32> { self.prop_f32("CharSpacing") }
    pub fn set_char_spacing(&self, v: f32) -> bool { self.put_f32("CharSpacing", v) }

    pub fn word_spacing(&self) -> Option<f32> { self.prop_f32("WordSpacing") }
    pub fn set_word_spacing(&self, v: f32) -> bool { self.put_f32("WordSpacing", v) }

    pub fn line_spacing(&self) -> Option<f32> { self.prop_f32("LineSpacing") }
    pub fn set_line_spacing(&self, v: f32) -> bool { self.put_f32("LineSpacing", v) }

    /// `cdrLineSpacingType`
    pub fn line_spacing_type(&self) -> Option<i64> { self.prop_i64("LineSpacingType") }

    pub fn para_spacing_before(&self) -> Option<f32> { self.prop_f32("ParaSpacingBefore") }
    pub fn set_para_spacing_before(&self, v: f32) -> bool {
        self.put_f32("ParaSpacingBefore", v)
    }

    pub fn para_spacing_after(&self) -> Option<f32> { self.prop_f32("ParaSpacingAfter") }
    pub fn set_para_spacing_after(&self, v: f32) -> bool {
        self.put_f32("ParaSpacingAfter", v)
    }

    // ---------------------------------------------------------
    // 浣嶇Щ
    // ---------------------------------------------------------

    pub fn horiz_shift(&self) -> Option<i64> { self.prop_i64("HorizShift") }
    pub fn set_horiz_shift(&self, v: i32) -> bool { self.put_i64("HorizShift", v as i64) }

    pub fn vert_shift(&self) -> Option<i64> { self.prop_i64("VertShift") }
    pub fn set_vert_shift(&self, v: i32) -> bool { self.put_i64("VertShift", v as i64) }

    // ---------------------------------------------------------
    // 鏂瓧
    // ---------------------------------------------------------

    /// `cdrTriState`
    pub fn auto_hyphenate(&self) -> Option<i64> { self.prop_i64("AutoHyphenate") }
    pub fn set_auto_hyphenate(&self, v: i32) -> bool { self.put_i64("AutoHyphenate", v as i64) }

    pub fn hyphen_hot_zone(&self) -> Option<f64> { self.prop_f64("HyphenHotZone") }
    pub fn set_hyphen_hot_zone(&self, v: f64) -> bool { self.put_f64("HyphenHotZone", v) }

    pub fn hyphen_min_chars_before(&self) -> Option<i64> { self.prop_i64("HyphenMinCharsBefore") }
    pub fn set_hyphen_min_chars_before(&self, v: i32) -> bool {
        self.put_i64("HyphenMinCharsBefore", v as i64)
    }

    pub fn hyphen_min_chars_after(&self) -> Option<i64> { self.prop_i64("HyphenMinCharsAfter") }
    pub fn set_hyphen_min_chars_after(&self, v: i32) -> bool {
        self.put_i64("HyphenMinCharsAfter", v as i64)
    }

    pub fn hyphen_min_word_length(&self) -> Option<i64> { self.prop_i64("HyphenMinWordLength") }
    pub fn set_hyphen_min_word_length(&self, v: i32) -> bool {
        self.put_i64("HyphenMinWordLength", v as i64)
    }

    pub fn hyphenate_capitals(&self) -> Option<bool> { self.prop_bool("HyphenateCapitals") }
    pub fn set_hyphenate_capitals(&self, v: bool) -> bool {
        self.put_bool("HyphenateCapitals", v)
    }

    pub fn hyphenate_all_cap_words(&self) -> Option<bool> {
        self.prop_bool("HyphenateAllCapWords")
    }
    pub fn set_hyphenate_all_cap_words(&self, v: bool) -> bool {
        self.put_bool("HyphenateAllCapWords", v)
    }

    // ---------------------------------------------------------
    // 濉厖 / 杞粨
    // ---------------------------------------------------------

    pub fn fill(&self) -> Option<IvgFill> {
        self.prop_dispatch("Fill").map(IvgFill::new)
    }

    pub fn outline(&self) -> Option<IvgOutline> {
        self.prop_dispatch("Outline").map(IvgOutline::new)
    }

    pub fn char_back_fill(&self) -> Option<IvgFill> {
        self.prop_dispatch("CharBackFill").map(IvgFill::new)
    }

    // ---------------------------------------------------------
    // 鏁堟灉
    // ---------------------------------------------------------

    /// `cdrTextEffect`
    pub fn effect(&self) -> Option<i64> { self.prop_i64("Effect") }

    pub fn apply_no_effect(&self) -> bool {
        self.disp.invoke_method("ApplyNoEffect", vec![]).is_ok()
    }

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
    // 棣栧瓧涓嬫矇 / 椤圭洰绗﹀彿灞炴€?
    // ---------------------------------------------------------

    pub fn drop_cap_lines_dropped(&self) -> Option<i64> {
        self.prop_i64("DropCapLinesDropped")
    }
    pub fn set_drop_cap_lines_dropped(&self, v: i32) -> bool {
        self.put_i64("DropCapLinesDropped", v as i64)
    }

    pub fn drop_cap_distance_from_text(&self) -> Option<f64> {
        self.prop_f64("DropCapDistanceFromText")
    }
    pub fn set_drop_cap_distance_from_text(&self, v: f64) -> bool {
        self.put_f64("DropCapDistanceFromText", v)
    }

    pub fn drop_cap_hanging_indent(&self) -> Option<bool> {
        self.prop_bool("DropCapHangingIndent")
    }
    pub fn set_drop_cap_hanging_indent(&self, v: bool) -> bool {
        self.put_bool("DropCapHangingIndent", v)
    }

    pub fn bullet_font(&self) -> Option<String> { self.prop_string("BulletFont") }
    pub fn set_bullet_font(&self, v: impl Into<String>) -> bool {
        self.put_string("BulletFont", v)
    }

    pub fn bullet_symbol(&self) -> Option<String> { self.prop_string("BulletSymbol") }
    pub fn set_bullet_symbol(&self, v: impl Into<String>) -> bool {
        self.put_string("BulletSymbol", v)
    }

    pub fn bullet_size(&self) -> Option<f32> { self.prop_f32("BulletSize") }
    pub fn set_bullet_size(&self, v: f32) -> bool { self.put_f32("BulletSize", v) }

    pub fn bullet_baseline_shift(&self) -> Option<f32> { self.prop_f32("BulletBaselineShift") }
    pub fn set_bullet_baseline_shift(&self, v: f32) -> bool {
        self.put_f32("BulletBaselineShift", v)
    }

    pub fn bullet_horizontal_position(&self) -> Option<f64> {
        self.prop_f64("BulletHorizontalPosition")
    }
    pub fn set_bullet_horizontal_position(&self, v: f64) -> bool {
        self.put_f64("BulletHorizontalPosition", v)
    }

    pub fn bullet_hanging_indent(&self) -> Option<bool> {
        self.prop_bool("BulletHangingIndent")
    }
    pub fn set_bullet_hanging_indent(&self, v: bool) -> bool {
        self.put_bool("BulletHangingIndent", v)
    }

    // ---------------------------------------------------------
    // 缂栬緫
    // ---------------------------------------------------------

    pub fn delete(&self) -> bool { self.disp.invoke_method("Delete", vec![]).is_ok() }
    pub fn select(&self) -> bool { self.disp.invoke_method("Select", vec![]).is_ok() }
    pub fn copy(&self) -> bool { self.disp.invoke_method("Copy", vec![]).is_ok() }

    pub fn paste(&self) -> Option<IvgTextRange> {
        self.disp
            .invoke_method("Paste", vec![])
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgTextRange::new)
    }

    pub fn collapse(&self, to_end: bool) -> bool {
        let args = vec![Variant::from_bool(to_end)];
        self.disp.invoke_method("Collapse", args).is_ok()
    }

    pub fn combine(&self, range: &IvgTextRange) -> bool {
        let args = vec![range.as_variant()];
        self.disp.invoke_method("Combine", args).is_ok()
    }

    pub fn in_range(&self, range: &IvgTextRange) -> Option<bool> {
        let args = vec![range.as_variant()];
        self.disp.invoke_method("InRange", args).ok()?.to_bool().ok()
    }

    pub fn is_same(&self, range: &IvgTextRange) -> Option<bool> {
        let args = vec![range.as_variant()];
        self.disp.invoke_method("IsSame", args).ok()?.to_bool().ok()
    }

    /// `cdrTextChangeCase`
    pub fn change_case(&self, case: i32) -> bool {
        let args = vec![Variant::from_i64(case as i64)];
        self.disp.invoke_method("ChangeCase", args).is_ok()
    }

    pub fn duplicate(&self) -> Option<IvgTextRange> {
        self.disp
            .invoke_method("Duplicate", vec![])
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgTextRange::new)
    }

    // ---------------------------------------------------------
    // 鎻掑叆
    // ---------------------------------------------------------

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

    /// 瀛愯寖鍥淬€?
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
    // 琛岃窛 / 灞炴€?/ 鏍峰紡
    // ---------------------------------------------------------

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

    pub fn copy_attributes(&self, source: &IvgTextRange) -> bool {
        let args = vec![source.as_variant()];
        self.disp.invoke_method("CopyAttributes", args).is_ok()
    }

    /// 搴旂敤鏍峰紡銆?
    pub fn apply_style(&self, style_name: impl Into<String>) -> bool {
        let args = vec![Variant::from_str(style_name.into())];
        self.disp.invoke_method("ApplyStyle", args).is_ok()
    }

    pub fn object_style(&self) -> Option<crate::style::IvgStyle> {
        self.prop_dispatch("ObjectStyle").map(crate::style::IvgStyle::new)
    }

    // ---------------------------------------------------------
    // 鍩虹嚎 / 鐩寸嚎鍖?
    // ---------------------------------------------------------

    pub fn baselines(&self) -> Option<IvgCurve> {
        self.prop_dispatch("Baselines").map(IvgCurve::new)
    }

    pub fn text_line_rects(&self) -> Option<IvgCurve> {
        self.prop_dispatch("TextLineRects").map(IvgCurve::new)
    }

    pub fn straighten(&self) -> bool {
        self.disp.invoke_method("Straighten", vec![]).is_ok()
    }

    pub fn align_to_baseline(&self) -> bool {
        self.disp.invoke_method("AlignToBaseline", vec![]).is_ok()
    }

    // ---------------------------------------------------------
    // OpenType / 鏂囨湰鏍煎紡鍖?
    // ---------------------------------------------------------

    pub fn get_open_type_feature(&self, feature: impl Into<String>) -> Option<i64> {
        let args = vec![Variant::from_str(feature.into())];
        self.disp
            .invoke_method("GetOpenTypeFeature", args)
            .ok()?
            .to_i64()
            .ok()
    }

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

    pub fn text_formatter(&self) -> Option<i64> { self.prop_i64("TextFormatter") }
    pub fn set_text_formatter(&self, v: i32) -> bool {
        self.put_i64("TextFormatter", v as i64)
    }

    // ---------------------------------------------------------
    // 鏋氫妇瀛愯寖鍥?
    // ---------------------------------------------------------

    /// `cdrTextPropertySet` 灞炴€ч泦銆?
    pub fn enum_ranges(&self, property_filter: i32) -> Option<IvgTextRanges> {
        let args = vec![Variant::from_i64(property_filter as i64)];
        self.disp
            .invoke_method("EnumRanges", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgTextRanges::new)
    }

    pub fn find_ranges(&self, query: impl Into<String>) -> Option<IvgTextRanges> {
        let args = vec![Variant::from_str(query.into())];
        self.disp
            .invoke_method("FindRanges", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgTextRanges::new)
    }

    pub fn evaluate(&self, expr: impl Into<String>) -> Option<Variant> {
        let args = vec![Variant::from_str(expr.into())];
        self.disp.invoke_method("Evaluate", args).ok()
    }
}

// =============================================================
// IvgTextRanges
// =============================================================

pub struct IvgTextRanges {
    disp: ComObject,
}

impl IvgTextRanges {
    pub fn new(disp: IDispatch) -> Self {
        Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) }
    }

    pub fn from_disp(disp: IDispatch) -> Self {
        Self::new(disp)
    }

    fn prop_i64(&self, name: &str) -> Option<i64> {
        self.disp.get_property(name).ok()?.to_i64().ok()
    }

    fn prop_dispatch(&self, name: &str) -> Option<IDispatch> {
        self.disp.get_property(name).ok()?.to_idispatch().ok()
    }

    pub fn count(&self) -> Option<i64> { self.prop_i64("Count") }

    pub fn item(&self, index: i32) -> Option<IvgTextRange> {
        let args = vec![Variant::from_i64(index as i64)];
        self.disp
            .invoke_method("Item", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgTextRange::new)
    }

    pub fn first(&self) -> Option<IvgTextRange> {
        self.prop_dispatch("First").map(IvgTextRange::new)
    }

    pub fn last(&self) -> Option<IvgTextRange> {
        self.prop_dispatch("Last").map(IvgTextRange::new)
    }

    pub fn reverse(&self) -> Option<IvgTextRanges> {
        self.disp
            .invoke_method("Reverse", vec![])
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgTextRanges::new)
    }
}