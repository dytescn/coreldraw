//! `ICUIImageList` / `ICUIBitmapImage` —— 图像列表 / 位图图像

use wincom::{ComObject, Variant};
use windows::Win32::System::Com::IDispatch;

// =============================================================
// ICuiBitmapImage
// =============================================================

/// CorelDRAW 对象的后期绑定包装（位图图像）。
///
/// 所有属性与方法通过 `IDispatch::Invoke` 按名称调用，调用失败时一般返回 `None` 或 `false`。
pub struct ICuiBitmapImage {
    disp: ComObject,
}

impl ICuiBitmapImage {
    /// 由 COM `IDispatch` 构造包装对象。
    pub fn new(disp: IDispatch) -> Self {
        Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) }
    }

    /// 从 COM `IDispatch` 构造包装对象，等价于 `new`。
    pub fn from_disp(disp: IDispatch) -> Self {
        Self::new(disp)
    }

    /// 把底层 COM 对象包装成 `VARIANT`，用于传给其它 COM 调用。
    pub fn as_variant(&self) -> Variant {
        self.disp.as_variant()
    }

    fn prop_i64(&self, name: &str) -> Option<i64> {
        self.disp.get_property(name).ok()?.to_i64().ok()
    }

    fn prop_bool(&self, name: &str) -> Option<bool> {
        self.disp.get_property(name).ok()?.to_bool().ok()
    }

    /// 读取 COM 属性 `Valid`。
    pub fn valid(&self) -> Option<bool> { self.prop_bool("Valid") }
    /// 读取宽度（`Width`）。
    pub fn width(&self) -> Option<i64> { self.prop_i64("Width") }
    /// 读取高度（`Height`）。
    pub fn height(&self) -> Option<i64> { self.prop_i64("Height") }
}

// =============================================================
// ICuiImageList
// =============================================================

/// CorelDRAW 对象的后期绑定包装（图像列表）。
///
/// 所有属性与方法通过 `IDispatch::Invoke` 按名称调用，调用失败时一般返回 `None` 或 `false`。
pub struct ICuiImageList {
    disp: ComObject,
}

impl ICuiImageList {
    /// 由 COM `IDispatch` 构造包装对象。
    pub fn new(disp: IDispatch) -> Self {
        Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) }
    }

    /// 从 COM `IDispatch` 构造包装对象，等价于 `new`。
    pub fn from_disp(disp: IDispatch) -> Self {
        Self::new(disp)
    }

    fn prop_i64(&self, name: &str) -> Option<i64> {
        self.disp.get_property(name).ok()?.to_i64().ok()
    }

    /// 读取图像数量（`ImageCount`）。
    pub fn image_count(&self) -> Option<i64> { self.prop_i64("ImageCount") }

    /// 图像 exists（COM 方法 `ImageExists`）。
    pub fn image_exists(&self, key: impl Into<String>) -> Option<bool> {
        let args = vec![Variant::from_str(key.into())];
        self.disp
            .invoke_method("ImageExists", args)
            .ok()?
            .to_bool()
            .ok()
    }

    /// 移除全部（COM 方法 `RemoveAll`）。
    pub fn remove_all(&self) -> bool {
        self.disp.invoke_method("RemoveAll", vec![]).is_ok()
    }

    /// `ImageData` 是 `VARIANT`（通常是 SAFEARRAY 或 IStream）。
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

    /// 读取 COM 属性 `ImageKeys`。
    pub fn image_keys(&self) -> Option<Variant> {
        self.disp.get_property("ImageKeys").ok()
    }

    /// 移除图像（COM 方法 `RemoveImage`）。
    pub fn remove_image(&self, key: impl Into<String>) -> Option<bool> {
        let args = vec![Variant::from_str(key.into())];
        self.disp
            .invoke_method("RemoveImage", args)
            .ok()?
            .to_bool()
            .ok()
    }

    /// 添加位图（COM 方法 `AddBitmap`）。
    pub fn add_bitmap(&self, key: impl Into<String>, bitmap: &ICuiBitmapImage) -> bool {
        let args = vec![
            Variant::from_str(key.into()),
            bitmap.as_variant(),
        ];
        self.disp.invoke_method("AddBitmap", args).is_ok()
    }
}