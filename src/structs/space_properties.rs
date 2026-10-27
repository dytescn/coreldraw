//! `IVGStructSpaceProperties` —— 文本间距属性

use wincom::{ComObject, Variant};
use windows::Win32::System::Com::IDispatch;

/// CorelDRAW `IVGStructSpaceProperties` 对象的后期绑定包装（间距属性集）。
///
/// 所有属性与方法通过 `IDispatch::Invoke` 按名称调用，调用失败时一般返回 `None` 或 `false`。
pub struct IvgStructSpaceProperties {
    disp: ComObject,
}

impl IvgStructSpaceProperties {
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

    fn prop_i64(&self, name: &str) -> Option<i64> {
        self.disp.get_property(name).ok()?.to_i64().ok()
    }

    fn prop_f32(&self, name: &str) -> Option<f32> {
        self.disp.get_property(name).ok()?.to_f64().ok().map(|v| v as f32)
    }

    fn put_i64(&self, name: &str, v: i64) -> bool {
        let arg = Variant::from_i64(v);
        self.disp.set_property(name, vec![arg]).is_ok()
    }

    fn put_f32(&self, name: &str, v: f32) -> bool {
        let arg = Variant::from_f64(v as f64);
        self.disp.set_property(name, vec![arg]).is_ok()
    }

    // ---- 字符 / 词间距----

    /// 读取字符间距（`CharacterSpacing`）。
    pub fn character_spacing(&self) -> Option<f32> { self.prop_f32("CharacterSpacing") }
    /// 设置字符间距（`CharacterSpacing`）。
    pub fn set_character_spacing(&self, v: f32) -> bool {
        self.put_f32("CharacterSpacing", v)
    }

    /// 读取词间距（`WordSpacing`）。
    pub fn word_spacing(&self) -> Option<f32> { self.prop_f32("WordSpacing") }
    /// 设置词间距（`WordSpacing`）。
    pub fn set_word_spacing(&self, v: f32) -> bool { self.put_f32("WordSpacing", v) }

    // ---- 行距 ----

    /// 读取线间距（`LineSpacing`）。
    pub fn line_spacing(&self) -> Option<f32> { self.prop_f32("LineSpacing") }
    /// 设置线间距（`LineSpacing`）。
    pub fn set_line_spacing(&self, v: f32) -> bool {
        self.put_f32("LineSpacing", v)
    }

    /// `cdrLineSpacingType`
    pub fn line_spacing_type(&self) -> Option<i64> { self.prop_i64("LineSpacingType") }
    /// 设置线间距类型（`LineSpacingType`）。
    pub fn set_line_spacing_type(&self, v: i32) -> bool {
        self.put_i64("LineSpacingType", v as i64)
    }

    // ---- 段前 / 段后 ----

    /// 读取之前段落间距（`BeforeParagraphSpacing`）。
    pub fn before_paragraph_spacing(&self) -> Option<f32> {
        self.prop_f32("BeforeParagraphSpacing")
    }
    /// 设置之前段落间距（`BeforeParagraphSpacing`）。
    pub fn set_before_paragraph_spacing(&self, v: f32) -> bool {
        self.put_f32("BeforeParagraphSpacing", v)
    }

    /// 读取之后段落间距（`AfterParagraphSpacing`）。
    pub fn after_paragraph_spacing(&self) -> Option<f32> {
        self.prop_f32("AfterParagraphSpacing")
    }
    /// 设置之后段落间距（`AfterParagraphSpacing`）。
    pub fn set_after_paragraph_spacing(&self, v: f32) -> bool {
        self.put_f32("AfterParagraphSpacing", v)
    }
}