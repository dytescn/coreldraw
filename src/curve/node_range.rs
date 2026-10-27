//! `IVGNodeRange` —— 节点范围

use wincom::{ComObject, Variant};
use windows::Win32::System::Com::IDispatch;

use crate::curve::{IvgNode, IvgSegmentRange};
use crate::geometry::IvgRect;

/// CorelDRAW `IVGNodeRange` 对象的后期绑定包装（节点范围）。
///
/// 所有属性与方法通过 `IDispatch::Invoke` 按名称调用，调用失败时一般返回 `None` 或 `false`。
pub struct IvgNodeRange {
    disp: ComObject,
}

impl IvgNodeRange {
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

    // ---- 基本信息 ----

    /// 读取返回条目数量（`Count`）。
    pub fn count(&self) -> Option<i64> { self.prop_i64("Count") }
    /// 读取节点类型（`Type`）。
    pub fn node_type(&self) -> Option<i64> { self.prop_i64("Type") }
    /// 读取位置 X 坐标（`PositionX`）。
    pub fn position_x(&self) -> Option<f64> { self.prop_f64("PositionX") }
    /// 读取位置 Y 坐标（`PositionY`）。
    pub fn position_y(&self) -> Option<f64> { self.prop_f64("PositionY") }
    /// 读取尺寸宽度（`SizeWidth`）。
    pub fn size_width(&self) -> Option<f64> { self.prop_f64("SizeWidth") }
    /// 读取尺寸高度（`SizeHeight`）。
    pub fn size_height(&self) -> Option<f64> { self.prop_f64("SizeHeight") }

    /// 按索引或名称取条目（COM 方法 `Item`）。
    pub fn item(&self, index: i32) -> Option<IvgNode> {
        let args = vec![Variant::from_i64(index as i64)];
        self.disp.invoke_method("Item", args).ok()?.to_idispatch().ok().map(IvgNode::new)
    }

    /// 读取第一个节点（`FirstNode`）。
    pub fn first_node(&self) -> Option<IvgNode> {
        self.prop_dispatch("FirstNode").map(IvgNode::new)
    }

    /// 读取最后一个节点（`LastNode`）。
    pub fn last_node(&self) -> Option<IvgNode> {
        self.prop_dispatch("LastNode").map(IvgNode::new)
    }

    /// 读取线段范围（`SegmentRange`）。
    pub fn segment_range(&self) -> Option<IvgSegmentRange> {
        self.prop_dispatch("SegmentRange").map(IvgSegmentRange::new)
    }

    /// 读取边界框（`BoundingBox`）。
    pub fn bounding_box(&self) -> Option<IvgRect> {
        self.prop_dispatch("BoundingBox").map(IvgRect::new)
    }

    // ---- 编辑 ----

    /// 添加（COM 方法 `Add`）。
    pub fn add(&self, node: &IvgNode) -> bool {
        let args = vec![node.as_variant()];
        self.disp.invoke_method("Add", args).is_ok()
    }

    /// 添加范围（COM 方法 `AddRange`）。
    pub fn add_range(&self, range: &IvgNodeRange) -> bool {
        let args = vec![range.as_variant()];
        self.disp.invoke_method("AddRange", args).is_ok()
    }

    /// 移除（COM 方法 `Remove`）。
    pub fn remove(&self, index: i32) -> bool {
        let args = vec![Variant::from_i64(index as i64)];
        self.disp.invoke_method("Remove", args).is_ok()
    }

    /// 移除范围（COM 方法 `RemoveRange`）。
    pub fn remove_range(&self, range: &IvgNodeRange) -> bool {
        let args = vec![range.as_variant()];
        self.disp.invoke_method("RemoveRange", args).is_ok()
    }

    /// 移除全部（COM 方法 `RemoveAll`）。
    pub fn remove_all(&self) -> bool { self.disp.invoke_method("RemoveAll", vec![]).is_ok() }
    /// 删除（COM 方法 `Delete`）。
    pub fn delete(&self) -> bool { self.disp.invoke_method("Delete", vec![]).is_ok() }

    /// 移动（COM 方法 `Move`）。
    pub fn move_by(&self, dx: f64, dy: f64, anchor: i32, elastic: bool) -> bool {
        let args = vec![
            Variant::from_f64(dx),
            Variant::from_f64(dy),
            Variant::from_i64(anchor as i64),
            Variant::from_bool(elastic),
        ];
        self.disp.invoke_method("Move", args).is_ok()
    }

    /// 拉伸（COM 方法 `Stretch`）。
    pub fn stretch(&self, rx: f32, ry: f32, use_anchor: bool, ax: f64, ay: f64) -> bool {
        let args = vec![
            Variant::from_f64(rx as f64),
            Variant::from_f64(ry as f64),
            Variant::from_bool(use_anchor),
            Variant::from_f64(ax),
            Variant::from_f64(ay),
        ];
        self.disp.invoke_method("Stretch", args).is_ok()
    }

    /// rotate（COM 方法 `Rotate`）。
    pub fn rotate(&self, angle: f64, use_center: bool, cx: f64, cy: f64) -> bool {
        let args = vec![
            Variant::from_f64(angle),
            Variant::from_bool(use_center),
            Variant::from_f64(cx),
            Variant::from_f64(cy),
        ];
        self.disp.invoke_method("Rotate", args).is_ok()
    }

    /// 倾斜（COM 方法 `Skew`）。
    pub fn skew(&self, ax: f64, ay: f64, use_anchor: bool, sx: f64, sy: f64) -> bool {
        let args = vec![
            Variant::from_f64(ax),
            Variant::from_f64(ay),
            Variant::from_bool(use_anchor),
            Variant::from_f64(sx),
            Variant::from_f64(sy),
        ];
        self.disp.invoke_method("Skew", args).is_ok()
    }

    /// 设置类型（`SetType`）。
    pub fn set_type(&self, node_type: i32) -> bool {
        let args = vec![Variant::from_i64(node_type as i64)];
        self.disp.invoke_method("SetType", args).is_ok()
    }

    /// 调用 COM 方法 `AutoReduce`。
    pub fn auto_reduce(&self, margin: f64) -> bool {
        let args = vec![Variant::from_f64(margin)];
        self.disp.invoke_method("AutoReduce", args).is_ok()
    }

    /// 调用 COM 方法 `Smoothen`。
    pub fn smoothen(&self, smoothness: i32) -> bool {
        let args = vec![Variant::from_i64(smoothness as i64)];
        self.disp.invoke_method("Smoothen", args).is_ok()
    }

    /// 断开分开（COM 方法 `BreakApart`）。
    pub fn break_apart(&self) -> bool { self.disp.invoke_method("BreakApart", vec![]).is_ok() }

    /// 圆角（COM 方法 `Fillet`）。
    pub fn fillet(&self, radius: f64, combine: bool) -> bool {
        let args = vec![
            Variant::from_f64(radius),
            Variant::from_bool(combine),
        ];
        self.disp.invoke_method("Fillet", args).is_ok()
    }

    /// 倒角（COM 方法 `Chamfer`）。
    pub fn chamfer(&self, a: f64, b: f64, combine: bool) -> bool {
        let args = vec![
            Variant::from_f64(a),
            Variant::from_f64(b),
            Variant::from_bool(combine),
        ];
        self.disp.invoke_method("Chamfer", args).is_ok()
    }

    /// 扇贝（COM 方法 `Scallop`）。
    pub fn scallop(&self, radius: f64, combine: bool) -> bool {
        let args = vec![
            Variant::from_f64(radius),
            Variant::from_bool(combine),
        ];
        self.disp.invoke_method("Scallop", args).is_ok()
    }

    // ---- 选择 ----

    /// 创建选择（COM 方法 `CreateSelection`）。
    pub fn create_selection(&self) -> bool { self.disp.invoke_method("CreateSelection", vec![]).is_ok() }
    /// 添加选择（COM 方法 `AddToSelection`）。
    pub fn add_to_selection(&self) -> bool { self.disp.invoke_method("AddToSelection", vec![]).is_ok() }
    /// 移除选择（COM 方法 `RemoveFromSelection`）。
    pub fn remove_from_selection(&self) -> bool { self.disp.invoke_method("RemoveFromSelection", vec![]).is_ok() }
}