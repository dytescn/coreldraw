//! `IVGStructExportOptions` 鈥斺€?瀵煎嚭閫夐」

use wincom::{ComObject, Variant};
use windows::Win32::System::Com::IDispatch;

use crate::color::IvgColor;
use crate::geometry::IvgRect;

pub struct IvgStructExportOptions {
    disp: ComObject,
}

impl IvgStructExportOptions {
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

    fn prop_dispatch(&self, name: &str) -> Option<IDispatch> {
        self.disp.get_property(name).ok()?.to_idispatch().ok()
    }

    fn put_i64(&self, name: &str, v: i64) -> bool {
        let arg = Variant::from_i64(v);
        self.disp.set_property(name, vec![arg]).is_ok()
    }

    fn put_bool(&self, name: &str, v: bool) -> bool {
        let arg = Variant::from_bool(v);
        self.disp.set_property(name, vec![arg]).is_ok()
    }

    fn put_dispatch(&self, name: &str, v: Variant) -> bool {
        self.disp.set_property(name, vec![v]).is_ok()
    }

    // ---- 灏哄 / 鍒嗚鲸鐜?----

    pub fn size_x(&self) -> Option<i64> { self.prop_i64("SizeX") }
    pub fn set_size_x(&self, v: i32) -> bool { self.put_i64("SizeX", v as i64) }

    pub fn size_y(&self) -> Option<i64> { self.prop_i64("SizeY") }
    pub fn set_size_y(&self, v: i32) -> bool { self.put_i64("SizeY", v as i64) }

    pub fn resolution_x(&self) -> Option<i64> { self.prop_i64("ResolutionX") }
    pub fn set_resolution_x(&self, v: i32) -> bool { self.put_i64("ResolutionX", v as i64) }

    pub fn resolution_y(&self) -> Option<i64> { self.prop_i64("ResolutionY") }
    pub fn set_resolution_y(&self, v: i32) -> bool { self.put_i64("ResolutionY", v as i64) }

    // ---- 鎶楅敮榻?/ 鍥惧儚绫诲瀷 ----

    /// `cdrAntiAliasingType`
    pub fn anti_aliasing_type(&self) -> Option<i64> { self.prop_i64("AntiAliasingType") }
    pub fn set_anti_aliasing_type(&self, v: i32) -> bool {
        self.put_i64("AntiAliasingType", v as i64)
    }

    /// `cdrImageType`
    pub fn image_type(&self) -> Option<i64> { self.prop_i64("ImageType") }
    pub fn set_image_type(&self, v: i32) -> bool {
        self.put_i64("ImageType", v as i64)
    }

    /// `cdrCompressionType`
    pub fn compression(&self) -> Option<i64> { self.prop_i64("Compression") }
    pub fn set_compression(&self, v: i32) -> bool {
        self.put_i64("Compression", v as i64)
    }

    // ---- 鏍囧織浣?----

    pub fn overwrite(&self) -> Option<bool> { self.prop_bool("Overwrite") }
    pub fn set_overwrite(&self, v: bool) -> bool { self.put_bool("Overwrite", v) }

    pub fn dithered(&self) -> Option<bool> { self.prop_bool("Dithered") }
    pub fn set_dithered(&self, v: bool) -> bool { self.put_bool("Dithered", v) }

    pub fn transparent(&self) -> Option<bool> { self.prop_bool("Transparent") }
    pub fn set_transparent(&self, v: bool) -> bool { self.put_bool("Transparent", v) }

    pub fn use_color_profile(&self) -> Option<bool> { self.prop_bool("UseColorProfile") }
    pub fn set_use_color_profile(&self, v: bool) -> bool {
        self.put_bool("UseColorProfile", v)
    }

    pub fn maintain_layers(&self) -> Option<bool> { self.prop_bool("MaintainLayers") }
    pub fn set_maintain_layers(&self, v: bool) -> bool {
        self.put_bool("MaintainLayers", v)
    }

    pub fn maintain_aspect(&self) -> Option<bool> { self.prop_bool("MaintainAspect") }
    pub fn set_maintain_aspect(&self, v: bool) -> bool {
        self.put_bool("MaintainAspect", v)
    }

    pub fn matted(&self) -> Option<bool> { self.prop_bool("Matted") }
    pub fn set_matted(&self, v: bool) -> bool { self.put_bool("Matted", v) }

    pub fn matte_masked_only(&self) -> Option<bool> { self.prop_bool("MatteMaskedOnly") }
    pub fn set_matte_masked_only(&self, v: bool) -> bool {
        self.put_bool("MatteMaskedOnly", v)
    }

    pub fn always_overprint_black(&self) -> Option<bool> {
        self.prop_bool("AlwaysOverprintBlack")
    }
    pub fn set_always_overprint_black(&self, v: bool) -> bool {
        self.put_bool("AlwaysOverprintBlack", v)
    }

    // ---- 瀵煎嚭鍖哄煙 ----

    pub fn export_area(&self) -> Option<IvgRect> {
        self.prop_dispatch("ExportArea").map(IvgRect::new)
    }

    pub fn set_export_area(&self, r: &IvgRect) -> bool {
        self.put_dispatch("ExportArea", r.as_variant())
    }

    // ---- 閬僵鑹?----

    pub fn matte_color(&self) -> Option<IvgColor> {
        self.prop_dispatch("MatteColor").map(IvgColor::new)
    }
    pub fn set_matte_color(&self, c: &IvgColor) -> bool {
        self.put_dispatch("MatteColor", c.as_variant())
    }

    // ---- 鏍℃牱 ----

    pub fn proof_color_settings(&self) -> Option<crate::misc::IvgProperties> {
        // 瀹為檯鏄?IVGProofColorSettings锛屾殏鐢?properties 鍗犱綅
        self.prop_dispatch("ProofColorSettings").map(crate::misc::IvgProperties::new)
    }
}