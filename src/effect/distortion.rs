//! `IVGEffectDistortion` 鈥斺€?鍙樺舰鏁堟灉
//!
//! 鍥涚妯″紡锛?
//! - Push-Pull锛堟帹鎷夛級
//! - Zipper锛堟媺閾撅級
//! - Twister锛堟壄鏇诧級
//! - Custom锛堣嚜瀹氫箟锛?

use wincom::{ComObject, Variant};
use windows::Win32::System::Com::IDispatch;

// =============================================================
// IvgEffectDistortion
// =============================================================

pub struct IvgEffectDistortion {
    disp: ComObject,
}

impl IvgEffectDistortion {
    pub fn new(disp: IDispatch) -> Self {
        Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) }
    }

    pub fn as_variant(&self) -> Variant {
        self.disp.as_variant()
    }

    fn prop_i64(&self, name: &str) -> Option<i64> {
        self.disp.get_property(name).ok()?.to_i64().ok()
    }

    fn prop_f64(&self, name: &str) -> Option<f64> {
        self.disp.get_property(name).ok()?.to_f64().ok()
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

    /// `cdrDistortionType`
    pub fn distortion_type(&self) -> Option<i64> { self.prop_i64("Type") }
    pub fn set_distortion_type(&self, v: i32) -> bool { self.put_i64("Type", v as i64) }

    pub fn origin_x(&self) -> Option<f64> { self.prop_f64("OriginX") }
    pub fn set_origin_x(&self, v: f64) -> bool { self.put_f64("OriginX", v) }

    pub fn origin_y(&self) -> Option<f64> { self.prop_f64("OriginY") }
    pub fn set_origin_y(&self, v: f64) -> bool { self.put_f64("OriginY", v) }

    pub fn push_pull(&self) -> Option<IvgEffectPushPullDistortion> {
        self.prop_dispatch("PushPull").map(IvgEffectPushPullDistortion::new)
    }

    pub fn zipper(&self) -> Option<IvgEffectZipperDistortion> {
        self.prop_dispatch("Zipper").map(IvgEffectZipperDistortion::new)
    }

    pub fn twister(&self) -> Option<IvgEffectTwisterDistortion> {
        self.prop_dispatch("Twister").map(IvgEffectTwisterDistortion::new)
    }

    pub fn custom(&self) -> Option<IvgEffectCustomDistortion> {
        self.prop_dispatch("Custom").map(IvgEffectCustomDistortion::new)
    }

    /// 鎶婂彉褰腑蹇冨鍑嗗浘褰腑蹇冦€?
    pub fn center_distortion(&self) -> bool {
        self.disp.invoke_method("CenterDistortion", vec![]).is_ok()
    }
}

// =============================================================
// IvgEffectPushPullDistortion
// =============================================================

pub struct IvgEffectPushPullDistortion {
    disp: ComObject,
}

impl IvgEffectPushPullDistortion {
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

    pub fn amplitude(&self) -> Option<i64> { self.prop_i64("Amplitude") }
    pub fn set_amplitude(&self, v: i32) -> bool { self.put_i64("Amplitude", v as i64) }
}

// =============================================================
// IvgEffectZipperDistortion
// =============================================================

pub struct IvgEffectZipperDistortion {
    disp: ComObject,
}

impl IvgEffectZipperDistortion {
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

    pub fn amplitude(&self) -> Option<i64> { self.prop_i64("Amplitude") }
    pub fn set_amplitude(&self, v: i32) -> bool { self.put_i64("Amplitude", v as i64) }

    pub fn frequency(&self) -> Option<i64> { self.prop_i64("Frequency") }
    pub fn set_frequency(&self, v: i32) -> bool { self.put_i64("Frequency", v as i64) }

    pub fn random(&self) -> Option<bool> { self.prop_bool("Random") }
    pub fn set_random(&self, v: bool) -> bool { self.put_bool("Random", v) }

    pub fn smooth(&self) -> Option<bool> { self.prop_bool("Smooth") }
    pub fn set_smooth(&self, v: bool) -> bool { self.put_bool("Smooth", v) }

    pub fn local(&self) -> Option<bool> { self.prop_bool("Local") }
    pub fn set_local(&self, v: bool) -> bool { self.put_bool("Local", v) }
}

// =============================================================
// IvgEffectTwisterDistortion
// =============================================================

pub struct IvgEffectTwisterDistortion {
    disp: ComObject,
}

impl IvgEffectTwisterDistortion {
    pub fn new(disp: IDispatch) -> Self {
        Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) }
    }

    fn prop_f64(&self, name: &str) -> Option<f64> {
        self.disp.get_property(name).ok()?.to_f64().ok()
    }

    fn put_f64(&self, name: &str, v: f64) -> bool {
        let arg = Variant::from_f64(v);
        self.disp.set_property(name, vec![arg]).is_ok()
    }

    pub fn angle(&self) -> Option<f64> { self.prop_f64("Angle") }
    pub fn set_angle(&self, v: f64) -> bool { self.put_f64("Angle", v) }
}

// =============================================================
// IvgEffectCustomDistortion
// =============================================================

pub struct IvgEffectCustomDistortion {
    disp: ComObject,
}

impl IvgEffectCustomDistortion {
    pub fn new(disp: IDispatch) -> Self {
        Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) }
    }

    /// 鑷畾涔夊彉褰㈢殑 ID锛圙UID 瀛楃涓诧級銆?
    pub fn distortion_id(&self) -> Option<String> {
        self.disp.get_property("DistortionID").ok()?.to_string().ok()
    }
}