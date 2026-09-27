//! `IVGCurve` 鈥斺€?鏇茬嚎

use wincom::{ComObject, Variant};
use windows::Win32::System::Com::IDispatch;

use crate::curve::{
    IvgNode, IvgNodeRange, IvgSegment, IvgSubPath, IvgSubPaths,
};
use crate::geometry::IvgRect;
use crate::shape::IvgShape;

pub struct IvgCurve {
    disp: ComObject,
}

impl IvgCurve {
    pub fn new(disp: IDispatch) -> Self {
        Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) }
    }

    pub fn from_disp(disp: IDispatch) -> Self {
        Self::new(disp)
    }

    pub fn raw(&self) -> &ComObject {
        &self.disp
    }

    pub fn as_variant(&self) -> Variant {
        self.disp.as_variant()
    }

    // ---------------------------------------------------------
    // 閫氱敤灞炴€ц鍙?
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
    // 鍩烘湰灞炴€?
    // ---------------------------------------------------------

    /// 鏇茬嚎鎬婚暱
    pub fn length(&self) -> Option<f64> {
        self.prop_f64("Length")
    }

    pub fn sub_paths(&self) -> Option<IvgSubPaths> {
        self.prop_dispatch("SubPaths").map(IvgSubPaths::new)
    }

    pub fn nodes(&self) -> Option<crate::curve::node::IvgNodes> {
        self.prop_dispatch("Nodes").map(crate::curve::node::IvgNodes::new)
    }

    pub fn segments(&self) -> Option<crate::curve::segment::IvgSegments> {
        self.prop_dispatch("Segments").map(crate::curve::segment::IvgSegments::new)
    }

    pub fn is_closed(&self) -> Option<bool> {
        self.prop_bool("Closed")
    }

    pub fn set_closed(&self, v: bool) -> bool {
        self.put_bool("Closed", v)
    }

    // ---------------------------------------------------------
    // 閫夋嫨
    // ---------------------------------------------------------

    /// 杩斿洖褰撳墠閫変腑鑺傜偣闆嗗悎锛坄IVGNodeRange`锛?
    pub fn selection(&self) -> Option<IvgNodeRange> {
        self.disp
            .invoke_method("Selection", vec![])
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgNodeRange::new)
    }

    pub fn clear_selection(&self) -> bool {
        self.disp.invoke_method("ClearSelection", vec![]).is_ok()
    }

    // ---------------------------------------------------------
    // 瀛愯矾寰勬瀯閫?
    // ---------------------------------------------------------

    /// 鏂板缓涓€涓粠 (x, y) 寮€濮嬬殑瀛愯矾寰勩€?
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

    /// 鐢ㄥ潗鏍囨暟缁勬壒閲忓缓瀛愯矾寰勩€?
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
    // 杩藉姞鍥惧舰
    // ---------------------------------------------------------

    pub fn append_subpath_circle(&self, cx: f64, cy: f64, r: f64) -> bool {
        let args = vec![
            Variant::from_f64(cx),
            Variant::from_f64(cy),
            Variant::from_f64(r),
        ];
        self.disp.invoke_method("AppendSubpathCircle", args).is_ok()
    }

    pub fn append_subpath_rectangle(&self, l: f64, t: f64, r: f64, b: f64) -> bool {
        let args = vec![
            Variant::from_f64(l),
            Variant::from_f64(t),
            Variant::from_f64(r),
            Variant::from_f64(b),
        ];
        self.disp.invoke_method("AppendSubpathRectangle", args).is_ok()
    }

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

    pub fn append_subpath_ellipse(&self, cx: f64, cy: f64, rh: f64, rv: f64) -> bool {
        let args = vec![
            Variant::from_f64(cx),
            Variant::from_f64(cy),
            Variant::from_f64(rh),
            Variant::from_f64(rv),
        ];
        self.disp.invoke_method("AppendSubpathEllipse", args).is_ok()
    }

    /// 鎶婂彟涓€鏉℃洸绾胯拷鍔犲埌褰撳墠鏇茬嚎銆?
    pub fn append_curve(&self, other: &IvgCurve) -> bool {
        let args = vec![other.as_variant()];
        self.disp.invoke_method("AppendCurve", args).is_ok()
    }

    // ---------------------------------------------------------
    // 鏂瑰悜 / 闂悎
    // ---------------------------------------------------------

    pub fn reverse_direction(&self) -> bool {
        self.disp.invoke_method("ReverseDirection", vec![]).is_ok()
    }

    pub fn self_weld_closed_subpaths(&self) -> bool {
        self.disp.invoke_method("SelfWeldClosedSubpaths", vec![]).is_ok()
    }

    // ---------------------------------------------------------
    // 鍑犱綍鏌ヨ
    // ---------------------------------------------------------

    pub fn is_on_curve(&self, x: f64, y: f64, hot_area: f64) -> Option<i64> {
        let args = vec![
            Variant::from_f64(x),
            Variant::from_f64(y),
            Variant::from_f64(hot_area),
        ];
        self.disp.invoke_method("IsOnCurve", args).ok()?.to_i64().ok()
    }

    pub fn is_point_inside(&self, x: f64, y: f64) -> Option<bool> {
        let args = vec![
            Variant::from_f64(x),
            Variant::from_f64(y),
        ];
        self.disp.invoke_method("IsPointInside", args).ok()?.to_bool().ok()
    }

    pub fn is_rect_on_edge(&self, x1: f64, y1: f64, x2: f64, y2: f64) -> Option<bool> {
        let args = vec![
            Variant::from_f64(x1),
            Variant::from_f64(y1),
            Variant::from_f64(x2),
            Variant::from_f64(y2),
        ];
        self.disp.invoke_method("IsRectOnEdge", args).ok()?.to_bool().ok()
    }

    pub fn is_clockwise(&self) -> Option<bool> {
        self.prop_bool("IsClockwise")
    }

    pub fn area(&self) -> Option<f64> {
        self.prop_f64("Area")
    }

    pub fn bounding_box(&self) -> Option<IvgRect> {
        self.prop_dispatch("BoundingBox").map(IvgRect::new)
    }

    /// 鐢?`(x, y, w, h)` 杈撳嚭鍖呭洿鐩?
    pub fn get_bounding_box(&self) -> Option<(f64, f64, f64, f64)> {
        // 璇ユ柟娉曟湁 5 涓?out 鍙傛暟锛屾敼鐢?IDispatch 鍚庢湡缁戝畾姣旇緝楹荤儲銆?
        // 绠€鍗曡捣瑙侊紝鐢?BoundingBox 灞炴€ф浛浠ｃ€?
        let r = self.bounding_box()?;
        Some((r.x().unwrap_or(0.0), r.y().unwrap_or(0.0), r.width().unwrap_or(0.0), r.height().unwrap_or(0.0)))
    }

    // ---------------------------------------------------------
    // 鏌ユ壘
    // ---------------------------------------------------------

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

    pub fn find_closest_segment(&self, x: f64, y: f64) -> Option<(IvgSegment, f64)> {
        // 杩斿洖 (娈? 鍙傛暟鍋忕Щ)銆侰OM 鍙屽嚭鍙傚湪 IDispatch 閲屾嬁涓嶅埌锛屾敼鐢ㄨ繎浼硷細
        // 鐩存帴璋冪敤锛屾嬁绗竴涓繑鍥炵殑 IDispatch銆?
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
    // 鍙樻崲
    // ---------------------------------------------------------

    pub fn apply_transform_matrix(&self, matrix: &crate::geometry::IvgTransformMatrix) -> bool {
        let args = vec![matrix.as_variant()];
        self.disp.invoke_method("ApplyTransformMatrix", args).is_ok()
    }

    // ---------------------------------------------------------
    // 澶嶅埗 / 鐒婃帴
    // ---------------------------------------------------------

    pub fn get_copy(&self) -> Option<IvgCurve> {
        self.disp.invoke_method("GetCopy", vec![]).ok()?.to_idispatch().ok().map(IvgCurve::new)
    }

    pub fn copy_assign(&self, other: &IvgCurve) -> bool {
        let args = vec![other.as_variant()];
        self.disp.invoke_method("CopyAssign", args).is_ok()
    }

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
    // 鍏跺畠
    // ---------------------------------------------------------

    pub fn bind_to_document(&self, doc: &crate::document::IvgDocument) -> bool {
        let args = vec![doc.as_variant()];
        self.disp.invoke_method("BindToDocument", args).is_ok()
    }

    pub fn get_polyline(&self, precision: i32) -> Option<IvgCurve> {
        let args = vec![Variant::from_i64(precision as i64)];
        self.disp
            .invoke_method("GetPolyline", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgCurve::new)
    }

    pub fn remove_overlaps(&self) -> Option<IvgCurve> {
        self.disp
            .invoke_method("RemoveOverlaps", vec![])
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgCurve::new)
    }

    pub fn auto_reduce_nodes(&self, amount: f64, selected_only: bool) -> bool {
        let args = vec![
            Variant::from_f64(amount),
            Variant::from_bool(selected_only),
        ];
        self.disp.invoke_method("AutoReduceNodes", args).is_ok()
    }

    pub fn join_touching_subpaths(&self, allow_reversal: bool, tolerance: f64) -> bool {
        let args = vec![
            Variant::from_bool(allow_reversal),
            Variant::from_f64(tolerance),
        ];
        self.disp.invoke_method("JoinTouchingSubpaths", args).is_ok()
    }

    // ---------------------------------------------------------
    // 涓?Shape 鐨勪簰鎿嶄綔锛堝鏋?curve 鏄粠 shape 閲屾嬁鍒扮殑锛?
    // ---------------------------------------------------------

    pub fn to_shape(&self) -> Option<IvgShape> {
        // IVGCurve 鏈韩涓嶇洿鎺ユ毚闇诧紱濡傛灉涓氬姟闇€瑕侊紝閫氳繃
        // `layer.create_curve(shape)` 璧般€?
        None
    }
}