//! `IVGShape` —— 图形对象（核心）

use wincom::{ComObject, Variant};
use windows::Win32::System::Com::IDispatch;

use crate::color::IvgColor;
use crate::curve::IvgCurve;
use crate::effect::{IvgEffect, IvgEffects};
use crate::fill::IvgFill;
use crate::geometry::{IvgRect, IvgSnapPoint, IvgSnapPoints, IvgTransformMatrix};
use crate::layer::IvgLayer;
use crate::misc::{IvgCloneLink, IvgProperties, IvgTransparency, IvgUrl};
use crate::outline::IvgOutline;
use crate::page::IvgPage;
use crate::shape::{
    IvgBitmap, IvgConnector, IvgCustomShape, IvgEllipse, IvgEps, IvgGuide,
    IvgPolygon, IvgPowerClip, IvgRectangle, IvgShapeRange, IvgShapes,
};
use crate::symbol::IvgSymbol;
use crate::text::IvgText;
use crate::tree::IvgTreeNode;

/// CorelDRAW `IVGShape` 对象的后期绑定包装（图形）。
///
/// 所有属性与方法通过 `IDispatch::Invoke` 按名称调用，调用失败时一般返回 `None` 或 `false`。
pub struct IvgShape {
    disp: ComObject,
}

impl IvgShape {
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

    /// 分发。
    pub fn dispatch(&self) -> &IDispatch {
        self.disp.as_dispatch()
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

    /// 图形类型（`cdrShapeType`）。
    pub fn shape_type(&self) -> Option<i64> { self.prop_i64("Type") }

    /// 图形的静态 ID（在文档内唯一）。
    pub fn static_id(&self) -> Option<i64> { self.prop_i64("StaticID") }

    /// 读取名称（`Name`）。
    pub fn name(&self) -> Option<String> { self.prop_string("Name") }
    /// 设置名称（`Name`）。
    pub fn set_name(&self, v: impl Into<String>) -> bool { self.put_string("Name", v) }

    /// 读取锁定状态（`Locked`）。
    pub fn locked(&self) -> Option<bool> { self.prop_bool("Locked") }
    /// 设置锁定状态（`Locked`）。
    pub fn set_locked(&self, v: bool) -> bool { self.put_bool("Locked", v) }

    /// 读取已选中（`Selected`）。
    pub fn selected(&self) -> Option<bool> { self.prop_bool("Selected") }
    /// 设置已选中（`Selected`）。
    pub fn set_selected(&self, v: bool) -> bool { self.put_bool("Selected", v) }

    /// 读取可见性（`Visible`）。
    pub fn visible(&self) -> Option<bool> { self.prop_bool("Visible") }
    /// 设置可见性（`Visible`）。
    pub fn set_visible(&self, v: bool) -> bool { self.put_bool("Visible", v) }

    /// 读取可选择（`Selectable`）。
    pub fn selectable(&self) -> Option<bool> { self.prop_bool("Selectable") }

    /// 读取虚拟图形（`Virtual`）。
    pub fn virtual_shape(&self) -> Option<bool> { self.prop_bool("Virtual") }
    /// 读取 COM 属性 `CanHaveFill`。
    pub fn can_have_fill(&self) -> Option<bool> { self.prop_bool("CanHaveFill") }
    /// 读取 COM 属性 `CanHaveOutline`。
    pub fn can_have_outline(&self) -> Option<bool> { self.prop_bool("CanHaveOutline") }
    /// 读取 COM 属性 `IsSimpleShape`。
    pub fn is_simple_shape(&self) -> Option<bool> { self.prop_bool("IsSimpleShape") }

    // ---------------------------------------------------------
    // 位置 / 尺寸
    // ---------------------------------------------------------

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
    /// 设置尺寸宽度（`SizeWidth`）。
    pub fn set_size_width(&self, v: f64) -> bool { self.put_f64("SizeWidth", v) }

    /// 读取尺寸高度（`SizeHeight`）。
    pub fn size_height(&self) -> Option<f64> { self.prop_f64("SizeHeight") }
    /// 设置尺寸高度（`SizeHeight`）。
    pub fn set_size_height(&self, v: f64) -> bool { self.put_f64("SizeHeight", v) }

    /// 读取原始宽度（`OriginalWidth`）。
    pub fn original_width(&self) -> Option<f64> { self.prop_f64("OriginalWidth") }
    /// 读取原始高度（`OriginalHeight`）。
    pub fn original_height(&self) -> Option<f64> { self.prop_f64("OriginalHeight") }

    /// 读取左 X 坐标（`LeftX`）。
    pub fn left_x(&self) -> Option<f64> { self.prop_f64("LeftX") }
    /// 设置左 X 坐标（`LeftX`）。
    pub fn set_left_x(&self, v: f64) -> bool { self.put_f64("LeftX", v) }

    /// 读取右 X 坐标（`RightX`）。
    pub fn right_x(&self) -> Option<f64> { self.prop_f64("RightX") }
    /// 设置右 X 坐标（`RightX`）。
    pub fn set_right_x(&self, v: f64) -> bool { self.put_f64("RightX", v) }

    /// 读取顶部 Y 坐标（`TopY`）。
    pub fn top_y(&self) -> Option<f64> { self.prop_f64("TopY") }
    /// 设置顶部 Y 坐标（`TopY`）。
    pub fn set_top_y(&self, v: f64) -> bool { self.put_f64("TopY", v) }

    /// 读取底部 Y 坐标（`BottomY`）。
    pub fn bottom_y(&self) -> Option<f64> { self.prop_f64("BottomY") }
    /// 设置底部 Y 坐标（`BottomY`）。
    pub fn set_bottom_y(&self, v: f64) -> bool { self.put_f64("BottomY", v) }

    /// 读取中心 X 坐标（`CenterX`）。
    pub fn center_x(&self) -> Option<f64> { self.prop_f64("CenterX") }
    /// 设置中心 X 坐标（`CenterX`）。
    pub fn set_center_x(&self, v: f64) -> bool { self.put_f64("CenterX", v) }

    /// 读取中心 Y 坐标（`CenterY`）。
    pub fn center_y(&self) -> Option<f64> { self.prop_f64("CenterY") }
    /// 设置中心 Y 坐标（`CenterY`）。
    pub fn set_center_y(&self, v: f64) -> bool { self.put_f64("CenterY", v) }

    /// 读取边界框（`BoundingBox`）。
    pub fn bounding_box(&self) -> Option<IvgRect> {
        self.prop_dispatch("BoundingBox").map(IvgRect::new)
    }

    /// 读取位置。
    pub fn get_position(&self) -> Option<(f64, f64)> {
        Some((self.position_x()?, self.position_y()?))
    }

    /// 设置位置（`SetPosition`）。
    pub fn set_position(&self, x: f64, y: f64) -> bool {
        let args = vec![
            Variant::from_f64(x),
            Variant::from_f64(y),
        ];
        self.disp.invoke_method("SetPosition", args).is_ok()
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

    /// 用参考点定位。`cdrReferencePoint` 见 `enums::shape`。
    pub fn set_position_ex(&self, ref_point: i32, x: f64, y: f64) -> bool {
        let args = vec![
            Variant::from_i64(ref_point as i64),
            Variant::from_f64(x),
            Variant::from_f64(y),
        ];
        self.disp.invoke_method("SetPositionEx", args).is_ok()
    }

    // ---------------------------------------------------------
    // 变换
    // ---------------------------------------------------------

    /// 读取旋转角度（`RotationAngle`）。
    pub fn rotation_angle(&self) -> Option<f64> { self.prop_f64("RotationAngle") }
    /// 设置旋转角度（`RotationAngle`）。
    pub fn set_rotation_angle(&self, v: f64) -> bool { self.put_f64("RotationAngle", v) }

    /// 读取旋转中心 X 坐标（`RotationCenterX`）。
    pub fn rotation_center_x(&self) -> Option<f64> { self.prop_f64("RotationCenterX") }
    /// 设置旋转中心 X 坐标（`RotationCenterX`）。
    pub fn set_rotation_center_x(&self, v: f64) -> bool {
        self.put_f64("RotationCenterX", v)
    }

    /// 读取旋转中心 Y 坐标（`RotationCenterY`）。
    pub fn rotation_center_y(&self) -> Option<f64> { self.prop_f64("RotationCenterY") }
    /// 设置旋转中心 Y 坐标（`RotationCenterY`）。
    pub fn set_rotation_center_y(&self, v: f64) -> bool {
        self.put_f64("RotationCenterY", v)
    }

    /// 设置旋转中心（`SetRotationCenter`）。
    pub fn set_rotation_center(&self, x: f64, y: f64) -> bool {
        let args = vec![
            Variant::from_f64(x),
            Variant::from_f64(y),
        ];
        self.disp.invoke_method("SetRotationCenter", args).is_ok()
    }

    /// rotate（COM 方法 `Rotate`）。
    pub fn rotate(&self, angle: f64) -> bool {
        let args = vec![Variant::from_f64(angle)];
        self.disp.invoke_method("Rotate", args).is_ok()
    }

    /// 旋转扩展版（COM 方法 `RotateEx`）。
    pub fn rotate_ex(&self, angle: f64, cx: f64, cy: f64) -> bool {
        let args = vec![
            Variant::from_f64(angle),
            Variant::from_f64(cx),
            Variant::from_f64(cy),
        ];
        self.disp.invoke_method("RotateEx", args).is_ok()
    }

    /// 倾斜（COM 方法 `Skew`）。
    pub fn skew(&self, ax: f64, ay: f64) -> bool {
        let args = vec![
            Variant::from_f64(ax),
            Variant::from_f64(ay),
        ];
        self.disp.invoke_method("Skew", args).is_ok()
    }

    /// 倾斜扩展版（COM 方法 `SkewEx`）。
    pub fn skew_ex(&self, ax: f64, ay: f64, cx: f64, cy: f64) -> bool {
        let args = vec![
            Variant::from_f64(ax),
            Variant::from_f64(ay),
            Variant::from_f64(cx),
            Variant::from_f64(cy),
        ];
        self.disp.invoke_method("SkewEx", args).is_ok()
    }

    /// 拉伸（COM 方法 `Stretch`）。
    pub fn stretch(&self, sx: f64, sy: f64, stretch_chars: bool) -> bool {
        let args = vec![
            Variant::from_f64(sx),
            Variant::from_f64(sy),
            Variant::from_bool(stretch_chars),
        ];
        self.disp.invoke_method("Stretch", args).is_ok()
    }

    /// 移动（COM 方法 `Move`）。
    pub fn move_by(&self, dx: f64, dy: f64) -> bool {
        let args = vec![
            Variant::from_f64(dx),
            Variant::from_f64(dy),
        ];
        self.disp.invoke_method("Move", args).is_ok()
    }

    /// `cdrFlipAxes`
    pub fn flip(&self, axes: i32) -> bool {
        let args = vec![Variant::from_i64(axes as i64)];
        self.disp.invoke_method("Flip", args).is_ok()
    }

    /// 读取矩阵。
    pub fn get_matrix(&self) -> Option<(f64, f64, f64, f64, f64, f64)> {
        None
    }

    /// 设置矩阵（`SetMatrix`）。
    pub fn set_matrix(
        &self,
        d11: f64, d12: f64, d21: f64, d22: f64, tx: f64, ty: f64,
    ) -> bool {
        let args = vec![
            Variant::from_f64(d11),
            Variant::from_f64(d12),
            Variant::from_f64(d21),
            Variant::from_f64(d22),
            Variant::from_f64(tx),
            Variant::from_f64(ty),
        ];
        self.disp.invoke_method("SetMatrix", args).is_ok()
    }

    /// 读取 COM 属性 `TransformationMatrix`。
    pub fn transformation_matrix(&self) -> Option<IvgTransformMatrix> {
        self.prop_dispatch("TransformationMatrix").map(IvgTransformMatrix::new)
    }

    /// 设置 COM 属性 `TransformationMatrix`。
    pub fn set_transformation_matrix(&self, m: &IvgTransformMatrix) -> bool {
        self.put_dispatch("TransformationMatrix", m.as_variant())
    }

    /// 应用变换矩阵（COM 方法 `ApplyTransformMatrix`）。
    pub fn apply_transform_matrix(&self, m: &IvgTransformMatrix) -> bool {
        let args = vec![m.as_variant()];
        self.disp.invoke_method("ApplyTransformMatrix", args).is_ok()
    }

    /// 清除变换（COM 方法 `ClearTransformations`）。
    pub fn clear_transformations(&self) -> bool {
        self.disp.invoke_method("ClearTransformations", vec![]).is_ok()
    }

    // ---------------------------------------------------------
    // 层 / 页 / 样式
    // ---------------------------------------------------------

    /// 读取图层（`Layer`）。
    pub fn layer(&self) -> Option<IvgLayer> {
        self.prop_dispatch("Layer").map(IvgLayer::new)
    }
    /// 设置图层（`Layer`）。
    pub fn set_layer(&self, l: &IvgLayer) -> bool {
        self.put_dispatch("Layer", l.as_variant())
    }

    /// 移动图层（COM 方法 `MoveToLayer`）。
    pub fn move_to_layer(&self, l: &IvgLayer) -> bool {
        let args = vec![l.as_variant()];
        self.disp.invoke_method("MoveToLayer", args).is_ok()
    }

    /// 复制图层（COM 方法 `CopyToLayer`）。
    pub fn copy_to_layer(&self, l: &IvgLayer) -> Option<IvgShape> {
        let args = vec![l.as_variant()];
        self.disp
            .invoke_method("CopyToLayer", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgShape::new)
    }

    /// 复制图层 as 范围（COM 方法 `CopyToLayerAsRange`）。
    pub fn copy_to_layer_as_range(&self, l: &IvgLayer) -> Option<IvgShapeRange> {
        let args = vec![l.as_variant()];
        self.disp
            .invoke_method("CopyToLayerAsRange", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgShapeRange::new)
    }

    /// 读取页面（`Page`）。
    pub fn page(&self) -> Option<IvgPage> { self.prop_dispatch("Page").map(IvgPage::new) }
    /// 读取跨页（`Spread`）。
    pub fn spread(&self) -> Option<crate::page::IvgSpread> {
        self.prop_dispatch("Spread").map(crate::page::IvgSpread::new)
    }
    /// 读取对象树节点（`TreeNode`）。
    pub fn tree_node(&self) -> Option<IvgTreeNode> {
        self.prop_dispatch("TreeNode").map(IvgTreeNode::new)
    }

    // ---------------------------------------------------------
    // 填充 / 轮廓 / 透明
    // ---------------------------------------------------------

    /// 读取填充（`Fill`）。
    pub fn fill(&self) -> Option<IvgFill> {
        self.prop_dispatch("Fill").map(IvgFill::new)
    }
    /// 设置填充（`Fill`）。
    pub fn set_fill(&self, f: &IvgFill) -> bool {
        self.put_dispatch("Fill", f.as_variant())
    }

    /// 读取轮廓（`Outline`）。
    pub fn outline(&self) -> Option<IvgOutline> {
        self.prop_dispatch("Outline").map(IvgOutline::new)
    }

    /// 读取透明度（`Transparency`）。
    pub fn transparency(&self) -> Option<IvgTransparency> {
        self.prop_dispatch("Transparency").map(IvgTransparency::new)
    }

    /// `cdrFillMode`
    pub fn fill_mode(&self) -> Option<i64> { self.prop_i64("FillMode") }
    /// 设置填充模式（`FillMode`）。
    pub fn set_fill_mode(&self, v: i32) -> bool { self.put_i64("FillMode", v as i64) }

    /// 读取叠印填充（`OverprintFill`）。
    pub fn overprint_fill(&self) -> Option<bool> { self.prop_bool("OverprintFill") }
    /// 设置叠印填充（`OverprintFill`）。
    pub fn set_overprint_fill(&self, v: bool) -> bool { self.put_bool("OverprintFill", v) }

    /// 读取叠印轮廓（`OverprintOutline`）。
    pub fn overprint_outline(&self) -> Option<bool> { self.prop_bool("OverprintOutline") }
    /// 设置叠印轮廓（`OverprintOutline`）。
    pub fn set_overprint_outline(&self, v: bool) -> bool {
        self.put_bool("OverprintOutline", v)
    }

    /// 读取叠印位图（`OverprintBitmap`）。
    pub fn overprint_bitmap(&self) -> Option<bool> { self.prop_bool("OverprintBitmap") }
    /// 设置叠印位图（`OverprintBitmap`）。
    pub fn set_overprint_bitmap(&self, v: bool) -> bool {
        self.put_bool("OverprintBitmap", v)
    }

    /// 读取 COM 属性 `DrapeFill`。
    pub fn drape_fill(&self) -> Option<bool> { self.prop_bool("DrapeFill") }
    
    /// 设置 COM 属性 `DrapeFill`。
    pub fn set_drape_fill(&self, v: bool) -> bool { self.put_bool("DrapeFill", v) }

    /// 取消填充。
    pub fn apply_no_fill(&self) -> bool {
        self.fill().map(|f| f.apply_no_fill()).unwrap_or(false)
    }

    /// 应用均匀填充。
    pub fn apply_uniform_fill(&self, color: &IvgColor) -> bool {
        self.fill()
            .map(|f| f.apply_uniform_fill(color))
            .unwrap_or(false)
    }

    /// 应用渐变填充（COM 方法 `ApplyFountainFill`）。
    #[allow(clippy::too_many_arguments)]
    pub fn apply_fountain_fill(
        &self,
        start_color: &IvgColor,
        end_color: &IvgColor,
        fill_type: i32,
        angle: f64,
        steps: i32,
        edge_pad: i32,
        mid_point: i32,
        blend_type: i32,
        center_offset_x: f64,
        center_offset_y: f64,
    ) -> bool {
        let args = vec![
            start_color.as_variant(),
            end_color.as_variant(),
            Variant::from_i64(fill_type as i64),
            Variant::from_f64(angle),
            Variant::from_i64(steps as i64),
            Variant::from_i64(edge_pad as i64),
            Variant::from_i64(mid_point as i64),
            Variant::from_i64(blend_type as i64),
            Variant::from_f64(center_offset_x),
            Variant::from_f64(center_offset_y),
        ];
        self.disp.invoke_method("ApplyFountainFill", args).is_ok()
    }

    /// 把轮廓转为独立图形。
    pub fn convert_outline_to_object(&self) -> Option<IvgShapeRange> {
        self.disp
            .invoke_method("ConvertOutlineToObject", vec![])
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgShapeRange::new)
    }

    // ---------------------------------------------------------
    // 曲线
    // ---------------------------------------------------------

    /// 读取曲线（`Curve`）。
    pub fn curve(&self) -> Option<IvgCurve> {
        self.prop_dispatch("Curve").map(IvgCurve::new)
    }

    /// 转换曲线集（COM 方法 `ConvertToCurves`）。
    pub fn convert_to_curves(&self) -> bool {
        self.disp.invoke_method("ConvertToCurves", vec![]).is_ok()
    }

    /// 读取显示曲线（`DisplayCurve`）。
    pub fn display_curve(&self) -> Option<IvgCurve> {
        self.prop_dispatch("DisplayCurve").map(IvgCurve::new)
    }

    // ---------------------------------------------------------
    // 子类型视图
    // ---------------------------------------------------------

    /// 读取矩形（`Rectangle`）。
    pub fn rectangle(&self) -> Option<IvgRectangle> {
        self.prop_dispatch("Rectangle").map(IvgRectangle::new)
    }
    /// 读取椭圆（`Ellipse`）。
    pub fn ellipse(&self) -> Option<IvgEllipse> {
        self.prop_dispatch("Ellipse").map(IvgEllipse::new)
    }
    /// 读取多边形（`Polygon`）。
    pub fn polygon(&self) -> Option<IvgPolygon> {
        self.prop_dispatch("Polygon").map(IvgPolygon::new)
    }
    /// 读取位图（`Bitmap`）。
    pub fn bitmap(&self) -> Option<IvgBitmap> {
        self.prop_dispatch("Bitmap").map(IvgBitmap::new)
    }
    /// 读取文本（`Text`）。
    pub fn text(&self) -> Option<IvgText> {
        self.prop_dispatch("Text").map(IvgText::new)
    }
    /// 读取 EPS（`EPS`）。
    pub fn eps(&self) -> Option<IvgEps> {
        self.prop_dispatch("EPS").map(IvgEps::new)
    }
    /// 读取连接器（`Connector`）。
    pub fn connector(&self) -> Option<IvgConnector> {
        self.prop_dispatch("Connector").map(IvgConnector::new)
    }
    /// 读取参考线（`Guide`）。
    pub fn guide(&self) -> Option<IvgGuide> {
        self.prop_dispatch("Guide").map(IvgGuide::new)
    }
    /// 读取幂裁剪（`PowerClip`）。
    pub fn power_clip(&self) -> Option<IvgPowerClip> {
        self.prop_dispatch("PowerClip").map(IvgPowerClip::new)
    }
    /// 读取自定义（`Custom`）。
    pub fn custom(&self) -> Option<IvgCustomShape> {
        self.prop_dispatch("Custom").map(IvgCustomShape::new)
    }
    /// 读取符号（`Symbol`）。
    pub fn symbol(&self) -> Option<IvgSymbol> {
        self.prop_dispatch("Symbol").map(IvgSymbol::new)
    }

    // ---------------------------------------------------------
    // 效果
    // ---------------------------------------------------------

    /// 读取效果集（`Effects`）。
    pub fn effects(&self) -> Option<IvgEffects> {
        self.prop_dispatch("Effects").map(IvgEffects::new)
    }
    /// 读取效果（`Effect`）。
    pub fn effect(&self) -> Option<IvgEffect> {
        self.prop_dispatch("Effect").map(IvgEffect::new)
    }

    /// 创建投影阴影（COM 方法 `CreateDropShadow`）。
    #[allow(clippy::too_many_arguments)]
    pub fn create_drop_shadow(
        &self,
        shadow_type: i32,
        opacity: i32,
        feather: i32,
        offset_x: f64,
        offset_y: f64,
        color: &IvgColor,
        feather_type: i32,
        feather_edge: i32,
        perspective_angle: f64,
        perspective_stretch: f64,
        fade: i32,
        merge_mode: i32,
    ) -> Option<IvgEffect> {
        let args = vec![
            Variant::from_i64(shadow_type as i64),
            Variant::from_i64(opacity as i64),
            Variant::from_i64(feather as i64),
            Variant::from_f64(offset_x),
            Variant::from_f64(offset_y),
            color.as_variant(),
            Variant::from_i64(feather_type as i64),
            Variant::from_i64(feather_edge as i64),
            Variant::from_f64(perspective_angle),
            Variant::from_f64(perspective_stretch),
            Variant::from_i64(fade as i64),
            Variant::from_i64(merge_mode as i64),
        ];
        self.disp
            .invoke_method("CreateDropShadow", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgEffect::new)
    }

    /// 创建调和（COM 方法 `CreateBlend`）。
    #[allow(clippy::too_many_arguments)]
    pub fn create_blend(
        &self,
        target: &IvgShape,
        steps: i64,
        color_blend_type: i32,
        mode: i32,
        spacing: f64,
        angle: f64,
        loop_: bool,
        path: Option<&IvgShape>,
        rotate_shapes: bool,
        spacing_accel: i32,
        color_accel: i32,
        accel_size: bool,
    ) -> Option<IvgEffect> {
        let path_var = match path {
            Some(p) => p.as_variant(),
            None => Variant::default(),
        };
        let args = vec![
            target.as_variant(),
            Variant::from_i64(steps),
            Variant::from_i64(color_blend_type as i64),
            Variant::from_i64(mode as i64),
            Variant::from_f64(spacing),
            Variant::from_f64(angle),
            Variant::from_bool(loop_),
            path_var,
            Variant::from_bool(rotate_shapes),
            Variant::from_i64(spacing_accel as i64),
            Variant::from_i64(color_accel as i64),
            Variant::from_bool(accel_size),
        ];
        self.disp
            .invoke_method("CreateBlend", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgEffect::new)
    }

    /// 创建轮廓（COM 方法 `CreateContour`）。
    #[allow(clippy::too_many_arguments)]
    pub fn create_contour(
        &self,
        direction: i32,
        offset: f64,
        steps: i32,
        blend_type: i32,
        outline_color: &IvgColor,
        fill_color: &IvgColor,
        fill_color2: &IvgColor,
        spacing_accel: i32,
        color_accel: i32,
        end_cap_type: i32,
        corner_type: i32,
        miter_limit: f64,
    ) -> Option<IvgEffect> {
        let args = vec![
            Variant::from_i64(direction as i64),
            Variant::from_f64(offset),
            Variant::from_i64(steps as i64),
            Variant::from_i64(blend_type as i64),
            outline_color.as_variant(),
            fill_color.as_variant(),
            fill_color2.as_variant(),
            Variant::from_i64(spacing_accel as i64),
            Variant::from_i64(color_accel as i64),
            Variant::from_i64(end_cap_type as i64),
            Variant::from_i64(corner_type as i64),
            Variant::from_f64(miter_limit),
        ];
        self.disp
            .invoke_method("CreateContour", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgEffect::new)
    }

    /// 创建封套（COM 方法 `CreateEnvelope`）。
    pub fn create_envelope(
        &self,
        preset_index: i32,
        mode: i32,
        keep_lines: bool,
    ) -> Option<IvgEffect> {
        let args = vec![
            Variant::from_i64(preset_index as i64),
            Variant::from_i64(mode as i64),
            Variant::from_bool(keep_lines),
        ];
        self.disp
            .invoke_method("CreateEnvelope", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgEffect::new)
    }

    /// 创建透视（COM 方法 `CreatePerspective`）。
    pub fn create_perspective(
        &self,
        hx: Variant,
        hy: Variant,
        vx: Variant,
        vy: Variant,
    ) -> Option<IvgEffect> {
        let args = vec![hx, hy, vx, vy];
        self.disp
            .invoke_method("CreatePerspective", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgEffect::new)
    }

    /// 清除效果（COM 方法 `ClearEffect`）。
    pub fn clear_effect(&self, effect_type: i32) -> bool {
        let args = vec![Variant::from_i64(effect_type as i64)];
        self.disp.invoke_method("ClearEffect", args).is_ok()
    }

    /// 展平效果集（COM 方法 `FlattenEffects`）。
    pub fn flatten_effects(&self) -> bool {
        self.disp.invoke_method("FlattenEffects", vec![]).is_ok()
    }

    // ---------------------------------------------------------
    // 群组 / 组合
    // ---------------------------------------------------------

    /// 组（COM 方法 `Group`）。
    pub fn group(&self) -> Option<IvgShape> {
        self.disp
            .invoke_method("Group", vec![])
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgShape::new)
    }

    /// 取消群组（COM 方法 `Ungroup`）。
    pub fn ungroup(&self) -> bool {
        self.disp.invoke_method("Ungroup", vec![]).is_ok()
    }

    /// 取消群组全部（COM 方法 `UngroupAll`）。
    pub fn ungroup_all(&self) -> bool {
        self.disp.invoke_method("UngroupAll", vec![]).is_ok()
    }

    /// 读取父对象组（`ParentGroup`）。
    pub fn parent_group(&self) -> Option<IvgShape> {
        self.prop_dispatch("ParentGroup").map(IvgShape::new)
    }

    /// combine（COM 方法 `Combine`）。
    pub fn combine(&self) -> Option<IvgShape> {
        self.disp
            .invoke_method("Combine", vec![])
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgShape::new)
    }

    /// 断开分开（COM 方法 `BreakApart`）。
    pub fn break_apart(&self) -> bool {
        self.disp.invoke_method("BreakApart", vec![]).is_ok()
    }

    /// 调用 COM 方法 `Separate`。
    pub fn separate(&self) -> bool {
        self.disp.invoke_method("Separate", vec![]).is_ok()
    }

    /// 读取子对象集（`Shapes`）。
    pub fn children(&self) -> Option<IvgShapes> {
        self.prop_dispatch("Shapes").map(IvgShapes::new)
    }

    // ---------------------------------------------------------
    // 布尔运算
    // ---------------------------------------------------------

    /// weld（COM 方法 `Weld`）。
    pub fn weld(&self, target: &IvgShape, leave_src: bool, leave_tgt: bool) -> Option<IvgShape> {
        let args = vec![
            target.as_variant(),
            Variant::from_bool(leave_src),
            Variant::from_bool(leave_tgt),
        ];
        self.disp
            .invoke_method("Weld", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgShape::new)
    }

    /// trim（COM 方法 `Trim`）。
    pub fn trim(&self, target: &IvgShape, leave_src: bool, leave_tgt: bool) -> Option<IvgShape> {
        let args = vec![
            target.as_variant(),
            Variant::from_bool(leave_src),
            Variant::from_bool(leave_tgt),
        ];
        self.disp
            .invoke_method("Trim", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgShape::new)
    }

    /// intersect（COM 方法 `Intersect`）。
    pub fn intersect(&self, target: &IvgShape, leave_src: bool, leave_tgt: bool) -> Option<IvgShape> {
        let args = vec![
            target.as_variant(),
            Variant::from_bool(leave_src),
            Variant::from_bool(leave_tgt),
        ];
        self.disp
            .invoke_method("Intersect", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgShape::new)
    }

    // ---------------------------------------------------------
    // 复制 / 克隆
    // ---------------------------------------------------------

    /// duplicate（COM 方法 `Duplicate`）。
    pub fn duplicate(&self, dx: f64, dy: f64) -> Option<IvgShape> {
        let args = vec![
            Variant::from_f64(dx),
            Variant::from_f64(dy),
        ];
        self.disp
            .invoke_method("Duplicate", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgShape::new)
    }

    /// 再制 as 范围（COM 方法 `DuplicateAsRange`）。
    pub fn duplicate_as_range(&self, dx: f64, dy: f64) -> Option<IvgShapeRange> {
        let args = vec![
            Variant::from_f64(dx),
            Variant::from_f64(dy),
        ];
        self.disp
            .invoke_method("DuplicateAsRange", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgShapeRange::new)
    }

    /// 克隆（COM 方法 `Clone`）。
    pub fn clone(&self, dx: f64, dy: f64) -> Option<IvgShape> {
        let args = vec![
            Variant::from_f64(dx),
            Variant::from_f64(dy),
        ];
        self.disp
            .invoke_method("Clone", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgShape::new)
    }

    /// 克隆 as 范围（COM 方法 `CloneAsRange`）。
    pub fn clone_as_range(&self, dx: f64, dy: f64) -> Option<IvgShapeRange> {
        let args = vec![
            Variant::from_f64(dx),
            Variant::from_f64(dy),
        ];
        self.disp
            .invoke_method("CloneAsRange", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgShapeRange::new)
    }

    /// 读取克隆链接（`CloneLink`）。
    pub fn clone_link(&self) -> Option<IvgCloneLink> {
        self.prop_dispatch("CloneLink").map(IvgCloneLink::new)
    }

    /// 读取 COM 属性 `Clones`。
    pub fn clones(&self) -> Option<IvgShapeRange> {
        self.prop_dispatch("Clones").map(IvgShapeRange::new)
    }

    /// 复制（COM 方法 `Copy`）。
    pub fn copy(&self) -> bool {
        self.disp.invoke_method("Copy", vec![]).is_ok()
    }
    /// cut（COM 方法 `Cut`）。
    pub fn cut(&self) -> bool {
        self.disp.invoke_method("Cut", vec![]).is_ok()
    }
    /// 删除（COM 方法 `Delete`）。
    pub fn delete(&self) -> bool {
        self.disp.invoke_method("Delete", vec![]).is_ok()
    }

    // ---------------------------------------------------------
    // Z 顺序
    // ---------------------------------------------------------

    /// 排序前（COM 方法 `OrderToFront`）。
    pub fn order_to_front(&self) -> bool {
        self.disp.invoke_method("OrderToFront", vec![]).is_ok()
    }
    /// 排序后（COM 方法 `OrderToBack`）。
    pub fn order_to_back(&self) -> bool {
        self.disp.invoke_method("OrderToBack", vec![]).is_ok()
    }
    /// 调用 COM 方法 `OrderForwardOne`。
    pub fn order_forward_one(&self) -> bool {
        self.disp.invoke_method("OrderForwardOne", vec![]).is_ok()
    }
    /// 调用 COM 方法 `OrderBackOne`。
    pub fn order_back_one(&self) -> bool {
        self.disp.invoke_method("OrderBackOne", vec![]).is_ok()
    }

    /// 排序前（COM 方法 `OrderFrontOf`）。
    pub fn order_front_of(&self, other: &IvgShape) -> bool {
        let args = vec![other.as_variant()];
        self.disp.invoke_method("OrderFrontOf", args).is_ok()
    }

    /// 排序后（COM 方法 `OrderBackOf`）。
    pub fn order_back_of(&self, other: &IvgShape) -> bool {
        let args = vec![other.as_variant()];
        self.disp.invoke_method("OrderBackOf", args).is_ok()
    }

    /// 排序 reverse（COM 方法 `OrderReverse`）。
    pub fn order_reverse(&self) -> bool {
        self.disp.invoke_method("OrderReverse", vec![]).is_ok()
    }

    /// 读取 Z 坐标 order（`ZOrder`）。
    pub fn z_order(&self) -> Option<i64> { self.prop_i64("ZOrder") }

    // ---------------------------------------------------------
    // 属性包 / URL / 元数据
    // ---------------------------------------------------------

    /// 读取属性集（`Properties`）。
    pub fn properties(&self) -> Option<IvgProperties> {
        self.prop_dispatch("Properties").map(IvgProperties::new)
    }

    /// 读取 URL（`URL`）。
    pub fn url(&self) -> Option<IvgUrl> {
        self.prop_dispatch("URL").map(IvgUrl::new)
    }

    // ---------------------------------------------------------
    // 抓取点
    // ---------------------------------------------------------

    /// 读取捕捉点集（`SnapPoints`）。
    pub fn snap_points(&self) -> Option<IvgSnapPoints> {
        self.prop_dispatch("SnapPoints").map(IvgSnapPoints::new)
    }

    /// 捕捉点集类型（COM 方法 `SnapPointsOfType`）。
    pub fn snap_points_of_type(&self, type_set: i32) -> Option<IvgSnapPoints> {
        let args = vec![Variant::from_i64(type_set as i64)];
        self.disp
            .invoke_method("SnapPointsOfType", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgSnapPoints::new)
    }

    /// 查找捕捉点（COM 方法 `FindSnapPoint`）。
    pub fn find_snap_point(&self, reference_data: impl Into<String>) -> Option<IvgSnapPoint> {
        let args = vec![Variant::from_str(reference_data.into())];
        self.disp
            .invoke_method("FindSnapPoint", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgSnapPoint::new)
    }

    // ---------------------------------------------------------
    // 布尔查询
    // ---------------------------------------------------------

    /// 是否于图形（COM 方法 `IsOnShape`）。
    pub fn is_on_shape(&self, x: f64, y: f64, hot_area: f64) -> Option<i64> {
        let args = vec![
            Variant::from_f64(x),
            Variant::from_f64(y),
            Variant::from_f64(hot_area),
        ];
        self.disp.invoke_method("IsOnShape", args).ok()?.to_i64().ok()
    }

    /// 查找点处的图形（COM 方法 `FindShapeAtPoint`）。
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

    /// 调用 COM 方法 `IsTypeAnyOf`。
    pub fn is_type_any_of(&self, type_list: Variant) -> Option<bool> {
        let args = vec![type_list];
        self.disp
            .invoke_method("IsTypeAnyOf", args)
            .ok()?
            .to_bool()
            .ok()
    }

    // ---------------------------------------------------------
    // 选择
    // ---------------------------------------------------------

    /// 添加选择（COM 方法 `AddToSelection`）。
    pub fn add_to_selection(&self) -> bool {
        self.disp.invoke_method("AddToSelection", vec![]).is_ok()
    }
    /// 移除选择（COM 方法 `RemoveFromSelection`）。
    pub fn remove_from_selection(&self) -> bool {
        self.disp.invoke_method("RemoveFromSelection", vec![]).is_ok()
    }
    /// 创建选择（COM 方法 `CreateSelection`）。
    pub fn create_selection(&self) -> bool {
        self.disp.invoke_method("CreateSelection", vec![]).is_ok()
    }

    // ---------------------------------------------------------
    // 位图区
    // ---------------------------------------------------------

    /// 转换位图（COM 方法 `ConvertToBitmap`）。
    #[allow(clippy::too_many_arguments)]
    pub fn convert_to_bitmap(
        &self,
        bit_depth: i32,
        grayscale: bool,
        dithered: bool,
        transparent_bg: bool,
        resolution: i32,
        anti_aliasing: i32,
        use_color_profile: bool,
        multi_channel: bool,
        always_overprint_black: bool,
        overprint_black_limit: i32,
    ) -> Option<IvgShape> {
        let args = vec![
            Variant::from_i64(bit_depth as i64),
            Variant::from_bool(grayscale),
            Variant::from_bool(dithered),
            Variant::from_bool(transparent_bg),
            Variant::from_i64(resolution as i64),
            Variant::from_i64(anti_aliasing as i64),
            Variant::from_bool(use_color_profile),
            Variant::from_bool(multi_channel),
            Variant::from_bool(always_overprint_black),
            Variant::from_i64(overprint_black_limit as i64),
        ];
        self.disp
            .invoke_method("ConvertToBitmap", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgShape::new)
    }

    // ---------------------------------------------------------
    // 图框精确裁剪 / 边界 / 相除
    // ---------------------------------------------------------

    /// 添加幂裁剪（COM 方法 `AddToPowerClip`）。
    pub fn add_to_power_clip(&self, shape: &IvgShape, center_in_container: i32) -> bool {
        let args = vec![
            shape.as_variant(),
            Variant::from_i64(center_in_container as i64),
        ];
        self.disp.invoke_method("AddToPowerClip", args).is_ok()
    }

    /// 移除容器（COM 方法 `RemoveFromContainer`）。
    pub fn remove_from_container(&self, level: i32) -> bool {
        let args = vec![Variant::from_i64(level as i64)];
        self.disp.invoke_method("RemoveFromContainer", args).is_ok()
    }

    /// 读取幂裁剪父对象（`PowerClipParent`）。
    pub fn power_clip_parent(&self) -> Option<IvgShape> {
        self.prop_dispatch("PowerClipParent").map(IvgShape::new)
    }

    /// 调用 COM 方法 `CreateBoundary`。
    pub fn create_boundary(
        &self,
        x: f64,
        y: f64,
        place_on_top: bool,
        delete_source: bool,
    ) -> Option<IvgShape> {
        let args = vec![
            Variant::from_f64(x),
            Variant::from_f64(y),
            Variant::from_bool(place_on_top),
            Variant::from_bool(delete_source),
        ];
        self.disp
            .invoke_method("CreateBoundary", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgShape::new)
    }

    /// 调用 COM 方法 `EqualDivide`。
    pub fn equal_divide(
        &self,
        divisions: i32,
        gap: f64,
        group: bool,
        combine: bool,
        delete_source: bool,
    ) -> Option<IvgShapeRange> {
        let args = vec![
            Variant::from_i64(divisions as i64),
            Variant::from_f64(gap),
            Variant::from_bool(group),
            Variant::from_bool(combine),
            Variant::from_bool(delete_source),
        ];
        self.disp
            .invoke_method("EqualDivide", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgShapeRange::new)
    }

    /// 调用 COM 方法 `StepAndRepeat`。
    pub fn step_and_repeat(
        &self,
        num_copies: i32,
        distance_x: f64,
        distance_y: f64,
        mode_x: i32,
        direction_x: i32,
        mode_y: i32,
        direction_y: i32,
    ) -> Option<IvgShapeRange> {
        let args = vec![
            Variant::from_i64(num_copies as i64),
            Variant::from_f64(distance_x),
            Variant::from_f64(distance_y),
            Variant::from_i64(mode_x as i64),
            Variant::from_i64(direction_x as i64),
            Variant::from_i64(mode_y as i64),
            Variant::from_i64(direction_y as i64),
        ];
        self.disp
            .invoke_method("StepAndRepeat", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgShapeRange::new)
    }

    // ---------------------------------------------------------
    // 对齐 / 分布
    // ---------------------------------------------------------

    /// `cdrAlignType` / `cdrTextAlignOrigin`
    pub fn align_to_shape(&self, align_type: i32, shape: &IvgShape, text_origin: i32) -> bool {
        let args = vec![
            Variant::from_i64(align_type as i64),
            shape.as_variant(),
            Variant::from_i64(text_origin as i64),
        ];
        self.disp.invoke_method("AlignToShape", args).is_ok()
    }

    /// 对齐图形范围（COM 方法 `AlignToShapeRange`）。
    pub fn align_to_shape_range(
        &self,
        align_type: i32,
        range: &IvgShapeRange,
        text_origin: i32,
    ) -> bool {
        let args = vec![
            Variant::from_i64(align_type as i64),
            range.as_variant(),
            Variant::from_i64(text_origin as i64),
        ];
        self.disp.invoke_method("AlignToShapeRange", args).is_ok()
    }

    /// 对齐页面（COM 方法 `AlignToPage`）。
    pub fn align_to_page(&self, align_type: i32, text_origin: i32) -> bool {
        let args = vec![
            Variant::from_i64(align_type as i64),
            Variant::from_i64(text_origin as i64),
        ];
        self.disp.invoke_method("AlignToPage", args).is_ok()
    }

    /// 对齐页面中心（COM 方法 `AlignToPageCenter`）。
    pub fn align_to_page_center(&self, align_type: i32, text_origin: i32) -> bool {
        let args = vec![
            Variant::from_i64(align_type as i64),
            Variant::from_i64(text_origin as i64),
        ];
        self.disp.invoke_method("AlignToPageCenter", args).is_ok()
    }

    /// 对齐网格（COM 方法 `AlignToGrid`）。
    pub fn align_to_grid(&self, align_type: i32, text_origin: i32) -> bool {
        let args = vec![
            Variant::from_i64(align_type as i64),
            Variant::from_i64(text_origin as i64),
        ];
        self.disp.invoke_method("AlignToGrid", args).is_ok()
    }

    /// 对齐点（COM 方法 `AlignToPoint`）。
    pub fn align_to_point(
        &self,
        align_type: i32,
        x: f64,
        y: f64,
        text_origin: i32,
    ) -> bool {
        let args = vec![
            Variant::from_i64(align_type as i64),
            Variant::from_f64(x),
            Variant::from_f64(y),
            Variant::from_i64(text_origin as i64),
        ];
        self.disp.invoke_method("AlignToPoint", args).is_ok()
    }

    /// `cdrDistributeType`
    pub fn distribute(&self, dist_type: i32, page_extent: bool) -> bool {
        let args = vec![
            Variant::from_i64(dist_type as i64),
            Variant::from_bool(page_extent),
        ];
        self.disp.invoke_method("Distribute", args).is_ok()
    }

    // ---------------------------------------------------------
    // 圆角 / 倒角 / 扇贝
    // ---------------------------------------------------------

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

    // ---------------------------------------------------------
    // 样式
    // ---------------------------------------------------------

    /// 应用样式（COM 方法 `ApplyStyle`）。
    pub fn apply_style(&self, style_name: impl Into<String>) -> bool {
        let args = vec![Variant::from_str(style_name.into())];
        self.disp.invoke_method("ApplyStyle", args).is_ok()
    }

    /// 读取样式（`Style`）。
    pub fn style(&self) -> Option<crate::style::IvgStyle> {
        self.prop_dispatch("Style").map(crate::style::IvgStyle::new)
    }

    // ---------------------------------------------------------
    // 文本环绕
    // ---------------------------------------------------------

    /// `cdrWrapStyle`
    pub fn wrap_text(&self) -> Option<i64> { self.prop_i64("WrapText") }
    /// 设置绕排文本（`WrapText`）。
    pub fn set_wrap_text(&self, v: i32) -> bool { self.put_i64("WrapText", v as i64) }

    /// 读取文本绕排偏移（`TextWrapOffset`）。
    pub fn text_wrap_offset(&self) -> Option<f64> { self.prop_f64("TextWrapOffset") }
    /// 设置文本绕排偏移（`TextWrapOffset`）。
    pub fn set_text_wrap_offset(&self, v: f64) -> bool {
        self.put_f64("TextWrapOffset", v)
    }

    /// 放置文本内部（COM 方法 `PlaceTextInside`）。
    pub fn place_text_inside(&self, text_shape: &IvgShape) -> Option<IvgShape> {
        let args = vec![text_shape.as_variant()];
        self.disp
            .invoke_method("PlaceTextInside", args)
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

    // ---------------------------------------------------------
    // 复制属性/ 比较
    // ---------------------------------------------------------

    /// `cdrCopyProperties` 位掩码。
    pub fn copy_properties_from(&self, source: &IvgShape, properties: i32) -> Option<bool> {
        let args = vec![
            source.as_variant(),
            Variant::from_i64(properties as i64),
        ];
        self.disp
            .invoke_method("CopyPropertiesFrom", args)
            .ok()?
            .to_bool()
            .ok()
    }

    /// `cdrCompareType` / `cdrCompareCondition`
    pub fn compare_to(
        &self,
        other: &IvgShape,
        compare_type: i32,
        condition: i32,
    ) -> Option<bool> {
        let args = vec![
            other.as_variant(),
            Variant::from_i64(compare_type as i64),
            Variant::from_i64(condition as i64),
        ];
        self.disp
            .invoke_method("CompareTo", args)
            .ok()?
            .to_bool()
            .ok()
    }

    // ---------------------------------------------------------
    // 
    // ---------------------------------------------------------

    /// 读取颜色集（`GetColors`）。
    pub fn get_colors(&self, max_bitmap_colors: i32) -> Option<Variant> {
        let args = vec![Variant::from_i64(max_bitmap_colors as i64)];
        self.disp.invoke_method("GetColors", args).ok()
    }

    /// 读取 COM 属性 `GetColorTypes`。
    pub fn get_color_types(&self) -> Option<Variant> {
        self.disp.invoke_method("GetColorTypes", vec![]).ok()
    }

    /// evaluate（COM 方法 `Evaluate`）。
    pub fn evaluate(&self, expr: impl Into<String>) -> Option<Variant> {
        let args = vec![Variant::from_str(expr.into())];
        self.disp.invoke_method("Evaluate", args).ok()
    }
}