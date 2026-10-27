//! `IVGArrowHead` / `IVGArrowHeads` / `IVGArrowHeadOptions` —— 箭头

use wincom::{ComObject, Variant};
use windows::Win32::System::Com::IDispatch;

use crate::curve::IvgCurve;

// =============================================================
// IvgArrowHead
// =============================================================

/// CorelDRAW `IVGArrowHead` 对象的后期绑定包装（箭头头）。
///
/// 所有属性与方法通过 `IDispatch::Invoke` 按名称调用，调用失败时一般返回 `None` 或 `false`。
pub struct IvgArrowHead {
    disp: ComObject,
}

impl IvgArrowHead {
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

    fn prop_string(&self, name: &str) -> Option<String> {
        self.disp.get_property(name).ok()?.to_string().ok()
    }

    fn prop_dispatch(&self, name: &str) -> Option<IDispatch> {
        self.disp.get_property(name).ok()?.to_idispatch().ok()
    }

    fn put_string(&self, name: &str, v: impl Into<String>) -> bool {
        let arg = Variant::from_str(v.into());
        self.disp.set_property(name, vec![arg]).is_ok()
    }

    // ---------------------------------------------------------
    // 索引 / 名称
    // ---------------------------------------------------------

    /// 读取索引（`Index`）。
    pub fn index(&self) -> Option<i64> { self.prop_i64("Index") }

    /// 读取名称（`Name`）。
    pub fn name(&self) -> Option<String> { self.prop_string("Name") }
    /// 设置名称（`Name`）。
    pub fn set_name(&self, v: impl Into<String>) -> bool { self.put_string("Name", v) }

    /// 读取显示名称（`DisplayName`）。
    pub fn display_name(&self) -> Option<String> { self.prop_string("DisplayName") }

    // ---------------------------------------------------------
    // 形状
    // ---------------------------------------------------------

    /// 箭头形状的轮廓曲线（本地坐标）。
    pub fn curve(&self) -> Option<IvgCurve> {
        self.prop_dispatch("Curve").map(IvgCurve::new)
    }

    /// 相对于线宽的基准缩放。
    pub fn base_outline_scale(&self) -> Option<f64> {
        self.prop_f64("BaseOutlineScale")
    }

    /// 箭头中心相对本地坐标原点的 X 偏移。
    pub fn center_x(&self) -> Option<f64> { self.prop_f64("CenterX") }
    /// 读取中心 Y 坐标（`CenterY`）。
    pub fn center_y(&self) -> Option<f64> { self.prop_f64("CenterY") }

    /// 相对线段端点的偏移。
    pub fn line_offset(&self) -> Option<f64> { self.prop_f64("LineOffset") }

    // ---------------------------------------------------------
    // 操作
    // ---------------------------------------------------------

    /// 绑定到指定文档（用于跨文档复制时确保归属）。
    pub fn bind_to_document(&self, doc: &crate::document::IvgDocument) -> Option<IvgArrowHead> {
        let args = vec![doc.as_variant()];
        self.disp
            .invoke_method("BindToDocument", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgArrowHead::new)
    }

    /// 比较（COM 方法 `CompareWith`）。
    pub fn compare_with(&self, other: &IvgArrowHead) -> Option<bool> {
        let args = vec![other.as_variant()];
        self.disp
            .invoke_method("CompareWith", args)
            .ok()?
            .to_bool()
            .ok()
    }
}

// =============================================================
// IvgArrowHeadOptions
// =============================================================

/// 单个图形上箭头的**几何调整**（与 `IvgArrowHead` 是模板不同）。
pub struct IvgArrowHeadOptions {
    disp: ComObject,
}

impl IvgArrowHeadOptions {
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

    fn prop_f64(&self, name: &str) -> Option<f64> {
        self.disp.get_property(name).ok()?.to_f64().ok()
    }

    fn prop_bool(&self, name: &str) -> Option<bool> {
        self.disp.get_property(name).ok()?.to_bool().ok()
    }

    fn put_f64(&self, name: &str, v: f64) -> bool {
        let arg = Variant::from_f64(v);
        self.disp.set_property(name, vec![arg]).is_ok()
    }

    fn put_bool(&self, name: &str, v: bool) -> bool {
        let arg = Variant::from_bool(v);
        self.disp.set_property(name, vec![arg]).is_ok()
    }

    // ---- 长度 / 宽度 ----

    /// 读取长度（`Length`）。
    pub fn length(&self) -> Option<f64> { self.prop_f64("Length") }
    /// 设置长度（`Length`）。
    pub fn set_length(&self, v: f64) -> bool { self.put_f64("Length", v) }

    /// 读取宽度（`Width`）。
    pub fn width(&self) -> Option<f64> { self.prop_f64("Width") }
    /// 设置宽度（`Width`）。
    pub fn set_width(&self, v: f64) -> bool { self.put_f64("Width", v) }

    // ---- 偏移 ----

    /// 读取偏移 X 坐标（`OffsetX`）。
    pub fn offset_x(&self) -> Option<f64> { self.prop_f64("OffsetX") }
    /// 设置偏移 X 坐标（`OffsetX`）。
    pub fn set_offset_x(&self, v: f64) -> bool { self.put_f64("OffsetX", v) }

    /// 读取偏移 Y 坐标（`OffsetY`）。
    pub fn offset_y(&self) -> Option<f64> { self.prop_f64("OffsetY") }
    /// 设置偏移 Y 坐标（`OffsetY`）。
    pub fn set_offset_y(&self, v: f64) -> bool { self.put_f64("OffsetY", v) }

    // ---- 旋转 ----

    /// 读取旋转角度（`RotationAngle`）。
    pub fn rotation_angle(&self) -> Option<f64> { self.prop_f64("RotationAngle") }
    /// 设置旋转角度（`RotationAngle`）。
    pub fn set_rotation_angle(&self, v: f64) -> bool {
        self.put_f64("RotationAngle", v)
    }

    // ---- 翻转 ----

    /// 读取水平翻转（`FlipHorizontal`）。
    pub fn flip_horizontal(&self) -> Option<bool> { self.prop_bool("FlipHorizontal") }
    /// 设置水平翻转（`FlipHorizontal`）。
    pub fn set_flip_horizontal(&self, v: bool) -> bool {
        self.put_bool("FlipHorizontal", v)
    }

    /// 读取垂直翻转（`FlipVertical`）。
    pub fn flip_vertical(&self) -> Option<bool> { self.prop_bool("FlipVertical") }
    /// 设置垂直翻转（`FlipVertical`）。
    pub fn set_flip_vertical(&self, v: bool) -> bool {
        self.put_bool("FlipVertical", v)
    }

    /// RIDL 里还有一个拼写错误的 `FlipVerical`，一并暴露。
    pub fn flip_verical(&self) -> Option<bool> { self.prop_bool("FlipVerical") }
    /// 设置 COM 属性 `FlipVerical`。
    pub fn set_flip_verical(&self, v: bool) -> bool {
        self.put_bool("FlipVerical", v)
    }

    /// `cdrFlipAxes`
    pub fn flip(&self, axes: i32) -> bool {
        let args = vec![Variant::from_i64(axes as i64)];
        self.disp.invoke_method("Flip", args).is_ok()
    }

    // ---- 复制 ----

    /// 复制 assign（COM 方法 `CopyAssign`）。
    pub fn copy_assign(&self, src: &IvgArrowHeadOptions) -> bool {
        let args = vec![src.as_variant()];
        self.disp.invoke_method("CopyAssign", args).is_ok()
    }

    /// 读取复制（`GetCopy`）。
    pub fn get_copy(&self) -> Option<IvgArrowHeadOptions> {
        self.disp
            .invoke_method("GetCopy", vec![])
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgArrowHeadOptions::new)
    }
}

// =============================================================
// IvgArrowHeads
// =============================================================

/// 文档级箭头集合（`Application.ArrowHeads`）。
pub struct IvgArrowHeads {
    disp: ComObject,
}

impl IvgArrowHeads {
    /// 由 COM `IDispatch` 构造包装对象。
    pub fn new(disp: IDispatch) -> Self {
        Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) }
    }

    fn prop_i64(&self, name: &str) -> Option<i64> {
        self.disp.get_property(name).ok()?.to_i64().ok()
    }

    /// 读取返回条目数量（`Count`）。
    pub fn count(&self) -> Option<i64> { self.prop_i64("Count") }

    /// 按索引或名称取条目（COM 方法 `Item`）。
    pub fn item(&self, index: i32) -> Option<IvgArrowHead> {
        let args = vec![Variant::from_i64(index as i64)];
        self.disp
            .invoke_method("Item", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgArrowHead::new)
    }

    /// 添加一个箭头。
    pub fn add(&self, arrow: &IvgArrowHead) -> Option<IvgArrowHead> {
        let args = vec![arrow.as_variant()];
        self.disp
            .invoke_method("Add", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgArrowHead::new)
    }

    /// 替换指定索引的箭头。
    pub fn replace(&self, index: i32, arrow: &IvgArrowHead) -> Option<IvgArrowHead> {
        let args = vec![
            Variant::from_i64(index as i64),
            arrow.as_variant(),
        ];
        self.disp
            .invoke_method("Replace", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgArrowHead::new)
    }

    /// 移除（COM 方法 `Remove`）。
    pub fn remove(&self, index: i32) -> bool {
        let args = vec![Variant::from_i64(index as i64)];
        self.disp.invoke_method("Remove", args).is_ok()
    }
}