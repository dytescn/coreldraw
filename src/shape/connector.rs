//! `IVGConnector` 鈥斺€?杩炴帴鍣?

use wincom::{ComObject, Variant};
use windows::Win32::System::Com::IDispatch;

use crate::geometry::IvgSnapPoint;

pub struct IvgConnector {
    disp: ComObject,
}

impl IvgConnector {
    pub fn new(disp: IDispatch) -> Self {
        Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) }
    }

    pub fn from_disp(disp: IDispatch) -> Self { Self::new(disp) }

    fn prop_i64(&self, name: &str) -> Option<i64> {
        self.disp.get_property(name).ok()?.to_i64().ok()
    }

    fn prop_dispatch(&self, name: &str) -> Option<IDispatch> {
        self.disp.get_property(name).ok()?.to_idispatch().ok()
    }

    fn put_dispatch(&self, name: &str, v: Variant) -> bool {
        self.disp.set_property(name, vec![v]).is_ok()
    }

    pub fn start_point(&self) -> Option<IvgSnapPoint> {
        self.prop_dispatch("StartPoint").map(IvgSnapPoint::new)
    }
    pub fn set_start_point(&self, p: &IvgSnapPoint) -> bool {
        self.put_dispatch("StartPoint", p.as_variant())
    }

    pub fn end_point(&self) -> Option<IvgSnapPoint> {
        self.prop_dispatch("EndPoint").map(IvgSnapPoint::new)
    }
    pub fn set_end_point(&self, p: &IvgSnapPoint) -> bool {
        self.put_dispatch("EndPoint", p.as_variant())
    }

    /// `cdrConnectorType`
    pub fn connector_type(&self) -> Option<i64> { self.prop_i64("Type") }
}