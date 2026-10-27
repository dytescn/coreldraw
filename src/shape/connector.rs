//! `IVGConnector` —— 连接器

use wincom::{ComObject, Variant};
use windows::Win32::System::Com::IDispatch;

use crate::geometry::IvgSnapPoint;

/// CorelDRAW `IVGConnector` 对象的后期绑定包装（连接器）。
///
/// 所有属性与方法通过 `IDispatch::Invoke` 按名称调用，调用失败时一般返回 `None` 或 `false`。
pub struct IvgConnector {
    disp: ComObject,
}

impl IvgConnector {
    /// 由 COM `IDispatch` 构造包装对象。
    pub fn new(disp: IDispatch) -> Self {
        Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) }
    }

    /// 从 COM `IDispatch` 构造包装对象，等价于 `new`。
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

    /// 读取启动点（`StartPoint`）。
    pub fn start_point(&self) -> Option<IvgSnapPoint> {
        self.prop_dispatch("StartPoint").map(IvgSnapPoint::new)
    }
    /// 设置启动点（`StartPoint`）。
    pub fn set_start_point(&self, p: &IvgSnapPoint) -> bool {
        self.put_dispatch("StartPoint", p.as_variant())
    }

    /// 读取结束点（`EndPoint`）。
    pub fn end_point(&self) -> Option<IvgSnapPoint> {
        self.prop_dispatch("EndPoint").map(IvgSnapPoint::new)
    }
    /// 设置结束点（`EndPoint`）。
    pub fn set_end_point(&self, p: &IvgSnapPoint) -> bool {
        self.put_dispatch("EndPoint", p.as_variant())
    }

    /// `cdrConnectorType`
    pub fn connector_type(&self) -> Option<i64> { self.prop_i64("Type") }
}