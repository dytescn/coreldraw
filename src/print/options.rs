//! `IPrnVBAPrintOptions` 鈥斺€?鎵撳嵃閫夐」

use wincom::{ComObject, Variant};
use windows::Win32::System::Com::IDispatch;

pub struct IvgPrnOptions {
    disp: ComObject,
}

impl IvgPrnOptions {
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

    // ---- 浣跨敤鑹插僵閰嶇疆鏂囦欢 ----

    pub fn use_color_profile(&self) -> Option<bool> { self.prop_bool("UseColorProfile") }
    pub fn set_use_color_profile(&self, v: bool) -> bool {
        self.put_bool("UseColorProfile", v)
    }

    // ---- 鎵撳嵃鍐呭 ----

    pub fn print_vectors(&self) -> Option<bool> { self.prop_bool("PrintVectors") }
    pub fn set_print_vectors(&self, v: bool) -> bool { self.put_bool("PrintVectors", v) }

    pub fn print_bitmaps(&self) -> Option<bool> { self.prop_bool("PrintBitmaps") }
    pub fn set_print_bitmaps(&self, v: bool) -> bool { self.put_bool("PrintBitmaps", v) }

    pub fn print_text(&self) -> Option<bool> { self.prop_bool("PrintText") }
    pub fn set_print_text(&self, v: bool) -> bool { self.put_bool("PrintText", v) }

    pub fn text_in_black(&self) -> Option<bool> { self.prop_bool("TextInBlack") }
    pub fn set_text_in_black(&self, v: bool) -> bool { self.put_bool("TextInBlack", v) }

    /// `PrnColorMode`
    pub fn color_mode(&self) -> Option<i64> { self.prop_i64("ColorMode") }
    pub fn set_color_mode(&self, v: i32) -> bool { self.put_i64("ColorMode", v as i64) }

    /// `PrnBitmapColorMode`
    pub fn bitmap_color_mode(&self) -> Option<i64> { self.prop_i64("BitmapColorMode") }
    pub fn set_bitmap_color_mode(&self, v: i32) -> bool {
        self.put_i64("BitmapColorMode", v as i64)
    }

    /// `PrnObjectsColorMode`
    pub fn objects_color_mode(&self) -> Option<i64> { self.prop_i64("ObjectsColorMode") }
    pub fn set_objects_color_mode(&self, v: i32) -> bool {
        self.put_i64("ObjectsColorMode", v as i64)
    }

    // ---- 鍗板墠鏍囪 ----

    pub fn marks_to_page(&self) -> Option<bool> { self.prop_bool("MarksToPage") }
    pub fn set_marks_to_page(&self, v: bool) -> bool { self.put_bool("MarksToPage", v) }

    // ---- 娓愬彉姝ユ暟 ----

    pub fn fountain_steps(&self) -> Option<i64> { self.prop_i64("FountainSteps") }
    pub fn set_fountain_steps(&self, v: i32) -> bool { self.put_i64("FountainSteps", v as i64) }

    // ---- 鏍呮牸鍖?----

    pub fn rasterize_page(&self) -> Option<bool> { self.prop_bool("RasterizePage") }
    pub fn set_rasterize_page(&self, v: bool) -> bool { self.put_bool("RasterizePage", v) }

    pub fn rasterize_resolution(&self) -> Option<i64> { self.prop_i64("RasterizeResolution") }
    pub fn set_rasterize_resolution(&self, v: i32) -> bool {
        self.put_i64("RasterizeResolution", v as i64)
    }

    // ---- 闄嶉噰鏍?----

    pub fn downsample_color(&self) -> Option<bool> { self.prop_bool("DownsampleColor") }
    pub fn set_downsample_color(&self, v: bool) -> bool { self.put_bool("DownsampleColor", v) }

    pub fn downsample_gray(&self) -> Option<bool> { self.prop_bool("DownsampleGray") }
    pub fn set_downsample_gray(&self, v: bool) -> bool { self.put_bool("DownsampleGray", v) }

    pub fn downsample_mono(&self) -> Option<bool> { self.prop_bool("DownsampleMono") }
    pub fn set_downsample_mono(&self, v: bool) -> bool { self.put_bool("DownsampleMono", v) }

    pub fn color_resolution(&self) -> Option<i64> { self.prop_i64("ColorResolution") }
    pub fn set_color_resolution(&self, v: i32) -> bool { self.put_i64("ColorResolution", v as i64) }

    pub fn gray_resolution(&self) -> Option<i64> { self.prop_i64("GrayResolution") }
    pub fn set_gray_resolution(&self, v: i32) -> bool { self.put_i64("GrayResolution", v as i64) }

    pub fn mono_resolution(&self) -> Option<i64> { self.prop_i64("MonoResolution") }
    pub fn set_mono_resolution(&self, v: i32) -> bool { self.put_i64("MonoResolution", v as i64) }

    // ---- 鎵撳嵃浣滀笟淇℃伅 ----

    pub fn job_information(&self) -> Option<bool> { self.prop_bool("JobInformation") }
    pub fn set_job_information(&self, v: bool) -> bool { self.put_bool("JobInformation", v) }

    pub fn app_info(&self) -> Option<bool> { self.prop_bool("AppInfo") }
    pub fn set_app_info(&self, v: bool) -> bool { self.put_bool("AppInfo", v) }

    pub fn driver_info(&self) -> Option<bool> { self.prop_bool("DriverInfo") }
    pub fn set_driver_info(&self, v: bool) -> bool { self.put_bool("DriverInfo", v) }

    pub fn print_job_info(&self) -> Option<bool> { self.prop_bool("PrintJobInfo") }
    pub fn set_print_job_info(&self, v: bool) -> bool { self.put_bool("PrintJobInfo", v) }

    pub fn seps_info(&self) -> Option<bool> { self.prop_bool("SepsInfo") }
    pub fn set_seps_info(&self, v: bool) -> bool { self.put_bool("SepsInfo", v) }

    pub fn font_info(&self) -> Option<bool> { self.prop_bool("FontInfo") }
    pub fn set_font_info(&self, v: bool) -> bool { self.put_bool("FontInfo", v) }

    pub fn link_info(&self) -> Option<bool> { self.prop_bool("LinkInfo") }
    pub fn set_link_info(&self, v: bool) -> bool { self.put_bool("LinkInfo", v) }

    pub fn in_rip_trap_info(&self) -> Option<bool> { self.prop_bool("InRIPTrapInfo") }
    pub fn set_in_rip_trap_info(&self, v: bool) -> bool {
        self.put_bool("InRIPTrapInfo", v)
    }

    // ---- 淇濈暀绾粦 ----

    pub fn preserve_pure_black(&self) -> Option<bool> { self.prop_bool("PreservePureBlack") }
    pub fn set_preserve_pure_black(&self, v: bool) -> bool {
        self.put_bool("PreservePureBlack", v)
    }
}