//! `IVGLayer` 鈥斺€?鍥惧眰瀵硅薄
//!
//! 鍥惧眰鏃㈡壙杞藉浘褰紙`Shapes` / `SelectableShapes`锛夛紝
//! 鍙堟槸**鍥惧舰宸ュ巶**锛歚CreateRectangle` / `CreateEllipse` / `CreateArtisticText` ...

use wincom::{ComObject, Variant};
use windows::Win32::System::Com::IDispatch;

use crate::color::IvgColor;
use crate::curve::IvgCurve;
use crate::geometry::{IvgRect, IvgSnapPoint};
use crate::import_export::ICorelImportFilter;
use crate::page::IvgPage;
use crate::shape::{IvgShape, IvgShapeRange, IvgShapes};
use crate::structs::{IvgStructImportOptions, IvgStructPasteOptions};
use crate::tree::IvgTreeNode;

pub struct IvgLayer {
    disp: ComObject,
}

impl IvgLayer {
    pub fn new(disp: IDispatch) -> Self {
        Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) }
    }

    pub fn from_disp(disp: IDispatch) -> Self {
        Self::new(disp)
    }

    pub fn as_variant(&self) -> Variant {
        self.disp.as_variant()
    }
    pub fn raw(&self) -> &ComObject {
        &self.disp
    }

    // ---------------------------------------------------------
    // 閫氱敤宸ュ叿
    // ---------------------------------------------------------

    fn prop_string(&self, name: &str) -> Option<String> {
        self.disp.get_property(name).ok()?.to_string().ok()
    }

    fn prop_i64(&self, name: &str) -> Option<i64> {
        self.disp.get_property(name).ok()?.to_i64().ok()
    }

    fn prop_bool(&self, name: &str) -> Option<bool> {
        self.disp.get_property(name).ok()?.to_bool().ok()
    }

    fn prop_dispatch(&self, name: &str) -> Option<IDispatch> {
        self.disp.get_property(name).ok()?.to_idispatch().ok()
    }

    fn put_string(&self, name: &str, v: impl Into<String>) -> bool {
        let arg = Variant::from_str(v.into());
        self.disp.set_property(name, vec![arg]).is_ok()
    }

    fn put_bool(&self, name: &str, v: bool) -> bool {
        let arg = Variant::from_bool(v);
        self.disp.set_property(name, vec![arg]).is_ok()
    }

    fn put_dispatch(&self, name: &str, v: Variant) -> bool {
        self.disp.set_property(name, vec![v]).is_ok()
    }

    // ---------------------------------------------------------
    // 鍩烘湰灞炴€?
    // ---------------------------------------------------------

    pub fn name(&self) -> Option<String> { self.prop_string("Name") }
    pub fn set_name(&self, v: impl Into<String>) -> bool {
        self.put_string("Name", v)
    }

    pub fn index(&self) -> Option<i64> { self.prop_i64("Index") }
    pub fn absolute_index(&self) -> Option<i64> { self.prop_i64("AbsoluteIndex") }

    pub fn page(&self) -> Option<IvgPage> {
        self.prop_dispatch("Page").map(IvgPage::new)
    }

    pub fn tree_node(&self) -> Option<IvgTreeNode> {
        self.prop_dispatch("TreeNode").map(IvgTreeNode::new)
    }

    // ---------------------------------------------------------
    // 鍙鎬?/ 鎵撳嵃 / 缂栬緫 / 涓诲眰
    // ---------------------------------------------------------

    pub fn visible(&self) -> Option<bool> { self.prop_bool("Visible") }
    pub fn set_visible(&self, v: bool) -> bool { self.put_bool("Visible", v) }

    pub fn printable(&self) -> Option<bool> { self.prop_bool("Printable") }
    pub fn set_printable(&self, v: bool) -> bool { self.put_bool("Printable", v) }

    pub fn editable(&self) -> Option<bool> { self.prop_bool("Editable") }
    pub fn set_editable(&self, v: bool) -> bool { self.put_bool("Editable", v) }

    pub fn master(&self) -> Option<bool> { self.prop_bool("Master") }
    pub fn set_master(&self, v: bool) -> bool { self.put_bool("Master", v) }

    // ---------------------------------------------------------
    // 鍥惧眰棰滆壊锛堢敤浜?UI 鏄剧ず锛?
    // ---------------------------------------------------------

    pub fn override_color(&self) -> Option<bool> { self.prop_bool("OverrideColor") }
    pub fn set_override_color(&self, v: bool) -> bool {
        self.put_bool("OverrideColor", v)
    }

    pub fn color(&self) -> Option<IvgColor> {
        self.prop_dispatch("Color").map(IvgColor::new)
    }
    pub fn set_color(&self, c: &IvgColor) -> bool {
        self.put_dispatch("Color", c.as_variant())
    }

    // ---------------------------------------------------------
    // 绫诲瀷鍒ゅ畾锛堢壒娈婂浘灞傦級
    // ---------------------------------------------------------

    pub fn is_guides_layer(&self) -> Option<bool> { self.prop_bool("IsGuidesLayer") }
    pub fn is_desktop_layer(&self) -> Option<bool> { self.prop_bool("IsDesktopLayer") }
    pub fn is_grid_layer(&self) -> Option<bool> { self.prop_bool("IsGridLayer") }
    pub fn is_special_layer(&self) -> Option<bool> { self.prop_bool("IsSpecialLayer") }

    /// 涓诲浘灞傜殑寮曠敤銆?
    pub fn master_layer(&self) -> Option<IvgLayer> {
        self.prop_dispatch("MasterLayer").map(IvgLayer::new)
    }

    /// 涓婁竴灞傦紙`IgnoreMasters` 鍐冲畾鏄惁璺宠繃涓诲眰锛夈€?
    pub fn above(&self, ignore_masters: bool) -> Option<IvgLayer> {
        let args = vec![Variant::from_bool(ignore_masters)];
        self.disp
            .invoke_method("Above", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgLayer::new)
    }

    /// 涓嬩竴灞傘€?
    pub fn below(&self, ignore_masters: bool) -> Option<IvgLayer> {
        let args = vec![Variant::from_bool(ignore_masters)];
        self.disp
            .invoke_method("Below", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgLayer::new)
    }

    // ---------------------------------------------------------
    // 灞炴€у寘
    // ---------------------------------------------------------

    pub fn properties(&self) -> Option<crate::misc::IvgProperties> {
        self.prop_dispatch("Properties").map(crate::misc::IvgProperties::new)
    }

    pub fn master_properties(&self) -> Option<crate::misc::IvgProperties> {
        self.prop_dispatch("MasterProperties").map(crate::misc::IvgProperties::new)
    }

    // ---------------------------------------------------------
    // 鍥惧舰闆嗗悎
    // ---------------------------------------------------------

    pub fn shapes(&self) -> Option<IvgShapes> {
        self.prop_dispatch("Shapes").map(IvgShapes::new)
    }

    /// 浠呭彲閫夋嫨鐨勫浘褰€?
    pub fn selectable_shapes(&self) -> Option<IvgShapes> {
        self.prop_dispatch("SelectableShapes").map(IvgShapes::new)
    }

    // ---------------------------------------------------------
    // 婵€娲?/ 鍒犻櫎 / 绉诲姩
    // ---------------------------------------------------------

    pub fn activate(&self) -> bool {
        self.disp.invoke_method("Activate", vec![]).is_ok()
    }

    pub fn delete(&self) -> bool {
        self.disp.invoke_method("Delete", vec![]).is_ok()
    }

    pub fn move_above(&self, other: &IvgLayer) -> bool {
        let args = vec![other.as_variant()];
        self.disp.invoke_method("MoveAbove", args).is_ok()
    }

    pub fn move_below(&self, other: &IvgLayer) -> bool {
        let args = vec![other.as_variant()];
        self.disp.invoke_method("MoveBelow", args).is_ok()
    }

    // ---------------------------------------------------------
    // 瀵煎叆
    // ---------------------------------------------------------

    /// 瀵煎叆鏂囦欢鍒版湰鍥惧眰锛堥粯璁?filter锛夈€?
    pub fn import(
        &self,
        file_name: impl Into<String>,
        filter: i32,
        options: &IvgStructImportOptions,
    ) -> bool {
        let args = vec![
            Variant::from_str(file_name.into()),
            Variant::from_i64(filter as i64),
            options.as_variant(),
        ];
        self.disp.invoke_method("Import", args).is_ok()
    }

    /// 瀵煎叆鏂囦欢锛岃繑鍥?`ICorelImportFilter`锛堝彲缁х画浜や簰寮忔帶鍒跺鍏ワ級銆?
    pub fn import_ex(
        &self,
        file_name: impl Into<String>,
        filter: i32,
        options: &IvgStructImportOptions,
    ) -> Option<ICorelImportFilter> {
        let args = vec![
            Variant::from_str(file_name.into()),
            Variant::from_i64(filter as i64),
            options.as_variant(),
        ];
        self.disp
            .invoke_method("ImportEx", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(ICorelImportFilter::new)
    }

    // ---------------------------------------------------------
    // 绮樿创
    // ---------------------------------------------------------

    pub fn paste(&self) -> Option<IvgShape> {
        self.disp
            .invoke_method("Paste", vec![])
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgShape::new)
    }

    pub fn paste_ex(&self, options: &IvgStructPasteOptions) -> Option<IvgShapeRange> {
        let args = vec![options.as_variant()];
        self.disp
            .invoke_method("PasteEx", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgShapeRange::new)
    }

    pub fn paste_special(
        &self,
        format_name: impl Into<String>,
        paste_link: bool,
        display_as_icon: bool,
        caption: impl Into<String>,
        icon: Variant,
    ) -> Option<bool> {
        let args = vec![
            Variant::from_str(format_name.into()),
            Variant::from_bool(paste_link),
            Variant::from_bool(display_as_icon),
            Variant::from_str(caption.into()),
            icon,
        ];
        self.disp
            .invoke_method("PasteSpecial", args)
            .ok()?
            .to_bool()
            .ok()
    }

    // ---------------------------------------------------------
    // 鏌ユ壘
    // ---------------------------------------------------------

    pub fn find_shape(
        &self,
        name: impl Into<String>,
        shape_type: i32,
        static_id: i32,
        recursive: bool,
    ) -> Option<IvgShape> {
        let args = vec![
            Variant::from_str(name.into()),
            Variant::from_i64(shape_type as i64),
            Variant::from_i64(static_id as i64),
            Variant::from_bool(recursive),
        ];
        self.disp
            .invoke_method("FindShape", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgShape::new)
    }

    pub fn find_shapes(
        &self,
        name: impl Into<String>,
        shape_type: i32,
        recursive: bool,
    ) -> Option<IvgShapeRange> {
        let args = vec![
            Variant::from_str(name.into()),
            Variant::from_i64(shape_type as i64),
            Variant::from_bool(recursive),
        ];
        self.disp
            .invoke_method("FindShapes", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgShapeRange::new)
    }

    // ---------------------------------------------------------
    // 鍥惧舰宸ュ巶锛堢煩褰?/ 妞渾 / 澶氳竟褰?/ 铻烘棆锛?
    // ---------------------------------------------------------

    /// 鐢?4 鏉¤竟鍒涘缓鐭╁舰銆?
    #[allow(clippy::too_many_arguments)]
    pub fn create_rectangle(
        &self,
        left: f64, top: f64, right: f64, bottom: f64,
        corner_ul: i32, corner_ur: i32, corner_lr: i32, corner_ll: i32,
    ) -> Option<IvgShape> {
        let args = vec![
            Variant::from_f64(left),
            Variant::from_f64(top),
            Variant::from_f64(right),
            Variant::from_f64(bottom),
            Variant::from_i64(corner_ul as i64),
            Variant::from_i64(corner_ur as i64),
            Variant::from_i64(corner_lr as i64),
            Variant::from_i64(corner_ll as i64),
        ];
        self.disp
            .invoke_method("CreateRectangle", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgShape::new)
    }

    /// 鐢?浣嶇疆 + 灏哄 + 4 涓崐寰?鍒涘缓鐭╁舰銆?
    #[allow(clippy::too_many_arguments)]
    pub fn create_rectangle2(
        &self,
        x: f64, y: f64, w: f64, h: f64,
        radius_ul: f64, radius_ur: f64, radius_lr: f64, radius_ll: f64,
    ) -> Option<IvgShape> {
        let args = vec![
            Variant::from_f64(x),
            Variant::from_f64(y),
            Variant::from_f64(w),
            Variant::from_f64(h),
            Variant::from_f64(radius_ul),
            Variant::from_f64(radius_ur),
            Variant::from_f64(radius_lr),
            Variant::from_f64(radius_ll),
        ];
        self.disp
            .invoke_method("CreateRectangle2", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgShape::new)
    }

    /// 鐢?`IvgRect` 鍒涘缓鐭╁舰銆?
    #[allow(clippy::too_many_arguments)]
    pub fn create_rectangle_rect(
        &self,
        rect: &IvgRect,
        radius_ul: f64, radius_ur: f64, radius_lr: f64, radius_ll: f64,
    ) -> Option<IvgShape> {
        let args = vec![
            rect.as_variant(),
            Variant::from_f64(radius_ul),
            Variant::from_f64(radius_ur),
            Variant::from_f64(radius_lr),
            Variant::from_f64(radius_ll),
        ];
        self.disp
            .invoke_method("CreateRectangleRect", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgShape::new)
    }

    #[allow(clippy::too_many_arguments)]
    pub fn create_ellipse(
        &self,
        left: f64, top: f64, right: f64, bottom: f64,
        start_angle: f64, end_angle: f64, pie: bool,
    ) -> Option<IvgShape> {
        let args = vec![
            Variant::from_f64(left),
            Variant::from_f64(top),
            Variant::from_f64(right),
            Variant::from_f64(bottom),
            Variant::from_f64(start_angle),
            Variant::from_f64(end_angle),
            Variant::from_bool(pie),
        ];
        self.disp
            .invoke_method("CreateEllipse", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgShape::new)
    }

    #[allow(clippy::too_many_arguments)]
    pub fn create_ellipse2(
        &self,
        cx: f64, cy: f64, r1: f64, r2: f64,
        start_angle: f64, end_angle: f64, pie: bool,
    ) -> Option<IvgShape> {
        let args = vec![
            Variant::from_f64(cx),
            Variant::from_f64(cy),
            Variant::from_f64(r1),
            Variant::from_f64(r2),
            Variant::from_f64(start_angle),
            Variant::from_f64(end_angle),
            Variant::from_bool(pie),
        ];
        self.disp
            .invoke_method("CreateEllipse2", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgShape::new)
    }

    pub fn create_ellipse_rect(
        &self,
        rect: &IvgRect,
        start_angle: f64, end_angle: f64, pie: bool,
    ) -> Option<IvgShape> {
        let args = vec![
            rect.as_variant(),
            Variant::from_f64(start_angle),
            Variant::from_f64(end_angle),
            Variant::from_bool(pie),
        ];
        self.disp
            .invoke_method("CreateEllipseRect", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgShape::new)
    }

    #[allow(clippy::too_many_arguments)]
    pub fn create_polygon(
        &self,
        left: f64, top: f64, right: f64, bottom: f64,
        sides: i32, sub_paths: i32, complexity: i32,
        star: bool, star_complexity: i32, max_complexity: i32,
    ) -> Option<IvgShape> {
        let args = vec![
            Variant::from_f64(left),
            Variant::from_f64(top),
            Variant::from_f64(right),
            Variant::from_f64(bottom),
            Variant::from_i64(sides as i64),
            Variant::from_i64(sub_paths as i64),
            Variant::from_i64(complexity as i64),
            Variant::from_bool(star),
            Variant::from_i64(star_complexity as i64),
            Variant::from_i64(max_complexity as i64),
        ];
        self.disp
            .invoke_method("CreatePolygon", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgShape::new)
    }

    #[allow(clippy::too_many_arguments)]
    pub fn create_polygon2(
        &self,
        cx: f64, cy: f64, radius: f64, sides: i32,
        angle: f64, inner_radius: f64, star: bool, sharpness: i32,
    ) -> Option<IvgShape> {
        let args = vec![
            Variant::from_f64(cx),
            Variant::from_f64(cy),
            Variant::from_f64(radius),
            Variant::from_i64(sides as i64),
            Variant::from_f64(angle),
            Variant::from_f64(inner_radius),
            Variant::from_bool(star),
            Variant::from_i64(sharpness as i64),
        ];
        self.disp
            .invoke_method("CreatePolygon2", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgShape::new)
    }

    pub fn create_grid_boxes(
        &self,
        left: f64, top: f64, right: f64, bottom: f64,
        wide: i32, high: i32,
    ) -> Option<IvgShape> {
        let args = vec![
            Variant::from_f64(left),
            Variant::from_f64(top),
            Variant::from_f64(right),
            Variant::from_f64(bottom),
            Variant::from_i64(wide as i64),
            Variant::from_i64(high as i64),
        ];
        self.disp
            .invoke_method("CreateGridBoxes", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgShape::new)
    }

    pub fn create_spiral(
        &self,
        left: f64, top: f64, right: f64, bottom: f64,
        revolutions: i32, spiral_type: i32, growth_rate: i32,
    ) -> Option<IvgShape> {
        let args = vec![
            Variant::from_f64(left),
            Variant::from_f64(top),
            Variant::from_f64(right),
            Variant::from_f64(bottom),
            Variant::from_i64(revolutions as i64),
            Variant::from_i64(spiral_type as i64),
            Variant::from_i64(growth_rate as i64),
        ];
        self.disp
            .invoke_method("CreateSpiral", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgShape::new)
    }

    // ---------------------------------------------------------
    // 鍥惧舰宸ュ巶锛堢嚎娈?/ 鏇茬嚎 / 杩炴帴鍣級
    // ---------------------------------------------------------

    pub fn create_line_segment(
        &self,
        start_x: f64, start_y: f64, end_x: f64, end_y: f64,
    ) -> Option<IvgShape> {
        let args = vec![
            Variant::from_f64(start_x),
            Variant::from_f64(start_y),
            Variant::from_f64(end_x),
            Variant::from_f64(end_y),
        ];
        self.disp
            .invoke_method("CreateLineSegment", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgShape::new)
    }

    #[allow(clippy::too_many_arguments)]
    pub fn create_curve_segment(
        &self,
        start_x: f64, start_y: f64, end_x: f64, end_y: f64,
        start_ctrl_len: f64, start_ctrl_angle: f64,
        end_ctrl_len: f64, end_ctrl_angle: f64,
    ) -> Option<IvgShape> {
        let args = vec![
            Variant::from_f64(start_x),
            Variant::from_f64(start_y),
            Variant::from_f64(end_x),
            Variant::from_f64(end_y),
            Variant::from_f64(start_ctrl_len),
            Variant::from_f64(start_ctrl_angle),
            Variant::from_f64(end_ctrl_len),
            Variant::from_f64(end_ctrl_angle),
        ];
        self.disp
            .invoke_method("CreateCurveSegment", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgShape::new)
    }

    #[allow(clippy::too_many_arguments)]
    pub fn create_curve_segment2(
        &self,
        x1: f64, y1: f64,
        start_ctrl_x: f64, start_ctrl_y: f64,
        end_ctrl_x: f64, end_ctrl_y: f64,
        x2: f64, y2: f64,
    ) -> Option<IvgShape> {
        let args = vec![
            Variant::from_f64(x1),
            Variant::from_f64(y1),
            Variant::from_f64(start_ctrl_x),
            Variant::from_f64(start_ctrl_y),
            Variant::from_f64(end_ctrl_x),
            Variant::from_f64(end_ctrl_y),
            Variant::from_f64(x2),
            Variant::from_f64(y2),
        ];
        self.disp
            .invoke_method("CreateCurveSegment2", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgShape::new)
    }

    /// 鐢?`IvgCurve` 鍒涘缓鏇茬嚎鍥惧舰銆?
    pub fn create_curve(&self, source: &IvgCurve) -> Option<IvgShape> {
        let args = vec![source.as_variant()];
        self.disp
            .invoke_method("CreateCurve", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgShape::new)
    }

    /// 鐢?`IvgBSpline` 鍒涘缓 B 鏍锋潯鍥惧舰銆?
    pub fn create_bspline(&self, source: &crate::curve::IvgBSpline) -> Option<IvgShape> {
        let args = vec![source.as_variant()];
        self.disp
            .invoke_method("CreateBSpline", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgShape::new)
    }

    // ---- 杩炴帴鍣?----

    pub fn create_connector(
        &self,
        start: &IvgSnapPoint,
        end: &IvgSnapPoint,
    ) -> Option<IvgShape> {
        let args = vec![start.as_variant(), end.as_variant()];
        self.disp
            .invoke_method("CreateConnector", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgShape::new)
    }

    pub fn create_connector_ex(
        &self,
        conn_type: i32,
        start: &IvgSnapPoint,
        end: &IvgSnapPoint,
    ) -> Option<IvgShape> {
        let args = vec![
            Variant::from_i64(conn_type as i64),
            start.as_variant(),
            end.as_variant(),
        ];
        self.disp
            .invoke_method("CreateConnectorEx", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgShape::new)
    }

    pub fn create_right_angle_connector(
        &self,
        start: &IvgSnapPoint,
        end: &IvgSnapPoint,
        corner_radius: f64,
    ) -> Option<IvgShape> {
        let args = vec![
            start.as_variant(),
            end.as_variant(),
            Variant::from_f64(corner_radius),
        ];
        self.disp
            .invoke_method("CreateRightAngleConnector", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgShape::new)
    }

    pub fn create_free_connector(
        &self,
        x1: f64, y1: f64, x2: f64, y2: f64,
    ) -> Option<IvgShape> {
        let args = vec![
            Variant::from_f64(x1),
            Variant::from_f64(y1),
            Variant::from_f64(x2),
            Variant::from_f64(y2),
        ];
        self.disp
            .invoke_method("CreateFreeConnector", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgShape::new)
    }

    // ---------------------------------------------------------
    // 鍥惧舰宸ュ巶锛堟枃鏈級
    // ---------------------------------------------------------

    #[allow(clippy::too_many_arguments)]
    pub fn create_artistic_text(
        &self,
        left: f64, bottom: f64,
        text: impl Into<String>,
        language_id: i32, char_set: i32,
        font: impl Into<String>,
        size: f32,
        bold: i32, italic: i32, underline: i32,
        alignment: i32,
    ) -> Option<IvgShape> {
        let args = vec![
            Variant::from_f64(left),
            Variant::from_f64(bottom),
            Variant::from_str(text.into()),
            Variant::from_i64(language_id as i64),
            Variant::from_i64(char_set as i64),
            Variant::from_str(font.into()),
            Variant::from_f64(size as f64),
            Variant::from_i64(bold as i64),
            Variant::from_i64(italic as i64),
            Variant::from_i64(underline as i64),
            Variant::from_i64(alignment as i64),
        ];
        self.disp
            .invoke_method("CreateArtisticText", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgShape::new)
    }

    /// 涓?`create_artistic_text` 鍚岋紝浣嗙敤瀹藉瓧绗︾増锛堢敤浜庡璇█锛夈€?
    #[allow(clippy::too_many_arguments)]
    pub fn create_artistic_text_wide(
        &self,
        left: f64, bottom: f64,
        text: impl Into<String>,
        language_id: i32, char_set: i32,
        font: impl Into<String>,
        size: f32,
        bold: i32, italic: i32, underline: i32,
        alignment: i32,
    ) -> Option<IvgShape> {
        let args = vec![
            Variant::from_f64(left),
            Variant::from_f64(bottom),
            Variant::from_str(text.into()),
            Variant::from_i64(language_id as i64),
            Variant::from_i64(char_set as i64),
            Variant::from_str(font.into()),
            Variant::from_f64(size as f64),
            Variant::from_i64(bold as i64),
            Variant::from_i64(italic as i64),
            Variant::from_i64(underline as i64),
            Variant::from_i64(alignment as i64),
        ];
        self.disp
            .invoke_method("CreateArtisticTextWide", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgShape::new)
    }

    #[allow(clippy::too_many_arguments)]
    pub fn create_paragraph_text(
        &self,
        left: f64, top: f64, right: f64, bottom: f64,
        text: impl Into<String>,
        language_id: i32, char_set: i32,
        font: impl Into<String>,
        size: f32,
        bold: i32, italic: i32, underline: i32,
        alignment: i32,
    ) -> Option<IvgShape> {
        let args = vec![
            Variant::from_f64(left),
            Variant::from_f64(top),
            Variant::from_f64(right),
            Variant::from_f64(bottom),
            Variant::from_str(text.into()),
            Variant::from_i64(language_id as i64),
            Variant::from_i64(char_set as i64),
            Variant::from_str(font.into()),
            Variant::from_f64(size as f64),
            Variant::from_i64(bold as i64),
            Variant::from_i64(italic as i64),
            Variant::from_i64(underline as i64),
            Variant::from_i64(alignment as i64),
        ];
        self.disp
            .invoke_method("CreateParagraphText", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgShape::new)
    }

    #[allow(clippy::too_many_arguments)]
    pub fn create_paragraph_text_wide(
        &self,
        left: f64, top: f64, right: f64, bottom: f64,
        text: impl Into<String>,
        language_id: i32, char_set: i32,
        font: impl Into<String>,
        size: f32,
        bold: i32, italic: i32, underline: i32,
        alignment: i32,
    ) -> Option<IvgShape> {
        let args = vec![
            Variant::from_f64(left),
            Variant::from_f64(top),
            Variant::from_f64(right),
            Variant::from_f64(bottom),
            Variant::from_str(text.into()),
            Variant::from_i64(language_id as i64),
            Variant::from_i64(char_set as i64),
            Variant::from_str(font.into()),
            Variant::from_f64(size as f64),
            Variant::from_i64(bold as i64),
            Variant::from_i64(italic as i64),
            Variant::from_i64(underline as i64),
            Variant::from_i64(alignment as i64),
        ];
        self.disp
            .invoke_method("CreateParagraphTextWide", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgShape::new)
    }

    // ---------------------------------------------------------
    // 鍥惧舰宸ュ巶锛堝紩瀵肩嚎锛?
    // ---------------------------------------------------------

    pub fn create_guide_angle(
        &self,
        x: f64, y: f64, angle: f64,
    ) -> Option<IvgShape> {
        let args = vec![
            Variant::from_f64(x),
            Variant::from_f64(y),
            Variant::from_f64(angle),
        ];
        self.disp
            .invoke_method("CreateGuideAngle", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgShape::new)
    }

    pub fn create_guide(
        &self,
        x1: f64, y1: f64, x2: f64, y2: f64,
    ) -> Option<IvgShape> {
        let args = vec![
            Variant::from_f64(x1),
            Variant::from_f64(y1),
            Variant::from_f64(x2),
            Variant::from_f64(y2),
        ];
        self.disp
            .invoke_method("CreateGuide", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgShape::new)
    }

    // ---------------------------------------------------------
    // 鍥惧舰宸ュ巶锛堣嚜瀹氫箟 / 宸ュ叿鍥惧舰 / 浣嶅浘锛?
    // ---------------------------------------------------------

    /// `Parameters` 涓?`SAFEARRAY`銆?
    pub fn create_custom_shape(
        &self,
        type_id: impl Into<String>,
        parameters: Variant,
    ) -> Option<IvgShape> {
        let args = vec![
            Variant::from_str(type_id.into()),
            parameters,
        ];
        self.disp
            .invoke_method("CreateCustomShape", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgShape::new)
    }

    pub fn create_tool_shape(
        &self,
        tool_shape_guid: impl Into<String>,
        shape_properties: &crate::misc::IvgProperties,
    ) -> Option<IvgShape> {
        let args = vec![
            Variant::from_str(tool_shape_guid.into()),
            shape_properties.as_variant(),
        ];
        self.disp
            .invoke_method("CreateToolShape", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgShape::new)
    }

    #[allow(clippy::too_many_arguments,unused)]
    pub fn create_bitmap(
        &self,
        left: f64, top: f64, right: f64, bottom: f64,
        image: &crate::shape::IvgImage,
        image_alpha: &crate::shape::IvgImage,
    ) -> Option<IvgShape> {
        let args = vec![
            Variant::from_f64(left),
            Variant::from_f64(top),
            Variant::from_f64(right),
            Variant::from_f64(bottom),
            image.as_variant(),
            image.as_variant(),
        ];
        self.disp
            .invoke_method("CreateBitmap", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgShape::new)
    }
    #[allow(unused)]
    pub fn create_bitmap2(
        &self,
        x: f64, y: f64, w: f64, h: f64,
        image: &crate::shape::IvgImage,
        image_alpha: &crate::shape::IvgImage,
    ) -> Option<IvgShape> {
        let args = vec![
            Variant::from_f64(x),
            Variant::from_f64(y),
            Variant::from_f64(w),
            Variant::from_f64(h),
            image.as_variant(),
            image.as_variant(),
        ];
        self.disp
            .invoke_method("CreateBitmap2", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgShape::new)
    }
    #[allow(unused)]
    pub fn create_bitmap_rect(
        &self,
        rect: &IvgRect,
        image: &crate::shape::IvgImage,
        image_alpha: &crate::shape::IvgImage,
    ) -> Option<IvgShape> {
        let args = vec![
            rect.as_variant(),
            image.as_variant(),
            image.as_variant(),
        ];
        self.disp
            .invoke_method("CreateBitmapRect", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgShape::new)
    }

    // ---------------------------------------------------------
    // 鍥惧舰宸ュ巶锛堟爣娉?/ 绗﹀彿 / OLE锛?
    // ---------------------------------------------------------

    #[allow(clippy::too_many_arguments)]
    pub fn create_linear_dimension(
        &self,
        dim_type: i32,
        point1: &IvgSnapPoint,
        point2: &IvgSnapPoint,
        text_centered: bool,
        text_x: f64, text_y: f64,
        style: i32,
        precision: i32,
        show_units: bool,
        units: i32,
        placement: i32,
        horizontal_text: bool,
        boxed_text: bool,
        leading_zero: bool,
        prefix: impl Into<String>,
        suffix: impl Into<String>,
        outline_width: f64,
        arrows: &crate::outline::IvgArrowHead,
        outline_color: &IvgColor,
        text_font: impl Into<String>,
        text_size: f64,
        text_color: &IvgColor,
    ) -> Option<IvgShape> {
        let args = vec![
            Variant::from_i64(dim_type as i64),
            point1.as_variant(),
            point2.as_variant(),
            Variant::from_bool(text_centered),
            Variant::from_f64(text_x),
            Variant::from_f64(text_y),
            Variant::from_i64(style as i64),
            Variant::from_i64(precision as i64),
            Variant::from_bool(show_units),
            Variant::from_i64(units as i64),
            Variant::from_i64(placement as i64),
            Variant::from_bool(horizontal_text),
            Variant::from_bool(boxed_text),
            Variant::from_bool(leading_zero),
            Variant::from_str(prefix.into()),
            Variant::from_str(suffix.into()),
            Variant::from_f64(outline_width),
            arrows.as_variant(),
            outline_color.as_variant(),
            Variant::from_str(text_font.into()),
            Variant::from_f64(text_size),
            text_color.as_variant(),
        ];
        self.disp
            .invoke_method("CreateLinearDimension", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgShape::new)
    }

    #[allow(clippy::too_many_arguments)]
    pub fn create_angular_dimension(
        &self,
        center: &IvgSnapPoint,
        point1: &IvgSnapPoint,
        point2: &IvgSnapPoint,
        text_x: f64, text_y: f64,
        precision: i32,
        show_units: bool,
        units: i32,
        boxed_text: bool,
        leading_zero: bool,
        prefix: impl Into<String>,
        suffix: impl Into<String>,
        outline_width: f64,
        arrows: &crate::outline::IvgArrowHead,
        outline_color: &IvgColor,
        text_font: impl Into<String>,
        text_size: f64,
        text_color: &IvgColor,
    ) -> Option<IvgShape> {
        let args = vec![
            center.as_variant(),
            point1.as_variant(),
            point2.as_variant(),
            Variant::from_f64(text_x),
            Variant::from_f64(text_y),
            Variant::from_i64(precision as i64),
            Variant::from_bool(show_units),
            Variant::from_i64(units as i64),
            Variant::from_bool(boxed_text),
            Variant::from_bool(leading_zero),
            Variant::from_str(prefix.into()),
            Variant::from_str(suffix.into()),
            Variant::from_f64(outline_width),
            arrows.as_variant(),
            outline_color.as_variant(),
            Variant::from_str(text_font.into()),
            Variant::from_f64(text_size),
            text_color.as_variant(),
        ];
        self.disp
            .invoke_method("CreateAngularDimension", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgShape::new)
    }

    pub fn create_symbol(
        &self,
        x: f64, y: f64,
        symbol_name: impl Into<String>,
        library: &crate::symbol::IvgSymbolLibrary,
    ) -> Option<IvgShape> {
        let args = vec![
            Variant::from_f64(x),
            Variant::from_f64(y),
            Variant::from_str(symbol_name.into()),
            library.as_variant(),
        ];
        self.disp
            .invoke_method("CreateSymbol", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgShape::new)
    }

    pub fn create_ole_object(
        &self,
        object_id: impl Into<String>,
        display_as_icon: bool,
        caption: impl Into<String>,
        icon: Variant,
    ) -> Option<IvgShape> {
        let args = vec![
            Variant::from_str(object_id.into()),
            Variant::from_bool(display_as_icon),
            Variant::from_str(caption.into()),
            icon,
        ];
        self.disp
            .invoke_method("CreateOLEObject", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgShape::new)
    }

    pub fn create_ole_object_from_file(
        &self,
        file_name: impl Into<String>,
        link: bool,
        display_as_icon: bool,
        caption: impl Into<String>,
        icon: Variant,
    ) -> Option<IvgShape> {
        let args = vec![
            Variant::from_str(file_name.into()),
            Variant::from_bool(link),
            Variant::from_bool(display_as_icon),
            Variant::from_str(caption.into()),
            icon,
        ];
        self.disp
            .invoke_method("CreateOLEObjectFromFile", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgShape::new)
    }

    // ---------------------------------------------------------
    // 鑷畾涔夊懡浠?
    // ---------------------------------------------------------

    pub fn custom_command(
        &self,
        component_id: impl Into<String>,
        command_id: impl Into<String>,
        parameters: Variant,
    ) -> Option<Variant> {
        let args = vec![
            Variant::from_str(component_id.into()),
            Variant::from_str(command_id.into()),
            parameters,
        ];
        self.disp.invoke_method("CustomCommand", args).ok()
    }
}