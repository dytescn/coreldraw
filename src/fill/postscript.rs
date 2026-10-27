//! `IVGPostScriptFill` —— PostScript 填充
//! `IVGPSScreenOptions` —— PostScript 屏幕选项

use wincom::{ComObject, Variant};
use windows::Win32::System::Com::IDispatch;

// =============================================================
// IvgPostScriptFill
// =============================================================

/// CorelDRAW `IVGPostScriptFill` 对象的后期绑定包装（PostScript 填充）。
///
/// 所有属性与方法通过 `IDispatch::Invoke` 按名称调用，调用失败时一般返回 `None` 或 `false`。
pub struct IvgPostScriptFill {
    disp: ComObject,
}

impl IvgPostScriptFill {
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

    fn prop_i64(&self, name: &str) -> Option<i64> {
        self.disp.get_property(name).ok()?.to_i64().ok()
    }

    fn prop_string(&self, name: &str) -> Option<String> {
        self.disp.get_property(name).ok()?.to_string().ok()
    }

    // fn put_i64(&self, name: &str, v: i64) -> bool {
    //     let arg = Variant::from_i64(v);
    //     self.disp.set_property(name, vec![arg]).is_ok()
    // }

    /// PostScript 填充名。
    pub fn name(&self) -> Option<String> { self.prop_string("Name") }

    /// PostScript 填充索引。
    pub fn index(&self) -> Option<i64> { self.prop_i64("Index") }

    /// 读写单个属性（0..4 或 0..N）。
    pub fn get_property_at(&self, index: i32) -> Option<i64> {
        let args = vec![Variant::from_i64(index as i64)];
        self.disp.invoke_method("Properties", args).ok()?.to_i64().ok()
    }

    /// 设置属性在（`put_Properties`）。
    pub fn set_property_at(&self, index: i32, v: i32) -> bool {
        let args = vec![
            Variant::from_i64(index as i64),
            Variant::from_i64(v as i64),
        ];
        self.disp.invoke_method("put_Properties", args).is_ok()
    }

    /// 选择 PostScript 填充。
    pub fn select(&self, index_or_name: Variant) -> bool {
        let args = vec![index_or_name];
        self.disp.invoke_method("Select", args).is_ok()
    }

    /// 一次设置 5 个属性。
    pub fn set_properties(
        &self,
        p1: i32,
        p2: i32,
        p3: i32,
        p4: i32,
        p5: i32,
    ) -> bool {
        let args = vec![
            Variant::from_i64(p1 as i64),
            Variant::from_i64(p2 as i64),
            Variant::from_i64(p3 as i64),
            Variant::from_i64(p4 as i64),
            Variant::from_i64(p5 as i64),
        ];
        self.disp.invoke_method("SetProperties", args).is_ok()
    }
}

// =============================================================
// IvgPSScreenOptions
// =============================================================

/// CorelDRAW `IVGPostScriptFill` 对象的后期绑定包装（ps 屏幕选项集）。
///
/// 所有属性与方法通过 `IDispatch::Invoke` 按名称调用，调用失败时一般返回 `None` 或 `false`。
pub struct IvgPSScreenOptions {
    disp: ComObject,
}

impl IvgPSScreenOptions {
    /// 由 COM `IDispatch` 构造包装对象。
    pub fn new(disp: IDispatch) -> Self {
        Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) }
    }

    fn prop_i64(&self, name: &str) -> Option<i64> {
        self.disp.get_property(name).ok()?.to_i64().ok()
    }

    fn prop_f64(&self, name: &str) -> Option<f64> {
        self.disp.get_property(name).ok()?.to_f64().ok()
    }

    fn prop_string(&self, name: &str) -> Option<String> {
        self.disp.get_property(name).ok()?.to_string().ok()
    }

    fn put_i64(&self, name: &str, v: i64) -> bool {
        let arg = Variant::from_i64(v);
        self.disp.set_property(name, vec![arg]).is_ok()
    }

    fn put_f64(&self, name: &str, v: f64) -> bool {
        let arg = Variant::from_f64(v);
        self.disp.set_property(name, vec![arg]).is_ok()
    }

    /// 读取索引（`Index`）。
    pub fn index(&self) -> Option<i64> { self.prop_i64("Index") }
    /// 读取名称（`Name`）。
    pub fn name(&self) -> Option<String> { self.prop_string("Name") }

    /// 读取角度（`Angle`）。
    pub fn angle(&self) -> Option<f64> { self.prop_f64("Angle") }
    /// 设置角度（`Angle`）。
    pub fn set_angle(&self, v: f64) -> bool { self.put_f64("Angle", v) }

    /// 读取频率（`Frequency`）。
    pub fn frequency(&self) -> Option<i64> { self.prop_i64("Frequency") }
    /// 设置频率（`Frequency`）。
    pub fn set_frequency(&self, v: i32) -> bool { self.put_i64("Frequency", v as i64) }

    /// 按名称选择。
    pub fn select(&self, name: impl Into<String>) -> Option<bool> {
        let args = vec![Variant::from_str(name.into())];
        self.disp.invoke_method("Select", args).ok()?.to_bool().ok()
    }

    /// 一次性设置参数。
    pub fn set_properties(
        &self,
        index_or_name: Variant,
        angle: f64,
        frequency: i32,
    ) -> Option<bool> {
        let args = vec![
            index_or_name,
            Variant::from_f64(angle),
            Variant::from_i64(frequency as i64),
        ];
        self.disp
            .invoke_method("SetProperties", args)
            .ok()?
            .to_bool()
            .ok()
    }

    /// 通过索引获取名称。
    pub fn name_by_index(&self, index: i32) -> Option<String> {
        let args = vec![Variant::from_i64(index as i64)];
        self.disp.invoke_method("NameByIndex", args).ok()?.to_string().ok()
    }

    /// 重置（COM 方法 `Reset`）。
    pub fn reset(&self) -> bool {
        self.disp.invoke_method("Reset", vec![]).is_ok()
    }
}