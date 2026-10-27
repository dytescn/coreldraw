//! `IVGEffectDistortion` —— 变形效果
//!
//! 四种模式：
//! - Push-Pull（推拉）
//! - Zipper（拉链）
//! - Twister（扭曲）
//! - Custom（自定义）

use wincom::{ComObject, Variant};
use windows::Win32::System::Com::IDispatch;

// =============================================================
// IvgEffectDistortion
// =============================================================

/// CorelDRAW `IVGEffectDistortion` 对象的后期绑定包装（效果变形）。
///
/// 所有属性与方法通过 `IDispatch::Invoke` 按名称调用，调用失败时一般返回 `None` 或 `false`。
pub struct IvgEffectDistortion {
    disp: ComObject,
}

impl IvgEffectDistortion {
    /// 由 COM `IDispatch` 构造包装对象。
    pub fn new(disp: IDispatch) -> Self {
        Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) }
    }

    /// 把底层 COM 对象包装成 `VARIANT`，用于传给其它 COM 调用。
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
    /// 设置变形类型（`Type`）。
    pub fn set_distortion_type(&self, v: i32) -> bool { self.put_i64("Type", v as i64) }

    /// 读取原点 X 坐标（`OriginX`）。
    pub fn origin_x(&self) -> Option<f64> { self.prop_f64("OriginX") }
    /// 设置原点 X 坐标（`OriginX`）。
    pub fn set_origin_x(&self, v: f64) -> bool { self.put_f64("OriginX", v) }

    /// 读取原点 Y 坐标（`OriginY`）。
    pub fn origin_y(&self) -> Option<f64> { self.prop_f64("OriginY") }
    /// 设置原点 Y 坐标（`OriginY`）。
    pub fn set_origin_y(&self, v: f64) -> bool { self.put_f64("OriginY", v) }

    /// 读取 COM 属性 `PushPull`。
    pub fn push_pull(&self) -> Option<IvgEffectPushPullDistortion> {
        self.prop_dispatch("PushPull").map(IvgEffectPushPullDistortion::new)
    }

    /// 读取 COM 属性 `Zipper`。
    pub fn zipper(&self) -> Option<IvgEffectZipperDistortion> {
        self.prop_dispatch("Zipper").map(IvgEffectZipperDistortion::new)
    }

    /// 读取 COM 属性 `Twister`。
    pub fn twister(&self) -> Option<IvgEffectTwisterDistortion> {
        self.prop_dispatch("Twister").map(IvgEffectTwisterDistortion::new)
    }

    /// 读取自定义（`Custom`）。
    pub fn custom(&self) -> Option<IvgEffectCustomDistortion> {
        self.prop_dispatch("Custom").map(IvgEffectCustomDistortion::new)
    }

    /// 把变形中心对准图形中心。
    pub fn center_distortion(&self) -> bool {
        self.disp.invoke_method("CenterDistortion", vec![]).is_ok()
    }
}

// =============================================================
// IvgEffectPushPullDistortion
// =============================================================

/// CorelDRAW `IVGEffectDistortion` 对象的后期绑定包装（推拉变形效果）。
///
/// 所有属性与方法通过 `IDispatch::Invoke` 按名称调用，调用失败时一般返回 `None` 或 `false`。
pub struct IvgEffectPushPullDistortion {
    disp: ComObject,
}

impl IvgEffectPushPullDistortion {
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

    /// 读取振幅（`Amplitude`）。
    pub fn amplitude(&self) -> Option<i64> { self.prop_i64("Amplitude") }
    /// 设置振幅（`Amplitude`）。
    pub fn set_amplitude(&self, v: i32) -> bool { self.put_i64("Amplitude", v as i64) }
}

// =============================================================
// IvgEffectZipperDistortion
// =============================================================

/// CorelDRAW `IVGEffectDistortion` 对象的后期绑定包装（效果拉链变形）。
///
/// 所有属性与方法通过 `IDispatch::Invoke` 按名称调用，调用失败时一般返回 `None` 或 `false`。
pub struct IvgEffectZipperDistortion {
    disp: ComObject,
}

impl IvgEffectZipperDistortion {
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

    /// 读取振幅（`Amplitude`）。
    pub fn amplitude(&self) -> Option<i64> { self.prop_i64("Amplitude") }
    /// 设置振幅（`Amplitude`）。
    pub fn set_amplitude(&self, v: i32) -> bool { self.put_i64("Amplitude", v as i64) }

    /// 读取频率（`Frequency`）。
    pub fn frequency(&self) -> Option<i64> { self.prop_i64("Frequency") }
    /// 设置频率（`Frequency`）。
    pub fn set_frequency(&self, v: i32) -> bool { self.put_i64("Frequency", v as i64) }

    /// 读取随机（`Random`）。
    pub fn random(&self) -> Option<bool> { self.prop_bool("Random") }
    /// 设置随机（`Random`）。
    pub fn set_random(&self, v: bool) -> bool { self.put_bool("Random", v) }

    /// 读取 smooth（`Smooth`）。
    pub fn smooth(&self) -> Option<bool> { self.prop_bool("Smooth") }
    /// 设置 smooth（`Smooth`）。
    pub fn set_smooth(&self, v: bool) -> bool { self.put_bool("Smooth", v) }

    /// 读取本地（`Local`）。
    pub fn local(&self) -> Option<bool> { self.prop_bool("Local") }
    /// 设置本地（`Local`）。
    pub fn set_local(&self, v: bool) -> bool { self.put_bool("Local", v) }
}

// =============================================================
// IvgEffectTwisterDistortion
// =============================================================

/// CorelDRAW `IVGEffectDistortion` 对象的后期绑定包装（效果扭曲变形）。
///
/// 所有属性与方法通过 `IDispatch::Invoke` 按名称调用，调用失败时一般返回 `None` 或 `false`。
pub struct IvgEffectTwisterDistortion {
    disp: ComObject,
}

impl IvgEffectTwisterDistortion {
    /// 由 COM `IDispatch` 构造包装对象。
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

    /// 读取角度（`Angle`）。
    pub fn angle(&self) -> Option<f64> { self.prop_f64("Angle") }
    /// 设置角度（`Angle`）。
    pub fn set_angle(&self, v: f64) -> bool { self.put_f64("Angle", v) }
}

// =============================================================
// IvgEffectCustomDistortion
// =============================================================

/// CorelDRAW `IVGEffectDistortion` 对象的后期绑定包装（效果自定义变形）。
///
/// 所有属性与方法通过 `IDispatch::Invoke` 按名称调用，调用失败时一般返回 `None` 或 `false`。
pub struct IvgEffectCustomDistortion {
    disp: ComObject,
}

impl IvgEffectCustomDistortion {
    /// 由 COM `IDispatch` 构造包装对象。
    pub fn new(disp: IDispatch) -> Self {
        Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) }
    }

    /// 自定义变形的 ID（GUID 字符串）。
    pub fn distortion_id(&self) -> Option<String> {
        self.disp.get_property("DistortionID").ok()?.to_string().ok()
    }
}