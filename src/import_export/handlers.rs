//! 瀵煎叆鍥炶皟鎺ュ彛 & 鍙傛暟缁撴瀯浣?
//!
//! - [`IImportCropHandler`]      : 瀵煎叆鏃惰鍓洖璋?
//! - [`IImportResampleHandler`]  : 瀵煎叆鏃堕噸閲囨牱鍥炶皟
//! - [`IStructImportCropOptions`] / [`IStructImportResampleOptions`] : 鍙傛暟

use wincom::{ComObject, Variant};
use windows::Win32::System::Com::IDispatch;

// =============================================================
// IStructImportCropOptions
// =============================================================

/// `IImportCropHandler::Crop` 鍥炶皟鏀跺埌鐨勫弬鏁般€?
pub struct IStructImportCropOptions {
    disp: ComObject,
}

impl IStructImportCropOptions {
    pub fn new(disp: IDispatch) -> Self {
        Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) }
    }

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

    // ---- 鍙淇℃伅锛堢敱 CorelDRAW 濉ソ锛?----

    pub fn file_name(&self) -> Option<String> {
        self.prop_string("FileName")
    }

    pub fn filter_id(&self) -> Option<i64> {
        self.prop_i64("FilterID")
    }

    pub fn custom_data(&self) -> Option<i64> {
        self.prop_i64("CustomData")
    }

    pub fn image_width(&self) -> Option<i64> {
        self.prop_i64("ImageWidth")
    }

    pub fn image_height(&self) -> Option<i64> {
        self.prop_i64("ImageHeight")
    }

    pub fn dpi_x(&self) -> Option<i64> {
        self.prop_i64("DpiX")
    }

    pub fn dpi_y(&self) -> Option<i64> {
        self.prop_i64("DpiY")
    }

    // ---- 鍙鍐欙細瑁佸壀鍖哄煙 ----

    pub fn width(&self) -> Option<i64> { self.prop_i64("Width") }
    pub fn set_width(&self, v: i32) -> bool { self.put_i64("Width", v as i64) }

    pub fn height(&self) -> Option<i64> { self.prop_i64("Height") }
    pub fn set_height(&self, v: i32) -> bool { self.put_i64("Height", v as i64) }

    pub fn left(&self) -> Option<i64> { self.prop_i64("Left") }
    pub fn set_left(&self, v: i32) -> bool { self.put_i64("Left", v as i64) }

    pub fn top(&self) -> Option<i64> { self.prop_i64("Top") }
    pub fn set_top(&self, v: i32) -> bool { self.put_i64("Top", v as i64) }
}

// =============================================================
// IImportCropHandler
// =============================================================

/// 鑷畾涔夊鍏ヨ鍓洖璋冦€?
///
/// 鈿狅笍 瑕佺湡姝ｈ CorelDRAW 璋冪敤锛岄渶瑕佸疄鐜?`IDispatch` 骞堕€氳繃
/// `IImportCropHandler` 娉ㄥ唽锛堟殏鏈彁渚?Rust 渚?trait 妗ユ帴锛夈€?
pub struct IImportCropHandler {
    disp: ComObject,
}

impl IImportCropHandler {
    pub fn new(disp: IDispatch) -> Self {
        Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) }
    }

    pub fn from_disp(disp: IDispatch) -> Self {
        Self::new(disp)
    }

    /// 瑙﹀彂 `Crop(options)` 鏂规硶锛堜竴鑸敱 CorelDRAW 璋冪敤锛夈€?
    pub fn crop(&self, options: &IStructImportCropOptions) -> Option<bool> {
        let args = vec![options.disp.as_variant()];
        self.disp.invoke_method("Crop", args).ok()?.to_bool().ok()
    }
}

// =============================================================
// IStructImportResampleOptions
// =============================================================

/// `IImportResampleHandler::Resample` 鍥炶皟鏀跺埌鐨勫弬鏁般€?
pub struct IStructImportResampleOptions {
    disp: ComObject,
}

impl IStructImportResampleOptions {
    pub fn new(disp: IDispatch) -> Self {
        Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) }
    }

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

    // ---- 鍙 ----

    pub fn file_name(&self) -> Option<String> {
        self.prop_string("FileName")
    }

    pub fn filter_id(&self) -> Option<i64> {
        self.prop_i64("FilterID")
    }

    pub fn custom_data(&self) -> Option<i64> {
        self.prop_i64("CustomData")
    }

    // ---- 鍙鍐欙細鐩爣灏哄 / DPI ----

    pub fn width(&self) -> Option<i64> { self.prop_i64("Width") }
    pub fn set_width(&self, v: i32) -> bool { self.put_i64("Width", v as i64) }

    pub fn height(&self) -> Option<i64> { self.prop_i64("Height") }
    pub fn set_height(&self, v: i32) -> bool { self.put_i64("Height", v as i64) }

    pub fn dpi_x(&self) -> Option<i64> { self.prop_i64("DpiX") }
    pub fn set_dpi_x(&self, v: i32) -> bool { self.put_i64("DpiX", v as i64) }

    pub fn dpi_y(&self) -> Option<i64> { self.prop_i64("DpiY") }
    pub fn set_dpi_y(&self, v: i32) -> bool { self.put_i64("DpiY", v as i64) }
}

// =============================================================
// IImportResampleHandler
// =============================================================

/// 鑷畾涔夊鍏ラ噸閲囨牱鍥炶皟銆?
///
/// 鈿狅笍 鍚?[`IImportCropHandler`]锛岄渶瑕?`IDispatch` 妗ユ帴鎵嶈兘琚?CorelDRAW 璋冪敤銆?
pub struct IImportResampleHandler {
    disp: ComObject,
}

impl IImportResampleHandler {
    pub fn new(disp: IDispatch) -> Self {
        Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) }
    }

    pub fn from_disp(disp: IDispatch) -> Self {
        Self::new(disp)
    }

    /// 瑙﹀彂 `Resample(options)`銆?
    pub fn resample(&self, options: &IStructImportResampleOptions) -> Option<bool> {
        let args = vec![options.disp.as_variant()];
        self.disp.invoke_method("Resample", args).ok()?.to_bool().ok()
    }
}