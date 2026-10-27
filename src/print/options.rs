//! `IPrnVBAPrintOptions` —— 打印选项

use wincom::{ComObject, Variant};
use windows::Win32::System::Com::IDispatch;

/// CorelDRAW `IPrnVBAPrintOptions` 对象的后期绑定包装（选项集）。
///
/// 所有属性与方法通过 `IDispatch::Invoke` 按名称调用，调用失败时一般返回 `None` 或 `false`。
pub struct IvgPrnOptions {
    disp: ComObject,
}

impl IvgPrnOptions {
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

    fn put_i64(&self, name: &str, v: i64) -> bool {
        let arg = Variant::from_i64(v);
        self.disp.set_property(name, vec![arg]).is_ok()
    }

    fn put_bool(&self, name: &str, v: bool) -> bool {
        let arg = Variant::from_bool(v);
        self.disp.set_property(name, vec![arg]).is_ok()
    }

    // ---- 使用色彩配置文件 ----

    /// 读取使用颜色配置文件（`UseColorProfile`）。
    pub fn use_color_profile(&self) -> Option<bool> { self.prop_bool("UseColorProfile") }
    /// 设置使用颜色配置文件（`UseColorProfile`）。
    pub fn set_use_color_profile(&self, v: bool) -> bool {
        self.put_bool("UseColorProfile", v)
    }

    // ---- 打印内容 ----

    /// 读取打印向量集（`PrintVectors`）。
    pub fn print_vectors(&self) -> Option<bool> { self.prop_bool("PrintVectors") }
    /// 设置打印向量集（`PrintVectors`）。
    pub fn set_print_vectors(&self, v: bool) -> bool { self.put_bool("PrintVectors", v) }

    /// 读取打印位图集（`PrintBitmaps`）。
    pub fn print_bitmaps(&self) -> Option<bool> { self.prop_bool("PrintBitmaps") }
    /// 设置打印位图集（`PrintBitmaps`）。
    pub fn set_print_bitmaps(&self, v: bool) -> bool { self.put_bool("PrintBitmaps", v) }

    /// 读取打印文本（`PrintText`）。
    pub fn print_text(&self) -> Option<bool> { self.prop_bool("PrintText") }
    /// 设置打印文本（`PrintText`）。
    pub fn set_print_text(&self, v: bool) -> bool { self.put_bool("PrintText", v) }

    /// 读取黑色中的文本（`TextInBlack`）。
    pub fn text_in_black(&self) -> Option<bool> { self.prop_bool("TextInBlack") }
    /// 设置黑色中的文本（`TextInBlack`）。
    pub fn set_text_in_black(&self, v: bool) -> bool { self.put_bool("TextInBlack", v) }

    /// `PrnColorMode`
    pub fn color_mode(&self) -> Option<i64> { self.prop_i64("ColorMode") }
    /// 设置颜色模式（`ColorMode`）。
    pub fn set_color_mode(&self, v: i32) -> bool { self.put_i64("ColorMode", v as i64) }

    /// `PrnBitmapColorMode`
    pub fn bitmap_color_mode(&self) -> Option<i64> { self.prop_i64("BitmapColorMode") }
    /// 设置位图颜色模式（`BitmapColorMode`）。
    pub fn set_bitmap_color_mode(&self, v: i32) -> bool {
        self.put_i64("BitmapColorMode", v as i64)
    }

    /// `PrnObjectsColorMode`
    pub fn objects_color_mode(&self) -> Option<i64> { self.prop_i64("ObjectsColorMode") }
    /// 设置对象集颜色模式（`ObjectsColorMode`）。
    pub fn set_objects_color_mode(&self, v: i32) -> bool {
        self.put_i64("ObjectsColorMode", v as i64)
    }

    // ---- 印前标记 ----

    /// 读取标记页面（`MarksToPage`）。
    pub fn marks_to_page(&self) -> Option<bool> { self.prop_bool("MarksToPage") }
    /// 设置标记页面（`MarksToPage`）。
    pub fn set_marks_to_page(&self, v: bool) -> bool { self.put_bool("MarksToPage", v) }

    // ---- 渐变步数 ----

    /// 读取渐变步数（`FountainSteps`）。
    pub fn fountain_steps(&self) -> Option<i64> { self.prop_i64("FountainSteps") }
    /// 设置渐变步数（`FountainSteps`）。
    pub fn set_fountain_steps(&self, v: i32) -> bool { self.put_i64("FountainSteps", v as i64) }

    // ---- 栅格化----

    /// 读取栅格化页面（`RasterizePage`）。
    pub fn rasterize_page(&self) -> Option<bool> { self.prop_bool("RasterizePage") }
    /// 设置栅格化页面（`RasterizePage`）。
    pub fn set_rasterize_page(&self, v: bool) -> bool { self.put_bool("RasterizePage", v) }

    /// 读取栅格化分辨率（`RasterizeResolution`）。
    pub fn rasterize_resolution(&self) -> Option<i64> { self.prop_i64("RasterizeResolution") }
    /// 设置栅格化分辨率（`RasterizeResolution`）。
    pub fn set_rasterize_resolution(&self, v: i32) -> bool {
        self.put_i64("RasterizeResolution", v as i64)
    }

    // ---- 降采样----

    /// 读取降采样颜色（`DownsampleColor`）。
    pub fn downsample_color(&self) -> Option<bool> { self.prop_bool("DownsampleColor") }
    /// 设置降采样颜色（`DownsampleColor`）。
    pub fn set_downsample_color(&self, v: bool) -> bool { self.put_bool("DownsampleColor", v) }

    /// 读取降采样灰度（`DownsampleGray`）。
    pub fn downsample_gray(&self) -> Option<bool> { self.prop_bool("DownsampleGray") }
    /// 设置降采样灰度（`DownsampleGray`）。
    pub fn set_downsample_gray(&self, v: bool) -> bool { self.put_bool("DownsampleGray", v) }

    /// 读取降采样单色（`DownsampleMono`）。
    pub fn downsample_mono(&self) -> Option<bool> { self.prop_bool("DownsampleMono") }
    /// 设置降采样单色（`DownsampleMono`）。
    pub fn set_downsample_mono(&self, v: bool) -> bool { self.put_bool("DownsampleMono", v) }

    /// 读取颜色分辨率（`ColorResolution`）。
    pub fn color_resolution(&self) -> Option<i64> { self.prop_i64("ColorResolution") }
    /// 设置颜色分辨率（`ColorResolution`）。
    pub fn set_color_resolution(&self, v: i32) -> bool { self.put_i64("ColorResolution", v as i64) }

    /// 读取灰度分辨率（`GrayResolution`）。
    pub fn gray_resolution(&self) -> Option<i64> { self.prop_i64("GrayResolution") }
    /// 设置灰度分辨率（`GrayResolution`）。
    pub fn set_gray_resolution(&self, v: i32) -> bool { self.put_i64("GrayResolution", v as i64) }

    /// 读取单色分辨率（`MonoResolution`）。
    pub fn mono_resolution(&self) -> Option<i64> { self.prop_i64("MonoResolution") }
    /// 设置单色分辨率（`MonoResolution`）。
    pub fn set_mono_resolution(&self, v: i32) -> bool { self.put_i64("MonoResolution", v as i64) }

    // ---- 打印作业信息 ----

    /// 读取任务信息（`JobInformation`）。
    pub fn job_information(&self) -> Option<bool> { self.prop_bool("JobInformation") }
    /// 设置任务信息（`JobInformation`）。
    pub fn set_job_information(&self, v: bool) -> bool { self.put_bool("JobInformation", v) }

    /// 读取应用信息（`AppInfo`）。
    pub fn app_info(&self) -> Option<bool> { self.prop_bool("AppInfo") }
    /// 设置应用信息（`AppInfo`）。
    pub fn set_app_info(&self, v: bool) -> bool { self.put_bool("AppInfo", v) }

    /// 读取 COM 属性 `DriverInfo`。
    pub fn driver_info(&self) -> Option<bool> { self.prop_bool("DriverInfo") }
    /// 设置 COM 属性 `DriverInfo`。
    pub fn set_driver_info(&self, v: bool) -> bool { self.put_bool("DriverInfo", v) }

    /// 读取打印任务信息（`PrintJobInfo`）。
    pub fn print_job_info(&self) -> Option<bool> { self.prop_bool("PrintJobInfo") }
    /// 设置打印任务信息（`PrintJobInfo`）。
    pub fn set_print_job_info(&self, v: bool) -> bool { self.put_bool("PrintJobInfo", v) }

    /// 读取 COM 属性 `SepsInfo`。
    pub fn seps_info(&self) -> Option<bool> { self.prop_bool("SepsInfo") }
    /// 设置 COM 属性 `SepsInfo`。
    pub fn set_seps_info(&self, v: bool) -> bool { self.put_bool("SepsInfo", v) }

    /// 读取字体信息（`FontInfo`）。
    pub fn font_info(&self) -> Option<bool> { self.prop_bool("FontInfo") }
    /// 设置字体信息（`FontInfo`）。
    pub fn set_font_info(&self, v: bool) -> bool { self.put_bool("FontInfo", v) }

    /// 读取链接信息（`LinkInfo`）。
    pub fn link_info(&self) -> Option<bool> { self.prop_bool("LinkInfo") }
    /// 设置链接信息（`LinkInfo`）。
    pub fn set_link_info(&self, v: bool) -> bool { self.put_bool("LinkInfo", v) }

    /// 读取 COM 属性 `InRIPTrapInfo`。
    pub fn in_rip_trap_info(&self) -> Option<bool> { self.prop_bool("InRIPTrapInfo") }
    /// 设置 COM 属性 `InRIPTrapInfo`。
    pub fn set_in_rip_trap_info(&self, v: bool) -> bool {
        self.put_bool("InRIPTrapInfo", v)
    }

    // ---- 保留纯黑 ----

    /// 读取保留纯黑色（`PreservePureBlack`）。
    pub fn preserve_pure_black(&self) -> Option<bool> { self.prop_bool("PreservePureBlack") }
    /// 设置保留纯黑色（`PreservePureBlack`）。
    pub fn set_preserve_pure_black(&self, v: bool) -> bool {
        self.put_bool("PreservePureBlack", v)
    }
}