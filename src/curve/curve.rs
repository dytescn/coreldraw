//! `IVGCurve` —— 曲线

use wincom::{ComObject, Variant};
use windows::Win32::System::Com::IDispatch;

use crate::curve::{
    IvgNode, IvgNodeRange, IvgSegment, IvgSubPath, IvgSubPaths,
};
use crate::geometry::IvgRect;
use crate::shape::IvgShape;

/// CorelDRAW `IVGCurve` 对象的后期绑定包装（曲线）。
///
/// 所有属性与方法通过 `IDispatch::Invoke` 按名称调用，调用失败时一般返回 `None` 或 `false`。
pub struct IvgCurve {
    disp: ComObject,
}

impl IvgCurve {
    /// 由 COM `IDispatch` 构造包装对象。
    pub fn new(disp: IDispatch) -> Self {
        Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) }
    }

    /// 从 COM `IDispatch` 构造包装对象，等价于 `new`。
    pub fn from_disp(disp: IDispatch) -> Self {
        Self::new(disp)
    }

    /// 获取底层 `ComObject` 的引用。
    pub fn raw(&self) -> &ComObject {
        &self.disp
    }

    /// 把底层 COM 对象包装成 `VARIANT`，用于传给其它 COM 调用。
    pub fn as_variant(&self) -> Variant {
        self.disp.as_variant()
    }

    // ---------------------------------------------------------
    // 通用属性读取
    // ---------------------------------------------------------
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

    // ---------------------------------------------------------
    // 基本属性
    // ---------------------------------------------------------

    /// 曲线总长
    pub fn length(&self) -> Option<f64> {
        self.prop_f64("Length")
    }

    /// 读取子路径集（`SubPaths`）。
    pub fn sub_paths(&self) -> Option<IvgSubPaths> {
        self.prop_dispatch("SubPaths").map(IvgSubPaths::new)
    }

    /// 读取节点集（`Nodes`）。
    pub fn nodes(&self) -> Option<crate::curve::node::IvgNodes> {
        self.prop_dispatch("Nodes").map(crate::curve::node::IvgNodes::new)
    }

    /// 读取线段集（`Segments`）。
    pub fn segments(&self) -> Option<crate::curve::segment::IvgSegments> {
        self.prop_dispatch("Segments").map(crate::curve::segment::IvgSegments::new)
    }

    /// 是否闭合（`Closed`）。
    pub fn is_closed(&self) -> Option<bool> {
        self.prop_bool("Closed")
    }

    /// 设置闭合（`Closed`）。
    pub fn set_closed(&self, v: bool) -> bool {
        self.put_bool("Closed", v)
    }

    // ---------------------------------------------------------
    // 选择
    // ---------------------------------------------------------

    /// 返回当前选中节点集合（`IVGNodeRange`）
    pub fn selection(&self) -> Option<IvgNodeRange> {
        self.disp
            .invoke_method("Selection", vec![])
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgNodeRange::new)
    }

    /// 清除选择（COM 方法 `ClearSelection`）。
    pub fn clear_selection(&self) -> bool {
        self.disp.invoke_method("ClearSelection", vec![]).is_ok()
    }

    // ---------------------------------------------------------
    // 子路径构造
    // ---------------------------------------------------------

    /// 新建一个从 (x, y) 开始的子路径。
    pub fn create_sub_path(&self, x: f64, y: f64) -> Option<IvgSubPath> {
        let args = vec![
            Variant::from_f64(x),
            Variant::from_f64(y),
        ];
        self.disp
            .invoke_method("CreateSubPath", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgSubPath::new)
    }

    /// 用坐标数组批量建子路径。
    pub fn create_sub_path_from_array(
        &self,
        coords: Variant,
        closed: bool,
        num_elements: i32,
    ) -> Option<IvgSubPath> {
        let args = vec![
            coords,
            Variant::from_bool(closed),
            Variant::from_i64(num_elements as i64),
        ];
        self.disp
            .invoke_method("CreateSubPathFromArray", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgSubPath::new)
    }

    // ---------------------------------------------------------
    // 追加图形
    // ---------------------------------------------------------

    /// 调用 COM 方法 `AppendSubpathCircle`。
    pub fn append_subpath_circle(&self, cx: f64, cy: f64, r: f64) -> bool {
        let args = vec![
            Variant::from_f64(cx),
            Variant::from_f64(cy),
            Variant::from_f64(r),
        ];
        self.disp.invoke_method("AppendSubpathCircle", args).is_ok()
    }

    /// 追加子路径矩形（COM 方法 `AppendSubpathRectangle`）。
    pub fn append_subpath_rectangle(&self, l: f64, t: f64, r: f64, b: f64) -> bool {
        let args = vec![
            Variant::from_f64(l),
            Variant::from_f64(t),
            Variant::from_f64(r),
            Variant::from_f64(b),
        ];
        self.disp.invoke_method("AppendSubpathRectangle", args).is_ok()
    }

    /// 调用 COM 方法 `AppendSubpathThreePointArc`。
    pub fn append_subpath_three_point_arc(
        &self,
        sx: f64, sy: f64, ex: f64, ey: f64, tx: f64, ty: f64,
    ) -> bool {
        let args = vec![
            Variant::from_f64(sx),
            Variant::from_f64(sy),
            Variant::from_f64(ex),
            Variant::from_f64(ey),
            Variant::from_f64(tx),
            Variant::from_f64(ty),
        ];
        self.disp.invoke_method("AppendSubpathThreePointArc", args).is_ok()
    }

    /// 追加子路径椭圆（COM 方法 `AppendSubpathEllipse`）。
    pub fn append_subpath_ellipse(&self, cx: f64, cy: f64, rh: f64, rv: f64) -> bool {
        let args = vec![
            Variant::from_f64(cx),
            Variant::from_f64(cy),
            Variant::from_f64(rh),
            Variant::from_f64(rv),
        ];
        self.disp.invoke_method("AppendSubpathEllipse", args).is_ok()
    }

    /// 把另一条曲线追加到当前曲线。
    pub fn append_curve(&self, other: &IvgCurve) -> bool {
        let args = vec![other.as_variant()];
        self.disp.invoke_method("AppendCurve", args).is_ok()
    }

    // ---------------------------------------------------------
    // 方向 / 闭合
    // ---------------------------------------------------------

    /// 反转方向（COM 方法 `ReverseDirection`）。
    pub fn reverse_direction(&self) -> bool {
        self.disp.invoke_method("ReverseDirection", vec![]).is_ok()
    }

    /// 调用 COM 方法 `SelfWeldClosedSubpaths`。
    pub fn self_weld_closed_subpaths(&self) -> bool {
        self.disp.invoke_method("SelfWeldClosedSubpaths", vec![]).is_ok()
    }

    // ---------------------------------------------------------
    // 几何查询
    // ---------------------------------------------------------

    /// 是否于曲线（COM 方法 `IsOnCurve`）。
    pub fn is_on_curve(&self, x: f64, y: f64, hot_area: f64) -> Option<i64> {
        let args = vec![
            Variant::from_f64(x),
            Variant::from_f64(y),
            Variant::from_f64(hot_area),
        ];
        self.disp.invoke_method("IsOnCurve", args).ok()?.to_i64().ok()
    }

    /// 是否点内部（COM 方法 `IsPointInside`）。
    pub fn is_point_inside(&self, x: f64, y: f64) -> Option<bool> {
        let args = vec![
            Variant::from_f64(x),
            Variant::from_f64(y),
        ];
        self.disp.invoke_method("IsPointInside", args).ok()?.to_bool().ok()
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

    /// 是否顺时针（`IsClockwise`）。
    pub fn is_clockwise(&self) -> Option<bool> {
        self.prop_bool("IsClockwise")
    }

    /// 读取区域（`Area`）。
    pub fn area(&self) -> Option<f64> {
        self.prop_f64("Area")
    }

    /// 读取边界框（`BoundingBox`）。
    pub fn bounding_box(&self) -> Option<IvgRect> {
        self.prop_dispatch("BoundingBox").map(IvgRect::new)
    }

    /// 由 `(x, y, w, h)` 输出包围盒
    pub fn get_bounding_box(&self) -> Option<(f64, f64, f64, f64)> {
        // 该方法有 5 个 out 参数，改用 IDispatch 后期绑定比较麻烦。
        // 简单起见，用 BoundingBox 属性替代。
        let r = self.bounding_box()?;
        Some((r.x().unwrap_or(0.0), r.y().unwrap_or(0.0), r.width().unwrap_or(0.0), r.height().unwrap_or(0.0)))
    }

    // ---------------------------------------------------------
    // 查找
    // ---------------------------------------------------------

    /// 查找点处的节点（COM 方法 `FindNodeAtPoint`）。
    pub fn find_node_at_point(&self, x: f64, y: f64, hot_area: f64) -> Option<IvgNode> {
        let args = vec![
            Variant::from_f64(x),
            Variant::from_f64(y),
            Variant::from_f64(hot_area),
        ];
        self.disp
            .invoke_method("FindNodeAtPoint", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgNode::new)
    }

    /// 查找最近线段（COM 方法 `FindClosestSegment`）。
    pub fn find_closest_segment(&self, x: f64, y: f64) -> Option<(IvgSegment, f64)> {
        // 返回 (段 参数偏移)。COM 双出参在 IDispatch 里拿不到，改用近似：
        // 直接调用，拿第一个返回的 IDispatch。
        let args = vec![
            Variant::from_f64(x),
            Variant::from_f64(y),
        ];
        let seg = self.disp
            .invoke_method("FindClosestSegment", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgSegment::new)?;
        Some((seg, 0.0))
    }

    // ---------------------------------------------------------
    // 变换
    // ---------------------------------------------------------

    /// 应用变换矩阵（COM 方法 `ApplyTransformMatrix`）。
    pub fn apply_transform_matrix(&self, matrix: &crate::geometry::IvgTransformMatrix) -> bool {
        let args = vec![matrix.as_variant()];
        self.disp.invoke_method("ApplyTransformMatrix", args).is_ok()
    }

    // ---------------------------------------------------------
    // 复制 / 焊接
    // ---------------------------------------------------------

    /// 读取复制（`GetCopy`）。
    pub fn get_copy(&self) -> Option<IvgCurve> {
        self.disp.invoke_method("GetCopy", vec![]).ok()?.to_idispatch().ok().map(IvgCurve::new)
    }

    /// 复制 assign（COM 方法 `CopyAssign`）。
    pub fn copy_assign(&self, other: &IvgCurve) -> bool {
        let args = vec![other.as_variant()];
        self.disp.invoke_method("CopyAssign", args).is_ok()
    }

    /// 焊接（COM 方法 `WeldWith`）。
    pub fn weld_with(&self, other: &IvgCurve) -> Option<IvgCurve> {
        let args = vec![other.as_variant()];
        self.disp
            .invoke_method("WeldWith", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgCurve::new)
    }

    /// `cdrWeldMethod` 瑙?`enums::curve`
    pub fn weld_ex(
        &self,
        target: &IvgCurve,
        method: i32,
        winding_source: bool,
        winding_target: bool,
        flags: i32,
    ) -> Option<IvgCurve> {
        let args = vec![
            target.as_variant(),
            Variant::from_i64(method as i64),
            Variant::from_bool(winding_source),
            Variant::from_bool(winding_target),
            Variant::from_i64(flags as i64),
        ];
        self.disp
            .invoke_method("WeldEx", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgCurve::new)
    }

    // ---------------------------------------------------------
    // 其它
    // ---------------------------------------------------------

    /// 绑定文档（COM 方法 `BindToDocument`）。
    pub fn bind_to_document(&self, doc: &crate::document::IvgDocument) -> bool {
        let args = vec![doc.as_variant()];
        self.disp.invoke_method("BindToDocument", args).is_ok()
    }

    /// 读取 COM 属性 `GetPolyline`。
    pub fn get_polyline(&self, precision: i32) -> Option<IvgCurve> {
        let args = vec![Variant::from_i64(precision as i64)];
        self.disp
            .invoke_method("GetPolyline", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgCurve::new)
    }

    /// 调用 COM 方法 `RemoveOverlaps`。
    pub fn remove_overlaps(&self) -> Option<IvgCurve> {
        self.disp
            .invoke_method("RemoveOverlaps", vec![])
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgCurve::new)
    }

    /// 调用 COM 方法 `AutoReduceNodes`。
    pub fn auto_reduce_nodes(&self, amount: f64, selected_only: bool) -> bool {
        let args = vec![
            Variant::from_f64(amount),
            Variant::from_bool(selected_only),
        ];
        self.disp.invoke_method("AutoReduceNodes", args).is_ok()
    }

    /// 调用 COM 方法 `JoinTouchingSubpaths`。
    pub fn join_touching_subpaths(&self, allow_reversal: bool, tolerance: f64) -> bool {
        let args = vec![
            Variant::from_bool(allow_reversal),
            Variant::from_f64(tolerance),
        ];
        self.disp.invoke_method("JoinTouchingSubpaths", args).is_ok()
    }

    // ---------------------------------------------------------
    // 与 Shape 的互操作（如果 curve 是从 shape 里拿到的）
    // ---------------------------------------------------------

    /// 转换为图形。
    pub fn to_shape(&self) -> Option<IvgShape> {
        // IVGCurve 本身不直接暴露；如果业务需要，通过
        // `layer.create_curve(shape)` 走。
        None
    }
}