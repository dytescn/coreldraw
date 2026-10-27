//! `IVGShapeRange` —— 图形范围

use wincom::{ComObject, Variant};
use windows::Win32::System::Com::IDispatch;

use crate::color::IvgColor;
use crate::geometry::IvgRect;
use crate::layer::IvgLayer;
use crate::outline::IvgArrowHead;
use crate::shape::{IvgShape, IvgShapes};

/// CorelDRAW `IVGShapeRange` 对象的后期绑定包装（图形范围）。
///
/// 所有属性与方法通过 `IDispatch::Invoke` 按名称调用，调用失败时一般返回 `None` 或 `false`。
pub struct IvgShapeRange {
    disp: ComObject,
}

impl IvgShapeRange {
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

    fn prop_i64(&self, name: &str) -> Option<i64> {
        self.disp.get_property(name).ok()?.to_i64().ok()
    }

    fn prop_f64(&self, name: &str) -> Option<f64> {
        self.disp.get_property(name).ok()?.to_f64().ok()
    }

    fn prop_dispatch(&self, name: &str) -> Option<IDispatch> {
        self.disp.get_property(name).ok()?.to_idispatch().ok()
    }

    fn put_f64(&self, name: &str, v: f64) -> bool {
        let arg = Variant::from_f64(v);
        self.disp.set_property(name, vec![arg]).is_ok()
    }

    // fn put_i64(&self, name: &str, v: i64) -> bool {
    //     let arg = Variant::from_i64(v);
    //     self.disp.set_property(name, vec![arg]).is_ok()
    // }

    // ---- 基本 ----

    /// 读取返回条目数量（`Count`）。
    pub fn count(&self) -> Option<i64> { self.prop_i64("Count") }
    /// 读取图形集（`Shapes`）。
    pub fn shapes(&self) -> Option<IvgShapes> {
        self.prop_dispatch("Shapes").map(IvgShapes::new)
    }
    /// 读取第一个图形（`FirstShape`）。
    pub fn first_shape(&self) -> Option<IvgShape> {
        self.prop_dispatch("FirstShape").map(IvgShape::new)
    }
    /// 读取最后一个图形（`LastShape`）。
    pub fn last_shape(&self) -> Option<IvgShape> {
        self.prop_dispatch("LastShape").map(IvgShape::new)
    }

    /// 读取按索引或名称取条目。
    pub fn item(&self, index_or_name: Variant) -> Option<IvgShape> {
        // 带参数的属性：见 `IvgShapes::item`。
        let args = vec![index_or_name];
        self.disp
            .get_property_with("Item", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgShape::new)
    }

    /// 读取条目索引（`Item`）。
    pub fn item_by_index(&self, i: i32) -> Option<IvgShape> {
        let v = self
            .disp
            .get_property_with("Item", vec![Variant::from_i64(i as i64)])
            .or_else(|_| self.disp.invoke_method("Item", vec![Variant::from_i64(i as i64)]))
            .ok()?;
        v.to_idispatch().ok().map(IvgShape::new)
    }

    // ---- 几何 ----

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

    // ---- 变换 ----

    /// 移动（COM 方法 `Move`）。
    pub fn move_by(&self, dx: f64, dy: f64) -> bool {
        let args = vec![
            Variant::from_f64(dx),
            Variant::from_f64(dy),
        ];
        self.disp.invoke_method("Move", args).is_ok()
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

    /// 设置位置（`SetPosition`）。
    pub fn set_position(&self, x: f64, y: f64) -> bool {
        let args = vec![
            Variant::from_f64(x),
            Variant::from_f64(y),
        ];
        self.disp.invoke_method("SetPosition", args).is_ok()
    }

    /// 设置尺寸（`SetSize`）。
    pub fn set_size(&self, w: f64, h: f64) -> bool {
        let args = vec![
            Variant::from_f64(w),
            Variant::from_f64(h),
        ];
        self.disp.invoke_method("SetSize", args).is_ok()
    }

    /// 翻转（COM 方法 `Flip`）。
    pub fn flip(&self, axes: i32) -> bool {
        let args = vec![Variant::from_i64(axes as i64)];
        self.disp.invoke_method("Flip", args).is_ok()
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

    // ---- 复制 / 删除 ----

    /// duplicate（COM 方法 `Duplicate`）。
    pub fn duplicate(&self, dx: f64, dy: f64) -> Option<IvgShapeRange> {
        let args = vec![
            Variant::from_f64(dx),
            Variant::from_f64(dy),
        ];
        self.disp
            .invoke_method("Duplicate", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgShapeRange::new)
    }

    /// 克隆（COM 方法 `Clone`）。
    pub fn clone(&self, dx: f64, dy: f64) -> Option<IvgShapeRange> {
        let args = vec![
            Variant::from_f64(dx),
            Variant::from_f64(dy),
        ];
        self.disp
            .invoke_method("Clone", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgShapeRange::new)
    }

    /// 删除（COM 方法 `Delete`）。
    pub fn delete(&self) -> bool {
        self.disp.invoke_method("Delete", vec![]).is_ok()
    }

    /// 复制（COM 方法 `Copy`）。
    pub fn copy(&self) -> bool {
        self.disp.invoke_method("Copy", vec![]).is_ok()
    }

    /// cut（COM 方法 `Cut`）。
    pub fn cut(&self) -> bool {
        self.disp.invoke_method("Cut", vec![]).is_ok()
    }

    // ---- 群组 ----

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

    /// 取消群组扩展版（COM 方法 `UngroupEx`）。
    pub fn ungroup_ex(&self) -> Option<IvgShapeRange> {
        self.disp
            .invoke_method("UngroupEx", vec![])
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgShapeRange::new)
    }

    /// 取消群组全部扩展版（COM 方法 `UngroupAllEx`）。
    pub fn ungroup_all_ex(&self) -> Option<IvgShapeRange> {
        self.disp
            .invoke_method("UngroupAllEx", vec![])
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgShapeRange::new)
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

    // ---- Z 顺序 ----

    /// 排序前（COM 方法 `OrderToFront`）。
    pub fn order_to_front(&self) -> bool {
        self.disp.invoke_method("OrderToFront", vec![]).is_ok()
    }
    /// 排序后（COM 方法 `OrderToBack`）。
    pub fn order_to_back(&self) -> bool {
        self.disp.invoke_method("OrderToBack", vec![]).is_ok()
    }
    /// 排序 reverse（COM 方法 `OrderReverse`）。
    pub fn order_reverse(&self) -> bool {
        self.disp.invoke_method("OrderReverse", vec![]).is_ok()
    }

    // ---- 集合操作 ----

    /// 添加（COM 方法 `Add`）。
    pub fn add(&self, shape: &IvgShape) -> bool {
        let args = vec![shape.as_variant()];
        self.disp.invoke_method("Add", args).is_ok()
    }
    /// 添加范围（COM 方法 `AddRange`）。
    pub fn add_range(&self, other: &IvgShapeRange) -> bool {
        let args = vec![other.as_variant()];
        self.disp.invoke_method("AddRange", args).is_ok()
    }
    /// 移除（COM 方法 `Remove`）。
    pub fn remove(&self, index: i32) -> bool {
        let args = vec![Variant::from_i64(index as i64)];
        self.disp.invoke_method("Remove", args).is_ok()
    }
    /// 移除范围（COM 方法 `RemoveRange`）。
    pub fn remove_range(&self, other: &IvgShapeRange) -> bool {
        let args = vec![other.as_variant()];
        self.disp.invoke_method("RemoveRange", args).is_ok()
    }
    /// 移除全部（COM 方法 `RemoveAll`）。
    pub fn remove_all(&self) -> bool {
        self.disp.invoke_method("RemoveAll", vec![]).is_ok()
    }

    /// 索引（COM 方法 `IndexOf`）。
    pub fn index_of(&self, shape: &IvgShape) -> Option<i64> {
        let args = vec![shape.as_variant()];
        self.disp.invoke_method("IndexOf", args).ok()?.to_i64().ok()
    }

    /// exists（COM 方法 `Exists`）。
    pub fn exists(&self, shape: &IvgShape) -> Option<bool> {
        let args = vec![shape.as_variant()];
        self.disp.invoke_method("Exists", args).ok()?.to_bool().ok()
    }

    // ---- 选择 ----

    /// 创建选择（COM 方法 `CreateSelection`）。
    pub fn create_selection(&self) -> bool {
        self.disp.invoke_method("CreateSelection", vec![]).is_ok()
    }
    /// 添加选择（COM 方法 `AddToSelection`）。
    pub fn add_to_selection(&self) -> bool {
        self.disp.invoke_method("AddToSelection", vec![]).is_ok()
    }
    /// 移除选择（COM 方法 `RemoveFromSelection`）。
    pub fn remove_from_selection(&self) -> bool {
        self.disp.invoke_method("RemoveFromSelection", vec![]).is_ok()
    }

    // ---- 填充 / 轮廓 ----

    /// 取消填充（COM 方法 `ApplyNoFill`）。
    pub fn apply_no_fill(&self) -> bool {
        self.disp.invoke_method("ApplyNoFill", vec![]).is_ok()
    }
    /// 应用均匀填充（COM 方法 `ApplyUniformFill`）。
    pub fn apply_uniform_fill(&self, color: &IvgColor) -> bool {
        let args = vec![color.as_variant()];
        self.disp.invoke_method("ApplyUniformFill", args).is_ok()
    }

    /// 应用轮廓（COM 方法 `ApplyOutline`）。
    pub fn apply_outline(&self, outline: &crate::outline::IvgOutline) -> bool {
        let args = vec![outline.as_variant()];
        self.disp.invoke_method("ApplyOutline", args).is_ok()
    }
    /// 应用填充（COM 方法 `ApplyFill`）。
    pub fn apply_fill(&self, fill: &crate::fill::IvgFill) -> bool {
        let args = vec![fill.as_variant()];
        self.disp.invoke_method("ApplyFill", args).is_ok()
    }

    /// 设置轮廓属性集（`SetOutlineProperties`）。
    #[allow(clippy::too_many_arguments)]
    pub fn set_outline_properties(
        &self,
        width: f64,
        style: &crate::outline::IvgOutlineStyle,
        color: &IvgColor,
        start_arrow: &IvgArrowHead,
        end_arrow: &IvgArrowHead,
        behind_fill: i32,
        scale_with_shape: i32,
        line_caps: i32,
        line_join: i32,
        nib_angle: f64,
        nib_stretch: i32,
        dash_dot_length: f64,
        pen_width: f64,
        miter_limit: f64,
    ) -> bool {
        let args = vec![
            Variant::from_f64(width),
            style.as_variant(),
            color.as_variant(),
            start_arrow.as_variant(),
            end_arrow.as_variant(),
            Variant::from_i64(behind_fill as i64),
            Variant::from_i64(scale_with_shape as i64),
            Variant::from_i64(line_caps as i64),
            Variant::from_i64(line_join as i64),
            Variant::from_f64(nib_angle),
            Variant::from_i64(nib_stretch as i64),
            Variant::from_f64(dash_dot_length),
            Variant::from_f64(pen_width),
            Variant::from_f64(miter_limit),
        ];
        self.disp.invoke_method("SetOutlineProperties", args).is_ok()
    }

    // ---- 变换 / 转换 ----

    /// 转换曲线集（COM 方法 `ConvertToCurves`）。
    pub fn convert_to_curves(&self) -> bool {
        self.disp.invoke_method("ConvertToCurves", vec![]).is_ok()
    }

    /// 转换位图（COM 方法 `ConvertToBitmap`）。
    pub fn convert_to_bitmap(
        &self,
        bit_depth: i32, grayscale: bool, dithered: bool, transparent_bg: bool,
        resolution: i32, anti_aliasing: i32, use_color_profile: bool,
        multi_channel: bool, always_overprint_black: bool, overprint_black_limit: i32,
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

    // ---- 样式 ----

    /// 应用样式（COM 方法 `ApplyStyle`）。
    pub fn apply_style(&self, style_name: impl Into<String>) -> bool {
        let args = vec![Variant::from_str(style_name.into())];
        self.disp.invoke_method("ApplyStyle", args).is_ok()
    }

    // ---- 灞?----

    /// 移动图层（COM 方法 `MoveToLayer`）。
    pub fn move_to_layer(&self, l: &IvgLayer) -> bool {
        let args = vec![l.as_variant()];
        self.disp.invoke_method("MoveToLayer", args).is_ok()
    }

    /// 复制图层（COM 方法 `CopyToLayer`）。
    pub fn copy_to_layer(&self, l: &IvgLayer) -> Option<IvgShapeRange> {
        let args = vec![l.as_variant()];
        self.disp
            .invoke_method("CopyToLayer", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgShapeRange::new)
    }

    // ---- 对齐 / 分布 ----

    /// 对齐图形（COM 方法 `AlignToShape`）。
    pub fn align_to_shape(&self, align_type: i32, shape: &IvgShape, text_origin: i32) -> bool {
        let args = vec![
            Variant::from_i64(align_type as i64),
            shape.as_variant(),
            Variant::from_i64(text_origin as i64),
        ];
        self.disp.invoke_method("AlignToShape", args).is_ok()
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
    pub fn align_to_point(&self, align_type: i32, x: f64, y: f64, text_origin: i32) -> bool {
        let args = vec![
            Variant::from_i64(align_type as i64),
            Variant::from_f64(x),
            Variant::from_f64(y),
            Variant::from_i64(text_origin as i64),
        ];
        self.disp.invoke_method("AlignToPoint", args).is_ok()
    }

    /// distribute（COM 方法 `Distribute`）。
    pub fn distribute(&self, dist_type: i32, page_extent: bool) -> bool {
        let args = vec![
            Variant::from_i64(dist_type as i64),
            Variant::from_bool(page_extent),
        ];
        self.disp.invoke_method("Distribute", args).is_ok()
    }

    // ---- 其它 ----

    /// 设置边界框（`SetBoundingBox`）。
    pub fn set_bounding_box(
        &self,
        x: f64, y: f64, w: f64, h: f64,
        keep_aspect: bool, ref_point: i32,
    ) -> bool {
        let args = vec![
            Variant::from_f64(x),
            Variant::from_f64(y),
            Variant::from_f64(w),
            Variant::from_f64(h),
            Variant::from_bool(keep_aspect),
            Variant::from_i64(ref_point as i64),
        ];
        self.disp.invoke_method("SetBoundingBox", args).is_ok()
    }

    /// 锁定（COM 方法 `Lock`）。
    pub fn lock(&self) -> bool { self.disp.invoke_method("Lock", vec![]).is_ok() }
    /// 解锁（COM 方法 `Unlock`）。
    pub fn unlock(&self) -> bool { self.disp.invoke_method("Unlock", vec![]).is_ok() }

    /// 清除变换（COM 方法 `ClearTransformations`）。
    pub fn clear_transformations(&self) -> bool {
        self.disp.invoke_method("ClearTransformations", vec![]).is_ok()
    }

    /// 展平效果集（COM 方法 `FlattenEffects`）。
    pub fn flatten_effects(&self) -> bool {
        self.disp.invoke_method("FlattenEffects", vec![]).is_ok()
    }
}