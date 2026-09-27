//! `IPrnVBAPrintPrepress` 鈥斺€?鍗板墠璁剧疆锛堟爣璁?/ 鎷肩増锛?

use wincom::{ComObject, Variant};
use windows::Win32::System::Com::IDispatch;

pub struct IvgPrnPrepress {
    disp: ComObject,
}

impl IvgPrnPrepress {
    pub fn new(disp: IDispatch) -> Self {
        Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) }
    }

    fn prop_i64(&self, name: &str) -> Option<i64> {
        self.disp.get_property(name).ok()?.to_i64().ok()
    }

    fn prop_bool(&self, name: &str) -> Option<bool> {
        self.disp.get_property(name).ok()?.to_bool().ok()
    }

    fn prop_string(&self, name: &str) -> Option<String> {
        self.disp.get_property(name).ok()?.to_string().ok()
    }

    fn put_i64(&self, name: &str, v: i64) -> bool {
        let arg = Variant::from_i64(v);
        self.disp.set_property(name, vec![arg]).is_ok()
    }

    fn put_bool(&self, name: &str, v: bool) -> bool {
        let arg = Variant::from_bool(v);
        self.disp.set_property(name, vec![arg]).is_ok()
    }

    fn put_string(&self, name: &str, v: impl Into<String>) -> bool {
        let arg = Variant::from_str(v.into());
        self.disp.set_property(name, vec![arg]).is_ok()
    }

    // ---- 缈昏浆 / 闀滃儚 ----

    pub fn invert(&self) -> Option<bool> { self.prop_bool("Invert") }
    pub fn set_invert(&self, v: bool) -> bool { self.put_bool("Invert", v) }

    pub fn mirror(&self) -> Option<bool> { self.prop_bool("Mirror") }
    pub fn set_mirror(&self, v: bool) -> bool { self.put_bool("Mirror", v) }

    // ---- 浣滀笟淇℃伅 ----

    pub fn file_info(&self) -> Option<bool> { self.prop_bool("FileInfo") }
    pub fn set_file_info(&self, v: bool) -> bool { self.put_bool("FileInfo", v) }

    pub fn job_name(&self) -> Option<String> { self.prop_string("JobName") }
    pub fn set_job_name(&self, v: impl Into<String>) -> bool {
        self.put_string("JobName", v)
    }

    pub fn page_numbers(&self) -> Option<bool> { self.prop_bool("PageNumbers") }
    pub fn set_page_numbers(&self, v: bool) -> bool { self.put_bool("PageNumbers", v) }

    pub fn info_within_page(&self) -> Option<bool> { self.prop_bool("InfoWithinPage") }
    pub fn set_info_within_page(&self, v: bool) -> bool {
        self.put_bool("InfoWithinPage", v)
    }

    // ---- 瑁佸垏鏍囪 ----

    pub fn crop_marks(&self) -> Option<bool> { self.prop_bool("CropMarks") }
    pub fn set_crop_marks(&self, v: bool) -> bool { self.put_bool("CropMarks", v) }

    pub fn exterior_crop_marks(&self) -> Option<bool> { self.prop_bool("ExteriorCropMarks") }
    pub fn set_exterior_crop_marks(&self, v: bool) -> bool {
        self.put_bool("ExteriorCropMarks", v)
    }

    // ---- 濂楀噯鏍囪 ----

    pub fn registration_marks(&self) -> Option<bool> { self.prop_bool("RegistrationMarks") }
    pub fn set_registration_marks(&self, v: bool) -> bool {
        self.put_bool("RegistrationMarks", v)
    }

    /// `PrnRegistrationStyle`
    pub fn registration_style(&self) -> Option<i64> { self.prop_i64("RegistrationStyle") }
    pub fn set_registration_style(&self, v: i32) -> bool {
        self.put_i64("RegistrationStyle", v as i64)
    }

    // ---- 鑹叉爣 / 瀵嗗害璁?----

    pub fn color_calibration_bar(&self) -> Option<bool> {
        self.prop_bool("ColorCalibrationBar")
    }
    pub fn set_color_calibration_bar(&self, v: bool) -> bool {
        self.put_bool("ColorCalibrationBar", v)
    }

    pub fn densitometer_scale(&self) -> Option<bool> { self.prop_bool("DensitometerScale") }
    pub fn set_densitometer_scale(&self, v: bool) -> bool {
        self.put_bool("DensitometerScale", v)
    }

    /// `Densities(Index)` 鈥斺€?0..N 鐨勫瘑搴﹀€笺€?
    pub fn densities(&self, index: i32) -> Option<i64> {
        let args = vec![Variant::from_i64(index as i64)];
        self.disp.invoke_method("Densities", args).ok()?.to_i64().ok()
    }

    pub fn set_densities(&self, index: i32, v: i32) -> bool {
        let args = vec![
            Variant::from_i64(index as i64),
            Variant::from_i64(v as i64),
        ];
        self.disp.invoke_method("put_Densities", args).is_ok()
    }

    // ---- 鏍囪鍒板璞?----

    pub fn marks_to_objects(&self) -> Option<bool> { self.prop_bool("MarksToObjects") }
    pub fn set_marks_to_objects(&self, v: bool) -> bool {
        self.put_bool("MarksToObjects", v)
    }
}