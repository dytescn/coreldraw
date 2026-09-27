//! `IVGBitmap` 鈥斺€?浣嶅浘

use wincom::{ComObject, Variant};
use windows::Win32::System::Com::IDispatch;

use crate::curve::IvgCurve;
use crate::import_export::ICorelExportFilter;
use crate::shape::IvgImage;
use crate::structs::IvgStructPaletteOptions;

pub struct IvgBitmap {
    disp: ComObject,
}

impl IvgBitmap {
    pub fn new(disp: IDispatch) -> Self {
        Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) }
    }

    pub fn from_disp(disp: IDispatch) -> Self { Self::new(disp) }

    pub fn raw(&self) -> &ComObject { &self.disp }

    fn prop_i64(&self, name: &str) -> Option<i64> {
        self.disp.get_property(name).ok()?.to_i64().ok()
    }

    fn prop_bool(&self, name: &str) -> Option<bool> {
        self.disp.get_property(name).ok()?.to_bool().ok()
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

    // ---- 鍩烘湰淇℃伅 ----

    pub fn size_width(&self) -> Option<i64> { self.prop_i64("SizeWidth") }
    pub fn size_height(&self) -> Option<i64> { self.prop_i64("SizeHeight") }
    pub fn resolution_x(&self) -> Option<i64> { self.prop_i64("ResolutionX") }
    pub fn resolution_y(&self) -> Option<i64> { self.prop_i64("ResolutionY") }

    pub fn externally_linked(&self) -> Option<bool> { self.prop_bool("ExternallyLinked") }
    pub fn link_file_name(&self) -> Option<String> { self.prop_string("LinkFileName") }
    pub fn set_link_file_name(&self, v: impl Into<String>) -> bool {
        self.put_string("LinkFileName", v)
    }

    /// `cdrImageType`
    pub fn mode(&self) -> Option<i64> { self.prop_i64("Mode") }

    pub fn transparent(&self) -> Option<bool> { self.prop_bool("Transparent") }
    pub fn watermarked(&self) -> Option<bool> { self.prop_bool("Watermarked") }
    pub fn opi_linked(&self) -> Option<bool> { self.prop_bool("OPILinked") }
    pub fn is_eps(&self) -> Option<bool> { self.prop_bool("IsEPS") }
    pub fn embedded(&self) -> Option<bool> { self.prop_bool("Embedded") }
    pub fn cropped(&self) -> Option<bool> { self.prop_bool("Cropped") }
    pub fn crop_envelope_modified(&self) -> Option<bool> { self.prop_bool("CropEnvelopeModified") }

    pub fn crop_envelope(&self) -> Option<IvgCurve> {
        self.prop_dispatch("CropEnvelope").map(IvgCurve::new)
    }

    pub fn bounding_box_path(&self) -> Option<IvgCurve> {
        self.prop_dispatch("BoundingBoxPath").map(IvgCurve::new)
    }

    // ---- 鎿嶄綔 ----

    pub fn resolve_link(&self) -> bool { self.disp.invoke_method("ResolveLink", vec![]).is_ok() }
    pub fn update_link(&self) -> bool { self.disp.invoke_method("UpdateLink", vec![]).is_ok() }

    pub fn inflate(&self, w: i32, h: i32) -> bool {
        let args = vec![
            Variant::from_i64(w as i64),
            Variant::from_i64(h as i64),
        ];
        self.disp.invoke_method("Inflate", args).is_ok()
    }

    pub fn resample(
        &self,
        width: i32, height: i32,
        anti_alias: bool,
        resolution_x: f64, resolution_y: f64,
    ) -> bool {
        let args = vec![
            Variant::from_i64(width as i64),
            Variant::from_i64(height as i64),
            Variant::from_bool(anti_alias),
            Variant::from_f64(resolution_x),
            Variant::from_f64(resolution_y),
        ];
        self.disp.invoke_method("Resample", args).is_ok()
    }

    pub fn convert_to(&self, mode: i32) -> bool {
        let args = vec![Variant::from_i64(mode as i64)];
        self.disp.invoke_method("ConvertTo", args).is_ok()
    }

    pub fn apply_bitmap_effect(
        &self,
        undo_string: impl Into<String>,
        command: impl Into<String>,
    ) -> bool {
        let args = vec![
            Variant::from_str(undo_string.into()),
            Variant::from_str(command.into()),
        ];
        self.disp.invoke_method("ApplyBitmapEffect", args).is_ok()
    }

    pub fn crop(&self) -> bool { self.disp.invoke_method("Crop", vec![]).is_ok() }
    pub fn reset_crop_envelope(&self) -> bool {
        self.disp.invoke_method("ResetCropEnvelope", vec![]).is_ok()
    }

    // ---- 淇濆瓨 ----

    pub fn save_as(
        &self,
        file_name: impl Into<String>,
        filter: i32,
        compression: i32,
    ) -> Option<ICorelExportFilter> {
        let args = vec![
            Variant::from_str(file_name.into()),
            Variant::from_i64(filter as i64),
            Variant::from_i64(compression as i64),
        ];
        self.disp
            .invoke_method("SaveAs", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(ICorelExportFilter::new)
    }

    // ---- 璋冭壊鏉?----

    pub fn convert_to_paletted(
        &self,
        palette_type: i32,
        dither_type: i32,
        dither_intensity: i32,
        smoothing: i32,
        num_colors: i32,
        color_sensitive: bool,
        target_color: i32,
        importance: i32,
        lightness: i32,
        tolerance_a: i32,
        tolerance_b: i32,
        palette: Variant,
    ) -> bool {
        let args = vec![
            Variant::from_i64(palette_type as i64),
            Variant::from_i64(dither_type as i64),
            Variant::from_i64(dither_intensity as i64),
            Variant::from_i64(smoothing as i64),
            Variant::from_i64(num_colors as i64),
            Variant::from_bool(color_sensitive),
            Variant::from_i64(target_color as i64),
            Variant::from_i64(importance as i64),
            Variant::from_i64(lightness as i64),
            Variant::from_i64(tolerance_a as i64),
            Variant::from_i64(tolerance_b as i64),
            palette,
        ];
        self.disp.invoke_method("ConvertToPaletted", args).is_ok()
    }

    pub fn convert_to_paletted2(&self, options: &IvgStructPaletteOptions) -> bool {
        let args = vec![options.as_variant()];
        self.disp.invoke_method("ConvertToPaletted2", args).is_ok()
    }

    pub fn convert_to_bw(
        &self,
        render_type: i32,
        intensity: i32,
        threshold: i32,
        halftone: i32,
        halftone_angle: i32,
        halftone_size: i32,
    ) -> bool {
        let args = vec![
            Variant::from_i64(render_type as i64),
            Variant::from_i64(intensity as i64),
            Variant::from_i64(threshold as i64),
            Variant::from_i64(halftone as i64),
            Variant::from_i64(halftone_angle as i64),
            Variant::from_i64(halftone_size as i64),
        ];
        self.disp.invoke_method("ConvertToBW", args).is_ok()
    }

    // ---- 鍥惧儚鏁版嵁 ----

    pub fn image(&self) -> Option<IvgImage> {
        self.prop_dispatch("Image").map(IvgImage::new)
    }

    pub fn image_alpha(&self) -> Option<IvgImage> {
        self.prop_dispatch("ImageAlpha").map(IvgImage::new)
    }
    #[allow(unused)]
    pub fn set_image_data(
        &self,
        image: &IvgImage,
        alpha: &IvgImage,
        offset_x: i32,
        offset_y: i32,
    ) -> bool {
        let args = vec![
            image.as_variant(),
            image.as_variant(),
            Variant::from_i64(offset_x as i64),
            Variant::from_i64(offset_y as i64),
        ];
        self.disp.invoke_method("SetImageData", args).is_ok()
    }

    // ---- 鎻忔懝 ----

    #[allow(clippy::too_many_arguments)]
    pub fn trace(
        &self,
        trace_type: i32,
        smoothing: i16,
        detail_level_percent: i16,
        color_mode: i32,
        palette_id: i32,
        color_count: i32,
        delete_original: bool,
        remove_background: bool,
        remove_entire_back: bool,
    ) -> Option<crate::misc::IvgTraceSettings> {
        let args = vec![
            Variant::from_i64(trace_type as i64),
            Variant::from_i64(smoothing as i64),
            Variant::from_i64(detail_level_percent as i64),
            Variant::from_i64(color_mode as i64),
            Variant::from_i64(palette_id as i64),
            Variant::from_i64(color_count as i64),
            Variant::from_bool(delete_original),
            Variant::from_bool(remove_background),
            Variant::from_bool(remove_entire_back),
        ];
        self.disp
            .invoke_method("Trace", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(crate::misc::IvgTraceSettings::new)
    }
}