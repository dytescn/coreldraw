//! `IVGShapes` 鈥斺€?鍥惧舰闆嗗悎

use wincom::{ComObject, Variant};
use windows::Win32::System::Com::IDispatch;

use crate::shape::{IvgShape, IvgShapeRange};

pub struct IvgShapes {
    disp: ComObject,
}

impl IvgShapes {
    pub fn new(disp: IDispatch) -> Self {
        Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) }
    }

    pub fn from_disp(disp: IDispatch) -> Self {
        Self::new(disp)
    }

    pub fn raw(&self) -> &ComObject {
        &self.disp
    }

    fn prop_i64(&self, name: &str) -> Option<i64> {
        self.disp.get_property(name).ok()?.to_i64().ok()
    }

    fn prop_dispatch(&self, name: &str) -> Option<IDispatch> {
        self.disp.get_property(name).ok()?.to_idispatch().ok()
    }

    pub fn count(&self) -> Option<i64> { self.prop_i64("Count") }

    pub fn item(&self, index_or_name: Variant) -> Option<IvgShape> {
        let args = vec![index_or_name];
        self.disp
            .invoke_method("Item", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgShape::new)
    }

    pub fn item_by_index(&self, i: i32) -> Option<IvgShape> {
        self.item(Variant::from_i64(i as i64))
    }

    pub fn item_by_name(&self, name: impl Into<String>) -> Option<IvgShape> {
        self.item(Variant::from_str(name.into()))
    }

    pub fn first(&self) -> Option<IvgShape> { self.prop_dispatch("First").map(IvgShape::new) }
    pub fn last(&self) -> Option<IvgShape> { self.prop_dispatch("Last").map(IvgShape::new) }

    pub fn all(&self) -> Vec<IvgShape> {
        let n = self.count().unwrap_or(0) as i32;
        (1..=n).filter_map(|i| self.item_by_index(i)).collect()
    }

    /// 鐢ㄧ储寮曟暟缁勫彇瀛愯寖鍥淬€?
    pub fn range(&self, indices: Variant) -> Option<IvgShapeRange> {
        let args = vec![indices];
        self.disp
            .invoke_method("Range", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgShapeRange::new)
    }

    /// 鍏ㄩ儴鍥惧舰锛堣繑鍥炶寖鍥达級銆?
    pub fn all_range(&self) -> Option<IvgShapeRange> {
        self.disp
            .invoke_method("All", vec![])
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgShapeRange::new)
    }

    pub fn all_excluding(&self, indices: Variant) -> Option<IvgShapeRange> {
        let args = vec![indices];
        self.disp
            .invoke_method("AllExcluding", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgShapeRange::new)
    }

    pub fn find_shape(
        &self,
        name: impl Into<String>,
        shape_type: i32,
        static_id: i32,
        recursive: bool,
        query: impl Into<String>,
    ) -> Option<IvgShape> {
        let args = vec![
            Variant::from_str(name.into()),
            Variant::from_i64(shape_type as i64),
            Variant::from_i64(static_id as i64),
            Variant::from_bool(recursive),
            Variant::from_str(query.into()),
        ];
        self.disp
            .invoke_method("FindShape", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgShape::new)
    }

    pub fn find_shapes(
        &self,
        name: impl Into<String>,
        shape_type: i32,
        recursive: bool,
        query: impl Into<String>,
    ) -> Option<IvgShapeRange> {
        let args = vec![
            Variant::from_str(name.into()),
            Variant::from_i64(shape_type as i64),
            Variant::from_bool(recursive),
            Variant::from_str(query.into()),
        ];
        self.disp
            .invoke_method("FindShapes", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgShapeRange::new)
    }
}