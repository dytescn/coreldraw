//! 导入回调接口 & 参数结构体
//!
//! - [`IImportCropHandler`]      : 导入时裁剪回调
//! - [`IImportResampleHandler`]  : 导入时重采样回调
//! - [`IStructImportCropOptions`] / [`IStructImportResampleOptions`] : 参数

use wincom::{ComObject, Variant};
use windows::Win32::System::Com::IDispatch;

// =============================================================
// IStructImportCropOptions
// =============================================================

/// `IImportCropHandler::Crop` 回调收到的参数。
pub struct IStructImportCropOptions {
    disp: ComObject,
}

impl IStructImportCropOptions {
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

    fn prop_string(&self, name: &str) -> Option<String> {
        self.disp.get_property(name).ok()?.to_string().ok()
    }

    fn put_i64(&self, name: &str, v: i64) -> bool {
        let arg = Variant::from_i64(v);
        self.disp.set_property(name, vec![arg]).is_ok()
    }

    // ---- 只读信息（由 CorelDRAW 填好）----

    /// 读取文件名称（`FileName`）。
    pub fn file_name(&self) -> Option<String> {
        self.prop_string("FileName")
    }

    /// 读取过滤器标识（`FilterID`）。
    pub fn filter_id(&self) -> Option<i64> {
        self.prop_i64("FilterID")
    }

    /// 读取自定义数据（`CustomData`）。
    pub fn custom_data(&self) -> Option<i64> {
        self.prop_i64("CustomData")
    }

    /// 读取图像宽度（`ImageWidth`）。
    pub fn image_width(&self) -> Option<i64> {
        self.prop_i64("ImageWidth")
    }

    /// 读取图像高度（`ImageHeight`）。
    pub fn image_height(&self) -> Option<i64> {
        self.prop_i64("ImageHeight")
    }

    /// 读取 DPIX 坐标（`DpiX`）。
    pub fn dpi_x(&self) -> Option<i64> {
        self.prop_i64("DpiX")
    }

    /// 读取 DPIY 坐标（`DpiY`）。
    pub fn dpi_y(&self) -> Option<i64> {
        self.prop_i64("DpiY")
    }

    // ---- 可读写：裁剪区域 ----

    /// 读取宽度（`Width`）。
    pub fn width(&self) -> Option<i64> { self.prop_i64("Width") }
    /// 设置宽度（`Width`）。
    pub fn set_width(&self, v: i32) -> bool { self.put_i64("Width", v as i64) }

    /// 读取高度（`Height`）。
    pub fn height(&self) -> Option<i64> { self.prop_i64("Height") }
    /// 设置高度（`Height`）。
    pub fn set_height(&self, v: i32) -> bool { self.put_i64("Height", v as i64) }

    /// 读取左（`Left`）。
    pub fn left(&self) -> Option<i64> { self.prop_i64("Left") }
    /// 设置左（`Left`）。
    pub fn set_left(&self, v: i32) -> bool { self.put_i64("Left", v as i64) }

    /// 读取顶（`Top`）。
    pub fn top(&self) -> Option<i64> { self.prop_i64("Top") }
    /// 设置顶（`Top`）。
    pub fn set_top(&self, v: i32) -> bool { self.put_i64("Top", v as i64) }
}

// =============================================================
// IImportCropHandler
// =============================================================

/// 自定义导入裁剪回调。
///
/// ⚠️ 要真正被 CorelDRAW 调用，需要实现 `IDispatch` 并通过
/// `IImportCropHandler` 注册（暂未提供 Rust 便 trait 桥接）。
pub struct IImportCropHandler {
    disp: ComObject,
}

impl IImportCropHandler {
    /// 由 COM `IDispatch` 构造包装对象。
    pub fn new(disp: IDispatch) -> Self {
        Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) }
    }

    /// 从 COM `IDispatch` 构造包装对象，等价于 `new`。
    pub fn from_disp(disp: IDispatch) -> Self {
        Self::new(disp)
    }

    /// 触发 `Crop(options)` 方法（一般由 CorelDRAW 调用）。
    pub fn crop(&self, options: &IStructImportCropOptions) -> Option<bool> {
        let args = vec![options.disp.as_variant()];
        self.disp.invoke_method("Crop", args).ok()?.to_bool().ok()
    }
}

// =============================================================
// IStructImportResampleOptions
// =============================================================

/// `IImportResampleHandler::Resample` 回调收到的参数。
pub struct IStructImportResampleOptions {
    disp: ComObject,
}

impl IStructImportResampleOptions {
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

    fn prop_string(&self, name: &str) -> Option<String> {
        self.disp.get_property(name).ok()?.to_string().ok()
    }

    fn put_i64(&self, name: &str, v: i64) -> bool {
        let arg = Variant::from_i64(v);
        self.disp.set_property(name, vec![arg]).is_ok()
    }

    // ---- 只读 ----

    /// 读取文件名称（`FileName`）。
    pub fn file_name(&self) -> Option<String> {
        self.prop_string("FileName")
    }

    /// 读取过滤器标识（`FilterID`）。
    pub fn filter_id(&self) -> Option<i64> {
        self.prop_i64("FilterID")
    }

    /// 读取自定义数据（`CustomData`）。
    pub fn custom_data(&self) -> Option<i64> {
        self.prop_i64("CustomData")
    }

    // ---- 可读写：目标尺寸 / DPI ----

    /// 读取宽度（`Width`）。
    pub fn width(&self) -> Option<i64> { self.prop_i64("Width") }
    /// 设置宽度（`Width`）。
    pub fn set_width(&self, v: i32) -> bool { self.put_i64("Width", v as i64) }

    /// 读取高度（`Height`）。
    pub fn height(&self) -> Option<i64> { self.prop_i64("Height") }
    /// 设置高度（`Height`）。
    pub fn set_height(&self, v: i32) -> bool { self.put_i64("Height", v as i64) }

    /// 读取 DPIX 坐标（`DpiX`）。
    pub fn dpi_x(&self) -> Option<i64> { self.prop_i64("DpiX") }
    /// 设置 DPIX 坐标（`DpiX`）。
    pub fn set_dpi_x(&self, v: i32) -> bool { self.put_i64("DpiX", v as i64) }

    /// 读取 DPIY 坐标（`DpiY`）。
    pub fn dpi_y(&self) -> Option<i64> { self.prop_i64("DpiY") }
    /// 设置 DPIY 坐标（`DpiY`）。
    pub fn set_dpi_y(&self, v: i32) -> bool { self.put_i64("DpiY", v as i64) }
}

// =============================================================
// IImportResampleHandler
// =============================================================

/// 自定义导入重采样回调。
///
/// ⚠️ 后[`IImportCropHandler`]，需要 `IDispatch` 桥接才能被 CorelDRAW 调用。
pub struct IImportResampleHandler {
    disp: ComObject,
}

impl IImportResampleHandler {
    /// 由 COM `IDispatch` 构造包装对象。
    pub fn new(disp: IDispatch) -> Self {
        Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) }
    }

    /// 从 COM `IDispatch` 构造包装对象，等价于 `new`。
    pub fn from_disp(disp: IDispatch) -> Self {
        Self::new(disp)
    }

    /// 触发 `Resample(options)`。
    pub fn resample(&self, options: &IStructImportResampleOptions) -> Option<bool> {
        let args = vec![options.disp.as_variant()];
        self.disp.invoke_method("Resample", args).ok()?.to_bool().ok()
    }
}