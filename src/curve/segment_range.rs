//! `IVGSegmentRange` —— 线段范围

use wincom::{ComObject, Variant};
use windows::Win32::System::Com::IDispatch;

use crate::curve::{IvgNodeRange, IvgSegment};

/// CorelDRAW `IVGSegmentRange` 对象的后期绑定包装（线段范围）。
///
/// 所有属性与方法通过 `IDispatch::Invoke` 按名称调用，调用失败时一般返回 `None` 或 `false`。
pub struct IvgSegmentRange {
    disp: ComObject,
}

impl IvgSegmentRange {
    /// 由 COM `IDispatch` 构造包装对象。
    pub fn new(disp: IDispatch) -> Self { Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) } }

    /// 把底层 COM 对象包装成 `VARIANT`，用于传给其它 COM 调用。
    pub fn as_variant(&self) -> Variant {
        self.disp.as_variant()
    }

    fn prop_i64(&self, name: &str) -> Option<i64> {
        self.disp.get_property(name).ok()?.to_i64().ok()
    }

    fn prop_f64(&self, name: &str) -> Option<f64> {
        self.disp.get_property(name).ok()?.to_f64().ok()
    }

    fn prop_dispatch(&self, name: &str) -> Option<IDispatch> {
        self.disp.get_property(name).ok()?.to_idispatch().ok()
    }

    /// 读取返回条目数量（`Count`）。
    pub fn count(&self) -> Option<i64> { self.prop_i64("Count") }
    /// 读取线段类型（`Type`）。
    pub fn segment_type(&self) -> Option<i64> { self.prop_i64("Type") }
    /// 读取长度（`Length`）。
    pub fn length(&self) -> Option<f64> { self.prop_f64("Length") }

    /// 按索引或名称取条目（COM 方法 `Item`）。
    pub fn item(&self, index: i32) -> Option<IvgSegment> {
        let args = vec![Variant::from_i64(index as i64)];
        self.disp.invoke_method("Item", args).ok()?.to_idispatch().ok().map(IvgSegment::new)
    }

    /// 读取第一个线段（`FirstSegment`）。
    pub fn first_segment(&self) -> Option<IvgSegment> {
        self.prop_dispatch("FirstSegment").map(IvgSegment::new)
    }

    /// 读取最后一个线段（`LastSegment`）。
    pub fn last_segment(&self) -> Option<IvgSegment> {
        self.prop_dispatch("LastSegment").map(IvgSegment::new)
    }

    /// 读取节点范围（`NodeRange`）。
    pub fn node_range(&self) -> Option<IvgNodeRange> {
        self.prop_dispatch("NodeRange").map(IvgNodeRange::new)
    }

    /// 添加（COM 方法 `Add`）。
    pub fn add(&self, segment: &IvgSegment) -> bool {
        let args = vec![segment.as_variant()];
        self.disp.invoke_method("Add", args).is_ok()
    }

    /// 添加范围（COM 方法 `AddRange`）。
    pub fn add_range(&self, range: &IvgSegmentRange) -> bool {
        let args = vec![range.as_variant()];
        self.disp.invoke_method("AddRange", args).is_ok()
    }

    /// 移除（COM 方法 `Remove`）。
    pub fn remove(&self, index: i32) -> bool {
        let args = vec![Variant::from_i64(index as i64)];
        self.disp.invoke_method("Remove", args).is_ok()
    }

    /// 移除范围（COM 方法 `RemoveRange`）。
    pub fn remove_range(&self, range: &IvgSegmentRange) -> bool {
        let args = vec![range.as_variant()];
        self.disp.invoke_method("RemoveRange", args).is_ok()
    }

    /// 移除全部（COM 方法 `RemoveAll`）。
    pub fn remove_all(&self) -> bool { self.disp.invoke_method("RemoveAll", vec![]).is_ok() }
    /// 添加节点（COM 方法 `AddNode`）。
    pub fn add_node(&self) -> bool { self.disp.invoke_method("AddNode", vec![]).is_ok() }

    /// 设置类型（`SetType`）。
    pub fn set_type(&self, segment_type: i32) -> bool {
        let args = vec![Variant::from_i64(segment_type as i64)];
        self.disp.invoke_method("SetType", args).is_ok()
    }

    /// 创建选择（COM 方法 `CreateSelection`）。
    pub fn create_selection(&self) -> bool { self.disp.invoke_method("CreateSelection", vec![]).is_ok() }
    /// 添加选择（COM 方法 `AddToSelection`）。
    pub fn add_to_selection(&self) -> bool { self.disp.invoke_method("AddToSelection", vec![]).is_ok() }
    /// 移除选择（COM 方法 `RemoveFromSelection`）。
    pub fn remove_from_selection(&self) -> bool { self.disp.invoke_method("RemoveFromSelection", vec![]).is_ok() }
}