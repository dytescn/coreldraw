//! `IVGStructSaveAsOptions` 鈥斺€?鍙﹀瓨涓洪€夐」

use wincom::{ComObject, Variant};
use windows::Win32::System::Com::IDispatch;

pub struct IvgStructSaveAsOptions {
    disp: ComObject,
}

impl IvgStructSaveAsOptions {
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

    /// `cdrFilter` 鈥斺€?鏂囦欢绫诲瀷銆?
    pub fn filter(&self) -> Option<i64> { self.prop_i64("Filter") }
    pub fn set_filter(&self, v: i32) -> bool { self.put_i64("Filter", v as i64) }

    /// `cdrFileVersion` 鈥斺€?鏂囦欢鐗堟湰銆?
    pub fn version(&self) -> Option<i64> { self.prop_i64("Version") }
    pub fn set_version(&self, v: i32) -> bool { self.put_i64("Version", v as i64) }

    /// `cdrThumbnailSize` 鈥斺€?缂╃暐鍥惧ぇ灏忋€?
    pub fn thumbnail_size(&self) -> Option<i64> { self.prop_i64("ThumbnailSize") }
    pub fn set_thumbnail_size(&self, v: i32) -> bool {
        self.put_i64("ThumbnailSize", v as i64)
    }

    /// `cdrExportRange` 鈥斺€?瀵煎嚭鑼冨洿銆?
    pub fn range(&self) -> Option<i64> { self.prop_i64("Range") }
    pub fn set_range(&self, v: i32) -> bool { self.put_i64("Range", v as i64) }

    pub fn overwrite(&self) -> Option<bool> { self.prop_bool("Overwrite") }
    pub fn set_overwrite(&self, v: bool) -> bool { self.put_bool("Overwrite", v) }

    pub fn embed_icc_profile(&self) -> Option<bool> { self.prop_bool("EmbedICCProfile") }
    pub fn set_embed_icc_profile(&self, v: bool) -> bool {
        self.put_bool("EmbedICCProfile", v)
    }

    pub fn embed_vba_project(&self) -> Option<bool> { self.prop_bool("EmbedVBAProject") }
    pub fn set_embed_vba_project(&self, v: bool) -> bool {
        self.put_bool("EmbedVBAProject", v)
    }

    pub fn include_cmx_data(&self) -> Option<bool> { self.prop_bool("IncludeCMXData") }
    pub fn set_include_cmx_data(&self, v: bool) -> bool {
        self.put_bool("IncludeCMXData", v)
    }

    pub fn keep_appearance(&self) -> Option<bool> { self.prop_bool("KeepAppearance") }
    pub fn set_keep_appearance(&self, v: bool) -> bool {
        self.put_bool("KeepAppearance", v)
    }
}