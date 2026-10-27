//! `IVGStructImportOptions` —— 导入选项

use wincom::{ComObject, Variant};
use windows::Win32::System::Com::IDispatch;

/// CorelDRAW `IVGStructImportOptions` 对象的后期绑定包装（导入选项集）。
///
/// 所有属性与方法通过 `IDispatch::Invoke` 按名称调用，调用失败时一般返回 `None` 或 `false`。
pub struct IvgStructImportOptions {
    disp: ComObject,
}

impl IvgStructImportOptions {
    /// 由 COM `IDispatch` 构造包装对象。
    pub fn new(disp: IDispatch) -> Self {
        Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) }
    }

    /// 从 COM `IDispatch` 构造包装对象，等价于 `new`。
    pub fn from_disp(disp: IDispatch) -> Self {
        Self::new(disp)
    }

    /// 把底层 COM 对象包装成 `VARIANT`，用于传给其它 COM 调用。
    pub fn as_variant(&self) -> Variant {
        self.disp.as_variant()
    }

    /// 获取底层 `ComObject` 的引用。
    pub fn raw(&self) -> &ComObject {
        &self.disp
    }

    fn prop_dispatch(&self, name: &str) -> Option<IDispatch> {
        self.disp.get_property(name).ok()?.to_idispatch().ok()
    }

    fn prop_string(&self, name: &str) -> Option<String> {
        self.disp.get_property(name).ok()?.to_string().ok()
    }

    fn prop_bool(&self, name: &str) -> Option<bool> {
        self.disp.get_property(name).ok()?.to_bool().ok()
    }

    fn put_string(&self, name: &str, v: impl Into<String>) -> bool {
        let arg = Variant::from_str(v.into());
        self.disp.set_property(name, vec![arg]).is_ok()
    }

    fn put_bool(&self, name: &str, v: bool) -> bool {
        let arg = Variant::from_bool(v);
        self.disp.set_property(name, vec![arg]).is_ok()
    }

    /// 用于裁剪的回调（`IImportCropHandler`）。
    pub fn crop_handler(&self) -> Option<IDispatch> {
        self.prop_dispatch("CropHandler")
    }

    /// 设置裁剪处理器（`CropHandler`）。
    pub fn set_crop_handler(&self, h: IDispatch) -> bool {
        let arg = Variant::from_dispatch(&h);
        self.disp.set_property("CropHandler", vec![arg]).is_ok()
    }

    /// 用于重采样的回调（`IImportResampleHandler`）。
    pub fn resample_handler(&self) -> Option<IDispatch> {
        self.prop_dispatch("ResampleHandler")
    }

    /// 设置重采样处理器（`ResampleHandler`）。
    pub fn set_resample_handler(&self, h: IDispatch) -> bool {
        let arg = Variant::from_dispatch(&h);
        self.disp.set_property("ResampleHandler", vec![arg]).is_ok()
    }

    // ---- 颜色 ----

    /// 读取使用颜色配置文件（`UseColorProfile`）。
    pub fn use_color_profile(&self) -> Option<bool> { self.prop_bool("UseColorProfile") }
    /// 设置使用颜色配置文件（`UseColorProfile`）。
    pub fn set_use_color_profile(&self, v: bool) -> bool {
        self.put_bool("UseColorProfile", v)
    }

    /// 读取保持图层集（`MaintainLayers`）。
    pub fn maintain_layers(&self) -> Option<bool> { self.prop_bool("MaintainLayers") }
    /// 设置保持图层集（`MaintainLayers`）。
    pub fn set_maintain_layers(&self, v: bool) -> bool {
        self.put_bool("MaintainLayers", v)
    }

    /// 读取 COM 属性 `DetectWatermark`。
    pub fn detect_watermark(&self) -> Option<bool> { self.prop_bool("DetectWatermark") }
    /// 设置 COM 属性 `DetectWatermark`。
    pub fn set_detect_watermark(&self, v: bool) -> bool {
        self.put_bool("DetectWatermark", v)
    }

    /// 读取 COM 属性 `CombineMultipage`。
    pub fn combine_multipage(&self) -> Option<bool> { self.prop_bool("CombineMultipage") }
    /// 设置 COM 属性 `CombineMultipage`。
    pub fn set_combine_multipage(&self, v: bool) -> bool {
        self.put_bool("CombineMultipage", v)
    }

    /// 读取 COM 属性 `ExtractEmbeddedICC`。
    pub fn extract_embedded_icc(&self) -> Option<bool> {
        self.prop_bool("ExtractEmbeddedICC")
    }
    /// 设置 COM 属性 `ExtractEmbeddedICC`。
    pub fn set_extract_embedded_icc(&self, v: bool) -> bool {
        self.put_bool("ExtractEmbeddedICC", v)
    }

    /// 读取 COM 属性 `NameConflict`。
    pub fn name_conflict(&self) -> Option<i64> {
        self.disp.get_property("NameConflict").ok()?.to_i64().ok()
    }
    /// 设置 COM 属性 `NameConflict`。
    pub fn set_name_conflict(&self, v: i32) -> bool {
        let arg = Variant::from_i64(v as i64);
        self.disp.set_property("NameConflict", vec![arg]).is_ok()
    }

    /// 密码（用于加密 PDF 等）。
    pub fn password(&self) -> Option<String> { self.prop_string("Password") }
    /// 设置密码（`Password`）。
    pub fn set_password(&self, v: impl Into<String>) -> bool {
        self.put_string("Password", v)
    }
}