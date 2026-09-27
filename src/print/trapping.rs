//! `IPrnVBAPrintTrapping` / `IPrnVBATrapLayer(s)` 鈥斺€?闄峰嵃

use wincom::{ComObject, Variant};
use windows::Win32::System::Com::IDispatch;

// =============================================================
// IvgPrnTrapping
// =============================================================

pub struct IvgPrnTrapping {
    disp: ComObject,
}

impl IvgPrnTrapping {
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

    pub fn enabled(&self) -> Option<bool> { self.prop_bool("Enabled") }
    pub fn set_enabled(&self, v: bool) -> bool { self.put_bool("Enabled", v) }

    pub fn layers(&self) -> Option<IvgPrnTrapLayers> {
        self.prop_dispatch("Layers").map(IvgPrnTrapLayers::new)
    }

    pub fn width(&self) -> Option<f64> { self.prop_f64("Width") }
    pub fn set_width(&self, v: f64) -> bool { self.put_f64("Width", v) }

    pub fn black_width(&self) -> Option<f64> { self.prop_f64("BlackWidth") }
    pub fn set_black_width(&self, v: f64) -> bool { self.put_f64("BlackWidth", v) }

    pub fn color_scaling(&self) -> Option<i64> { self.prop_i64("ColorScaling") }
    pub fn set_color_scaling(&self, v: i32) -> bool { self.put_i64("ColorScaling", v as i64) }

    pub fn step_limit(&self) -> Option<i64> { self.prop_i64("StepLimit") }
    pub fn set_step_limit(&self, v: i32) -> bool { self.put_i64("StepLimit", v as i64) }

    pub fn black_color_limit(&self) -> Option<i64> { self.prop_i64("BlackColorLimit") }
    pub fn set_black_color_limit(&self, v: i32) -> bool {
        self.put_i64("BlackColorLimit", v as i64)
    }

    pub fn black_density_limit(&self) -> Option<f64> {
        self.prop_f64("BlackDensityLimit")
    }
    pub fn set_black_density_limit(&self, v: f64) -> bool {
        self.put_f64("BlackDensityLimit", v)
    }

    pub fn sliding_trap_limit(&self) -> Option<i64> { self.prop_i64("SlidingTrapLimit") }
    pub fn set_sliding_trap_limit(&self, v: i32) -> bool {
        self.put_i64("SlidingTrapLimit", v as i64)
    }

    /// `PrnImageTrap`
    pub fn image_trap(&self) -> Option<i64> { self.prop_i64("ImageTrap") }
    pub fn set_image_trap(&self, v: i32) -> bool { self.put_i64("ImageTrap", v as i64) }

    pub fn objects_to_image(&self) -> Option<bool> { self.prop_bool("ObjectsToImage") }
    pub fn set_objects_to_image(&self, v: bool) -> bool {
        self.put_bool("ObjectsToImage", v)
    }

    pub fn internal_image_trapping(&self) -> Option<bool> {
        self.prop_bool("InternalImageTrapping")
    }
    pub fn set_internal_image_trapping(&self, v: bool) -> bool {
        self.put_bool("InternalImageTrapping", v)
    }

    pub fn trap_mono_bitmaps(&self) -> Option<bool> { self.prop_bool("TrapMonoBitmaps") }
    pub fn set_trap_mono_bitmaps(&self, v: bool) -> bool {
        self.put_bool("TrapMonoBitmaps", v)
    }
}

// =============================================================
// IvgPrnTrapLayers
// =============================================================

pub struct IvgPrnTrapLayers {
    disp: ComObject,
}

impl IvgPrnTrapLayers {
    pub fn new(disp: IDispatch) -> Self {
        Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) }
    }

    pub fn count(&self) -> Option<i64> {
        self.disp.get_property("Count").ok()?.to_i64().ok()
    }

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

pub struct IvgPrnTrapLayer {
    disp: ComObject,
}

impl IvgPrnTrapLayer {
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

    pub fn trap_type(&self) -> Option<i64> { self.prop_i64("Type") }
    pub fn color(&self) -> Option<String> { self.prop_string("Color") }

    pub fn density(&self) -> Option<f64> { self.prop_f64("Density") }
    pub fn set_density(&self, v: f64) -> bool { self.put_f64("Density", v) }

    /// `PrnTrapType`
    pub fn trap_kind(&self) -> Option<i64> { self.prop_i64("TrapType") }
    pub fn set_trap_kind(&self, v: i32) -> bool { self.put_i64("TrapType", v as i64) }

    pub fn order(&self) -> Option<i64> { self.prop_i64("Order") }
    pub fn set_order(&self, v: i32) -> bool { self.put_i64("Order", v as i64) }
}