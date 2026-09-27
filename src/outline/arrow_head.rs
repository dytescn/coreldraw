//! `IVGArrowHead` / `IVGArrowHeads` / `IVGArrowHeadOptions` 鈥斺€?绠ご

use wincom::{ComObject, Variant};
use windows::Win32::System::Com::IDispatch;

use crate::curve::IvgCurve;

// =============================================================
// IvgArrowHead
// =============================================================

pub struct IvgArrowHead {
    disp: ComObject,
}

impl IvgArrowHead {
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

    fn prop_i64(&self, name: &str) -> Option<i64> {
        self.disp.get_property(name).ok()?.to_i64().ok()
    }

    fn prop_f64(&self, name: &str) -> Option<f64> {
        self.disp.get_property(name).ok()?.to_f64().ok()
    }

    fn prop_string(&self, name: &str) -> Option<String> {
        self.disp.get_property(name).ok()?.to_string().ok()
    }

    fn prop_dispatch(&self, name: &str) -> Option<IDispatch> {
        self.disp.get_property(name).ok()?.to_idispatch().ok()
    }

    fn put_string(&self, name: &str, v: impl Into<String>) -> bool {
        let arg = Variant::from_str(v.into());
        self.disp.set_property(name, vec![arg]).is_ok()
    }

    // ---------------------------------------------------------
    // 绱㈠紩 / 鍚嶇О
    // ---------------------------------------------------------

    pub fn index(&self) -> Option<i64> { self.prop_i64("Index") }

    pub fn name(&self) -> Option<String> { self.prop_string("Name") }
    pub fn set_name(&self, v: impl Into<String>) -> bool { self.put_string("Name", v) }

    pub fn display_name(&self) -> Option<String> { self.prop_string("DisplayName") }

    // ---------------------------------------------------------
    // 褰㈢姸
    // ---------------------------------------------------------

    /// 绠ご褰㈢姸鐨勮疆寤撴洸绾匡紙鏈湴鍧愭爣锛夈€?
    pub fn curve(&self) -> Option<IvgCurve> {
        self.prop_dispatch("Curve").map(IvgCurve::new)
    }

    /// 鐩稿浜庣嚎瀹界殑鍩哄噯缂╂斁銆?
    pub fn base_outline_scale(&self) -> Option<f64> {
        self.prop_f64("BaseOutlineScale")
    }

    /// 绠ご涓績鐩稿鏈湴鍧愭爣鍘熺偣鐨?X 鍋忕Щ銆?
    pub fn center_x(&self) -> Option<f64> { self.prop_f64("CenterX") }
    pub fn center_y(&self) -> Option<f64> { self.prop_f64("CenterY") }

    /// 鐩稿绾挎绔偣鐨勫亸绉汇€?
    pub fn line_offset(&self) -> Option<f64> { self.prop_f64("LineOffset") }

    // ---------------------------------------------------------
    // 鎿嶄綔
    // ---------------------------------------------------------

    /// 缁戝畾鍒版寚瀹氭枃妗ｏ紙鐢ㄤ簬璺ㄦ枃妗ｅ鍒舵椂纭繚褰掑睘锛夈€?
    pub fn bind_to_document(&self, doc: &crate::document::IvgDocument) -> Option<IvgArrowHead> {
        let args = vec![doc.as_variant()];
        self.disp
            .invoke_method("BindToDocument", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgArrowHead::new)
    }

    pub fn compare_with(&self, other: &IvgArrowHead) -> Option<bool> {
        let args = vec![other.as_variant()];
        self.disp
            .invoke_method("CompareWith", args)
            .ok()?
            .to_bool()
            .ok()
    }
}

// =============================================================
// IvgArrowHeadOptions
// =============================================================

/// 鍗曚釜鍥惧舰涓婄澶寸殑**鍑犱綍璋冩暣**锛堜笌 `IvgArrowHead` 鏄ā鏉夸笉鍚岋級銆?
pub struct IvgArrowHeadOptions {
    disp: ComObject,
}

impl IvgArrowHeadOptions {
    pub fn new(disp: IDispatch) -> Self {
        Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) }
    }

    pub fn from_disp(disp: IDispatch) -> Self {
        Self::new(disp)
    }

    pub fn as_variant(&self) -> Variant {
        self.disp.as_variant()
    }

    fn prop_f64(&self, name: &str) -> Option<f64> {
        self.disp.get_property(name).ok()?.to_f64().ok()
    }

    fn prop_bool(&self, name: &str) -> Option<bool> {
        self.disp.get_property(name).ok()?.to_bool().ok()
    }

    fn put_f64(&self, name: &str, v: f64) -> bool {
        let arg = Variant::from_f64(v);
        self.disp.set_property(name, vec![arg]).is_ok()
    }

    fn put_bool(&self, name: &str, v: bool) -> bool {
        let arg = Variant::from_bool(v);
        self.disp.set_property(name, vec![arg]).is_ok()
    }

    // ---- 闀垮害 / 瀹藉害 ----

    pub fn length(&self) -> Option<f64> { self.prop_f64("Length") }
    pub fn set_length(&self, v: f64) -> bool { self.put_f64("Length", v) }

    pub fn width(&self) -> Option<f64> { self.prop_f64("Width") }
    pub fn set_width(&self, v: f64) -> bool { self.put_f64("Width", v) }

    // ---- 鍋忕Щ ----

    pub fn offset_x(&self) -> Option<f64> { self.prop_f64("OffsetX") }
    pub fn set_offset_x(&self, v: f64) -> bool { self.put_f64("OffsetX", v) }

    pub fn offset_y(&self) -> Option<f64> { self.prop_f64("OffsetY") }
    pub fn set_offset_y(&self, v: f64) -> bool { self.put_f64("OffsetY", v) }

    // ---- 鏃嬭浆 ----

    pub fn rotation_angle(&self) -> Option<f64> { self.prop_f64("RotationAngle") }
    pub fn set_rotation_angle(&self, v: f64) -> bool {
        self.put_f64("RotationAngle", v)
    }

    // ---- 缈昏浆 ----

    pub fn flip_horizontal(&self) -> Option<bool> { self.prop_bool("FlipHorizontal") }
    pub fn set_flip_horizontal(&self, v: bool) -> bool {
        self.put_bool("FlipHorizontal", v)
    }

    pub fn flip_vertical(&self) -> Option<bool> { self.prop_bool("FlipVertical") }
    pub fn set_flip_vertical(&self, v: bool) -> bool {
        self.put_bool("FlipVertical", v)
    }

    /// RIDL 閲岃繕鏈変竴涓嫾鍐欓敊璇殑 `FlipVerical`锛屼竴骞舵毚闇层€?
    pub fn flip_verical(&self) -> Option<bool> { self.prop_bool("FlipVerical") }
    pub fn set_flip_verical(&self, v: bool) -> bool {
        self.put_bool("FlipVerical", v)
    }

    /// `cdrFlipAxes`
    pub fn flip(&self, axes: i32) -> bool {
        let args = vec![Variant::from_i64(axes as i64)];
        self.disp.invoke_method("Flip", args).is_ok()
    }

    // ---- 澶嶅埗 ----

    pub fn copy_assign(&self, src: &IvgArrowHeadOptions) -> bool {
        let args = vec![src.as_variant()];
        self.disp.invoke_method("CopyAssign", args).is_ok()
    }

    pub fn get_copy(&self) -> Option<IvgArrowHeadOptions> {
        self.disp
            .invoke_method("GetCopy", vec![])
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgArrowHeadOptions::new)
    }
}

// =============================================================
// IvgArrowHeads
// =============================================================

/// 鏂囨。绾х澶撮泦鍚堬紙`Application.ArrowHeads`锛夈€?
pub struct IvgArrowHeads {
    disp: ComObject,
}

impl IvgArrowHeads {
    pub fn new(disp: IDispatch) -> Self {
        Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) }
    }

    fn prop_i64(&self, name: &str) -> Option<i64> {
        self.disp.get_property(name).ok()?.to_i64().ok()
    }

    pub fn count(&self) -> Option<i64> { self.prop_i64("Count") }

    pub fn item(&self, index: i32) -> Option<IvgArrowHead> {
        let args = vec![Variant::from_i64(index as i64)];
        self.disp
            .invoke_method("Item", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgArrowHead::new)
    }

    /// 娣诲姞涓€涓澶淬€?
    pub fn add(&self, arrow: &IvgArrowHead) -> Option<IvgArrowHead> {
        let args = vec![arrow.as_variant()];
        self.disp
            .invoke_method("Add", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgArrowHead::new)
    }

    /// 鏇挎崲鎸囧畾绱㈠紩鐨勭澶淬€?
    pub fn replace(&self, index: i32, arrow: &IvgArrowHead) -> Option<IvgArrowHead> {
        let args = vec![
            Variant::from_i64(index as i64),
            arrow.as_variant(),
        ];
        self.disp
            .invoke_method("Replace", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgArrowHead::new)
    }

    pub fn remove(&self, index: i32) -> bool {
        let args = vec![Variant::from_i64(index as i64)];
        self.disp.invoke_method("Remove", args).is_ok()
    }
}