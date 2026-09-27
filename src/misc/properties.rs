//! `IVGProperties` 鈥斺€?閫氱敤灞炴€у寘锛坣ame/id 鍙岄敭锛?

use wincom::{ComObject, Variant};
use windows::Win32::System::Com::IDispatch;

use crate::curve::IvgCurve;
use crate::geometry::{IvgPoint, IvgVector};

pub struct IvgProperties {
    disp: ComObject,
}

impl IvgProperties {
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

    // ---------------------------------------------------------
    // 璁℃暟
    // ---------------------------------------------------------

    pub fn count(&self) -> Option<i64> { self.prop_i64("Count") }

    // ---------------------------------------------------------
    // 閫氱敤璇诲啓锛坣ame + id锛?
    // ---------------------------------------------------------

    pub fn item(&self, name: impl Into<String>, id: i32) -> Option<Variant> {
        let args = vec![
            Variant::from_str(name.into()),
            Variant::from_i64(id as i64),
        ];
        self.disp.invoke_method("Item", args).ok()
    }

    pub fn set_item(
        &self,
        name: impl Into<String>,
        id: i32,
        value: Variant,
    ) -> bool {
        let args = vec![
            Variant::from_str(name.into()),
            Variant::from_i64(id as i64),
            value,
        ];
        self.disp.invoke_method("put_Item", args).is_ok()
    }

    pub fn item_by_index(&self, index: i32) -> Option<Variant> {
        let args = vec![Variant::from_i64(index as i64)];
        self.disp.invoke_method("ItemByIndex", args).ok()
    }

    pub fn index_of(&self, name: impl Into<String>, id: i32) -> Option<i64> {
        let args = vec![
            Variant::from_str(name.into()),
            Variant::from_i64(id as i64),
        ];
        self.disp.invoke_method("Index", args).ok()?.to_i64().ok()
    }

    pub fn exists(&self, name: impl Into<String>, id: i32) -> Option<bool> {
        let args = vec![
            Variant::from_str(name.into()),
            Variant::from_i64(id as i64),
        ];
        self.disp.invoke_method("Exists", args).ok()?.to_bool().ok()
    }

    pub fn description(&self, index: i32) -> Option<(String, i32)> {
        // 鍙屽嚭鍙傛暟锛屽彧鍙?name銆?
        let args = vec![Variant::from_i64(index as i64)];
        let name = self
            .disp
            .invoke_method("Description", args)
            .ok()?
            .to_string()
            .ok()?;
        Some((name, 0))
    }

    pub fn delete(&self, name: impl Into<String>, id: i32) -> bool {
        let args = vec![
            Variant::from_str(name.into()),
            Variant::from_i64(id as i64),
        ];
        self.disp.invoke_method("Delete", args).is_ok()
    }

    pub fn delete_by_index(&self, index: i32) -> bool {
        let args = vec![Variant::from_i64(index as i64)];
        self.disp.invoke_method("DeleteByIndex", args).is_ok()
    }

    pub fn delete_all(&self, name: impl Into<String>) -> Option<bool> {
        let args = vec![Variant::from_str(name.into())];
        self.disp.invoke_method("DeleteAll", args).ok()?.to_bool().ok()
    }

    // ---------------------------------------------------------
    // 鏂囦欢鎿嶄綔
    // ---------------------------------------------------------

    pub fn put_file(
        &self,
        name: impl Into<String>,
        id: i32,
        file_name: impl Into<String>,
    ) -> bool {
        let args = vec![
            Variant::from_str(name.into()),
            Variant::from_i64(id as i64),
            Variant::from_str(file_name.into()),
        ];
        self.disp.invoke_method("PutFile", args).is_ok()
    }

    pub fn get_file(
        &self,
        name: impl Into<String>,
        id: i32,
        file_name: impl Into<String>,
    ) -> bool {
        let args = vec![
            Variant::from_str(name.into()),
            Variant::from_i64(id as i64),
            Variant::from_str(file_name.into()),
        ];
        self.disp.invoke_method("GetFile", args).is_ok()
    }

    // ---------------------------------------------------------
    // 绫诲瀷鍖栧揩鎹锋柟娉曪紙uuidName锛?
    // ---------------------------------------------------------

    pub fn get_point(&self, uuid_name: impl Into<String>) -> Option<IvgPoint> {
        let args = vec![Variant::from_str(uuid_name.into())];
        self.disp
            .invoke_method("GetPoint", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgPoint::new)
    }

    pub fn set_point(&self, uuid_name: impl Into<String>, p: &IvgPoint) -> bool {
        let args = vec![
            Variant::from_str(uuid_name.into()),
            p.as_variant(),
        ];
        self.disp.invoke_method("SetPoint", args).is_ok()
    }

    pub fn get_vector(&self, uuid_name: impl Into<String>) -> Option<IvgVector> {
        let args = vec![Variant::from_str(uuid_name.into())];
        self.disp
            .invoke_method("GetVector", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgVector::new)
    }

    pub fn set_vector(&self, uuid_name: impl Into<String>, v: &IvgVector) -> bool {
        let args = vec![
            Variant::from_str(uuid_name.into()),
            v.as_variant(),
        ];
        self.disp.invoke_method("SetVector", args).is_ok()
    }

    pub fn get_curve(&self, uuid_name: impl Into<String>) -> Option<IvgCurve> {
        let args = vec![Variant::from_str(uuid_name.into())];
        self.disp
            .invoke_method("GetCurve", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgCurve::new)
    }

    pub fn set_curve(&self, uuid_name: impl Into<String>, c: &IvgCurve) -> bool {
        let args = vec![
            Variant::from_str(uuid_name.into()),
            c.as_variant(),
        ];
        self.disp.invoke_method("SetCurve", args).is_ok()
    }
}