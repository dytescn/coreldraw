//! `IPrnVBAPrintTrapping` / `IPrnVBATrapLayer(s)` —— 陷印

use wincom::{ComObject, Variant};
use windows::Win32::System::Com::IDispatch;

// =============================================================
// IvgPrnTrapping
// =============================================================

/// CorelDRAW `IPrnVBAPrintTrapping` 对象的后期绑定包装（陷印）。
///
/// 所有属性与方法通过 `IDispatch::Invoke` 按名称调用，调用失败时一般返回 `None` 或 `false`。
pub struct IvgPrnTrapping {
    disp: ComObject,
}

impl IvgPrnTrapping {
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

    fn prop_bool(&self, name: &str) -> Option<bool> {
        self.disp.get_property(name).ok()?.to_bool().ok()
    }

    fn prop_dispatch(&self, name: &str) -> Option<IDispatch> {
        self.disp.get_property(name).ok()?.to_idispatch().ok()
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

    /// 读取启用状态（`Enabled`）。
    pub fn enabled(&self) -> Option<bool> { self.prop_bool("Enabled") }
    /// 设置启用状态（`Enabled`）。
    pub fn set_enabled(&self, v: bool) -> bool { self.put_bool("Enabled", v) }

    /// 读取图层集（`Layers`）。
    pub fn layers(&self) -> Option<IvgPrnTrapLayers> {
        self.prop_dispatch("Layers").map(IvgPrnTrapLayers::new)
    }

    /// 读取宽度（`Width`）。
    pub fn width(&self) -> Option<f64> { self.prop_f64("Width") }
    /// 设置宽度（`Width`）。
    pub fn set_width(&self, v: f64) -> bool { self.put_f64("Width", v) }

    /// 读取黑色宽度（`BlackWidth`）。
    pub fn black_width(&self) -> Option<f64> { self.prop_f64("BlackWidth") }
    /// 设置黑色宽度（`BlackWidth`）。
    pub fn set_black_width(&self, v: f64) -> bool { self.put_f64("BlackWidth", v) }

    /// 读取颜色缩放（`ColorScaling`）。
    pub fn color_scaling(&self) -> Option<i64> { self.prop_i64("ColorScaling") }
    /// 设置颜色缩放（`ColorScaling`）。
    pub fn set_color_scaling(&self, v: i32) -> bool { self.put_i64("ColorScaling", v as i64) }

    /// 读取 COM 属性 `StepLimit`。
    pub fn step_limit(&self) -> Option<i64> { self.prop_i64("StepLimit") }
    /// 设置 COM 属性 `StepLimit`。
    pub fn set_step_limit(&self, v: i32) -> bool { self.put_i64("StepLimit", v as i64) }

    /// 读取黑色颜色限制（`BlackColorLimit`）。
    pub fn black_color_limit(&self) -> Option<i64> { self.prop_i64("BlackColorLimit") }
    /// 设置黑色颜色限制（`BlackColorLimit`）。
    pub fn set_black_color_limit(&self, v: i32) -> bool {
        self.put_i64("BlackColorLimit", v as i64)
    }

    /// 读取黑色密度限制（`BlackDensityLimit`）。
    pub fn black_density_limit(&self) -> Option<f64> {
        self.prop_f64("BlackDensityLimit")
    }
    /// 设置黑色密度限制（`BlackDensityLimit`）。
    pub fn set_black_density_limit(&self, v: f64) -> bool {
        self.put_f64("BlackDensityLimit", v)
    }

    /// 读取 COM 属性 `SlidingTrapLimit`。
    pub fn sliding_trap_limit(&self) -> Option<i64> { self.prop_i64("SlidingTrapLimit") }
    /// 设置 COM 属性 `SlidingTrapLimit`。
    pub fn set_sliding_trap_limit(&self, v: i32) -> bool {
        self.put_i64("SlidingTrapLimit", v as i64)
    }

    /// `PrnImageTrap`
    pub fn image_trap(&self) -> Option<i64> { self.prop_i64("ImageTrap") }
    /// 设置图像陷印（`ImageTrap`）。
    pub fn set_image_trap(&self, v: i32) -> bool { self.put_i64("ImageTrap", v as i64) }

    /// 读取对象集图像（`ObjectsToImage`）。
    pub fn objects_to_image(&self) -> Option<bool> { self.prop_bool("ObjectsToImage") }
    /// 设置对象集图像（`ObjectsToImage`）。
    pub fn set_objects_to_image(&self, v: bool) -> bool {
        self.put_bool("ObjectsToImage", v)
    }

    /// 读取内部图像陷印（`InternalImageTrapping`）。
    pub fn internal_image_trapping(&self) -> Option<bool> {
        self.prop_bool("InternalImageTrapping")
    }
    /// 设置内部图像陷印（`InternalImageTrapping`）。
    pub fn set_internal_image_trapping(&self, v: bool) -> bool {
        self.put_bool("InternalImageTrapping", v)
    }

    /// 读取陷印单色位图集（`TrapMonoBitmaps`）。
    pub fn trap_mono_bitmaps(&self) -> Option<bool> { self.prop_bool("TrapMonoBitmaps") }
    /// 设置陷印单色位图集（`TrapMonoBitmaps`）。
    pub fn set_trap_mono_bitmaps(&self, v: bool) -> bool {
        self.put_bool("TrapMonoBitmaps", v)
    }
}

// =============================================================
// IvgPrnTrapLayers
// =============================================================

/// CorelDRAW `IPrnVBAPrintTrapping` 对象的后期绑定包装（陷印图层集）。
///
/// 所有属性与方法通过 `IDispatch::Invoke` 按名称调用，调用失败时一般返回 `None` 或 `false`。
pub struct IvgPrnTrapLayers {
    disp: ComObject,
}

impl IvgPrnTrapLayers {
    /// 由 COM `IDispatch` 构造包装对象。
    pub fn new(disp: IDispatch) -> Self {
        Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) }
    }

    /// 读取返回条目数量（`Count`）。
    pub fn count(&self) -> Option<i64> {
        self.disp.get_property("Count").ok()?.to_i64().ok()
    }

    /// 按索引或名称取条目（COM 方法 `Item`）。
    pub fn item(&self, index: i32) -> Option<IvgPrnTrapLayer> {
        let args = vec![Variant::from_i64(index as i64)];
        self.disp
            .invoke_method("Item", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgPrnTrapLayer::new)
    }
}

// =============================================================
// IvgPrnTrapLayer
// =============================================================

/// CorelDRAW `IPrnVBAPrintTrapping` 对象的后期绑定包装（陷印图层）。
///
/// 所有属性与方法通过 `IDispatch::Invoke` 按名称调用，调用失败时一般返回 `None` 或 `false`。
pub struct IvgPrnTrapLayer {
    disp: ComObject,
}

impl IvgPrnTrapLayer {
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

    /// 读取陷印类型（`Type`）。
    pub fn trap_type(&self) -> Option<i64> { self.prop_i64("Type") }
    /// 读取颜色（`Color`）。
    pub fn color(&self) -> Option<String> { self.prop_string("Color") }

    /// 读取密度（`Density`）。
    pub fn density(&self) -> Option<f64> { self.prop_f64("Density") }
    /// 设置密度（`Density`）。
    pub fn set_density(&self, v: f64) -> bool { self.put_f64("Density", v) }

    /// `PrnTrapType`
    pub fn trap_kind(&self) -> Option<i64> { self.prop_i64("TrapType") }
    /// 设置 COM 属性 `TrapType`。
    pub fn set_trap_kind(&self, v: i32) -> bool { self.put_i64("TrapType", v as i64) }

    /// 读取 order（`Order`）。
    pub fn order(&self) -> Option<i64> { self.prop_i64("Order") }
    /// 设置 order（`Order`）。
    pub fn set_order(&self, v: i32) -> bool { self.put_i64("Order", v as i64) }
}