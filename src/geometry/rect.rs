//! `IVGRect` —— 矩形

use wincom::{ComObject, Variant};
use windows::Win32::System::Com::IDispatch;

/// CorelDRAW `IVGRect` 对象的后期绑定包装（矩形）。
///
/// 所有属性与方法通过 `IDispatch::Invoke` 按名称调用，调用失败时一般返回 `None` 或 `false`。
pub struct IvgRect {
    disp: ComObject,
}

impl IvgRect {
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

    // ---- 位置 + 尺寸（x / y 小写）----

    /// 读取 X 坐标（`x`）。
    pub fn x(&self) -> Option<f64> { self.prop_f64("x") }
    /// 设置 X 坐标（`x`）。
    pub fn set_x(&self, v: f64) -> bool { self.put_f64("x", v) }

    /// 读取 Y 坐标（`y`）。
    pub fn y(&self) -> Option<f64> { self.prop_f64("y") }
    /// 设置 Y 坐标（`y`）。
    pub fn set_y(&self, v: f64) -> bool { self.put_f64("y", v) }

    /// 读取宽度（`Width`）。
    pub fn width(&self) -> Option<f64> { self.prop_f64("Width") }
    /// 设置宽度（`Width`）。
    pub fn set_width(&self, v: f64) -> bool { self.put_f64("Width", v) }

    /// 读取高度（`Height`）。
    pub fn height(&self) -> Option<f64> { self.prop_f64("Height") }
    /// 设置高度（`Height`）。
    pub fn set_height(&self, v: f64) -> bool { self.put_f64("Height", v) }

    // ---- 便捷：四边+ 中心 ----

    /// 读取左（`Left`）。
    pub fn left(&self) -> Option<f64> { self.prop_f64("Left") }
    /// 读取右（`Right`）。
    pub fn right(&self) -> Option<f64> { self.prop_f64("Right") }
    /// 读取顶（`Top`）。
    pub fn top(&self) -> Option<f64> { self.prop_f64("Top") }
    /// 读取底（`Bottom`）。
    pub fn bottom(&self) -> Option<f64> { self.prop_f64("Bottom") }
    /// 读取中心 X 坐标（`CenterX`）。
    pub fn center_x(&self) -> Option<f64> { self.prop_f64("CenterX") }
    /// 读取中心 Y 坐标（`CenterY`）。
    pub fn center_y(&self) -> Option<f64> { self.prop_f64("CenterY") }

    // ---- 状态----

    /// 是否空（`IsEmpty`）。
    pub fn is_empty(&self) -> Option<bool> { self.prop_bool("IsEmpty") }

    // ---- 设置 / 获取 ----

    /// 设置矩形（`SetRect`）。
    pub fn set_rect(&self, x: f64, y: f64, w: f64, h: f64) -> bool {
        let args = vec![
            Variant::from_f64(x),
            Variant::from_f64(y),
            Variant::from_f64(w),
            Variant::from_f64(h),
        ];
        self.disp.invoke_method("SetRect", args).is_ok()
    }

    /// 读取矩形。
    pub fn get_rect(&self) -> Option<(f64, f64, f64, f64)> {
        // 4 out 参数用属性替代
        Some((self.x()?, self.y()?, self.width()?, self.height()?))
    }

    /// 清空全部内容（COM 方法 `Clear`）。
    pub fn clear(&self) -> bool {
        self.disp.invoke_method("Clear", vec![]).is_ok()
    }

    // ---- 复制 ----

    /// 复制 assign（COM 方法 `CopyAssign`）。
    pub fn copy_assign(&self, src: &IvgRect) -> bool {
        let args = vec![src.as_variant()];
        self.disp.invoke_method("CopyAssign", args).is_ok()
    }

    /// 读取复制（`GetCopy`）。
    pub fn get_copy(&self) -> Option<IvgRect> {
        self.disp
            .invoke_method("GetCopy", vec![])
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgRect::new)
    }

    // ---- 集合运算 ----

    /// intersect（COM 方法 `Intersect`）。
    pub fn intersect(&self, other: &IvgRect) -> Option<IvgRect> {
        let args = vec![other.as_variant()];
        self.disp
            .invoke_method("Intersect", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgRect::new)
    }

    /// 调用 COM 方法 `Union`。
    pub fn union(&self, other: &IvgRect) -> Option<IvgRect> {
        let args = vec![other.as_variant()];
        self.disp
            .invoke_method("Union", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgRect::new)
    }

    // ---- 平移 / 膨胀 ----

    /// 偏移（COM 方法 `Offset`）。
    pub fn offset(&self, dx: f64, dy: f64) -> bool {
        let args = vec![
            Variant::from_f64(dx),
            Variant::from_f64(dy),
        ];
        self.disp.invoke_method("Offset", args).is_ok()
    }

    /// 膨胀（COM 方法 `Inflate`）。
    pub fn inflate(&self, l: f64, t: f64, r: f64, b: f64) -> Option<bool> {
        let args = vec![
            Variant::from_f64(l),
            Variant::from_f64(t),
            Variant::from_f64(r),
            Variant::from_f64(b),
        ];
        self.disp.invoke_method("Inflate", args).ok()?.to_bool().ok()
    }

    // ---- 判断 ----

    /// 是否点内部（COM 方法 `IsPointInside`）。
    pub fn is_point_inside(&self, x: f64, y: f64) -> Option<bool> {
        let args = vec![
            Variant::from_f64(x),
            Variant::from_f64(y),
        ];
        self.disp
            .invoke_method("IsPointInside", args)
            .ok()?
            .to_bool()
            .ok()
    }

    // ---- 跨文档转换----

    /// 更改上下文（COM 方法 `ChangeContext`）。
    pub fn change_context(
        &self,
        src_doc: &crate::document::IvgDocument,
        dst_doc: &crate::document::IvgDocument,
    ) -> Option<IvgRect> {
        let args = vec![src_doc.as_variant(), dst_doc.as_variant()];
        self.disp
            .invoke_method("ChangeContext", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgRect::new)
    }
}