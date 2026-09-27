//! `IVGPowerClip` 鈥斺€?鍥炬绮剧‘瑁佸壀

use wincom::{ComObject, Variant};
use windows::Win32::System::Com::IDispatch;

use crate::shape::{IvgShapeRange, IvgShapes};

pub struct IvgPowerClip {
    disp: ComObject,
}

impl IvgPowerClip {
    pub fn new(disp: IDispatch) -> Self {
        Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) }
    }

    pub fn from_disp(disp: IDispatch) -> Self { Self::new(disp) }

    fn prop_dispatch(&self, name: &str) -> Option<IDispatch> {
        self.disp.get_property(name).ok()?.to_idispatch().ok()
    }

    fn prop_bool(&self, name: &str) -> Option<bool> {
        self.disp.get_property(name).ok()?.to_bool().ok()
    }

    fn put_bool(&self, name: &str, v: bool) -> bool {
        let arg = Variant::from_bool(v);
        self.disp.set_property(name, vec![arg]).is_ok()
    }

    pub fn shapes(&self) -> Option<IvgShapes> {
        self.prop_dispatch("Shapes").map(IvgShapes::new)
    }

    pub fn contents_locked(&self) -> Option<bool> { self.prop_bool("ContentsLocked") }
    pub fn set_contents_locked(&self, v: bool) -> bool {
        self.put_bool("ContentsLocked", v)
    }

    pub fn enter_edit_mode(&self) -> bool {
        self.disp.invoke_method("EnterEditMode", vec![]).is_ok()
    }

    pub fn leave_edit_mode(&self) -> bool {
        self.disp.invoke_method("LeaveEditMode", vec![]).is_ok()
    }

    pub fn extract_shapes(&self) -> Option<IvgShapeRange> {
        self.disp
            .invoke_method("ExtractShapes", vec![])
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgShapeRange::new)
    }
}