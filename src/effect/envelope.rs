//! `IVGEffectEnvelope` 鈥斺€?灏佸鏁堟灉

use wincom::{ComObject, Variant};
use windows::Win32::System::Com::IDispatch;

use crate::curve::IvgCurve;
use crate::shape::IvgShape;

pub struct IvgEffectEnvelope {
    disp: ComObject,
}

impl IvgEffectEnvelope {
    pub fn new(disp: IDispatch) -> Self {
        Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) }
    }

    pub fn as_variant(&self) -> Variant {
        self.disp.as_variant()
    }

    fn prop_i64(&self, name: &str) -> Option<i64> {
        self.disp.get_property(name).ok()?.to_i64().ok()
    }

    fn prop_bool(&self, name: &str) -> Option<bool> {
        self.disp.get_property(name).ok()?.to_bool().ok()
    }

    fn prop_dispatch(&self, name: &str) -> Option<IDispatch> {
        self.disp.get_property(name).ok()?.to_idispatch().ok()
    }

    fn put_bool(&self, name: &str, v: bool) -> bool {
        let arg = Variant::from_bool(v);
        self.disp.set_property(name, vec![arg]).is_ok()
    }

    // ---- 瀹瑰櫒鏇茬嚎 ----

    pub fn container(&self) -> Option<IvgCurve> {
        self.prop_dispatch("Container").map(IvgCurve::new)
    }

    pub fn set_container(&self, c: &IvgCurve) -> bool {
        let arg = c.as_variant();
        self.disp.set_property("Container", vec![arg]).is_ok()
    }

    // ---- 妯″紡 ----

    /// `cdrEnvelopeMode`
    pub fn mode(&self) -> Option<i64> { self.prop_i64("Mode") }
    pub fn set_mode(&self, v: i32) -> bool {
        let arg = Variant::from_i64(v as i64);
        self.disp.set_property("Mode", vec![arg]).is_ok()
    }

    pub fn keep_lines(&self) -> Option<bool> { self.prop_bool("KeepLines") }
    pub fn set_keep_lines(&self, v: bool) -> bool { self.put_bool("KeepLines", v) }

    // ---- 鎿嶄綔 ----

    /// 浠庨璁惧皝濂楀舰鐘剁储寮曢€夋嫨锛坄cdrEnvelopePreset`锛夈€?
    pub fn select(&self, preset_index: i32) -> bool {
        let args = vec![Variant::from_i64(preset_index as i64)];
        self.disp.invoke_method("Select", args).is_ok()
    }

    pub fn copy_from(&self, src: &IvgEffectEnvelope) -> bool {
        let args = vec![src.as_variant()];
        self.disp.invoke_method("CopyFrom", args).is_ok()
    }

    /// 浠庡浘褰㈠舰鐘跺垱寤哄皝濂椼€?
    pub fn create_from(&self, shape: &IvgShape) -> Option<bool> {
        let args = vec![shape.as_variant()];
        self.disp
            .invoke_method("CreateFrom", args)
            .ok()?
            .to_bool()
            .ok()
    }

    /// 浠庡浘褰㈢殑杞粨鏇茬嚎澶嶅埗灏佸銆?
    pub fn copy_from_shape(
        &self,
        source: &IvgShape,
        mode: i32,
        keep_lines: bool,
        copy_mode: i32,
        corner_indices: Variant,
    ) -> Option<bool> {
        let args = vec![
            source.as_variant(),
            Variant::from_i64(mode as i64),
            Variant::from_bool(keep_lines),
            Variant::from_i64(copy_mode as i64),
            corner_indices,
        ];
        self.disp
            .invoke_method("CopyFromShape", args)
            .ok()?
            .to_bool()
            .ok()
    }

    /// 浠庢洸绾垮鍒跺皝濂椼€?
    pub fn copy_from_curve(
        &self,
        source: &IvgCurve,
        mode: i32,
        keep_lines: bool,
        copy_mode: i32,
        corner_indices: Variant,
    ) -> Option<bool> {
        let args = vec![
            source.as_variant(),
            Variant::from_i64(mode as i64),
            Variant::from_bool(keep_lines),
            Variant::from_i64(copy_mode as i64),
            corner_indices,
        ];
        self.disp
            .invoke_method("CopyFromCurve", args)
            .ok()?
            .to_bool()
            .ok()
    }

    // ---- 瑙掔偣 ----

    pub fn corner_indices(&self) -> Option<Variant> {
        self.disp.get_property("CornerIndices").ok()
    }

    pub fn set_corner_indices(&self, v: Variant) -> bool {
        self.disp.set_property("CornerIndices", vec![v]).is_ok()
    }
}