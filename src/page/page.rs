//! `IVGPage` —— 页面对象
//!
//! 由 `document.active_page()` / `document.pages().item(n)` 取得。

use wincom::{ComObject, Variant};
use windows::Win32::System::Com::IDispatch;

use crate::color::IvgColor;
use crate::geometry::IvgRect;
use crate::page::IvgPages;
use crate::layer::{IvgLayer, IvgLayers};
use crate::shape::{IvgShape, IvgShapeRange, IvgShapes};
use crate::tree::IvgTreeNode;

/// CorelDRAW `IVGPage` 对象的后期绑定包装（页面）。
///
/// 所有属性与方法通过 `IDispatch::Invoke` 按名称调用，调用失败时一般返回 `None` 或 `false`。
pub struct IvgPage {
    disp: ComObject,
}

impl IvgPage {
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

    fn prop_f64(&self, name: &str) -> Option<f64> {
        self.disp.get_property(name).ok()?.to_f64().ok()
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

    fn put_dispatch(&self, name: &str, v: Variant) -> bool {
        self.disp.set_property(name, vec![v]).is_ok()
    }

    // ---------------------------------------------------------
    // 基本信息
    // ---------------------------------------------------------

    /// 读取名称（`Name`）。
    pub fn name(&self) -> Option<String> { self.prop_string("Name") }
    /// 设置名称（`Name`）。
    pub fn set_name(&self, v: impl Into<String>) -> bool {
        self.put_string("Name", v)
    }

    /// 1-based 页号。
    pub fn index(&self) -> Option<i64> { self.prop_i64("Index") }

    // ---------------------------------------------------------
    // 关系
    // ---------------------------------------------------------

    /// 读取父对象（`Parent`）。
    pub fn parent(&self) -> Option<IvgPages> {
        self.prop_dispatch("Parent").map(IvgPages::new)
    }

    /// 读取上一个（`Previous`）。
    pub fn previous(&self) -> Option<IvgPage> {
        self.prop_dispatch("Previous").map(IvgPage::new)
    }

    /// 读取下一个（`Next`）。
    pub fn next(&self) -> Option<IvgPage> {
        self.prop_dispatch("Next").map(IvgPage::new)
    }

    /// 读取跨页（`Spread`）。
    pub fn spread(&self) -> Option<crate::page::IvgSpread> {
        self.prop_dispatch("Spread").map(crate::page::IvgSpread::new)
    }

    /// 读取对象树节点（`TreeNode`）。
    pub fn tree_node(&self) -> Option<IvgTreeNode> {
        self.prop_dispatch("TreeNode").map(IvgTreeNode::new)
    }

    // ---------------------------------------------------------
    // 页面设置
    // ---------------------------------------------------------

    /// 页面预设名（如"A4"）。
    pub fn paper(&self) -> Option<String> { self.prop_string("Paper") }

    /// 读取尺寸宽度（`SizeWidth`）。
    pub fn size_width(&self) -> Option<f64> { self.prop_f64("SizeWidth") }
    /// 设置尺寸宽度（`SizeWidth`）。
    pub fn set_size_width(&self, v: f64) -> bool { self.put_f64("SizeWidth", v) }

    /// 读取尺寸高度（`SizeHeight`）。
    pub fn size_height(&self) -> Option<f64> { self.prop_f64("SizeHeight") }
    /// 设置尺寸高度（`SizeHeight`）。
    pub fn set_size_height(&self, v: f64) -> bool { self.put_f64("SizeHeight", v) }

    /// 读取分辨率（`Resolution`）。
    pub fn resolution(&self) -> Option<i64> { self.prop_i64("Resolution") }
    /// 设置分辨率（`Resolution`）。
    pub fn set_resolution(&self, v: i32) -> bool { self.put_i64("Resolution", v as i64) }

    /// 出血。
    pub fn bleed(&self) -> Option<f64> { self.prop_f64("Bleed") }
    /// 设置出血（`Bleed`）。
    pub fn set_bleed(&self, v: f64) -> bool { self.put_f64("Bleed", v) }

    /// `cdrPageOrientation`
    pub fn orientation(&self) -> Option<i64> { self.prop_i64("Orientation") }
    /// 设置方向（`Orientation`）。
    pub fn set_orientation(&self, v: i32) -> bool { self.put_i64("Orientation", v as i64) }

    /// `cdrPageBackground`
    pub fn background(&self) -> Option<i64> { self.prop_i64("Background") }
    /// 设置背景（`Background`）。
    pub fn set_background(&self, v: i32) -> bool { self.put_i64("Background", v as i64) }

    /// 读取颜色（`Color`）。
    pub fn color(&self) -> Option<IvgColor> {
        self.prop_dispatch("Color").map(IvgColor::new)
    }
    /// 设置颜色（`Color`）。
    pub fn set_color(&self, c: &IvgColor) -> bool {
        self.put_dispatch("Color", c.as_variant())
    }

    /// 读取打印导出背景（`PrintExportBackground`）。
    pub fn print_export_background(&self) -> Option<bool> {
        self.prop_bool("PrintExportBackground")
    }
    /// 设置打印导出背景（`PrintExportBackground`）。
    pub fn set_print_export_background(&self, v: bool) -> bool {
        self.put_bool("PrintExportBackground", v)
    }

    // ---------------------------------------------------------
    // 几何：四边 + 中心 + 包围盒
    // ---------------------------------------------------------

    /// 读取左 X 坐标（`LeftX`）。
    pub fn left_x(&self) -> Option<f64> { self.prop_f64("LeftX") }
    /// 读取右 X 坐标（`RightX`）。
    pub fn right_x(&self) -> Option<f64> { self.prop_f64("RightX") }
    /// 读取顶部 Y 坐标（`TopY`）。
    pub fn top_y(&self) -> Option<f64> { self.prop_f64("TopY") }
    /// 读取底部 Y 坐标（`BottomY`）。
    pub fn bottom_y(&self) -> Option<f64> { self.prop_f64("BottomY") }
    /// 读取中心 X 坐标（`CenterX`）。
    pub fn center_x(&self) -> Option<f64> { self.prop_f64("CenterX") }
    /// 读取中心 Y 坐标（`CenterY`）。
    pub fn center_y(&self) -> Option<f64> { self.prop_f64("CenterY") }

    /// 读取边界框（`BoundingBox`）。
    pub fn bounding_box(&self) -> Option<IvgRect> {
        self.prop_dispatch("BoundingBox").map(IvgRect::new)
    }

    /// 读取尺寸。
    pub fn get_size(&self) -> Option<(f64, f64)> {
        Some((self.size_width()?, self.size_height()?))
    }

    /// 设置尺寸（`SetSize`）。
    pub fn set_size(&self, w: f64, h: f64) -> bool {
        let args = vec![
            Variant::from_f64(w),
            Variant::from_f64(h),
        ];
        self.disp.invoke_method("SetSize", args).is_ok()
    }

    /// 读取中心位置。
    pub fn get_center_position(&self) -> Option<(f64, f64)> {
        Some((self.center_x()?, self.center_y()?))
    }

    /// 选择命名页面尺寸。
    pub fn select_size(&self, page_size_name: impl Into<String>, landscape: bool) -> bool {
        let args = vec![
            Variant::from_str(page_size_name.into()),
            Variant::from_bool(landscape),
        ];
        self.disp.invoke_method("SelectSize", args).is_ok()
    }

    // ---------------------------------------------------------
    // 图层
    // ---------------------------------------------------------

    /// 读取图层集（`Layers`）。
    pub fn layers(&self) -> Option<IvgLayers> {
        self.prop_dispatch("Layers").map(IvgLayers::new)
    }

    /// 读取全部图层集（`AllLayers`）。
    pub fn all_layers(&self) -> Option<IvgLayers> {
        self.prop_dispatch("AllLayers").map(IvgLayers::new)
    }

    /// 读取活动图层（`ActiveLayer`）。
    pub fn active_layer(&self) -> Option<IvgLayer> {
        self.prop_dispatch("ActiveLayer").map(IvgLayer::new)
    }

    /// 读取参考线集图层（`GuidesLayer`）。
    pub fn guides_layer(&self) -> Option<IvgLayer> {
        self.prop_dispatch("GuidesLayer").map(IvgLayer::new)
    }

    /// 读取桌面图层（`DesktopLayer`）。
    pub fn desktop_layer(&self) -> Option<IvgLayer> {
        self.prop_dispatch("DesktopLayer").map(IvgLayer::new)
    }

    /// 读取网格图层（`GridLayer`）。
    pub fn grid_layer(&self) -> Option<IvgLayer> {
        self.prop_dispatch("GridLayer").map(IvgLayer::new)
    }

    /// 创建图层（COM 方法 `CreateLayer`）。
    pub fn create_layer(&self, layer_name: impl Into<String>) -> Option<IvgLayer> {
        let args = vec![Variant::from_str(layer_name.into())];
        self.disp
            .invoke_method("CreateLayer", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgLayer::new)
    }

    // ---------------------------------------------------------
    // 图形
    // ---------------------------------------------------------

    /// 读取图形集（`Shapes`）。
    pub fn shapes(&self) -> Option<IvgShapes> {
        self.prop_dispatch("Shapes").map(IvgShapes::new)
    }

    /// 读取可选择图形集（`SelectableShapes`）。
    pub fn selectable_shapes(&self) -> Option<IvgShapes> {
        self.prop_dispatch("SelectableShapes").map(IvgShapes::new)
    }

    /// 按名称/ 类型 / 静态 ID 查找。
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

    /// 找到某点最上层的图形。
    pub fn find_shape_at_point(
        &self,
        x: f64,
        y: f64,
        treat_as_filled: bool,
    ) -> Option<IvgShape> {
        let args = vec![
            Variant::from_f64(x),
            Variant::from_f64(y),
            Variant::from_bool(treat_as_filled),
        ];
        self.disp
            .invoke_method("FindShapeAtPoint", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgShape::new)
    }

    // ---------------------------------------------------------
    // 选择
    // ---------------------------------------------------------

    /// 选择点处的图形集（COM 方法 `SelectShapesAtPoint`）。
    pub fn select_shapes_at_point(
        &self,
        x: f64,
        y: f64,
        select_unfilled: bool,
        hot_area: f64,
    ) -> Option<IvgShapeRange> {
        let args = vec![
            Variant::from_f64(x),
            Variant::from_f64(y),
            Variant::from_bool(select_unfilled),
            Variant::from_f64(hot_area),
        ];
        self.disp
            .invoke_method("SelectShapesAtPoint", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgShapeRange::new)
    }

    /// 选择图形集矩形（COM 方法 `SelectShapesFromRectangle`）。
    pub fn select_shapes_from_rectangle(
        &self,
        x1: f64, y1: f64, x2: f64, y2: f64,
        touch: bool,
    ) -> Option<IvgShapeRange> {
        let args = vec![
            Variant::from_f64(x1),
            Variant::from_f64(y1),
            Variant::from_f64(x2),
            Variant::from_f64(y2),
            Variant::from_bool(touch),
        ];
        self.disp
            .invoke_method("SelectShapesFromRectangle", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgShapeRange::new)
    }

    /// 解锁全部图形集（COM 方法 `UnlockAllShapes`）。
    pub fn unlock_all_shapes(&self) -> bool {
        self.disp.invoke_method("UnlockAllShapes", vec![]).is_ok()
    }

    // ---------------------------------------------------------
    // 辅助级
    // ---------------------------------------------------------

    /// `cdrGuideType`
    pub fn guides(&self, guide_type: i32) -> Option<IvgShapeRange> {
        let args = vec![Variant::from_i64(guide_type as i64)];
        self.disp
            .invoke_method("Guides", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgShapeRange::new)
    }

    // ---------------------------------------------------------
    // 其它
    // ---------------------------------------------------------

    /// 读取属性集（`Properties`）。
    pub fn properties(&self) -> Option<crate::misc::IvgProperties> {
        self.prop_dispatch("Properties").map(crate::misc::IvgProperties::new)
    }

    // ---------------------------------------------------------
    // 操作
    // ---------------------------------------------------------

    /// activate（COM 方法 `Activate`）。
    pub fn activate(&self) -> bool {
        self.disp.invoke_method("Activate", vec![]).is_ok()
    }

    /// 删除（COM 方法 `Delete`）。
    pub fn delete(&self) -> bool {
        self.disp.invoke_method("Delete", vec![]).is_ok()
    }

    /// 移动（COM 方法 `MoveTo`）。
    pub fn move_to(&self, index: i32) -> bool {
        let args = vec![Variant::from_i64(index as i64)];
        self.disp.invoke_method("MoveTo", args).is_ok()
    }

    // ---------------------------------------------------------
    // 文本查找
    // ---------------------------------------------------------

    /// 文本查找（COM 方法 `TextFind`）。
    pub fn text_find(
        &self,
        text: impl Into<String>,
        case_sensitive: bool,
    ) -> Option<IvgShape> {
        let args = vec![
            Variant::from_str(text.into()),
            Variant::from_bool(case_sensitive),
        ];
        self.disp
            .invoke_method("TextFind", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgShape::new)
    }

    /// 文本替换（COM 方法 `TextReplace`）。
    pub fn text_replace(
        &self,
        old_text: impl Into<String>,
        new_text: impl Into<String>,
        case_sensitive: bool,
        replace_selected_only: bool,
    ) -> bool {
        let args = vec![
            Variant::from_str(old_text.into()),
            Variant::from_str(new_text.into()),
            Variant::from_bool(case_sensitive),
            Variant::from_bool(replace_selected_only),
        ];
        self.disp.invoke_method("TextReplace", args).is_ok()
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