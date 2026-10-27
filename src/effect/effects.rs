//! `IVGEffects` —— 效果集合

use wincom::{ComObject, Variant};
use windows::Win32::System::Com::IDispatch;

use crate::effect::IvgEffect;

/// CorelDRAW `IVGEffects` 对象的后期绑定包装（效果集）。
///
/// 所有属性与方法通过 `IDispatch::Invoke` 按名称调用，调用失败时一般返回 `None` 或 `false`。
pub struct IvgEffects {
    disp: ComObject,
}

impl IvgEffects {
    /// 由 COM `IDispatch` 构造包装对象。
    pub fn new(disp: IDispatch) -> Self {
        Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) }
    }

    fn prop_i64(&self, name: &str) -> Option<i64> {
        self.disp.get_property(name).ok()?.to_i64().ok()
    }

    fn prop_dispatch(&self, name: &str) -> Option<IDispatch> {
        self.disp.get_property(name).ok()?.to_idispatch().ok()
    }

    // ---------------------------------------------------------
    // 基本信息
    // ---------------------------------------------------------

    /// 读取返回条目数量（`Count`）。
    pub fn count(&self) -> Option<i64> {
        self.prop_i64("Count")
    }

    /// 按索引或名称取条目（COM 方法 `Item`）。
    pub fn item(&self, index: i32) -> Option<IvgEffect> {
        let args = vec![Variant::from_i64(index as i64)];
        self.disp
            .invoke_method("Item", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgEffect::new)
    }

    // ---------------------------------------------------------
    // 分类效果集合
    // ---------------------------------------------------------

    /// 读取调和效果集（`BlendEffects`）。
    pub fn blend_effects(&self) -> Option<IvgEffects> {
        self.prop_dispatch("BlendEffects").map(IvgEffects::new)
    }

    /// 读取自定义效果集（`CustomEffects`）。
    pub fn custom_effects(&self) -> Option<IvgEffects> {
        self.prop_dispatch("CustomEffects").map(IvgEffects::new)
    }

    /// 读取变形效果集（`DistortionEffects`）。
    pub fn distortion_effects(&self) -> Option<IvgEffects> {
        self.prop_dispatch("DistortionEffects").map(IvgEffects::new)
    }

    /// 读取封套效果集（`EnvelopeEffects`）。
    pub fn envelope_effects(&self) -> Option<IvgEffects> {
        self.prop_dispatch("EnvelopeEffects").map(IvgEffects::new)
    }

    /// 读取透视效果集（`PerspectiveEffects`）。
    pub fn perspective_effects(&self) -> Option<IvgEffects> {
        self.prop_dispatch("PerspectiveEffects").map(IvgEffects::new)
    }

    // ---------------------------------------------------------
    // 单例效果（1 个）
    // ---------------------------------------------------------

    /// 读取轮廓效果（`ContourEffect`）。
    pub fn contour_effect(&self) -> Option<IvgEffect> {
        self.prop_dispatch("ContourEffect").map(IvgEffect::new)
    }

    /// 读取控件路径效果（`ControlPathEffect`）。
    pub fn control_path_effect(&self) -> Option<IvgEffect> {
        self.prop_dispatch("ControlPathEffect").map(IvgEffect::new)
    }

    /// 读取投影阴影效果（`DropShadowEffect`）。
    pub fn drop_shadow_effect(&self) -> Option<IvgEffect> {
        self.prop_dispatch("DropShadowEffect").map(IvgEffect::new)
    }

    /// 读取立体化效果（`ExtrudeEffect`）。
    pub fn extrude_effect(&self) -> Option<IvgEffect> {
        self.prop_dispatch("ExtrudeEffect").map(IvgEffect::new)
    }

    /// 读取透镜效果（`LensEffect`）。
    pub fn lens_effect(&self) -> Option<IvgEffect> {
        self.prop_dispatch("LensEffect").map(IvgEffect::new)
    }

    /// 读取路径效果上的文本（`TextOnPathEffect`）。
    pub fn text_on_path_effect(&self) -> Option<IvgEffect> {
        self.prop_dispatch("TextOnPathEffect").map(IvgEffect::new)
    }
}