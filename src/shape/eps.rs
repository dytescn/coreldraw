//! `IVGEPS` 鈥斺€?EPS 鍥惧舰

use wincom::{ComObject, Variant};
use windows::Win32::System::Com::IDispatch;

use crate::curve::IvgCurve;
use crate::shape::IvgBitmap;

pub struct IvgEps {
    disp: ComObject,
}

impl IvgEps {
    pub fn new(disp: IDispatch) -> Self {
        Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) }
    }

    pub fn from_disp(disp: IDispatch) -> Self { Self::new(disp) }

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

    pub fn preview_bitmap(&self) -> Option<IvgBitmap> {
        self.prop_dispatch("PreviewBitmap").map(IvgBitmap::new)
    }

    pub fn data(&self) -> Option<Variant> {
        self.disp.get_property("Data").ok()
    }

    pub fn data_as_string(&self) -> Option<String> {
        self.prop_string("DataAsString")
    }

    pub fn crop_envelope(&self) -> Option<IvgCurve> {
        self.prop_dispatch("CropEnvelope").map(IvgCurve::new)
    }

    pub fn reset_crop_envelope(&self) -> bool {
        self.disp.invoke_method("ResetCropEnvelope", vec![]).is_ok()
    }

    pub fn crop_envelope_modified(&self) -> Option<bool> {
        self.prop_bool("CropEnvelopeModified")
    }

    pub fn bounding_box_path(&self) -> Option<IvgCurve> {
        self.prop_dispatch("BoundingBoxPath").map(IvgCurve::new)
    }

    pub fn link_file_name(&self) -> Option<String> { self.prop_string("LinkFileName") }
    pub fn set_link_file_name(&self, v: impl Into<String>) -> bool {
        self.put_string("LinkFileName", v)
    }

    pub fn dcs_file_names(&self) -> Option<Variant> {
        self.disp.get_property("DCSFileNames").ok()
    }
}