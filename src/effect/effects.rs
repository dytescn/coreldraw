//! `IVGEffects` 鈥斺€?鏁堟灉闆嗗悎

use wincom::{ComObject, Variant};
use windows::Win32::System::Com::IDispatch;

use crate::effect::IvgEffect;

pub struct IvgEffects {
    disp: ComObject,
}

impl IvgEffects {
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
    // 鍩烘湰淇℃伅
    // ---------------------------------------------------------

    pub fn count(&self) -> Option<i64> {
        self.prop_i64("Count")
    }

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
    // 鍒嗙被鏁堟灉闆嗗悎
    // ---------------------------------------------------------

    pub fn blend_effects(&self) -> Option<IvgEffects> {
        self.prop_dispatch("BlendEffects").map(IvgEffects::new)
    }

    pub fn custom_effects(&self) -> Option<IvgEffects> {
        self.prop_dispatch("CustomEffects").map(IvgEffects::new)
    }

    pub fn distortion_effects(&self) -> Option<IvgEffects> {
        self.prop_dispatch("DistortionEffects").map(IvgEffects::new)
    }

    pub fn envelope_effects(&self) -> Option<IvgEffects> {
        self.prop_dispatch("EnvelopeEffects").map(IvgEffects::new)
    }

    pub fn perspective_effects(&self) -> Option<IvgEffects> {
        self.prop_dispatch("PerspectiveEffects").map(IvgEffects::new)
    }

    // ---------------------------------------------------------
    // 鍗曚緥鏁堟灉锛? 鎴?1 涓級
    // ---------------------------------------------------------

    pub fn contour_effect(&self) -> Option<IvgEffect> {
        self.prop_dispatch("ContourEffect").map(IvgEffect::new)
    }

    pub fn control_path_effect(&self) -> Option<IvgEffect> {
        self.prop_dispatch("ControlPathEffect").map(IvgEffect::new)
    }

    pub fn drop_shadow_effect(&self) -> Option<IvgEffect> {
        self.prop_dispatch("DropShadowEffect").map(IvgEffect::new)
    }

    pub fn extrude_effect(&self) -> Option<IvgEffect> {
        self.prop_dispatch("ExtrudeEffect").map(IvgEffect::new)
    }

    pub fn lens_effect(&self) -> Option<IvgEffect> {
        self.prop_dispatch("LensEffect").map(IvgEffect::new)
    }

    pub fn text_on_path_effect(&self) -> Option<IvgEffect> {
        self.prop_dispatch("TextOnPathEffect").map(IvgEffect::new)
    }
}