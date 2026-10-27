//! `IVGStructFontProperties` —— 字体属性

use wincom::{ComObject, Variant};
use windows::Win32::System::Com::IDispatch;

use crate::fill::IvgFill;
use crate::outline::IvgOutline;

/// CorelDRAW `IVGStructFontProperties` 对象的后期绑定包装（字体属性集）。
///
/// 所有属性与方法通过 `IDispatch::Invoke` 按名称调用，调用失败时一般返回 `None` 或 `false`。
pub struct IvgStructFontProperties {
    disp: ComObject,
}

impl IvgStructFontProperties {
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

    fn prop_string(&self, name: &str) -> Option<String> {
        self.disp.get_property(name).ok()?.to_string().ok()
    }

    fn prop_i64(&self, name: &str) -> Option<i64> {
        self.disp.get_property(name).ok()?.to_i64().ok()
    }

    fn prop_f32(&self, name: &str) -> Option<f32> {
        self.disp.get_property(name).ok()?.to_f64().ok().map(|v| v as f32)
    }

    fn prop_dispatch(&self, name: &str) -> Option<IDispatch> {
        self.disp.get_property(name).ok()?.to_idispatch().ok()
    }

    fn put_string(&self, name: &str, v: impl Into<String>) -> bool {
        let arg = Variant::from_str(v.into());
        self.disp.set_property(name, vec![arg]).is_ok()
    }

    fn put_i64(&self, name: &str, v: i64) -> bool {
        let arg = Variant::from_i64(v);
        self.disp.set_property(name, vec![arg]).is_ok()
    }

    fn put_f32(&self, name: &str, v: f32) -> bool {
        let arg = Variant::from_f64(v as f64);
        self.disp.set_property(name, vec![arg]).is_ok()
    }

    fn put_dispatch(&self, name: &str, v: Variant) -> bool {
        self.disp.set_property(name, vec![v]).is_ok()
    }

    // ---- 字体 ----

    /// 读取名称（`Name`）。
    pub fn name(&self) -> Option<String> { self.prop_string("Name") }
    /// 设置名称（`Name`）。
    pub fn set_name(&self, v: impl Into<String>) -> bool {
        self.put_string("Name", v)
    }

    /// `cdrFontStyle`
    pub fn style(&self) -> Option<i64> { self.prop_i64("Style") }
    /// 设置样式（`Style`）。
    pub fn set_style(&self, v: i32) -> bool { self.put_i64("Style", v as i64) }

    /// 读取尺寸（`Size`）。
    pub fn size(&self) -> Option<f32> { self.prop_f32("Size") }
    /// 设置尺寸（`Size`）。
    pub fn set_size(&self, v: f32) -> bool { self.put_f32("Size", v) }

    // ---- 瑁呴グ绾?----

    /// `cdrFontLine`
    pub fn underline(&self) -> Option<i64> { self.prop_i64("Underline") }
    /// 设置下划线（`Underline`）。
    pub fn set_underline(&self, v: i32) -> bool { self.put_i64("Underline", v as i64) }

    /// 读取上划线（`Overscore`）。
    pub fn overscore(&self) -> Option<i64> { self.prop_i64("Overscore") }
    /// 设置上划线（`Overscore`）。
    pub fn set_overscore(&self, v: i32) -> bool { self.put_i64("Overscore", v as i64) }

    /// 读取删除线（`Strikethru`）。
    pub fn strikethru(&self) -> Option<i64> { self.prop_i64("Strikethru") }
    /// 设置删除线（`Strikethru`）。
    pub fn set_strikethru(&self, v: i32) -> bool { self.put_i64("Strikethru", v as i64) }

    // ---- 大小写/ 位置 ----

    /// `cdrFontCase`
    pub fn uppercase(&self) -> Option<i64> { self.prop_i64("Uppercase") }
    /// 设置 COM 属性 `Uppercase`。
    pub fn set_uppercase(&self, v: i32) -> bool { self.put_i64("Uppercase", v as i64) }

    /// `cdrFontPosition`
    pub fn position(&self) -> Option<i64> { self.prop_i64("Position") }
    /// 设置位置（`Position`）。
    pub fn set_position(&self, v: i32) -> bool { self.put_i64("Position", v as i64) }

    /// 字距（range kerning）。
    pub fn range_kerning(&self) -> Option<i64> { self.prop_i64("RangeKerning") }
    /// 设置范围字距调整（`RangeKerning`）。
    pub fn set_range_kerning(&self, v: i32) -> bool {
        self.put_i64("RangeKerning", v as i64)
    }

    // ---- 填充 / 轮廓 ----

    /// 读取填充（`Fill`）。
    pub fn fill(&self) -> Option<IvgFill> {
        self.prop_dispatch("Fill").map(IvgFill::new)
    }
    /// 设置填充（`Fill`）。
    pub fn set_fill(&self, f: &IvgFill) -> bool {
        self.put_dispatch("Fill", f.as_variant())
    }

    /// 读取轮廓（`Outline`）。
    pub fn outline(&self) -> Option<IvgOutline> {
        self.prop_dispatch("Outline").map(IvgOutline::new)
    }
    /// 设置轮廓（`Outline`）。
    pub fn set_outline(&self, o: &IvgOutline) -> bool {
        self.put_dispatch("Outline", o.as_variant())
    }
}