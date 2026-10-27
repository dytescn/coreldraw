//! `IVGStructAlignProperties` —— 段落对齐属性

use wincom::{ComObject, Variant};
use windows::Win32::System::Com::IDispatch;

/// CorelDRAW `IVGStructAlignProperties` 对象的后期绑定包装（对齐属性集）。
///
/// 所有属性与方法通过 `IDispatch::Invoke` 按名称调用，调用失败时一般返回 `None` 或 `false`。
pub struct IvgStructAlignProperties {
    disp: ComObject,
}

impl IvgStructAlignProperties {
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

    fn prop_f64(&self, name: &str) -> Option<f64> {
        self.disp.get_property(name).ok()?.to_f64().ok()
    }

    fn prop_f32(&self, name: &str) -> Option<f32> {
        self.prop_f64(name).map(|v| v as f32)
    }

    fn put_i64(&self, name: &str, v: i64) -> bool {
        let arg = Variant::from_i64(v);
        self.disp.set_property(name, vec![arg]).is_ok()
    }

    fn put_f64(&self, name: &str, v: f64) -> bool {
        let arg = Variant::from_f64(v);
        self.disp.set_property(name, vec![arg]).is_ok()
    }

    fn put_f32(&self, name: &str, v: f32) -> bool {
        let arg = Variant::from_f64(v as f64);
        self.disp.set_property(name, vec![arg]).is_ok()
    }

    // ---- 对齐 ----

    /// `cdrAlignment`
    pub fn alignment(&self) -> Option<i64> { self.prop_i64("Alignment") }
    /// 设置对齐（`Alignment`）。
    pub fn set_alignment(&self, v: i32) -> bool { self.put_i64("Alignment", v as i64) }

    // ---- 缩进 ----

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

    // ---- 词间距----

    /// 读取最大词间距（`MaxWordSpacing`）。
    pub fn max_word_spacing(&self) -> Option<f32> { self.prop_f32("MaxWordSpacing") }
    /// 设置最大词间距（`MaxWordSpacing`）。
    pub fn set_max_word_spacing(&self, v: f32) -> bool {
        self.put_f32("MaxWordSpacing", v)
    }

    /// 读取最小词间距（`MinWordSpacing`）。
    pub fn min_word_spacing(&self) -> Option<f32> { self.prop_f32("MinWordSpacing") }
    /// 设置最小词间距（`MinWordSpacing`）。
    pub fn set_min_word_spacing(&self, v: f32) -> bool {
        self.put_f32("MinWordSpacing", v)
    }

    // ---- 字符间距 ----

    /// 读取最大字符间距（`MaxCharacterSpacing`）。
    pub fn max_character_spacing(&self) -> Option<f32> {
        self.prop_f32("MaxCharacterSpacing")
    }
    /// 设置最大字符间距（`MaxCharacterSpacing`）。
    pub fn set_max_character_spacing(&self, v: f32) -> bool {
        self.put_f32("MaxCharacterSpacing", v)
    }

    // ---- 位移 / 旋转 ----

    /// 读取水平字符偏移（`HorizontalCharacterShift`）。
    pub fn horizontal_character_shift(&self) -> Option<i64> {
        self.prop_i64("HorizontalCharacterShift")
    }
    /// 设置水平字符偏移（`HorizontalCharacterShift`）。
    pub fn set_horizontal_character_shift(&self, v: i32) -> bool {
        self.put_i64("HorizontalCharacterShift", v as i64)
    }

    /// 读取垂直字符偏移（`VerticalCharacterShift`）。
    pub fn vertical_character_shift(&self) -> Option<i64> {
        self.prop_i64("VerticalCharacterShift")
    }
    /// 设置垂直字符偏移（`VerticalCharacterShift`）。
    pub fn set_vertical_character_shift(&self, v: i32) -> bool {
        self.put_i64("VerticalCharacterShift", v as i64)
    }

    /// 读取字符旋转（`CharacterRotation`）。
    pub fn character_rotation(&self) -> Option<f32> { self.prop_f32("CharacterRotation") }
    /// 设置字符旋转（`CharacterRotation`）。
    pub fn set_character_rotation(&self, v: f32) -> bool {
        self.put_f32("CharacterRotation", v)
    }
}