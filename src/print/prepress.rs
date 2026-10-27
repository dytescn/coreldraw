//! `IPrnVBAPrintPrepress` —— 印前设置（标记/ 拼版）

use wincom::{ComObject, Variant};
use windows::Win32::System::Com::IDispatch;

/// CorelDRAW `IPrnVBAPrintPrepress` 对象的后期绑定包装（印前）。
///
/// 所有属性与方法通过 `IDispatch::Invoke` 按名称调用，调用失败时一般返回 `None` 或 `false`。
pub struct IvgPrnPrepress {
    disp: ComObject,
}

impl IvgPrnPrepress {
    /// 由 COM `IDispatch` 构造包装对象。
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

    // ---- 翻转 / 镜像 ----

    /// 读取反转（`Invert`）。
    pub fn invert(&self) -> Option<bool> { self.prop_bool("Invert") }
    /// 设置反转（`Invert`）。
    pub fn set_invert(&self, v: bool) -> bool { self.put_bool("Invert", v) }

    /// 读取镜像（`Mirror`）。
    pub fn mirror(&self) -> Option<bool> { self.prop_bool("Mirror") }
    /// 设置镜像（`Mirror`）。
    pub fn set_mirror(&self, v: bool) -> bool { self.put_bool("Mirror", v) }

    // ---- 作业信息 ----

    /// 读取文件信息（`FileInfo`）。
    pub fn file_info(&self) -> Option<bool> { self.prop_bool("FileInfo") }
    /// 设置文件信息（`FileInfo`）。
    pub fn set_file_info(&self, v: bool) -> bool { self.put_bool("FileInfo", v) }

    /// 读取任务名称（`JobName`）。
    pub fn job_name(&self) -> Option<String> { self.prop_string("JobName") }
    /// 设置任务名称（`JobName`）。
    pub fn set_job_name(&self, v: impl Into<String>) -> bool {
        self.put_string("JobName", v)
    }

    /// 读取 COM 属性 `PageNumbers`。
    pub fn page_numbers(&self) -> Option<bool> { self.prop_bool("PageNumbers") }
    /// 设置 COM 属性 `PageNumbers`。
    pub fn set_page_numbers(&self, v: bool) -> bool { self.put_bool("PageNumbers", v) }

    /// 读取 COM 属性 `InfoWithinPage`。
    pub fn info_within_page(&self) -> Option<bool> { self.prop_bool("InfoWithinPage") }
    /// 设置 COM 属性 `InfoWithinPage`。
    pub fn set_info_within_page(&self, v: bool) -> bool {
        self.put_bool("InfoWithinPage", v)
    }

    // ---- 裁切标记 ----

    /// 读取裁剪标记（`CropMarks`）。
    pub fn crop_marks(&self) -> Option<bool> { self.prop_bool("CropMarks") }
    /// 设置裁剪标记（`CropMarks`）。
    pub fn set_crop_marks(&self, v: bool) -> bool { self.put_bool("CropMarks", v) }

    /// 读取 COM 属性 `ExteriorCropMarks`。
    pub fn exterior_crop_marks(&self) -> Option<bool> { self.prop_bool("ExteriorCropMarks") }
    /// 设置 COM 属性 `ExteriorCropMarks`。
    pub fn set_exterior_crop_marks(&self, v: bool) -> bool {
        self.put_bool("ExteriorCropMarks", v)
    }

    // ---- 套准标记 ----

    /// 读取套准标记（`RegistrationMarks`）。
    pub fn registration_marks(&self) -> Option<bool> { self.prop_bool("RegistrationMarks") }
    /// 设置套准标记（`RegistrationMarks`）。
    pub fn set_registration_marks(&self, v: bool) -> bool {
        self.put_bool("RegistrationMarks", v)
    }

    /// `PrnRegistrationStyle`
    pub fn registration_style(&self) -> Option<i64> { self.prop_i64("RegistrationStyle") }
    /// 设置套准样式（`RegistrationStyle`）。
    pub fn set_registration_style(&self, v: i32) -> bool {
        self.put_i64("RegistrationStyle", v as i64)
    }

    // ---- 色标 / 密度计----

    /// 读取颜色校准栏（`ColorCalibrationBar`）。
    pub fn color_calibration_bar(&self) -> Option<bool> {
        self.prop_bool("ColorCalibrationBar")
    }
    /// 设置颜色校准栏（`ColorCalibrationBar`）。
    pub fn set_color_calibration_bar(&self, v: bool) -> bool {
        self.put_bool("ColorCalibrationBar", v)
    }

    /// 读取密度计缩放（`DensitometerScale`）。
    pub fn densitometer_scale(&self) -> Option<bool> { self.prop_bool("DensitometerScale") }
    /// 设置密度计缩放（`DensitometerScale`）。
    pub fn set_densitometer_scale(&self, v: bool) -> bool {
        self.put_bool("DensitometerScale", v)
    }

    /// `Densities(Index)` ——0..N 的密度值。
    pub fn densities(&self, index: i32) -> Option<i64> {
        let args = vec![Variant::from_i64(index as i64)];
        self.disp.invoke_method("Densities", args).ok()?.to_i64().ok()
    }

    /// 设置 COM 属性 `put_Densities`。
    pub fn set_densities(&self, index: i32, v: i32) -> bool {
        let args = vec![
            Variant::from_i64(index as i64),
            Variant::from_i64(v as i64),
        ];
        self.disp.invoke_method("put_Densities", args).is_ok()
    }

    // ---- 标记到对象----

    /// 读取标记对象集（`MarksToObjects`）。
    pub fn marks_to_objects(&self) -> Option<bool> { self.prop_bool("MarksToObjects") }
    /// 设置标记对象集（`MarksToObjects`）。
    pub fn set_marks_to_objects(&self, v: bool) -> bool {
        self.put_bool("MarksToObjects", v)
    }
}