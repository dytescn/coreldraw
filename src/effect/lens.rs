//! `IVGEffectLens` 鈥斺€?閫忛暅鏁堟灉

use wincom::{ComObject, Variant};
use windows::Win32::System::Com::IDispatch;

use crate::color::IvgColor;
use crate::shape::{IvgShape, IvgShapeRange, IvgShapes};

pub struct IvgEffectLens {
    disp: ComObject,
}

impl IvgEffectLens {
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

    fn put_color(&self, name: &str, c: &IvgColor) -> bool {
        let arg = c.as_variant();
        self.disp.set_property(name, vec![arg]).is_ok()
    }

    // ---- 鍐荤粨 ----

    pub fn freeze(&self) -> bool { self.disp.invoke_method("Freeze", vec![]).is_ok() }
    pub fn unfreeze(&self) -> bool { self.disp.invoke_method("Unfreeze", vec![]).is_ok() }

    pub fn ungroup(&self) -> Option<IvgShapeRange> {
        self.disp
            .invoke_method("Ungroup", vec![])
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgShapeRange::new)
    }

    pub fn frozen(&self) -> Option<bool> { self.prop_bool("Frozen") }

    pub fn shapes(&self) -> Option<IvgShapes> {
        self.prop_dispatch("Shapes").map(IvgShapes::new)
    }

    // ---- 绫诲瀷 ----

    /// `cdrLensType`
    pub fn lens_type(&self) -> Option<i64> { self.prop_i64("Type") }
    pub fn set_lens_type(&self, v: i32) -> bool { self.put_i64("Type", v as i64) }

    pub fn rate(&self) -> Option<i64> { self.prop_i64("Rate") }
    pub fn set_rate(&self, v: i32) -> bool { self.put_i64("Rate", v as i64) }

    pub fn magnification(&self) -> Option<f64> { self.prop_f64("Magnification") }
    pub fn set_magnification(&self, v: f64) -> bool { self.put_f64("Magnification", v) }

    // ---- 棰滆壊 ----

    pub fn color(&self) -> Option<IvgColor> {
        self.prop_dispatch("Color").map(IvgColor::new)
    }
    pub fn set_color(&self, c: &IvgColor) -> bool { self.put_color("Color", c) }

    pub fn outline_color(&self) -> Option<IvgColor> {
        self.prop_dispatch("OutlineColor").map(IvgColor::new)
    }
    pub fn set_outline_color(&self, c: &IvgColor) -> bool { self.put_color("OutlineColor", c) }

    pub fn fill_color(&self) -> Option<IvgColor> {
        self.prop_dispatch("FillColor").map(IvgColor::new)
    }
    pub fn set_fill_color(&self, c: &IvgColor) -> bool { self.put_color("FillColor", c) }

    pub fn from_color(&self) -> Option<IvgColor> {
        self.prop_dispatch("FromColor").map(IvgColor::new)
    }
    pub fn set_from_color(&self, c: &IvgColor) -> bool { self.put_color("FromColor", c) }

    pub fn to_color(&self) -> Option<IvgColor> {
        self.prop_dispatch("ToColor").map(IvgColor::new)
    }
    pub fn set_to_color(&self, c: &IvgColor) -> bool { self.put_color("ToColor", c) }

    pub fn use_outline_color(&self) -> Option<bool> {
        self.prop_bool("UseOutlineColor")
    }
    pub fn set_use_outline_color(&self, v: bool) -> bool { self.put_bool("UseOutlineColor", v) }

    pub fn use_fill_color(&self) -> Option<bool> {
        self.prop_bool("UseFillColor")
    }
    pub fn set_use_fill_color(&self, v: bool) -> bool { self.put_bool("UseFillColor", v) }

    /// `cdrFountainFillBlendType`
    pub fn color_map_palette(&self) -> Option<i64> {
        self.prop_i64("ColorMapPalette")
    }
    pub fn set_color_map_palette(&self, v: i32) -> bool {
        self.put_i64("ColorMapPalette", v as i64)
    }

    // ---- 瑙嗙偣 ----

    pub fn use_view_point(&self) -> Option<bool> { self.prop_bool("UseViewPoint") }
    pub fn set_use_view_point(&self, v: bool) -> bool { self.put_bool("UseViewPoint", v) }

    pub fn view_point_x(&self) -> Option<f64> { self.prop_f64("ViewPointX") }
    pub fn set_view_point_x(&self, v: f64) -> bool { self.put_f64("ViewPointX", v) }

    pub fn view_point_y(&self) -> Option<f64> { self.prop_f64("ViewPointY") }
    pub fn set_view_point_y(&self, v: f64) -> bool { self.put_f64("ViewPointY", v) }

    // ---- 绉婚櫎闈?/ 璋冭壊鏉挎棆杞?----

    pub fn remove_face(&self) -> Option<bool> { self.prop_bool("RemoveFace") }
    pub fn set_remove_face(&self, v: bool) -> bool { self.put_bool("RemoveFace", v) }

    pub fn palette_rotation(&self) -> Option<i64> { self.prop_i64("PaletteRotation") }
    pub fn set_palette_rotation(&self, v: i32) -> bool {
        self.put_i64("PaletteRotation", v as i64)
    }

    // ---- 渚挎嵎锛氱洿鎺ユ嬁鍒板叧鑱斿浘褰?----

    pub fn associated_shape(&self) -> Option<IvgShape> {
        self.prop_dispatch("Shapes")
            .and_then(|s| IvgShapes::new(s).item_by_index(1))
    }
}