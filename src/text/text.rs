//! `IVGText` 鈥斺€?鏂囨湰瀵硅薄
//!
//! 鐢?`shape.text()` 鍙栧緱锛屾槸鏂囨湰鎿嶄綔鐨?*鎬诲叆鍙?*銆?

use wincom::{ComObject, Variant};
use windows::Win32::System::Com::IDispatch;

use crate::effect::IvgEffect;
use crate::shape::IvgShape;
use crate::structs::{
    IvgStructAlignProperties, IvgStructFontProperties,
    IvgStructHyphenationSettings, IvgStructSpaceProperties,
};
use crate::text::{IvgTextFrame, IvgTextFrames, IvgTextRange};

pub struct IvgText {
    disp: ComObject,
}

impl IvgText {
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

    fn prop_bool(&self, name: &str) -> Option<bool> {
        self.disp.get_property(name).ok()?.to_bool().ok()
    }

    fn prop_dispatch(&self, name: &str) -> Option<IDispatch> {
        self.disp.get_property(name).ok()?.to_idispatch().ok()
    }

    // fn put_dispatch(&self, name: &str, v: Variant) -> bool {
    //     self.disp.set_property(name, vec![v]).is_ok()
    // }

    // ---------------------------------------------------------
    // 鍩烘湰灞炴€?
    // ---------------------------------------------------------

    /// `cdrTextType` 鈥斺€?Artistic / Paragraph
    pub fn text_type(&self) -> Option<i64> { self.prop_i64("Type") }

    /// 閾句腑鐨勬枃鏈鏁伴噺銆?
    pub fn frames_in_link(&self) -> Option<i64> { self.prop_i64("FramesInLink") }
    pub fn unused_frames_in_link(&self) -> Option<i64> {
        self.prop_i64("UnusedFramesInLink")
    }

    /// 鏄惁鏈夋孩鍑烘枃鏈€?
    pub fn overflow(&self) -> Option<bool> { self.prop_bool("Overflow") }

    pub fn is_artistic_text(&self) -> Option<bool> { self.prop_bool("IsArtisticText") }
    pub fn is_editing(&self) -> Option<bool> { self.prop_bool("IsEditing") }
    pub fn is_html_compatible(&self) -> Option<bool> {
        self.prop_bool("IsHTMLCompatible")
    }

    // ---------------------------------------------------------
    // 瀛椾綋灞炴€?
    // ---------------------------------------------------------

    /// `cdrTextFrames` 鑼冨洿锛岃繑鍥?`IvgStructFontProperties`銆?
    pub fn font_properties(&self, frames: i32) -> Option<IvgStructFontProperties> {
        let args = vec![Variant::from_i64(frames as i64)];
        self.disp
            .invoke_method("FontProperties", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgStructFontProperties::new)
    }

    pub fn set_font_properties(
        &self,
        frames: i32,
        props: &IvgStructFontProperties,
    ) -> bool {
        let args = vec![
            Variant::from_i64(frames as i64),
            props.as_variant(),
        ];
        self.disp.invoke_method("put_FontProperties", args).is_ok()
    }

    /// `cdrTextIndexingType`銆?
    pub fn font_properties_in_range(
        &self,
        start: i32,
        count: i32,
        indexing: i32,
    ) -> Option<IvgStructFontProperties> {
        let args = vec![
            Variant::from_i64(start as i64),
            Variant::from_i64(count as i64),
            Variant::from_i64(indexing as i64),
        ];
        self.disp
            .invoke_method("FontPropertiesInRange", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgStructFontProperties::new)
    }

    pub fn set_font_properties_in_range(
        &self,
        start: i32,
        count: i32,
        indexing: i32,
        props: &IvgStructFontProperties,
    ) -> bool {
        let args = vec![
            Variant::from_i64(start as i64),
            Variant::from_i64(count as i64),
            Variant::from_i64(indexing as i64),
            props.as_variant(),
        ];
        self.disp
            .invoke_method("put_FontPropertiesInRange", args)
            .is_ok()
    }

    // ---------------------------------------------------------
    // 瀵归綈灞炴€?
    // ---------------------------------------------------------

    pub fn align_properties(&self, frames: i32) -> Option<IvgStructAlignProperties> {
        let args = vec![Variant::from_i64(frames as i64)];
        self.disp
            .invoke_method("AlignProperties", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgStructAlignProperties::new)
    }

    pub fn set_align_properties(
        &self,
        frames: i32,
        props: &IvgStructAlignProperties,
    ) -> bool {
        let args = vec![
            Variant::from_i64(frames as i64),
            props.as_variant(),
        ];
        self.disp.invoke_method("put_AlignProperties", args).is_ok()
    }

    pub fn align_properties_in_range(
        &self,
        start: i32,
        count: i32,
        indexing: i32,
    ) -> Option<IvgStructAlignProperties> {
        let args = vec![
            Variant::from_i64(start as i64),
            Variant::from_i64(count as i64),
            Variant::from_i64(indexing as i64),
        ];
        self.disp
            .invoke_method("AlignPropertiesInRange", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgStructAlignProperties::new)
    }

    pub fn set_align_properties_in_range(
        &self,
        start: i32,
        count: i32,
        indexing: i32,
        props: &IvgStructAlignProperties,
    ) -> bool {
        let args = vec![
            Variant::from_i64(start as i64),
            Variant::from_i64(count as i64),
            Variant::from_i64(indexing as i64),
            props.as_variant(),
        ];
        self.disp
            .invoke_method("put_AlignPropertiesInRange", args)
            .is_ok()
    }

    // ---------------------------------------------------------
    // 闂磋窛灞炴€?
    // ---------------------------------------------------------

    pub fn space_properties(&self, frames: i32) -> Option<IvgStructSpaceProperties> {
        let args = vec![Variant::from_i64(frames as i64)];
        self.disp
            .invoke_method("SpaceProperties", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgStructSpaceProperties::new)
    }

    pub fn set_space_properties(
        &self,
        frames: i32,
        props: &IvgStructSpaceProperties,
    ) -> bool {
        let args = vec![
            Variant::from_i64(frames as i64),
            props.as_variant(),
        ];
        self.disp
            .invoke_method("put_SpaceProperties", args)
            .is_ok()
    }

    pub fn space_properties_in_range(
        &self,
        start: i32,
        count: i32,
        indexing: i32,
    ) -> Option<IvgStructSpaceProperties> {
        let args = vec![
            Variant::from_i64(start as i64),
            Variant::from_i64(count as i64),
            Variant::from_i64(indexing as i64),
        ];
        self.disp
            .invoke_method("SpacePropertiesInRange", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgStructSpaceProperties::new)
    }

    pub fn set_space_properties_in_range(
        &self,
        start: i32,
        count: i32,
        indexing: i32,
        props: &IvgStructSpaceProperties,
    ) -> bool {
        let args = vec![
            Variant::from_i64(start as i64),
            Variant::from_i64(count as i64),
            Variant::from_i64(indexing as i64),
            props.as_variant(),
        ];
        self.disp
            .invoke_method("put_SpacePropertiesInRange", args)
            .is_ok()
    }

    // ---------------------------------------------------------
    // 鏂瓧灞炴€?
    // ---------------------------------------------------------

    pub fn hyphenation_settings(
        &self,
        frames: i32,
    ) -> Option<IvgStructHyphenationSettings> {
        let args = vec![Variant::from_i64(frames as i64)];
        self.disp
            .invoke_method("HyphenationSettings", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgStructHyphenationSettings::new)
    }

    pub fn set_hyphenation_settings(
        &self,
        frames: i32,
        s: &IvgStructHyphenationSettings,
    ) -> bool {
        let args = vec![
            Variant::from_i64(frames as i64),
            s.as_variant(),
        ];
        self.disp
            .invoke_method("put_HyphenationSettings", args)
            .is_ok()
    }

    pub fn hyphenation_settings_in_range(
        &self,
        start: i32,
        count: i32,
        indexing: i32,
    ) -> Option<IvgStructHyphenationSettings> {
        let args = vec![
            Variant::from_i64(start as i64),
            Variant::from_i64(count as i64),
            Variant::from_i64(indexing as i64),
        ];
        self.disp
            .invoke_method("HyphenationSettingsInRange", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgStructHyphenationSettings::new)
    }

    pub fn set_hyphenation_settings_in_range(
        &self,
        start: i32,
        count: i32,
        indexing: i32,
        s: &IvgStructHyphenationSettings,
    ) -> bool {
        let args = vec![
            Variant::from_i64(start as i64),
            Variant::from_i64(count as i64),
            Variant::from_i64(indexing as i64),
            s.as_variant(),
        ];
        self.disp
            .invoke_method("put_HyphenationSettingsInRange", args)
            .is_ok()
    }

    // ---------------------------------------------------------
    // 鍐呭
    // ---------------------------------------------------------

    pub fn contents(&self, frames: i32) -> Option<String> {
        let args = vec![Variant::from_i64(frames as i64)];
        self.disp
            .invoke_method("Contents", args)
            .ok()?
            .to_string()
            .ok()
    }

    pub fn set_contents(&self, frames: i32, v: impl Into<String>) -> bool {
        let args = vec![
            Variant::from_i64(frames as i64),
            Variant::from_str(v.into()),
        ];
        self.disp.invoke_method("put_Contents", args).is_ok()
    }

    // ---------------------------------------------------------
    // 璺緞
    // ---------------------------------------------------------

    /// 璁╂枃鏈€傞厤鍒拌矾寰勩€?
    pub fn fit_to_path(&self, path: &IvgShape) -> Option<IvgEffect> {
        let args = vec![path.as_variant()];
        self.disp
            .invoke_method("FitToPath", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgEffect::new)
    }

    /// 璁╂枃鏈嚜鍔ㄩ€傞厤鏂囨湰妗嗐€?
    pub fn fit_text_to_frame(&self) -> bool {
        self.disp.invoke_method("FitTextToFrame", vec![]).is_ok()
    }

    // ---------------------------------------------------------
    // 鏌ユ壘 / 鏇挎崲
    // ---------------------------------------------------------

    pub fn find(
        &self,
        text: impl Into<String>,
        case_sensitive: bool,
        start_index: i32,
        wrap_around: bool,
        indexing: i32,
    ) -> Option<i64> {
        let args = vec![
            Variant::from_str(text.into()),
            Variant::from_bool(case_sensitive),
            Variant::from_i64(start_index as i64),
            Variant::from_bool(wrap_around),
            Variant::from_i64(indexing as i64),
        ];
        self.disp.invoke_method("Find", args).ok()?.to_i64().ok()
    }

    pub fn replace(
        &self,
        old_text: impl Into<String>,
        new_text: impl Into<String>,
        case_sensitive: bool,
        start_index: i32,
        replace_all: bool,
        wrap_around: bool,
        indexing: i32,
    ) -> bool {
        let args = vec![
            Variant::from_str(old_text.into()),
            Variant::from_str(new_text.into()),
            Variant::from_bool(case_sensitive),
            Variant::from_i64(start_index as i64),
            Variant::from_bool(replace_all),
            Variant::from_bool(wrap_around),
            Variant::from_i64(indexing as i64),
        ];
        self.disp.invoke_method("Replace", args).is_ok()
    }

    // ---------------------------------------------------------
    // 瀵煎叆 / 瀵煎嚭
    // ---------------------------------------------------------

    pub fn import_from_file(
        &self,
        file_name: impl Into<String>,
        start_index: i32,
        indexing: i32,
    ) -> bool {
        let args = vec![
            Variant::from_str(file_name.into()),
            Variant::from_i64(start_index as i64),
            Variant::from_i64(indexing as i64),
        ];
        self.disp.invoke_method("ImportFromFile", args).is_ok()
    }

    pub fn export_to_file(
        &self,
        file_name: impl Into<String>,
        start_index: i32,
        count: i32,
        indexing: i32,
    ) -> bool {
        let args = vec![
            Variant::from_str(file_name.into()),
            Variant::from_i64(start_index as i64),
            Variant::from_i64(count as i64),
            Variant::from_i64(indexing as i64),
        ];
        self.disp.invoke_method("ExportToFile", args).is_ok()
    }

    // ---------------------------------------------------------
    // 杞崲
    // ---------------------------------------------------------

    pub fn convert_to_artistic(&self) -> bool {
        self.disp.invoke_method("ConvertToArtistic", vec![]).is_ok()
    }

    pub fn convert_to_paragraph(&self) -> bool {
        self.disp.invoke_method("ConvertToParagraph", vec![]).is_ok()
    }

    pub fn make_html_compatible(&self, html: bool) -> Option<bool> {
        let args = vec![Variant::from_bool(html)];
        self.disp
            .invoke_method("MakeHTMLCompatible", args)
            .ok()?
            .to_bool()
            .ok()
    }

    // ---------------------------------------------------------
    // 鑼冨洿 / 閫夋嫨 / 缂栬緫
    // ---------------------------------------------------------

    /// 鏁翠釜鏁呬簨锛堝叏閮ㄦ枃鏈級銆?
    pub fn story(&self) -> Option<IvgTextRange> {
        self.prop_dispatch("Story").map(IvgTextRange::new)
    }

    /// 褰撳墠閫夋嫨鐨勬枃鏈寖鍥淬€?
    pub fn selection(&self) -> Option<IvgTextRange> {
        self.prop_dispatch("Selection").map(IvgTextRange::new)
    }

    /// 鎸夎捣姝綅缃彇鑼冨洿銆?
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

    pub fn begin_edit(&self) -> bool {
        self.disp.invoke_method("BeginEdit", vec![]).is_ok()
    }

    // ---------------------------------------------------------
    // 鏂囨湰妗?
    // ---------------------------------------------------------

    pub fn frame(&self) -> Option<IvgTextFrame> {
        self.prop_dispatch("Frame").map(IvgTextFrame::new)
    }

    pub fn frames(&self) -> Option<IvgTextFrames> {
        self.prop_dispatch("Frames").map(IvgTextFrames::new)
    }
}