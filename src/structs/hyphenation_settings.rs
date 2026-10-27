//! `IVGStructHyphenationSettings` —— 断字设置

use wincom::{ComObject, Variant};
use windows::Win32::System::Com::IDispatch;

/// CorelDRAW `IVGStructHyphenationSettings` 对象的后期绑定包装（断字设置集）。
///
/// 所有属性与方法通过 `IDispatch::Invoke` 按名称调用，调用失败时一般返回 `None` 或 `false`。
pub struct IvgStructHyphenationSettings {
    disp: ComObject,
}

impl IvgStructHyphenationSettings {
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

    fn prop_bool(&self, name: &str) -> Option<bool> {
        self.disp.get_property(name).ok()?.to_bool().ok()
    }

    fn put_i64(&self, name: &str, v: i64) -> bool {
        let arg = Variant::from_i64(v);
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

    // ---- 自动断字 ----

    /// 读取 COM 属性 `UseAutomaticHyphenation`。
    pub fn use_automatic_hyphenation(&self) -> Option<bool> {
        self.prop_bool("UseAutomaticHyphenation")
    }
    /// 设置 COM 属性 `UseAutomaticHyphenation`。
    pub fn set_use_automatic_hyphenation(&self, v: bool) -> bool {
        self.put_bool("UseAutomaticHyphenation", v)
    }

    // ---- 大写单词 ----

    /// 读取 COM 属性 `BreakCapitalized`。
    pub fn break_capitalized(&self) -> Option<bool> {
        self.prop_bool("BreakCapitalized")
    }
    /// 设置 COM 属性 `BreakCapitalized`。
    pub fn set_break_capitalized(&self, v: bool) -> bool {
        self.put_bool("BreakCapitalized", v)
    }

    /// 读取断开全部端点词集（`BreakAllCapWords`）。
    pub fn break_all_cap_words(&self) -> Option<bool> {
        self.prop_bool("BreakAllCapWords")
    }
    /// 设置断开全部端点词集（`BreakAllCapWords`）。
    pub fn set_break_all_cap_words(&self, v: bool) -> bool {
        self.put_bool("BreakAllCapWords", v)
    }

    // ---- 热区 / 长度 ----

    /// 读取热区域（`HotZone`）。
    pub fn hot_zone(&self) -> Option<f64> { self.prop_f64("HotZone") }
    /// 设置热区域（`HotZone`）。
    pub fn set_hot_zone(&self, v: f64) -> bool { self.put_f64("HotZone", v) }

    /// 读取最小词长度（`MinWordLength`）。
    pub fn min_word_length(&self) -> Option<i64> { self.prop_i64("MinWordLength") }
    /// 设置最小词长度（`MinWordLength`）。
    pub fn set_min_word_length(&self, v: i32) -> bool {
        self.put_i64("MinWordLength", v as i64)
    }

    /// 读取最小字符集之前（`MinCharactersBefore`）。
    pub fn min_characters_before(&self) -> Option<i64> {
        self.prop_i64("MinCharactersBefore")
    }
    /// 设置最小字符集之前（`MinCharactersBefore`）。
    pub fn set_min_characters_before(&self, v: i32) -> bool {
        self.put_i64("MinCharactersBefore", v as i64)
    }

    /// 读取最小字符集之后（`MinCharactersAfter`）。
    pub fn min_characters_after(&self) -> Option<i64> {
        self.prop_i64("MinCharactersAfter")
    }
    /// 设置最小字符集之后（`MinCharactersAfter`）。
    pub fn set_min_characters_after(&self, v: i32) -> bool {
        self.put_i64("MinCharactersAfter", v as i64)
    }
}