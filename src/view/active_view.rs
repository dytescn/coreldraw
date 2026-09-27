//! `IVGActiveView` / `IVGProofColorSettings` 鈥斺€?娲诲姩瑙嗗浘 / 鏍℃牱璁剧疆

use wincom::{ComObject, Variant};
use windows::Win32::System::Com::IDispatch;

use crate::color::{IvgColor, IvgColorContext};
use crate::shape::{IvgShape, IvgShapeRange};

// =============================================================
// IvgActiveView
// =============================================================

pub struct IvgActiveView {
    disp: ComObject,
}

impl IvgActiveView {
    pub fn new(disp: IDispatch) -> Self {
        Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) }
    }

    pub fn from_disp(disp: IDispatch) -> Self {
        Self::new(disp)
    }

    pub fn as_variant(&self) -> Variant {
        self.disp.as_variant()
    }

    pub fn raw(&self) -> &ComObject {
        &self.disp
    }

    // ---------------------------------------------------------
    // 閫氱敤宸ュ叿
    // ---------------------------------------------------------

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

    fn put_dispatch(&self, name: &str, v: Variant) -> bool {
        self.disp.set_property(name, vec![v]).is_ok()
    }

    // ---------------------------------------------------------
    // 瑙嗗浘绫诲瀷 / 缂╂斁 / 鍘熺偣
    // ---------------------------------------------------------

    /// `cdrViewType` 鈥斺€?瑙?`enums::view`
    pub fn view_type(&self) -> Option<i64> { self.prop_i64("Type") }
    pub fn set_view_type(&self, v: i32) -> bool { self.put_i64("Type", v as i64) }

    pub fn origin_x(&self) -> Option<f64> { self.prop_f64("OriginX") }
    pub fn set_origin_x(&self, v: f64) -> bool { self.put_f64("OriginX", v) }

    pub fn origin_y(&self) -> Option<f64> { self.prop_f64("OriginY") }
    pub fn set_origin_y(&self, v: f64) -> bool { self.put_f64("OriginY", v) }

    /// 缂╂斁锛坄Zoom` 涓虹櫨鍒嗘瘮锛屽 100.0锛夈€?
    pub fn zoom(&self) -> Option<f64> { self.prop_f64("Zoom") }
    pub fn set_zoom(&self, v: f64) -> bool { self.put_f64("Zoom", v) }

    // ---------------------------------------------------------
    // 閫傞厤瑙嗗浘
    // ---------------------------------------------------------

    pub fn to_fit_page(&self) -> bool {
        self.disp.invoke_method("ToFitPage", vec![]).is_ok()
    }

    pub fn to_fit_page_width(&self) -> bool {
        self.disp.invoke_method("ToFitPageWidth", vec![]).is_ok()
    }

    pub fn to_fit_page_height(&self) -> bool {
        self.disp.invoke_method("ToFitPageHeight", vec![]).is_ok()
    }

    pub fn to_fit_shape(&self, shape: &IvgShape) -> bool {
        let args = vec![shape.as_variant()];
        self.disp.invoke_method("ToFitShape", args).is_ok()
    }

    pub fn to_fit_selection(&self) -> bool {
        self.disp.invoke_method("ToFitSelection", vec![]).is_ok()
    }

    pub fn to_fit_area(&self, left: f64, top: f64, right: f64, bottom: f64) -> bool {
        let args = vec![
            Variant::from_f64(left),
            Variant::from_f64(top),
            Variant::from_f64(right),
            Variant::from_f64(bottom),
        ];
        self.disp.invoke_method("ToFitArea", args).is_ok()
    }

    pub fn to_fit_all_objects(&self) -> bool {
        self.disp.invoke_method("ToFitAllObjects", vec![]).is_ok()
    }

    pub fn to_fit_shape_range(&self, range: &IvgShapeRange) -> bool {
        let args = vec![range.as_variant()];
        self.disp.invoke_method("ToFitShapeRange", args).is_ok()
    }

    // ---------------------------------------------------------
    // 瑙嗙偣 / 缂╂斁鎿嶄綔
    // ---------------------------------------------------------

    pub fn set_view_point(&self, x: f64, y: f64, zoom: f64) -> bool {
        let args = vec![
            Variant::from_f64(x),
            Variant::from_f64(y),
            Variant::from_f64(zoom),
        ];
        self.disp.invoke_method("SetViewPoint", args).is_ok()
    }

    pub fn set_actual_size(&self) -> bool {
        self.disp.invoke_method("SetActualSize", vec![]).is_ok()
    }

    pub fn zoom_in(&self) -> bool {
        self.disp.invoke_method("ZoomIn", vec![]).is_ok()
    }

    pub fn zoom_in_at_point(&self, x: f64, y: f64) -> bool {
        let args = vec![
            Variant::from_f64(x),
            Variant::from_f64(y),
        ];
        self.disp.invoke_method("ZoomInAtPoint", args).is_ok()
    }

    pub fn zoom_out(&self) -> bool {
        self.disp.invoke_method("ZoomOut", vec![]).is_ok()
    }

    /// 鍙栬鍥惧尯鍩燂紙`x, y, w, h`锛夈€?
    pub fn get_view_area(&self) -> Option<(f64, f64, f64, f64)> {
        // 4 out 鍙傛暟鍦?IDispatch 閲屾嬁涓嶅埌锛岀敤灞炴€т唬鏇匡紙OriginX/Y + Zoom锛?
        // 濡傛灉纭疄闇€瑕侊紝鍙敼璧?vtable銆?
        None
    }

    pub fn set_view_area(&self, x: f64, y: f64, w: f64, h: f64) -> bool {
        let args = vec![
            Variant::from_f64(x),
            Variant::from_f64(y),
            Variant::from_f64(w),
            Variant::from_f64(h),
        ];
        self.disp.invoke_method("SetViewArea", args).is_ok()
    }

    // ---------------------------------------------------------
    // 鏍℃牱棰滆壊
    // ---------------------------------------------------------

    pub fn simulate_overprints(&self) -> Option<bool> {
        self.prop_bool("SimulateOverprints")
    }
    pub fn set_simulate_overprints(&self, v: bool) -> bool {
        self.put_bool("SimulateOverprints", v)
    }

    pub fn show_proof_colors(&self) -> Option<bool> { self.prop_bool("ShowProofColors") }
    pub fn set_show_proof_colors(&self, v: bool) -> bool {
        self.put_bool("ShowProofColors", v)
    }

    pub fn proof_color_settings(&self) -> Option<IvgProofColorSettings> {
        self.prop_dispatch("ProofColorSettings").map(IvgProofColorSettings::new)
    }

    pub fn set_proof_color_settings(&self, s: &IvgProofColorSettings) -> bool {
        self.put_dispatch("ProofColorSettings", s.as_variant())
    }
}

// =============================================================
// IvgProofColorSettings
// =============================================================

pub struct IvgProofColorSettings {
    disp: ComObject,
}

impl IvgProofColorSettings {
    pub fn new(disp: IDispatch) -> Self {
        Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) }
    }

    pub fn from_disp(disp: IDispatch) -> Self {
        Self::new(disp)
    }

    pub fn as_variant(&self) -> Variant {
        self.disp.as_variant()
    }

    pub fn raw(&self) -> &ComObject {
        &self.disp
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

    fn put_f64(&self, name: &str, v: f64) -> bool {
        let arg = Variant::from_f64(v);
        self.disp.set_property(name, vec![arg]).is_ok()
    }

    fn put_bool(&self, name: &str, v: bool) -> bool {
        let arg = Variant::from_bool(v);
        self.disp.set_property(name, vec![arg]).is_ok()
    }

    fn put_dispatch(&self, name: &str, v: Variant) -> bool {
        self.disp.set_property(name, vec![v]).is_ok()
    }

    /// 鏍℃牱鐢ㄧ殑棰滆壊涓婁笅鏂囷紙鐢ㄤ簬鎶婃枃妗ｉ鑹茶浆鎹㈠埌鎵撳嵃鏈?/ 鏄剧ず鍣ㄨ壊鍩燂級銆?
    pub fn color_context(&self) -> Option<IvgColorContext> {
        self.prop_dispatch("ColorContext").map(IvgColorContext::new)
    }
    pub fn set_color_context(&self, c: &IvgColorContext) -> bool {
        self.put_dispatch("ColorContext", c.as_variant())
    }

    /// 鏄惁鏄剧ず"瓒呭嚭鑹插煙"璀﹀憡銆?
    pub fn show_out_of_gamut_warning(&self) -> Option<bool> {
        self.prop_bool("ShowOutOfGamutWarning")
    }
    pub fn set_show_out_of_gamut_warning(&self, v: bool) -> bool {
        self.put_bool("ShowOutOfGamutWarning", v)
    }

    pub fn out_of_gamut_color(&self) -> Option<IvgColor> {
        self.prop_dispatch("OutOfGamutColor").map(IvgColor::new)
    }
    pub fn set_out_of_gamut_color(&self, c: &IvgColor) -> bool {
        self.put_dispatch("OutOfGamutColor", c.as_variant())
    }

    pub fn out_of_gamut_transparency(&self) -> Option<f64> {
        self.prop_f64("OutOfGamutTransparency")
    }
    pub fn set_out_of_gamut_transparency(&self, v: f64) -> bool {
        self.put_f64("OutOfGamutTransparency", v)
    }

    /// 鏄惁淇濈暀鍘熼鑹插€硷紙涓嶅仛棰滆壊杞崲锛夈€?
    pub fn preserve_color_values(&self) -> Option<bool> {
        self.prop_bool("PreserveColorValues")
    }
    pub fn set_preserve_color_values(&self, v: bool) -> bool {
        self.put_bool("PreserveColorValues", v)
    }

    // ---------------------------------------------------------
    // 澶嶅埗
    // ---------------------------------------------------------

    pub fn get_copy(&self) -> Option<IvgProofColorSettings> {
        self.disp
            .invoke_method("GetCopy", vec![])
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgProofColorSettings::new)
    }

    pub fn copy_assign(&self, src: &IvgProofColorSettings) -> bool {
        let args = vec![src.as_variant()];
        self.disp.invoke_method("CopyAssign", args).is_ok()
    }
}