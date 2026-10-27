//! `IVGEPS` —— EPS 图形

use wincom::{ComObject, Variant};
use windows::Win32::System::Com::IDispatch;

use crate::curve::IvgCurve;
use crate::shape::IvgBitmap;

/// CorelDRAW `IVGEPS` 对象的后期绑定包装（EPS）。
///
/// 所有属性与方法通过 `IDispatch::Invoke` 按名称调用，调用失败时一般返回 `None` 或 `false`。
pub struct IvgEps {
    disp: ComObject,
}

impl IvgEps {
    /// 由 COM `IDispatch` 构造包装对象。
    pub fn new(disp: IDispatch) -> Self {
        Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) }
    }

    /// 从 COM `IDispatch` 构造包装对象，等价于 `new`。
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

    /// 读取 COM 属性 `PreviewBitmap`。
    pub fn preview_bitmap(&self) -> Option<IvgBitmap> {
        self.prop_dispatch("PreviewBitmap").map(IvgBitmap::new)
    }

    /// 读取数据（`Data`）。
    pub fn data(&self) -> Option<Variant> {
        self.disp.get_property("Data").ok()
    }

    /// 读取数据 as 字符串（`DataAsString`）。
    pub fn data_as_string(&self) -> Option<String> {
        self.prop_string("DataAsString")
    }

    /// 读取裁剪封套（`CropEnvelope`）。
    pub fn crop_envelope(&self) -> Option<IvgCurve> {
        self.prop_dispatch("CropEnvelope").map(IvgCurve::new)
    }

    /// 重置裁剪封套（COM 方法 `ResetCropEnvelope`）。
    pub fn reset_crop_envelope(&self) -> bool {
        self.disp.invoke_method("ResetCropEnvelope", vec![]).is_ok()
    }

    /// 读取 COM 属性 `CropEnvelopeModified`。
    pub fn crop_envelope_modified(&self) -> Option<bool> {
        self.prop_bool("CropEnvelopeModified")
    }

    /// 读取边界框路径（`BoundingBoxPath`）。
    pub fn bounding_box_path(&self) -> Option<IvgCurve> {
        self.prop_dispatch("BoundingBoxPath").map(IvgCurve::new)
    }

    /// 读取链接文件名称（`LinkFileName`）。
    pub fn link_file_name(&self) -> Option<String> { self.prop_string("LinkFileName") }
    /// 设置链接文件名称（`LinkFileName`）。
    pub fn set_link_file_name(&self, v: impl Into<String>) -> bool {
        self.put_string("LinkFileName", v)
    }

    /// 读取 COM 属性 `DCSFileNames`。
    pub fn dcs_file_names(&self) -> Option<Variant> {
        self.disp.get_property("DCSFileNames").ok()
    }
}