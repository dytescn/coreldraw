//! `ICUIImageList` / `ICUIBitmapImage` 鈥斺€?鍥惧儚鍒楄〃 / 浣嶅浘鍥惧儚

use wincom::{ComObject, Variant};
use windows::Win32::System::Com::IDispatch;

// =============================================================
// ICuiBitmapImage
// =============================================================

pub struct ICuiBitmapImage {
    disp: ComObject,
}

impl ICuiBitmapImage {
    pub fn new(disp: IDispatch) -> Self {
        Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) }
    }

    pub fn from_disp(disp: IDispatch) -> Self {
        Self::new(disp)
    }

    pub fn as_variant(&self) -> Variant {
        self.disp.as_variant()
    }

    fn prop_i64(&self, name: &str) -> Option<i64> {
        self.disp.get_property(name).ok()?.to_i64().ok()
    }

    fn prop_bool(&self, name: &str) -> Option<bool> {
        self.disp.get_property(name).ok()?.to_bool().ok()
    }

    pub fn valid(&self) -> Option<bool> { self.prop_bool("Valid") }
    pub fn width(&self) -> Option<i64> { self.prop_i64("Width") }
    pub fn height(&self) -> Option<i64> { self.prop_i64("Height") }
}

// =============================================================
// ICuiImageList
// =============================================================

pub struct ICuiImageList {
    disp: ComObject,
}

impl ICuiImageList {
    pub fn new(disp: IDispatch) -> Self {
        Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) }
    }

    pub fn from_disp(disp: IDispatch) -> Self {
        Self::new(disp)
    }

    fn prop_i64(&self, name: &str) -> Option<i64> {
        self.disp.get_property(name).ok()?.to_i64().ok()
    }

    pub fn image_count(&self) -> Option<i64> { self.prop_i64("ImageCount") }

    pub fn image_exists(&self, key: impl Into<String>) -> Option<bool> {
        let args = vec![Variant::from_str(key.into())];
        self.disp
            .invoke_method("ImageExists", args)
            .ok()?
            .to_bool()
            .ok()
    }

    pub fn remove_all(&self) -> bool {
        self.disp.invoke_method("RemoveAll", vec![]).is_ok()
    }

    /// `ImageData` 鏄?`VARIANT`锛堥€氬父鏄?SAFEARRAY 鎴?IStream锛夈€?
    pub fn add_image(
        &self,
        key: impl Into<String>,
        image_data: Variant,
        max_size: i32,
    ) -> Option<bool> {
        let args = vec![
            Variant::from_str(key.into()),
            image_data,
            Variant::from_i64(max_size as i64),
        ];
        self.disp
            .invoke_method("AddImage", args)
            .ok()?
            .to_bool()
            .ok()
    }

    pub fn image_keys(&self) -> Option<Variant> {
        self.disp.get_property("ImageKeys").ok()
    }

    pub fn remove_image(&self, key: impl Into<String>) -> Option<bool> {
        let args = vec![Variant::from_str(key.into())];
        self.disp
            .invoke_method("RemoveImage", args)
            .ok()?
            .to_bool()
            .ok()
    }

    pub fn add_bitmap(&self, key: impl Into<String>, bitmap: &ICuiBitmapImage) -> bool {
        let args = vec![
            Variant::from_str(key.into()),
            bitmap.as_variant(),
        ];
        self.disp.invoke_method("AddBitmap", args).is_ok()
    }
}