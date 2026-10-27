//! `IVGDuotone` / `IVGDuotoneInk` / `IVGDuotoneOverprint` —— 双色套印

use wincom::{ComObject, Variant};
use windows::Win32::System::Com::IDispatch;

use crate::color::color::IvgColor;

// =============================================================
// IvgDuotoneOverprint
// =============================================================

/// CorelDRAW `IVGDuotone` 对象的后期绑定包装（双色叠印）。
///
/// 所有属性与方法通过 `IDispatch::Invoke` 按名称调用，调用失败时一般返回 `None` 或 `false`。
pub struct IvgDuotoneOverprint {
    disp: ComObject,
}

impl IvgDuotoneOverprint {
    /// 由 COM `IDispatch` 构造包装对象。
    pub fn new(disp: IDispatch) -> Self {
        Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) }
    }

    fn prop_i64(&self, name: &str) -> Option<i64> {
        self.disp.get_property(name).ok()?.to_i64().ok()
    }

    fn put_i64(&self, name: &str, v: i64) -> bool {
        let arg = Variant::from_i64(v);
        self.disp.set_property(name, vec![arg]).is_ok()
    }

    /// 读取青（`Cyan`）。
    pub fn cyan(&self) -> Option<i64> { self.prop_i64("Cyan") }
    /// 设置青（`Cyan`）。
    pub fn set_cyan(&self, v: i32) -> bool { self.put_i64("Cyan", v as i64) }

    /// 读取品红（`Magenta`）。
    pub fn magenta(&self) -> Option<i64> { self.prop_i64("Magenta") }
    /// 设置品红（`Magenta`）。
    pub fn set_magenta(&self, v: i32) -> bool { self.put_i64("Magenta", v as i64) }

    /// 读取黄（`Yellow`）。
    pub fn yellow(&self) -> Option<i64> { self.prop_i64("Yellow") }
    /// 设置黄（`Yellow`）。
    pub fn set_yellow(&self, v: i32) -> bool { self.put_i64("Yellow", v as i64) }

    /// 读取黑色（`Black`）。
    pub fn black(&self) -> Option<i64> { self.prop_i64("Black") }
    /// 设置黑色（`Black`）。
    pub fn set_black(&self, v: i32) -> bool { self.put_i64("Black", v as i64) }

    /// 读取颜色（`Color`）。
    pub fn color(&self) -> Option<IvgColor> {
        self.disp.get_property("Color").ok()?.to_idispatch().ok().map(IvgColor::new)
    }

    /// 重置（COM 方法 `Reset`）。
    pub fn reset(&self) -> bool {
        self.disp.invoke_method("Reset", vec![]).is_ok()
    }

    /// 设置值集（`SetValues`）。
    pub fn set_values(&self, c: i32, m: i32, y: i32, k: i32) -> bool {
        let args = vec![
            Variant::from_i64(c as i64),
            Variant::from_i64(m as i64),
            Variant::from_i64(y as i64),
            Variant::from_i64(k as i64),
        ];
        self.disp.invoke_method("SetValues", args).is_ok()
    }
}

// =============================================================
// IvgDuotoneInk
// =============================================================

/// CorelDRAW `IVGDuotone` 对象的后期绑定包装（双色油墨）。
///
/// 所有属性与方法通过 `IDispatch::Invoke` 按名称调用，调用失败时一般返回 `None` 或 `false`。
pub struct IvgDuotoneInk {
    disp: ComObject,
}

impl IvgDuotoneInk {
    /// 由 COM `IDispatch` 构造包装对象。
    pub fn new(disp: IDispatch) -> Self {
        Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) }
    }

    fn prop_i64(&self, name: &str) -> Option<i64> {
        self.disp.get_property(name).ok()?.to_i64().ok()
    }

    /// 读取颜色（`Color`）。
    pub fn color(&self) -> Option<IvgColor> {
        self.disp.get_property("Color").ok()?.to_idispatch().ok().map(IvgColor::new)
    }

    /// 读取手柄数量（`HandleCount`）。
    pub fn handle_count(&self) -> Option<i64> { self.prop_i64("HandleCount") }

    /// 手柄 X 坐标（COM 方法 `HandleX`）。
    pub fn handle_x(&self, index: i32) -> Option<i64> {
        let args = vec![Variant::from_i64(index as i64)];
        self.disp.invoke_method("HandleX", args).ok()?.to_i64().ok()
    }

    /// 设置手柄 X 坐标（`put_HandleX`）。
    pub fn set_handle_x(&self, index: i32, v: i32) -> bool {
        let args = vec![
            Variant::from_i64(index as i64),
            Variant::from_i64(v as i64),
        ];
        self.disp.invoke_method("put_HandleX", args).is_ok()
    }

    /// 手柄 Y 坐标（COM 方法 `HandleY`）。
    pub fn handle_y(&self, index: i32) -> Option<i64> {
        let args = vec![Variant::from_i64(index as i64)];
        self.disp.invoke_method("HandleY", args).ok()?.to_i64().ok()
    }

    /// 设置手柄 Y 坐标（`put_HandleY`）。
    pub fn set_handle_y(&self, index: i32, v: i32) -> bool {
        let args = vec![
            Variant::from_i64(index as i64),
            Variant::from_i64(v as i64),
        ];
        self.disp.invoke_method("put_HandleY", args).is_ok()
    }

    /// 添加手柄（COM 方法 `AddHandle`）。
    pub fn add_handle(&self, x: i32, y: i32) -> Option<i64> {
        let args = vec![
            Variant::from_i64(x as i64),
            Variant::from_i64(y as i64),
        ];
        self.disp.invoke_method("AddHandle", args).ok()?.to_i64().ok()
    }

    /// 移除手柄（COM 方法 `RemoveHandle`）。
    pub fn remove_handle(&self, index: i32) -> Option<bool> {
        let args = vec![Variant::from_i64(index as i64)];
        self.disp
            .invoke_method("RemoveHandle", args)
            .ok()?
            .to_bool()
            .ok()
    }

    /// 查找手柄（COM 方法 `FindHandle`）。
    pub fn find_handle(&self, x: i32, y: i32) -> Option<i64> {
        let args = vec![
            Variant::from_i64(x as i64),
            Variant::from_i64(y as i64),
        ];
        self.disp.invoke_method("FindHandle", args).ok()?.to_i64().ok()
    }

    /// 曲线级别（COM 方法 `CurveLevel`）。
    pub fn curve_level(&self, index: i32) -> Option<i64> {
        let args = vec![Variant::from_i64(index as i64)];
        self.disp.invoke_method("CurveLevel", args).ok()?.to_i64().ok()
    }

    /// 重置（COM 方法 `Reset`）。
    pub fn reset(&self) -> bool {
        self.disp.invoke_method("Reset", vec![]).is_ok()
    }

    /// load（COM 方法 `Load`）。
    pub fn load(&self, file: impl Into<String>) -> Option<bool> {
        let args = vec![Variant::from_str(file.into())];
        self.disp.invoke_method("Load", args).ok()?.to_bool().ok()
    }

    /// 保存（COM 方法 `Save`）。
    pub fn save(&self, file: impl Into<String>) -> Option<bool> {
        let args = vec![Variant::from_str(file.into())];
        self.disp.invoke_method("Save", args).ok()?.to_bool().ok()
    }
}

// =============================================================
// IvgDuotone
// =============================================================

/// CorelDRAW `IVGDuotone` 对象的后期绑定包装（双色）。
///
/// 所有属性与方法通过 `IDispatch::Invoke` 按名称调用，调用失败时一般返回 `None` 或 `false`。
pub struct IvgDuotone {
    disp: ComObject,
}

impl IvgDuotone {
    /// 由 COM `IDispatch` 构造包装对象。
    pub fn new(disp: IDispatch) -> Self {
        Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) }
    }

    fn prop_i64(&self, name: &str) -> Option<i64> {
        self.disp.get_property(name).ok()?.to_i64().ok()
    }

    fn prop_bool(&self, name: &str) -> Option<bool> {
        self.disp.get_property(name).ok()?.to_bool().ok()
    }

    fn put_i64(&self, name: &str, v: i64) -> bool {
        let arg = Variant::from_i64(v);
        self.disp.set_property(name, vec![arg]).is_ok()
    }

    fn put_bool(&self, name: &str, v: bool) -> bool {
        let arg = Variant::from_bool(v);
        self.disp.set_property(name, vec![arg]).is_ok()
    }

    /// 读取双色类型（`Type`）。
    pub fn duotone_type(&self) -> Option<i64> { self.prop_i64("Type") }
    /// 设置双色类型（`Type`）。
    pub fn set_duotone_type(&self, v: i32) -> bool { self.put_i64("Type", v as i64) }

    /// 读取使用叠印（`UseOverprints`）。
    pub fn use_overprints(&self) -> Option<bool> { self.prop_bool("UseOverprints") }
    /// 设置使用叠印（`UseOverprints`）。
    pub fn set_use_overprints(&self, v: bool) -> bool { self.put_bool("UseOverprints", v) }

    /// 读取叠印数量（`OverprintCount`）。
    pub fn overprint_count(&self) -> Option<i64> { self.prop_i64("OverprintCount") }

    /// 叠印（COM 方法 `Overprints`）。
    pub fn overprints(&self, index: i32) -> Option<IvgDuotoneOverprint> {
        let args = vec![Variant::from_i64(index as i64)];
        self.disp
            .invoke_method("Overprints", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgDuotoneOverprint::new)
    }

    /// 读取 COM 属性 `InkCount`。
    pub fn ink_count(&self) -> Option<i64> { self.prop_i64("InkCount") }

    /// 调用 COM 方法 `Inks`。
    pub fn inks(&self, index: i32) -> Option<IvgDuotoneInk> {
        let args = vec![Variant::from_i64(index as i64)];
        self.disp
            .invoke_method("Inks", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgDuotoneInk::new)
    }

    /// 读取复制（`GetCopy`）。
    pub fn get_copy(&self) -> Option<IvgDuotone> {
        self.disp
            .invoke_method("GetCopy", vec![])
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgDuotone::new)
    }

    /// 复制 assign（COM 方法 `CopyAssign`）。
    pub fn copy_assign(&self, other: &IvgDuotone) -> bool {
        let args = vec![other.as_variant()];
        self.disp.invoke_method("CopyAssign", args).is_ok()
    }

    /// 重置叠印（COM 方法 `ResetOverprints`）。
    pub fn reset_overprints(&self) -> bool {
        self.disp.invoke_method("ResetOverprints", vec![]).is_ok()
    }

    /// load（COM 方法 `Load`）。
    pub fn load(&self, file: impl Into<String>) -> Option<bool> {
        let args = vec![Variant::from_str(file.into())];
        self.disp.invoke_method("Load", args).ok()?.to_bool().ok()
    }

    /// 保存（COM 方法 `Save`）。
    pub fn save(&self, file: impl Into<String>) -> Option<bool> {
        let args = vec![Variant::from_str(file.into())];
        self.disp.invoke_method("Save", args).ok()?.to_bool().ok()
    }

    /// 把底层 COM 对象包装成 `VARIANT`，用于传给其它 COM 调用。
    pub fn as_variant(&self) -> Variant {
        self.disp.as_variant()
    }
}