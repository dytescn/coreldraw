//! `IVGLayer` —— 图层对象
//!
//! 图层既承载图形（`Shapes` / `SelectableShapes`），
//! 又是**图形工厂**：`CreateRectangle` / `CreateEllipse` / `CreateArtisticText` ...

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

/// CorelDRAW `IVGLayer` 对象的后期绑定包装（图层）。
///
/// 所有属性与方法通过 `IDispatch::Invoke` 按名称调用，调用失败时一般返回 `None` 或 `false`。
pub struct IvgLayer {
    disp: ComObject,
}

impl IvgLayer {
    /// 由 COM `IDispatch` 构造包装对象。
    pub fn new(disp: IDispatch) -> Self {
        Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) }
    }

    /// 从 COM `IDispatch` 构造包装对象，等价于 `new`。
    pub fn from_disp(disp: IDispatch) -> Self {
        Self::new(disp)
    }

    /// 把底层 COM 对象包装成 `VARIANT`，用于传给其它 COM 调用。
    pub fn as_variant(&self) -> Variant {
        self.disp.as_variant()
    }
    /// 获取底层 `ComObject` 的引用。
    pub fn raw(&self) -> &ComObject {
        &self.disp
    }

    // ---------------------------------------------------------
    // 通用工具
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
    // 基本属性
    // ---------------------------------------------------------

    /// 读取名称（`Name`）。
    pub fn name(&self) -> Option<String> { self.prop_string("Name") }
    /// 设置名称（`Name`）。
    pub fn set_name(&self, v: impl Into<String>) -> bool {
        self.put_string("Name", v)
    }

    /// 读取索引（`Index`）。
    pub fn index(&self) -> Option<i64> { self.prop_i64("Index") }
    /// 读取绝对索引（`AbsoluteIndex`）。
    pub fn absolute_index(&self) -> Option<i64> { self.prop_i64("AbsoluteIndex") }

    /// 读取页面（`Page`）。
    pub fn page(&self) -> Option<IvgPage> {
        self.prop_dispatch("Page").map(IvgPage::new)
    }

    /// 读取对象树节点（`TreeNode`）。
    pub fn tree_node(&self) -> Option<IvgTreeNode> {
        self.prop_dispatch("TreeNode").map(IvgTreeNode::new)
    }

    // ---------------------------------------------------------
    // 可见性/ 打印 / 编辑 / 主层
    // ---------------------------------------------------------

    /// 读取可见性（`Visible`）。
    pub fn visible(&self) -> Option<bool> { self.prop_bool("Visible") }
    /// 设置可见性（`Visible`）。
    pub fn set_visible(&self, v: bool) -> bool { self.put_bool("Visible", v) }

    /// 读取 COM 属性 `Printable`。
    pub fn printable(&self) -> Option<bool> { self.prop_bool("Printable") }
    /// 设置 COM 属性 `Printable`。
    pub fn set_printable(&self, v: bool) -> bool { self.put_bool("Printable", v) }

    /// 读取可编辑（`Editable`）。
    pub fn editable(&self) -> Option<bool> { self.prop_bool("Editable") }
    /// 设置可编辑（`Editable`）。
    pub fn set_editable(&self, v: bool) -> bool { self.put_bool("Editable", v) }

    /// 读取主（`Master`）。
    pub fn master(&self) -> Option<bool> { self.prop_bool("Master") }
    /// 设置主（`Master`）。
    pub fn set_master(&self, v: bool) -> bool { self.put_bool("Master", v) }

    // ---------------------------------------------------------
    // 图层颜色（用于 UI 显示）
    // ---------------------------------------------------------

    /// 读取覆盖颜色（`OverrideColor`）。
    pub fn override_color(&self) -> Option<bool> { self.prop_bool("OverrideColor") }
    /// 设置覆盖颜色（`OverrideColor`）。
    pub fn set_override_color(&self, v: bool) -> bool {
        self.put_bool("OverrideColor", v)
    }

    /// 读取颜色（`Color`）。
    pub fn color(&self) -> Option<IvgColor> {
        self.prop_dispatch("Color").map(IvgColor::new)
    }
    /// 设置颜色（`Color`）。
    pub fn set_color(&self, c: &IvgColor) -> bool {
        self.put_dispatch("Color", c.as_variant())
    }

    // ---------------------------------------------------------
    // 类型判定（特殊图层）
    // ---------------------------------------------------------

    /// 是否参考线集图层（`IsGuidesLayer`）。
    pub fn is_guides_layer(&self) -> Option<bool> { self.prop_bool("IsGuidesLayer") }
    /// 是否桌面图层（`IsDesktopLayer`）。
    pub fn is_desktop_layer(&self) -> Option<bool> { self.prop_bool("IsDesktopLayer") }
    /// 是否网格图层（`IsGridLayer`）。
    pub fn is_grid_layer(&self) -> Option<bool> { self.prop_bool("IsGridLayer") }
    /// 读取 COM 属性 `IsSpecialLayer`。
    pub fn is_special_layer(&self) -> Option<bool> { self.prop_bool("IsSpecialLayer") }

    /// 主图层的引用。
    pub fn master_layer(&self) -> Option<IvgLayer> {
        self.prop_dispatch("MasterLayer").map(IvgLayer::new)
    }

    /// 上一层（`IgnoreMasters` 决定是否跳过主层）。
    pub fn above(&self, ignore_masters: bool) -> Option<IvgLayer> {
        let args = vec![Variant::from_bool(ignore_masters)];
        self.disp
            .invoke_method("Above", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgLayer::new)
    }

    /// 下一层。
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
    // 属性包
    // ---------------------------------------------------------

    /// 读取属性集（`Properties`）。
    pub fn properties(&self) -> Option<crate::misc::IvgProperties> {
        self.prop_dispatch("Properties").map(crate::misc::IvgProperties::new)
    }

    /// 读取主属性集（`MasterProperties`）。
    pub fn master_properties(&self) -> Option<crate::misc::IvgProperties> {
        self.prop_dispatch("MasterProperties").map(crate::misc::IvgProperties::new)
    }

    // ---------------------------------------------------------
    // 图形集合
    // ---------------------------------------------------------

    /// 读取图形集（`Shapes`）。
    pub fn shapes(&self) -> Option<IvgShapes> {
        self.prop_dispatch("Shapes").map(IvgShapes::new)
    }

    /// 仅可选择的图形。
    pub fn selectable_shapes(&self) -> Option<IvgShapes> {
        self.prop_dispatch("SelectableShapes").map(IvgShapes::new)
    }

    // ---------------------------------------------------------
    // 激活/ 删除 / 移动
    // ---------------------------------------------------------

    /// activate（COM 方法 `Activate`）。
    pub fn activate(&self) -> bool {
        self.disp.invoke_method("Activate", vec![]).is_ok()
    }

    /// 删除（COM 方法 `Delete`）。
    pub fn delete(&self) -> bool {
        self.disp.invoke_method("Delete", vec![]).is_ok()
    }

    /// 移动上方（COM 方法 `MoveAbove`）。
    pub fn move_above(&self, other: &IvgLayer) -> bool {
        let args = vec![other.as_variant()];
        self.disp.invoke_method("MoveAbove", args).is_ok()
    }

    /// 调用 COM 方法 `MoveBelow`。
    pub fn move_below(&self, other: &IvgLayer) -> bool {
        let args = vec![other.as_variant()];
        self.disp.invoke_method("MoveBelow", args).is_ok()
    }

    // ---------------------------------------------------------
    // 导入
    // ---------------------------------------------------------

    /// 导入文件到本图层（默认 filter）。
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

    /// 导入文件，返回 `ICorelImportFilter`（可继续交互式控制导入）。
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
    // 粘贴
    // ---------------------------------------------------------

    /// paste（COM 方法 `Paste`）。
    pub fn paste(&self) -> Option<IvgShape> {
        self.disp
            .invoke_method("Paste", vec![])
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgShape::new)
    }

    /// 粘贴扩展版（COM 方法 `PasteEx`）。
    pub fn paste_ex(&self, options: &IvgStructPasteOptions) -> Option<IvgShapeRange> {
        let args = vec![options.as_variant()];
        self.disp
            .invoke_method("PasteEx", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgShapeRange::new)
    }

    /// 调用 COM 方法 `PasteSpecial`。
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
    // 查找
    // ---------------------------------------------------------

    /// 查找图形（COM 方法 `FindShape`）。
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

    /// 查找图形集（COM 方法 `FindShapes`）。
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
    // 图形工厂（矩形/ 椭圆 / 多边形/ 螺旋）
    // ---------------------------------------------------------

    /// 由 4 条边创建矩形。
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

    /// 由位置 + 尺寸 + 4 个半径创建矩形。
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

    /// 由 `IvgRect` 创建矩形。
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

    /// 创建椭圆（COM 方法 `CreateEllipse`）。
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

    /// 调用 COM 方法 `CreateEllipse2`。
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

    /// 创建椭圆矩形（COM 方法 `CreateEllipseRect`）。
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

    /// 创建多边形（COM 方法 `CreatePolygon`）。
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

    /// 调用 COM 方法 `CreatePolygon2`。
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

    /// 调用 COM 方法 `CreateGridBoxes`。
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

    /// 调用 COM 方法 `CreateSpiral`。
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
    // 图形工厂（线段/ 曲线 / 连接器）
    // ---------------------------------------------------------

    /// 创建线线段（COM 方法 `CreateLineSegment`）。
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

    /// 创建曲线线段（COM 方法 `CreateCurveSegment`）。
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

    /// 调用 COM 方法 `CreateCurveSegment2`。
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

    /// 由 `IvgCurve` 创建曲线图形。
    pub fn create_curve(&self, source: &IvgCurve) -> Option<IvgShape> {
        let args = vec![source.as_variant()];
        self.disp
            .invoke_method("CreateCurve", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgShape::new)
    }

    /// 由 `IvgBSpline` 创建 B 样条图形。
    pub fn create_bspline(&self, source: &crate::curve::IvgBSpline) -> Option<IvgShape> {
        let args = vec![source.as_variant()];
        self.disp
            .invoke_method("CreateBSpline", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgShape::new)
    }

    // ---- 连接器----

    /// 创建连接器（COM 方法 `CreateConnector`）。
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

    /// 创建连接器扩展版（COM 方法 `CreateConnectorEx`）。
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

    /// 创建右角度连接器（COM 方法 `CreateRightAngleConnector`）。
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

    /// 调用 COM 方法 `CreateFreeConnector`。
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
    // 图形工厂（文本）
    // ---------------------------------------------------------

    /// 创建艺术文本（COM 方法 `CreateArtisticText`）。
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

    /// 与 `create_artistic_text` 相同，但使用宽字符版（用于多语言）。
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

    /// 创建段落文本（COM 方法 `CreateParagraphText`）。
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

    /// 创建段落文本宽（COM 方法 `CreateParagraphTextWide`）。
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
    // 图形工厂（引导线）
    // ---------------------------------------------------------

    /// 创建参考线角度（COM 方法 `CreateGuideAngle`）。
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

    /// 创建参考线（COM 方法 `CreateGuide`）。
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
    // 图形工厂（自定义 / 工具图形 / 位图）
    // ---------------------------------------------------------

    /// `Parameters` 是 `SAFEARRAY`。
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

    /// 创建工具图形（COM 方法 `CreateToolShape`）。
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

    /// 创建位图（COM 方法 `CreateBitmap`）。
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
    /// 调用 COM 方法 `CreateBitmap2`。
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
    /// 创建位图矩形（COM 方法 `CreateBitmapRect`）。
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
    // 图形工厂（标法/ 符号 / OLE）
    // ---------------------------------------------------------

    /// 创建线性尺寸（COM 方法 `CreateLinearDimension`）。
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

    /// 调用 COM 方法 `CreateAngularDimension`。
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

    /// 创建符号（COM 方法 `CreateSymbol`）。
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

    /// 创建 OLE 对象（COM 方法 `CreateOLEObject`）。
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

    /// 创建 OLE 对象文件（COM 方法 `CreateOLEObjectFromFile`）。
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
    // 自定义命令
    // ---------------------------------------------------------

    /// 自定义命令（COM 方法 `CustomCommand`）。
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