//! `IVGCloneLink` 鈥斺€?鍥惧舰鍏嬮殕閾?

use wincom::{ComObject, Variant};
use windows::Win32::System::Com::IDispatch;

use crate::shape::IvgShape;

pub struct IvgCloneLink {
    disp: ComObject,
}

impl IvgCloneLink {
    pub fn new(disp: IDispatch) -> Self {
        Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) }
    }

    pub fn from_disp(disp: IDispatch) -> Self {
        Self::new(disp)
    }

    fn prop_bool(&self, name: &str) -> Option<bool> {
        self.disp.get_property(name).ok()?.to_bool().ok()
    }

    fn prop_dispatch(&self, name: &str) -> Option<IDispatch> {
        self.disp.get_property(name).ok()?.to_idispatch().ok()
    }

    fn put_bool(&self, name: &str, v: bool) -> bool {
        let arg = Variant::from_bool(v);
        self.disp.set_property(name, vec![arg]).is_ok()
    }

    pub fn clone_parent(&self) -> Option<IvgShape> {
        self.prop_dispatch("CloneParent").map(IvgShape::new)
    }

    pub fn fill_linked(&self) -> Option<bool> { self.prop_bool("FillLinked") }
    pub fn set_fill_linked(&self, v: bool) -> bool { self.put_bool("FillLinked", v) }

    pub fn outline_linked(&self) -> Option<bool> { self.prop_bool("OutlineLinked") }
    pub fn set_outline_linked(&self, v: bool) -> bool {
        self.put_bool("OutlineLinked", v)
    }

    pub fn shape_linked(&self) -> Option<bool> { self.prop_bool("ShapeLinked") }
    pub fn set_shape_linked(&self, v: bool) -> bool { self.put_bool("ShapeLinked", v) }

    pub fn transform_linked(&self) -> Option<bool> { self.prop_bool("TransformLinked") }
    pub fn set_transform_linked(&self, v: bool) -> bool {
        self.put_bool("TransformLinked", v)
    }

    pub fn bitmap_color_mask_linked(&self) -> Option<bool> {
        self.prop_bool("BitmapColorMaskLinked")
    }
    pub fn set_bitmap_color_mask_linked(&self, v: bool) -> bool {
        self.put_bool("BitmapColorMaskLinked", v)
    }

    pub fn restore_all_links(&self) -> bool {
        self.disp.invoke_method("RestoreAllLinks", vec![]).is_ok()
    }
}