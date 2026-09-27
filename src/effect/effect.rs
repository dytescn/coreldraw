//! `IVGEffect` 鈥斺€?鏁堟灉瀵硅薄锛堥€氱敤鍏ュ彛锛?
//!
//! 涓€涓?`IVGShape` 鍙互鎸傚涓晥鏋滐紙娣峰悎銆佽疆寤撱€侀槾褰扁€︹€︼級銆?
//! 閫氳繃 `shape.effect()` 鎴?`shape.effects().item(n)` 寰楀埌銆?
//!
//! `Type` 灞炴€у尯鍒嗗叿浣撶被鍨嬶紙`cdrEffectType`锛岃 `enums/effect`锛夛紝
//! 鐒跺悗鐢ㄥ搴?getter 鎷垮埌寮虹被鍨嬪瓙瀵硅薄锛?
//! - `.blend()`       鈫?`IvgEffectBlend`
//! - `.contour()`     鈫?`IvgEffectContour`
//! - `.extrude()`     鈫?`IvgEffectExtrude`
//! - `.envelope()`    鈫?`IvgEffectEnvelope`
//! - `.drop_shadow()` 鈫?`IvgEffectDropShadow`
//! - `.lens()`        鈫?`IvgEffectLens`
//! - `.perspective()` 鈫?`IvgEffectPerspective`
//! - `.distortion()`  鈫?`IvgEffectDistortion`
//! - `.text_on_path()`鈫?`IvgEffectTextOnPath`
//! - `.custom()`      鈫?`IvgCustomEffect`

use wincom::{ComObject, Variant};
use windows::Win32::System::Com::IDispatch;

use crate::effect::{
    IvgEffectBlend, IvgEffectContour, IvgEffectControlPath, IvgEffectDistortion,
    IvgEffectDropShadow, IvgEffectEnvelope, IvgEffectExtrude, IvgEffectLens,
    IvgEffectPerspective, IvgEffectTextOnPath, IvgEffects,
};
use crate::shape::IvgShapeRange;

// =============================================================
// IvgEffect
// =============================================================

pub struct IvgEffect {
    disp: ComObject,
}

impl IvgEffect {
    pub fn new(disp: IDispatch) -> Self {
        Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) }
    }

    pub fn from_disp(disp: IDispatch) -> Self {
        Self::new(disp)
    }

    pub fn raw(&self) -> &ComObject {
        &self.disp
    }

    pub fn as_variant(&self) -> Variant {
        self.disp.as_variant()
    }

    // ---------------------------------------------------------
    // 宸ュ叿
    // ---------------------------------------------------------

    fn prop_i64(&self, name: &str) -> Option<i64> {
        self.disp.get_property(name).ok()?.to_i64().ok()
    }

    fn prop_dispatch(&self, name: &str) -> Option<IDispatch> {
        self.disp.get_property(name).ok()?.to_idispatch().ok()
    }

    // ---------------------------------------------------------
    // 鍩烘湰淇℃伅
    // ---------------------------------------------------------

    /// `cdrEffectType` 鈥斺€?瑙?`enums/effect`
    pub fn effect_type(&self) -> Option<i64> {
        self.prop_i64("Type")
    }

    // ---------------------------------------------------------
    // 瀛愭晥鏋滐紙鎸夌被鍨嬪彇寮虹被鍨嬶級
    // ---------------------------------------------------------

    pub fn blend(&self) -> Option<IvgEffectBlend> {
        self.prop_dispatch("Blend").map(IvgEffectBlend::new)
    }

    pub fn control_path(&self) -> Option<IvgEffectControlPath> {
        self.prop_dispatch("ControlPath").map(IvgEffectControlPath::new)
    }

    pub fn extrude(&self) -> Option<IvgEffectExtrude> {
        self.prop_dispatch("Extrude").map(IvgEffectExtrude::new)
    }

    pub fn envelope(&self) -> Option<IvgEffectEnvelope> {
        self.prop_dispatch("Envelope").map(IvgEffectEnvelope::new)
    }

    pub fn text_on_path(&self) -> Option<IvgEffectTextOnPath> {
        self.prop_dispatch("TextOnPath").map(IvgEffectTextOnPath::new)
    }

    pub fn drop_shadow(&self) -> Option<IvgEffectDropShadow> {
        self.prop_dispatch("DropShadow").map(IvgEffectDropShadow::new)
    }

    pub fn contour(&self) -> Option<IvgEffectContour> {
        self.prop_dispatch("Contour").map(IvgEffectContour::new)
    }

    pub fn distortion(&self) -> Option<IvgEffectDistortion> {
        self.prop_dispatch("Distortion").map(IvgEffectDistortion::new)
    }

    pub fn lens(&self) -> Option<IvgEffectLens> {
        self.prop_dispatch("Lens").map(IvgEffectLens::new)
    }

    pub fn perspective(&self) -> Option<IvgEffectPerspective> {
        self.prop_dispatch("Perspective").map(IvgEffectPerspective::new)
    }

    pub fn custom(&self) -> Option<IvgCustomEffect> {
        self.prop_dispatch("Custom").map(IvgCustomEffect::new)
    }

    // ---------------------------------------------------------
    // 鍏嬮殕鍏崇郴
    // ---------------------------------------------------------

    /// 閫氳繃鍏嬮殕杩炴帴鍒版湰鏁堟灉鐨勫叾瀹冩晥鏋滈泦鍚堛€?
    pub fn clones(&self) -> Option<IvgEffects> {
        self.prop_dispatch("Clones").map(IvgEffects::new)
    }

    /// 鏈晥鏋滃厠闅嗚嚜鍝釜鏁堟灉銆?
    pub fn clone_parent(&self) -> Option<IvgEffect> {
        self.prop_dispatch("CloneParent").map(IvgEffect::new)
    }

    // ---------------------------------------------------------
    // 鎿嶄綔
    // ---------------------------------------------------------

    /// 绉婚櫎鏁堟灉銆?
    pub fn clear(&self) -> bool {
        self.disp.invoke_method("Clear", vec![]).is_ok()
    }

    /// 鍒嗙鏁堟灉锛堟妸鏁堟灉鎷嗘垚鐙珛鍥惧舰锛夈€?
    pub fn separate(&self) -> Option<IvgShapeRange> {
        self.disp
            .invoke_method("Separate", vec![])
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgShapeRange::new)
    }
}

// =============================================================
// IvgCustomEffect
// =============================================================

pub struct IvgCustomEffect {
    disp: ComObject,
}

impl IvgCustomEffect {
    pub fn new(disp: IDispatch) -> Self {
        Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) }
    }

    fn prop_string(&self, name: &str) -> Option<String> {
        self.disp.get_property(name).ok()?.to_string().ok()
    }

    fn prop_dispatch(&self, name: &str) -> Option<IDispatch> {
        self.disp.get_property(name).ok()?.to_idispatch().ok()
    }

    /// 鏁堟灉 ID锛圙UID 瀛楃涓诧級銆?
    pub fn effect_id(&self) -> Option<String> {
        self.prop_string("EffectID")
    }

    /// 鏁堟灉浜х敓鐨勫浘褰㈠垎缁勩€?
    pub fn effect_group(&self) -> Option<crate::shape::IvgShape> {
        self.prop_dispatch("EffectGroup").map(crate::shape::IvgShape::new)
    }
}