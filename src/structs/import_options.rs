//! `IVGStructImportOptions` 鈥斺€?瀵煎叆閫夐」

use wincom::{ComObject, Variant};
use windows::Win32::System::Com::IDispatch;

pub struct IvgStructImportOptions {
    disp: ComObject,
}

impl IvgStructImportOptions {
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

    /// 鐢ㄤ簬瑁佸壀鐨勫洖璋冿紙`IImportCropHandler`锛夈€?
    pub fn crop_handler(&self) -> Option<IDispatch> {
        self.prop_dispatch("CropHandler")
    }

    pub fn set_crop_handler(&self, h: IDispatch) -> bool {
        let arg = Variant::from_dispatch(&h);
        self.disp.set_property("CropHandler", vec![arg]).is_ok()
    }

    /// 鐢ㄤ簬閲嶉噰鏍风殑鍥炶皟锛坄IImportResampleHandler`锛夈€?
    pub fn resample_handler(&self) -> Option<IDispatch> {
        self.prop_dispatch("ResampleHandler")
    }

    pub fn set_resample_handler(&self, h: IDispatch) -> bool {
        let arg = Variant::from_dispatch(&h);
        self.disp.set_property("ResampleHandler", vec![arg]).is_ok()
    }

    // ---- 棰滆壊 ----

    pub fn use_color_profile(&self) -> Option<bool> { self.prop_bool("UseColorProfile") }
    pub fn set_use_color_profile(&self, v: bool) -> bool {
        self.put_bool("UseColorProfile", v)
    }

    pub fn maintain_layers(&self) -> Option<bool> { self.prop_bool("MaintainLayers") }
    pub fn set_maintain_layers(&self, v: bool) -> bool {
        self.put_bool("MaintainLayers", v)
    }

    pub fn detect_watermark(&self) -> Option<bool> { self.prop_bool("DetectWatermark") }
    pub fn set_detect_watermark(&self, v: bool) -> bool {
        self.put_bool("DetectWatermark", v)
    }

    pub fn combine_multipage(&self) -> Option<bool> { self.prop_bool("CombineMultipage") }
    pub fn set_combine_multipage(&self, v: bool) -> bool {
        self.put_bool("CombineMultipage", v)
    }

    pub fn extract_embedded_icc(&self) -> Option<bool> {
        self.prop_bool("ExtractEmbeddedICC")
    }
    pub fn set_extract_embedded_icc(&self, v: bool) -> bool {
        self.put_bool("ExtractEmbeddedICC", v)
    }

    pub fn name_conflict(&self) -> Option<i64> {
        self.disp.get_property("NameConflict").ok()?.to_i64().ok()
    }
    pub fn set_name_conflict(&self, v: i32) -> bool {
        let arg = Variant::from_i64(v as i64);
        self.disp.set_property("NameConflict", vec![arg]).is_ok()
    }

    /// 瀵嗙爜锛堢敤浜庡姞瀵?PDF 绛夛級銆?
    pub fn password(&self) -> Option<String> { self.prop_string("Password") }
    pub fn set_password(&self, v: impl Into<String>) -> bool {
        self.put_string("Password", v)
    }
}