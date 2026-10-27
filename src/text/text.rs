//! `IVGText` —— 文本对象
//!
//! 由 `shape.text()` 取得，是文本操作的**总入口**。

use wincom::{ComObject, Variant};
use windows::Win32::System::Com::IDispatch;

use crate::effect::IvgEffect;
use crate::shape::IvgShape;
use crate::structs::{
    IvgStructAlignProperties, IvgStructFontProperties,
    IvgStructHyphenationSettings, IvgStructSpaceProperties,
};
use crate::text::{IvgTextFrame, IvgTextFrames, IvgTextRange};

/// CorelDRAW `IVGText` 对象的后期绑定包装（文本）。
///
/// 所有属性与方法通过 `IDispatch::Invoke` 按名称调用，调用失败时一般返回 `None` 或 `false`。
pub struct IvgText {
    disp: ComObject,
}

impl IvgText {
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
    // 基本属性
    // ---------------------------------------------------------

    /// `cdrTextType` ——Artistic / Paragraph
    pub fn text_type(&self) -> Option<i64> { self.prop_i64("Type") }

    /// 链中的文本框数量。
    pub fn frames_in_link(&self) -> Option<i64> { self.prop_i64("FramesInLink") }
    /// 读取链接中的未使用文本框集（`UnusedFramesInLink`）。
    pub fn unused_frames_in_link(&self) -> Option<i64> {
        self.prop_i64("UnusedFramesInLink")
    }

    /// 是否有溢出文本。
    pub fn overflow(&self) -> Option<bool> { self.prop_bool("Overflow") }

    /// 是否艺术文本（`IsArtisticText`）。
    pub fn is_artistic_text(&self) -> Option<bool> { self.prop_bool("IsArtisticText") }
    /// 是否编辑中（`IsEditing`）。
    pub fn is_editing(&self) -> Option<bool> { self.prop_bool("IsEditing") }
    /// 读取 COM 属性 `IsHTMLCompatible`。
    pub fn is_html_compatible(&self) -> Option<bool> {
        self.prop_bool("IsHTMLCompatible")
    }

    // ---------------------------------------------------------
    // 字体属性
    // ---------------------------------------------------------

    /// `cdrTextFrames` 范围，返回 `IvgStructFontProperties`。
    pub fn font_properties(&self, frames: i32) -> Option<IvgStructFontProperties> {
        let args = vec![Variant::from_i64(frames as i64)];
        self.disp
            .invoke_method("FontProperties", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgStructFontProperties::new)
    }

    /// 设置字体属性集（`put_FontProperties`）。
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

    /// `cdrTextIndexingType`。
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

    /// 设置范围中的字体属性集（`put_FontPropertiesInRange`）。
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
    // 对齐属性
    // ---------------------------------------------------------

    /// 对齐属性集（COM 方法 `AlignProperties`）。
    pub fn align_properties(&self, frames: i32) -> Option<IvgStructAlignProperties> {
        let args = vec![Variant::from_i64(frames as i64)];
        self.disp
            .invoke_method("AlignProperties", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgStructAlignProperties::new)
    }

    /// 设置对齐属性集（`put_AlignProperties`）。
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

    /// 对齐范围中的属性集（COM 方法 `AlignPropertiesInRange`）。
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

    /// 设置对齐范围中的属性集（`put_AlignPropertiesInRange`）。
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
    // 间距属性
    // ---------------------------------------------------------

    /// 间距属性集（COM 方法 `SpaceProperties`）。
    pub fn space_properties(&self, frames: i32) -> Option<IvgStructSpaceProperties> {
        let args = vec![Variant::from_i64(frames as i64)];
        self.disp
            .invoke_method("SpaceProperties", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgStructSpaceProperties::new)
    }

    /// 设置间距属性集（`put_SpaceProperties`）。
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

    /// 范围中的间距属性集（COM 方法 `SpacePropertiesInRange`）。
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

    /// 设置范围中的间距属性集（`put_SpacePropertiesInRange`）。
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
    // 断字属性
    // ---------------------------------------------------------

    /// 断字设置集（COM 方法 `HyphenationSettings`）。
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

    /// 设置断字设置集（`put_HyphenationSettings`）。
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

    /// 范围中的断字设置集（COM 方法 `HyphenationSettingsInRange`）。
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

    /// 设置范围中的断字设置集（`put_HyphenationSettingsInRange`）。
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
    // 内容
    // ---------------------------------------------------------

    /// 内容（COM 方法 `Contents`）。
    pub fn contents(&self, frames: i32) -> Option<String> {
        let args = vec![Variant::from_i64(frames as i64)];
        self.disp
            .invoke_method("Contents", args)
            .ok()?
            .to_string()
            .ok()
    }

    /// 设置内容（`put_Contents`）。
    pub fn set_contents(&self, frames: i32, v: impl Into<String>) -> bool {
        let args = vec![
            Variant::from_i64(frames as i64),
            Variant::from_str(v.into()),
        ];
        self.disp.invoke_method("put_Contents", args).is_ok()
    }

    // ---------------------------------------------------------
    // 路径
    // ---------------------------------------------------------

    /// 让文本适配到路径。
    pub fn fit_to_path(&self, path: &IvgShape) -> Option<IvgEffect> {
        let args = vec![path.as_variant()];
        self.disp
            .invoke_method("FitToPath", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgEffect::new)
    }

    /// 让文本自动适配文本框。
    pub fn fit_text_to_frame(&self) -> bool {
        self.disp.invoke_method("FitTextToFrame", vec![]).is_ok()
    }

    // ---------------------------------------------------------
    // 查找 / 替换
    // ---------------------------------------------------------

    /// 查找（COM 方法 `Find`）。
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

    /// 替换（COM 方法 `Replace`）。
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
    // 导入 / 导出
    // ---------------------------------------------------------

    /// 导入文件（COM 方法 `ImportFromFile`）。
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

    /// 导出文件（COM 方法 `ExportToFile`）。
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
    // 转换
    // ---------------------------------------------------------

    /// 转换艺术（COM 方法 `ConvertToArtistic`）。
    pub fn convert_to_artistic(&self) -> bool {
        self.disp.invoke_method("ConvertToArtistic", vec![]).is_ok()
    }

    /// 转换段落（COM 方法 `ConvertToParagraph`）。
    pub fn convert_to_paragraph(&self) -> bool {
        self.disp.invoke_method("ConvertToParagraph", vec![]).is_ok()
    }

    /// 调用 COM 方法 `MakeHTMLCompatible`。
    pub fn make_html_compatible(&self, html: bool) -> Option<bool> {
        let args = vec![Variant::from_bool(html)];
        self.disp
            .invoke_method("MakeHTMLCompatible", args)
            .ok()?
            .to_bool()
            .ok()
    }

    // ---------------------------------------------------------
    // 范围 / 选择 / 编辑
    // ---------------------------------------------------------

    /// 整个故事（全部文本）。
    pub fn story(&self) -> Option<IvgTextRange> {
        self.prop_dispatch("Story").map(IvgTextRange::new)
    }

    /// 当前选择的文本范围。
    pub fn selection(&self) -> Option<IvgTextRange> {
        self.prop_dispatch("Selection").map(IvgTextRange::new)
    }

    /// 按起止位置取范围。
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

    /// 开始编辑（COM 方法 `BeginEdit`）。
    pub fn begin_edit(&self) -> bool {
        self.disp.invoke_method("BeginEdit", vec![]).is_ok()
    }

    // ---------------------------------------------------------
    // 文本框
    // ---------------------------------------------------------

    /// 读取文本框（`Frame`）。
    pub fn frame(&self) -> Option<IvgTextFrame> {
        self.prop_dispatch("Frame").map(IvgTextFrame::new)
    }

    /// 读取文本框集（`Frames`）。
    pub fn frames(&self) -> Option<IvgTextFrames> {
        self.prop_dispatch("Frames").map(IvgTextFrames::new)
    }
}