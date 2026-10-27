//! `IVGSubPath` / `IVGSubPaths` —— 子路径

use wincom::{ComObject, Variant};
use windows::Win32::System::Com::IDispatch;

use crate::curve::{
    IvgCurve, IvgNode, IvgNodeRange, IvgNodes,
    IvgSegment, IvgSegments,
};
use crate::geometry::IvgRect;

// =============================================================
// IvgSubPaths
// =============================================================

/// CorelDRAW `IVGSubPath` 对象的后期绑定包装（子路径集）。
///
/// 所有属性与方法通过 `IDispatch::Invoke` 按名称调用，调用失败时一般返回 `None` 或 `false`。
pub struct IvgSubPaths {
    disp: ComObject,
}

impl IvgSubPaths {
    /// 由 COM `IDispatch` 构造包装对象。
    pub fn new(disp: IDispatch) -> Self { Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) } }

    fn prop_i64(&self, name: &str) -> Option<i64> {
        self.disp.get_property(name).ok()?.to_i64().ok()
    }

    /// 读取返回条目数量（`Count`）。
    pub fn count(&self) -> Option<i64> { self.prop_i64("Count") }

    /// 按索引或名称取条目（COM 方法 `Item`）。
    pub fn item(&self, index: i32) -> Option<IvgSubPath> {
        let args = vec![Variant::from_i64(index as i64)];
        self.disp.invoke_method("Item", args).ok()?.to_idispatch().ok().map(IvgSubPath::new)
    }

    /// 读取返回第一个条目（`First`）。
    pub fn first(&self) -> Option<IvgSubPath> {
        self.disp.get_property("First").ok()?.to_idispatch().ok().map(IvgSubPath::new)
    }

    /// 读取返回最后一个条目（`Last`）。
    pub fn last(&self) -> Option<IvgSubPath> {
        self.disp.get_property("Last").ok()?.to_idispatch().ok().map(IvgSubPath::new)
    }
}

// =============================================================
// IvgSubPath
// =============================================================

/// CorelDRAW `IVGSubPath` 对象的后期绑定包装（子路径）。
///
/// 所有属性与方法通过 `IDispatch::Invoke` 按名称调用，调用失败时一般返回 `None` 或 `false`。
pub struct IvgSubPath {
    disp: ComObject,
}

impl IvgSubPath {
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

    fn put_bool(&self, name: &str, v: bool) -> bool {
        let arg = Variant::from_bool(v);
        self.disp.set_property(name, vec![arg]).is_ok()
    }

    fn put_f64(&self, name: &str, v: f64) -> bool {
        let arg = Variant::from_f64(v);
        self.disp.set_property(name, vec![arg]).is_ok()
    }

    // ---- 属性----

    /// 读取节点集（`Nodes`）。
    pub fn nodes(&self) -> Option<IvgNodes> { self.prop_dispatch("Nodes").map(IvgNodes::new) }
    /// 读取线段集（`Segments`）。
    pub fn segments(&self) -> Option<IvgSegments> { self.prop_dispatch("Segments").map(IvgSegments::new) }
    /// 读取索引（`Index`）。
    pub fn index(&self) -> Option<i64> { self.prop_i64("Index") }
    /// 读取长度（`Length`）。
    pub fn length(&self) -> Option<f64> { self.prop_f64("Length") }

    /// 是否闭合（`Closed`）。
    pub fn is_closed(&self) -> Option<bool> { self.prop_bool("Closed") }
    /// 设置闭合（`Closed`）。
    pub fn set_closed(&self, v: bool) -> bool { self.put_bool("Closed", v) }

    /// 读取位置 X 坐标（`PositionX`）。
    pub fn position_x(&self) -> Option<f64> { self.prop_f64("PositionX") }
    /// 设置位置 X 坐标（`PositionX`）。
    pub fn set_position_x(&self, v: f64) -> bool { self.put_f64("PositionX", v) }

    /// 读取位置 Y 坐标（`PositionY`）。
    pub fn position_y(&self) -> Option<f64> { self.prop_f64("PositionY") }
    /// 设置位置 Y 坐标（`PositionY`）。
    pub fn set_position_y(&self, v: f64) -> bool { self.put_f64("PositionY", v) }

    /// 读取尺寸宽度（`SizeWidth`）。
    pub fn size_width(&self) -> Option<f64> { self.prop_f64("SizeWidth") }
    /// 读取尺寸高度（`SizeHeight`）。
    pub fn size_height(&self) -> Option<f64> { self.prop_f64("SizeHeight") }

    /// 读取启动节点（`StartNode`）。
    pub fn start_node(&self) -> Option<IvgNode> {
        self.prop_dispatch("StartNode").map(IvgNode::new)
    }

    /// 读取结束节点（`EndNode`）。
    pub fn end_node(&self) -> Option<IvgNode> {
        self.prop_dispatch("EndNode").map(IvgNode::new)
    }

    /// 读取第一个线段（`FirstSegment`）。
    pub fn first_segment(&self) -> Option<IvgSegment> {
        self.prop_dispatch("FirstSegment").map(IvgSegment::new)
    }

    /// 读取最后一个线段（`LastSegment`）。
    pub fn last_segment(&self) -> Option<IvgSegment> {
        self.prop_dispatch("LastSegment").map(IvgSegment::new)
    }

    /// 是否顺时针（`IsClockwise`）。
    pub fn is_clockwise(&self) -> Option<bool> { self.prop_bool("IsClockwise") }
    /// 读取区域（`Area`）。
    pub fn area(&self) -> Option<f64> { self.prop_f64("Area") }
    /// 读取边界框（`BoundingBox`）。
    pub fn bounding_box(&self) -> Option<IvgRect> {
        self.prop_dispatch("BoundingBox").map(IvgRect::new)
    }

    // ---- 选择 / 方向 ----

    /// 选择（COM 方法 `Selection`）。
    pub fn selection(&self) -> Option<IvgNodeRange> {
        self.disp.invoke_method("Selection", vec![]).ok()?.to_idispatch().ok().map(IvgNodeRange::new)
    }

    /// 反转方向（COM 方法 `ReverseDirection`）。
    pub fn reverse_direction(&self) -> bool {
        self.disp.invoke_method("ReverseDirection", vec![]).is_ok()
    }

    /// 下一个（COM 方法 `Next`）。
    pub fn next(&self) -> Option<IvgSubPath> {
        self.disp.invoke_method("Next", vec![]).ok()?.to_idispatch().ok().map(IvgSubPath::new)
    }

    /// 上一个（COM 方法 `Previous`）。
    pub fn previous(&self) -> Option<IvgSubPath> {
        self.disp.invoke_method("Previous", vec![]).ok()?.to_idispatch().ok().map(IvgSubPath::new)
    }

    // ---- 追加段----

    /// 追加线线段（COM 方法 `AppendLineSegment`）。
    pub fn append_line_segment(&self, x: f64, y: f64, at_beginning: bool) -> Option<IvgSegment> {
        let args = vec![
            Variant::from_f64(x),
            Variant::from_f64(y),
            Variant::from_bool(at_beginning),
        ];
        self.disp.invoke_method("AppendLineSegment", args).ok()?.to_idispatch().ok().map(IvgSegment::new)
    }

    /// 追加曲线线段（COM 方法 `AppendCurveSegment`）。
    pub fn append_curve_segment(
        &self,
        x: f64, y: f64,
        sc_len: f64, sc_angle: f64,
        ec_len: f64, ec_angle: f64,
        at_beginning: bool,
    ) -> Option<IvgSegment> {
        let args = vec![
            Variant::from_f64(x),
            Variant::from_f64(y),
            Variant::from_f64(sc_len),
            Variant::from_f64(sc_angle),
            Variant::from_f64(ec_len),
            Variant::from_f64(ec_angle),
            Variant::from_bool(at_beginning),
        ];
        self.disp.invoke_method("AppendCurveSegment", args).ok()?.to_idispatch().ok().map(IvgSegment::new)
    }

    // ---- 编辑 ----

    /// 删除（COM 方法 `Delete`）。
    pub fn delete(&self) -> bool { self.disp.invoke_method("Delete", vec![]).is_ok() }

    /// 移动（COM 方法 `Move`）。
    pub fn move_by(&self, dx: f64, dy: f64) -> bool {
        let args = vec![
            Variant::from_f64(dx),
            Variant::from_f64(dy),
        ];
        self.disp.invoke_method("Move", args).is_ok()
    }

    /// 读取位置。
    pub fn get_position(&self) -> Option<(f64, f64)> {
        // 双出参简化：取属性
        Some((self.position_x()?, self.position_y()?))
    }

    /// 设置位置。
    pub fn set_position(&self, x: f64, y: f64) -> bool {
        self.set_position_x(x) && self.set_position_y(y)
    }

    // ---- 抽离 ----

    /// 提取。
    pub fn extract(&self) -> Option<(crate::shape::IvgShape, crate::shape::IvgShape)> {
        // COM 双出参：`Extract(OldCurve, ppVal)`。
        // 通过 IDispatch 只能拿一个；这里保留占位，建议直接改用
        // `shape.curve().sub_paths().item(n)` 提取。
        None
    }

    // ---- 几何查询 ----

    /// 是否于子路径（COM 方法 `IsOnSubPath`）。
    pub fn is_on_sub_path(&self, x: f64, y: f64, hot_area: f64) -> Option<i64> {
        let args = vec![
            Variant::from_f64(x),
            Variant::from_f64(y),
            Variant::from_f64(hot_area),
        ];
        self.disp.invoke_method("IsOnSubPath", args).ok()?.to_i64().ok()
    }

    /// 是否点内部（COM 方法 `IsPointInside`）。
    pub fn is_point_inside(&self, x: f64, y: f64) -> Option<bool> {
        let args = vec![
            Variant::from_f64(x),
            Variant::from_f64(y),
        ];
        self.disp.invoke_method("IsPointInside", args).ok()?.to_bool().ok()
    }

    /// 读取复制（`GetCopy`）。
    pub fn get_copy(&self) -> Option<IvgCurve> {
        self.disp.invoke_method("GetCopy", vec![]).ok()?.to_idispatch().ok().map(IvgCurve::new)
    }
}