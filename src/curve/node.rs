//! `IVGNode` / `IVGNodes` —— 曲线节点

use wincom::{ComObject, Variant};
use windows::Win32::System::Com::IDispatch;

use crate::curve::{IvgNodeRange, IvgSegment, IvgSubPath};

// =============================================================
// IvgNodes
// =============================================================

/// CorelDRAW `IVGNode` 对象的后期绑定包装（节点集）。
///
/// 所有属性与方法通过 `IDispatch::Invoke` 按名称调用，调用失败时一般返回 `None` 或 `false`。
pub struct IvgNodes {
    disp: ComObject,
}

impl IvgNodes {
    /// 由 COM `IDispatch` 构造包装对象。
    pub fn new(disp: IDispatch) -> Self { Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) } }

    fn prop_i64(&self, name: &str) -> Option<i64> {
        self.disp.get_property(name).ok()?.to_i64().ok()
    }

    /// 读取返回条目数量（`Count`）。
    pub fn count(&self) -> Option<i64> { self.prop_i64("Count") }

    /// 按索引或名称取条目（COM 方法 `Item`）。
    pub fn item(&self, index: i32) -> Option<IvgNode> {
        let args = vec![Variant::from_i64(index as i64)];
        self.disp.invoke_method("Item", args).ok()?.to_idispatch().ok().map(IvgNode::new)
    }

    /// 读取返回第一个条目（`First`）。
    pub fn first(&self) -> Option<IvgNode> {
        self.disp.get_property("First").ok()?.to_idispatch().ok().map(IvgNode::new)
    }

    /// 读取返回最后一个条目（`Last`）。
    pub fn last(&self) -> Option<IvgNode> {
        self.disp.get_property("Last").ok()?.to_idispatch().ok().map(IvgNode::new)
    }

    /// 按索引数组取节点范围
    pub fn range(&self, indices: Variant) -> Option<IvgNodeRange> {
        let args = vec![indices];
        self.disp.invoke_method("Range", args).ok()?.to_idispatch().ok().map(IvgNodeRange::new)
    }

    /// 返回全部条目（COM 方法 `All`）。
    pub fn all(&self) -> Option<IvgNodeRange> {
        self.disp.invoke_method("All", vec![]).ok()?.to_idispatch().ok().map(IvgNodeRange::new)
    }
}

// =============================================================
// IvgNode
// =============================================================

/// CorelDRAW `IVGNode` 对象的后期绑定包装（节点）。
///
/// 所有属性与方法通过 `IDispatch::Invoke` 按名称调用，调用失败时一般返回 `None` 或 `false`。
pub struct IvgNode {
    disp: ComObject,
}

impl IvgNode {
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

    fn prop_bool(&self, name: &str) -> Option<bool> {
        self.disp.get_property(name).ok()?.to_bool().ok()
    }

    fn prop_dispatch(&self, name: &str) -> Option<IDispatch> {
        self.disp.get_property(name).ok()?.to_idispatch().ok()
    }

    fn put_i64(&self, name: &str, v: i64) -> bool {
        let arg = Variant::from_i64(v);
        self.disp.set_property(name, vec![arg]).is_ok()
    }

    fn put_f64(&self, name: &str, v: f64) -> bool {
        let arg = Variant::from_f64(v);
        self.disp.set_property(name, vec![arg]).is_ok()
    }

    fn put_bool(&self, name: &str, v: bool) -> bool {
        let arg = Variant::from_bool(v);
        self.disp.set_property(name, vec![arg]).is_ok()
    }

    // ---- 位置 ----

    /// 读取位置 X 坐标（`PositionX`）。
    pub fn position_x(&self) -> Option<f64> { self.prop_f64("PositionX") }
    /// 设置位置 X 坐标（`PositionX`）。
    pub fn set_position_x(&self, v: f64) -> bool { self.put_f64("PositionX", v) }

    /// 读取位置 Y 坐标（`PositionY`）。
    pub fn position_y(&self) -> Option<f64> { self.prop_f64("PositionY") }
    /// 设置位置 Y 坐标（`PositionY`）。
    pub fn set_position_y(&self, v: f64) -> bool { self.put_f64("PositionY", v) }

    /// 读取位置。
    pub fn get_position(&self) -> Option<(f64, f64)> {
        Some((self.position_x()?, self.position_y()?))
    }

    /// 设置位置。
    pub fn set_position(&self, x: f64, y: f64) -> bool {
        self.set_position_x(x) && self.set_position_y(y)
    }

    /// 移动（COM 方法 `Move`）。
    pub fn move_by(&self, dx: f64, dy: f64) -> bool {
        let args = vec![
            Variant::from_f64(dx),
            Variant::from_f64(dy),
        ];
        self.disp.invoke_method("Move", args).is_ok()
    }

    // ---- 类型 / 索引 ----

    /// `cdrNodeType` 瑙?`enums::curve`
    pub fn node_type(&self) -> Option<i64> { self.prop_i64("Type") }
    /// 设置节点类型（`Type`）。
    pub fn set_node_type(&self, v: i32) -> bool { self.put_i64("Type", v as i64) }

    /// 读取索引（`Index`）。
    pub fn index(&self) -> Option<i64> { self.prop_i64("Index") }
    /// 读取子路径索引（`SubPathIndex`）。
    pub fn sub_path_index(&self) -> Option<i64> { self.prop_i64("SubPathIndex") }
    /// 读取绝对索引（`AbsoluteIndex`）。
    pub fn absolute_index(&self) -> Option<i64> { self.prop_i64("AbsoluteIndex") }

    /// 是否末端（`IsEnding`）。
    pub fn is_ending(&self) -> Option<bool> { self.prop_bool("IsEnding") }
    /// 是否已选中（`Selected`）。
    pub fn is_selected(&self) -> Option<bool> { self.prop_bool("Selected") }
    /// 设置已选中（`Selected`）。
    pub fn set_selected(&self, v: bool) -> bool { self.put_bool("Selected", v) }

    // ---- 关系 ----

    /// 读取子路径（`SubPath`）。
    pub fn sub_path(&self) -> Option<IvgSubPath> {
        self.prop_dispatch("SubPath").map(IvgSubPath::new)
    }

    /// 读取线段（`Segment`）。
    pub fn segment(&self) -> Option<IvgSegment> {
        self.prop_dispatch("Segment").map(IvgSegment::new)
    }

    /// 读取 COM 属性 `PrevSegment`。
    pub fn prev_segment(&self) -> Option<IvgSegment> {
        self.prop_dispatch("PrevSegment").map(IvgSegment::new)
    }

    /// 读取下一个线段（`NextSegment`）。
    pub fn next_segment(&self) -> Option<IvgSegment> {
        self.prop_dispatch("NextSegment").map(IvgSegment::new)
    }

    /// 下一个（COM 方法 `Next`）。
    pub fn next(&self) -> Option<IvgNode> {
        self.disp.invoke_method("Next", vec![]).ok()?.to_idispatch().ok().map(IvgNode::new)
    }

    /// 上一个（COM 方法 `Previous`）。
    pub fn previous(&self) -> Option<IvgNode> {
        self.disp.invoke_method("Previous", vec![]).ok()?.to_idispatch().ok().map(IvgNode::new)
    }

    // ---- 操作 ----

    /// 连接（COM 方法 `JoinWith`）。
    pub fn join_with(&self, target: &IvgNode) -> bool {
        let args = vec![target.as_variant()];
        self.disp.invoke_method("JoinWith", args).is_ok()
    }

    /// 调用 COM 方法 `ConnectWith`。
    pub fn connect_with(&self, target: &IvgNode) -> bool {
        let args = vec![target.as_variant()];
        self.disp.invoke_method("ConnectWith", args).is_ok()
    }

    /// 断开分开（COM 方法 `BreakApart`）。
    pub fn break_apart(&self) -> bool { self.disp.invoke_method("BreakApart", vec![]).is_ok() }
    /// 删除（COM 方法 `Delete`）。
    pub fn delete(&self) -> bool { self.disp.invoke_method("Delete", vec![]).is_ok() }

    /// 圆角（COM 方法 `Fillet`）。
    pub fn fillet(&self, radius: f64, combine: bool) -> Option<bool> {
        let args = vec![
            Variant::from_f64(radius),
            Variant::from_bool(combine),
        ];
        self.disp.invoke_method("Fillet", args).ok()?.to_bool().ok()
    }

    /// 倒角（COM 方法 `Chamfer`）。
    pub fn chamfer(&self, a: f64, b: f64, combine: bool) -> Option<bool> {
        let args = vec![
            Variant::from_f64(a),
            Variant::from_f64(b),
            Variant::from_bool(combine),
        ];
        self.disp.invoke_method("Chamfer", args).ok()?.to_bool().ok()
    }

    /// 扇贝（COM 方法 `Scallop`）。
    pub fn scallop(&self, radius: f64, combine: bool) -> Option<bool> {
        let args = vec![
            Variant::from_f64(radius),
            Variant::from_bool(combine),
        ];
        self.disp.invoke_method("Scallop", args).ok()?.to_bool().ok()
    }

    /// 创建选择（COM 方法 `CreateSelection`）。
    pub fn create_selection(&self) -> bool {
        self.disp.invoke_method("CreateSelection", vec![]).is_ok()
    }

    /// 读取距离（`GetDistanceFrom`）。
    pub fn get_distance_from(&self, other: &IvgNode) -> Option<f64> {
        let args = vec![other.as_variant()];
        self.disp.invoke_method("GetDistanceFrom", args).ok()?.to_f64().ok()
    }

    /// 调用 COM 方法 `ExtendSubPaths`。
    pub fn extend_sub_paths(&self, other: &IvgNode, join: bool) -> Option<bool> {
        let args = vec![
            other.as_variant(),
            Variant::from_bool(join),
        ];
        self.disp.invoke_method("ExtendSubPaths", args).ok()?.to_bool().ok()
    }

    /// 调用 COM 方法 `AveragePositionWith`。
    pub fn average_position_with(&self, other: &IvgNode, join: bool) -> bool {
        let args = vec![
            other.as_variant(),
            Variant::from_bool(join),
        ];
        self.disp.invoke_method("AveragePositionWith", args).is_ok()
    }
}