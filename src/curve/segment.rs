//! `IVGSegment` / `IVGSegments` —— 线段

use wincom::{ComObject, Variant};
use windows::Win32::System::Com::IDispatch;

use crate::curve::{IvgCrossPoints, IvgNode, IvgSegmentRange, IvgSubPath};

// =============================================================
// IvgSegments
// =============================================================

/// CorelDRAW `IVGSegment` 对象的后期绑定包装（线段集）。
///
/// 所有属性与方法通过 `IDispatch::Invoke` 按名称调用，调用失败时一般返回 `None` 或 `false`。
pub struct IvgSegments {
    disp: ComObject,
}

impl IvgSegments {
    /// 由 COM `IDispatch` 构造包装对象。
    pub fn new(disp: IDispatch) -> Self { Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) } }

    fn prop_i64(&self, name: &str) -> Option<i64> {
        self.disp.get_property(name).ok()?.to_i64().ok()
    }

    /// 读取返回条目数量（`Count`）。
    pub fn count(&self) -> Option<i64> { self.prop_i64("Count") }

    /// 按索引或名称取条目（COM 方法 `Item`）。
    pub fn item(&self, index: i32) -> Option<IvgSegment> {
        let args = vec![Variant::from_i64(index as i64)];
        self.disp.invoke_method("Item", args).ok()?.to_idispatch().ok().map(IvgSegment::new)
    }

    /// 读取返回第一个条目（`First`）。
    pub fn first(&self) -> Option<IvgSegment> {
        self.disp.get_property("First").ok()?.to_idispatch().ok().map(IvgSegment::new)
    }

    /// 读取返回最后一个条目（`Last`）。
    pub fn last(&self) -> Option<IvgSegment> {
        self.disp.get_property("Last").ok()?.to_idispatch().ok().map(IvgSegment::new)
    }

    /// 范围（COM 方法 `Range`）。
    pub fn range(&self, indices:Variant ) -> Option<IvgSegmentRange> {
        let args = vec![indices];
        self.disp.invoke_method("Range", args).ok()?.to_idispatch().ok().map(IvgSegmentRange::new)
    }

    /// 返回全部条目（COM 方法 `All`）。
    pub fn all(&self) -> Option<IvgSegmentRange> {
        self.disp.invoke_method("All", vec![]).ok()?.to_idispatch().ok().map(IvgSegmentRange::new)
    }
}

// =============================================================
// IvgSegment
// =============================================================

/// CorelDRAW `IVGSegment` 对象的后期绑定包装（线段）。
///
/// 所有属性与方法通过 `IDispatch::Invoke` 按名称调用，调用失败时一般返回 `None` 或 `false`。
pub struct IvgSegment {
    disp: ComObject,
}

impl IvgSegment {
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

    // ---- 类型 / 索引 ----

    /// `cdrSegmentType` 瑙?`enums::curve`
    pub fn segment_type(&self) -> Option<i64> { self.prop_i64("Type") }
    /// 设置线段类型（`Type`）。
    pub fn set_segment_type(&self, v: i32) -> bool { self.put_i64("Type", v as i64) }

    /// 读取索引（`Index`）。
    pub fn index(&self) -> Option<i64> { self.prop_i64("Index") }
    /// 读取子路径索引（`SubPathIndex`）。
    pub fn sub_path_index(&self) -> Option<i64> { self.prop_i64("SubPathIndex") }
    /// 读取绝对索引（`AbsoluteIndex`）。
    pub fn absolute_index(&self) -> Option<i64> { self.prop_i64("AbsoluteIndex") }
    /// 读取长度（`Length`）。
    pub fn length(&self) -> Option<f64> { self.prop_f64("Length") }

    /// 是否已选中（`Selected`）。
    pub fn is_selected(&self) -> Option<bool> { self.prop_bool("Selected") }
    /// 设置已选中（`Selected`）。
    pub fn set_selected(&self, v: bool) -> bool { self.put_bool("Selected", v) }

    // ---- 关联 ----

    /// 读取子路径（`SubPath`）。
    pub fn sub_path(&self) -> Option<IvgSubPath> {
        self.prop_dispatch("SubPath").map(IvgSubPath::new)
    }

    /// 读取启动节点（`StartNode`）。
    pub fn start_node(&self) -> Option<IvgNode> {
        self.prop_dispatch("StartNode").map(IvgNode::new)
    }

    /// 读取结束节点（`EndNode`）。
    pub fn end_node(&self) -> Option<IvgNode> {
        self.prop_dispatch("EndNode").map(IvgNode::new)
    }

    // ---- 控制点（角度 + 长度）----

    /// 读取起始控件点长度（`StartingControlPointLength`）。
    pub fn starting_control_point_length(&self) -> Option<f64> {
        self.prop_f64("StartingControlPointLength")
    }
    /// 设置起始控件点长度（`StartingControlPointLength`）。
    pub fn set_starting_control_point_length(&self, v: f64) -> bool {
        self.put_f64("StartingControlPointLength", v)
    }

    /// 读取起始控件点角度（`StartingControlPointAngle`）。
    pub fn starting_control_point_angle(&self) -> Option<f64> {
        self.prop_f64("StartingControlPointAngle")
    }
    /// 设置起始控件点角度（`StartingControlPointAngle`）。
    pub fn set_starting_control_point_angle(&self, v: f64) -> bool {
        self.put_f64("StartingControlPointAngle", v)
    }

    /// 读取末端控件点长度（`EndingControlPointLength`）。
    pub fn ending_control_point_length(&self) -> Option<f64> {
        self.prop_f64("EndingControlPointLength")
    }
    /// 设置末端控件点长度（`EndingControlPointLength`）。
    pub fn set_ending_control_point_length(&self, v: f64) -> bool {
        self.put_f64("EndingControlPointLength", v)
    }

    /// 读取末端控件点角度（`EndingControlPointAngle`）。
    pub fn ending_control_point_angle(&self) -> Option<f64> {
        self.prop_f64("EndingControlPointAngle")
    }
    /// 设置末端控件点角度（`EndingControlPointAngle`）。
    pub fn set_ending_control_point_angle(&self, v: f64) -> bool {
        self.put_f64("EndingControlPointAngle", v)
    }

    // ---- 控制点（x + y）----

    /// 读取起始控件点 X 坐标（`StartingControlPointX`）。
    pub fn starting_control_point_x(&self) -> Option<f64> {
        self.prop_f64("StartingControlPointX")
    }
    /// 设置起始控件点 X 坐标（`StartingControlPointX`）。
    pub fn set_starting_control_point_x(&self, v: f64) -> bool {
        self.put_f64("StartingControlPointX", v)
    }

    /// 读取起始控件点 Y 坐标（`StartingControlPointY`）。
    pub fn starting_control_point_y(&self) -> Option<f64> {
        self.prop_f64("StartingControlPointY")
    }
    /// 设置起始控件点 Y 坐标（`StartingControlPointY`）。
    pub fn set_starting_control_point_y(&self, v: f64) -> bool {
        self.put_f64("StartingControlPointY", v)
    }

    /// 读取末端控件点 X 坐标（`EndingControlPointX`）。
    pub fn ending_control_point_x(&self) -> Option<f64> {
        self.prop_f64("EndingControlPointX")
    }
    /// 设置末端控件点 X 坐标（`EndingControlPointX`）。
    pub fn set_ending_control_point_x(&self, v: f64) -> bool {
        self.put_f64("EndingControlPointX", v)
    }

    /// 读取末端控件点 Y 坐标（`EndingControlPointY`）。
    pub fn ending_control_point_y(&self) -> Option<f64> {
        self.prop_f64("EndingControlPointY")
    }
    /// 设置末端控件点 Y 坐标（`EndingControlPointY`）。
    pub fn set_ending_control_point_y(&self, v: f64) -> bool {
        self.put_f64("EndingControlPointY", v)
    }

    // ---- 遍历 ----

    /// 下一个（COM 方法 `Next`）。
    pub fn next(&self) -> Option<IvgSegment> {
        self.disp.invoke_method("Next", vec![]).ok()?.to_idispatch().ok().map(IvgSegment::new)
    }

    /// 上一个（COM 方法 `Previous`）。
    pub fn previous(&self) -> Option<IvgSegment> {
        self.disp.invoke_method("Previous", vec![]).ok()?.to_idispatch().ok().map(IvgSegment::new)
    }

    // ---- 编辑 ----

    /// 读取复制（`GetCopy`）。
    pub fn get_copy(&self) -> Option<crate::curve::IvgCurve> {
        self.disp.invoke_method("GetCopy", vec![]).ok()?.to_idispatch().ok()
            .map(crate::curve::IvgCurve::new)
    }

    /// 创建选择（COM 方法 `CreateSelection`）。
    pub fn create_selection(&self) -> bool {
        self.disp.invoke_method("CreateSelection", vec![]).is_ok()
    }

    /// 添加节点在（COM 方法 `AddNodeAt`）。
    pub fn add_node_at(&self, offset: f64, offset_type: i32) -> Option<IvgNode> {
        let args = vec![
            Variant::from_f64(offset),
            Variant::from_i64(offset_type as i64),
        ];
        self.disp.invoke_method("AddNodeAt", args).ok()?.to_idispatch().ok().map(IvgNode::new)
    }

    /// 断开分开在（COM 方法 `BreakApartAt`）。
    pub fn break_apart_at(&self, offset: f64, offset_type: i32) -> Option<IvgNode> {
        let args = vec![
            Variant::from_f64(offset),
            Variant::from_i64(offset_type as i64),
        ];
        self.disp.invoke_method("BreakApartAt", args).ok()?.to_idispatch().ok().map(IvgNode::new)
    }

    // ---- 几何查询 ----

    /// 读取 COM 属性 `GetPerpendicularAt`。
    pub fn get_perpendicular_at(&self, offset: f64, offset_type: i32) -> Option<f64> {
        let args = vec![
            Variant::from_f64(offset),
            Variant::from_i64(offset_type as i64),
        ];
        self.disp.invoke_method("GetPerpendicularAt", args).ok()?.to_f64().ok()
    }

    /// 读取 COM 属性 `GetTangentAt`。
    pub fn get_tangent_at(&self, offset: f64, offset_type: i32) -> Option<f64> {
        let args = vec![
            Variant::from_f64(offset),
            Variant::from_i64(offset_type as i64),
        ];
        self.disp.invoke_method("GetTangentAt", args).ok()?.to_f64().ok()
    }

    /// 读取 COM 属性 `GetCurvatureAt`。
    pub fn get_curvature_at(&self, offset: f64, offset_type: i32) -> Option<f64> {
        let args = vec![
            Variant::from_f64(offset),
            Variant::from_i64(offset_type as i64),
        ];
        self.disp.invoke_method("GetCurvatureAt", args).ok()?.to_f64().ok()
    }

    /// 读取 COM 属性 `GetIntersections`。
    pub fn get_intersections(&self, target: &IvgSegment, offset_type: i32) -> Option<IvgCrossPoints> {
        let args = vec![
            target.as_variant(),
            Variant::from_i64(offset_type as i64),
        ];
        self.disp.invoke_method("GetIntersections", args).ok()?.to_idispatch().ok().map(IvgCrossPoints::new)
    }

    /// 是否边缘上的矩形（COM 方法 `IsRectOnEdge`）。
    pub fn is_rect_on_edge(&self, x1: f64, y1: f64, x2: f64, y2: f64) -> Option<bool> {
        let args = vec![
            Variant::from_f64(x1),
            Variant::from_f64(y1),
            Variant::from_f64(x2),
            Variant::from_f64(y2),
        ];
        self.disp.invoke_method("IsRectOnEdge", args).ok()?.to_bool().ok()
    }
}