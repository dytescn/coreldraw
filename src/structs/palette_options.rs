//! `IVGStructPaletteOptions` 鈥斺€?璋冭壊鏉?/ 浣嶅浘杞皟鑹叉澘閫夐」
//!
//! 鐢ㄤ簬 `IVGBitmap::ConvertToPaletted2` 绛夈€?

use wincom::{ComObject, Variant};
use windows::Win32::System::Com::IDispatch;

pub struct IvgStructPaletteOptions {
    disp: ComObject,
}

impl IvgStructPaletteOptions {
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

    // ---- 璋冭壊鏉跨被鍨?/ 棰滆壊鏁?----

    /// `cdrImagePaletteType`
    pub fn palette_type(&self) -> Option<i64> { self.prop_i64("PaletteType") }
    pub fn set_palette_type(&self, v: i32) -> bool {
        self.put_i64("PaletteType", v as i64)
    }

    pub fn num_colors(&self) -> Option<i64> { self.prop_i64("NumColors") }
    pub fn set_num_colors(&self, v: i32) -> bool { self.put_i64("NumColors", v as i64) }

    // ---- 鎶栧姩 ----

    /// `cdrDitherType`
    pub fn dither_type(&self) -> Option<i64> { self.prop_i64("DitherType") }
    pub fn set_dither_type(&self, v: i32) -> bool { self.put_i64("DitherType", v as i64) }

    pub fn dither_intensity(&self) -> Option<i64> { self.prop_i64("DitherIntensity") }
    pub fn set_dither_intensity(&self, v: i32) -> bool {
        self.put_i64("DitherIntensity", v as i64)
    }

    // ---- 骞虫粦 / 棰滆壊鏁忔劅 ----

    pub fn smoothing(&self) -> Option<i64> { self.prop_i64("Smoothing") }
    pub fn set_smoothing(&self, v: i32) -> bool { self.put_i64("Smoothing", v as i64) }

    pub fn color_sensitive(&self) -> Option<bool> { self.prop_bool("ColorSensitive") }
    pub fn set_color_sensitive(&self, v: bool) -> bool {
        self.put_bool("ColorSensitive", v)
    }

    // ---- 鐩爣鑹?/ 閲嶈鎬?----

    pub fn target_color(&self) -> Option<i64> { self.prop_i64("TargetColor") }
    pub fn set_target_color(&self, v: i32) -> bool { self.put_i64("TargetColor", v as i64) }

    pub fn importance(&self) -> Option<i64> { self.prop_i64("Importance") }
    pub fn set_importance(&self, v: i32) -> bool { self.put_i64("Importance", v as i64) }

    // ---- 鏄庡害 / 瀹瑰樊 ----

    pub fn lightness(&self) -> Option<i64> { self.prop_i64("Lightness") }
    pub fn set_lightness(&self, v: i32) -> bool { self.put_i64("Lightness", v as i64) }

    pub fn tolerance_a(&self) -> Option<i64> { self.prop_i64("ToleranceA") }
    pub fn set_tolerance_a(&self, v: i32) -> bool { self.put_i64("ToleranceA", v as i64) }

    pub fn tolerance_b(&self) -> Option<i64> { self.prop_i64("ToleranceB") }
    pub fn set_tolerance_b(&self, v: i32) -> bool { self.put_i64("ToleranceB", v as i64) }

    // ---- 璋冭壊鏉挎暟鎹?----

    /// 璋冭壊鏉挎暟缁勶紙`SAFEARRAY`锛夈€?
    pub fn palette(&self) -> Option<Variant> {
        self.disp.get_property("Palette").ok()
    }
    pub fn set_palette(&self, v: Variant) -> bool {
        self.disp.set_property("Palette", vec![v]).is_ok()
    }
}