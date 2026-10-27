//! `ICUIScreenRect` —— 屏幕坐标矩形

use wincom::{ComObject, Variant};
use windows::Win32::System::Com::IDispatch;

/// CorelDRAW 对象的后期绑定包装（屏幕矩形）。
///
/// 所有属性与方法通过 `IDispatch::Invoke` 按名称调用，调用失败时一般返回 `None` 或 `false`。
pub struct ICuiScreenRect {
    disp: ComObject,
}

impl ICuiScreenRect {
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

    fn prop_i64(&self, name: &str) -> Option<i64> {
        self.disp.get_property(name).ok()?.to_i64().ok()
    }

    fn prop_bool(&self, name: &str) -> Option<bool> {
        self.disp.get_property(name).ok()?.to_bool().ok()
    }

    fn put_i64(&self, name: &str, v: i64) -> bool {
        let arg = Variant::from_i64(v);
        self.disp.set_property(name, vec![arg]).is_ok()
    }

    // ---- 四边 ----

    /// 读取左（`Left`）。
    pub fn left(&self) -> Option<i64> { self.prop_i64("Left") }
    /// 设置左（`Left`）。
    pub fn set_left(&self, v: i32) -> bool { self.put_i64("Left", v as i64) }

    /// 读取右（`Right`）。
    pub fn right(&self) -> Option<i64> { self.prop_i64("Right") }
    /// 设置右（`Right`）。
    pub fn set_right(&self, v: i32) -> bool { self.put_i64("Right", v as i64) }

    /// 读取顶（`Top`）。
    pub fn top(&self) -> Option<i64> { self.prop_i64("Top") }
    /// 设置顶（`Top`）。
    pub fn set_top(&self, v: i32) -> bool { self.put_i64("Top", v as i64) }

    /// 读取底（`Bottom`）。
    pub fn bottom(&self) -> Option<i64> { self.prop_i64("Bottom") }
    /// 设置底（`Bottom`）。
    pub fn set_bottom(&self, v: i32) -> bool { self.put_i64("Bottom", v as i64) }

    // ---- 尺寸 ----

    /// 读取宽度（`Width`）。
    pub fn width(&self) -> Option<i64> { self.prop_i64("Width") }
    /// 设置宽度（`Width`）。
    pub fn set_width(&self, v: i32) -> bool { self.put_i64("Width", v as i64) }

    /// 读取高度（`Height`）。
    pub fn height(&self) -> Option<i64> { self.prop_i64("Height") }
    /// 设置高度（`Height`）。
    pub fn set_height(&self, v: i32) -> bool { self.put_i64("Height", v as i64) }

    // ---- 中心 ----

    /// 读取中心 X 坐标（`CenterX`）。
    pub fn center_x(&self) -> Option<i64> { self.prop_i64("CenterX") }
    /// 设置中心 X 坐标（`CenterX`）。
    pub fn set_center_x(&self, v: i32) -> bool { self.put_i64("CenterX", v as i64) }

    /// 读取中心 Y 坐标（`CenterY`）。
    pub fn center_y(&self) -> Option<i64> { self.prop_i64("CenterY") }
    /// 设置中心 Y 坐标（`CenterY`）。
    pub fn set_center_y(&self, v: i32) -> bool { self.put_i64("CenterY", v as i64) }

    // ---- 只读 ----

    /// 是否 read 仅（`ReadOnly`）。
    pub fn is_read_only(&self) -> Option<bool> { self.prop_bool("ReadOnly") }
    /// 是否空（`IsEmpty`）。
    pub fn is_empty(&self) -> Option<bool> { self.prop_bool("IsEmpty") }

    // ---- 操作 ----

    /// 设置位置（`SetPosition`）。
    pub fn set_position(&self, left: i32, top: i32, w: i32, h: i32) -> bool {
        let args = vec![
            Variant::from_i64(left as i64),
            Variant::from_i64(top as i64),
            Variant::from_i64(w as i64),
            Variant::from_i64(h as i64),
        ];
        self.disp.invoke_method("SetPosition", args).is_ok()
    }

    /// resize（COM 方法 `Resize`）。
    pub fn resize(&self, w: i32, h: i32) -> bool {
        let args = vec![
            Variant::from_i64(w as i64),
            Variant::from_i64(h as i64),
        ];
        self.disp.invoke_method("Resize", args).is_ok()
    }

    /// 移动（COM 方法 `Move`）。
    pub fn move_by(&self, left: i32, top: i32) -> bool {
        let args = vec![
            Variant::from_i64(left as i64),
            Variant::from_i64(top as i64),
        ];
        self.disp.invoke_method("Move", args).is_ok()
    }

    /// 读取复制（`GetCopy`）。
    pub fn get_copy(&self) -> Option<ICuiScreenRect> {
        self.disp
            .invoke_method("GetCopy", vec![])
            .ok()?
            .to_idispatch()
            .ok()
            .map(ICuiScreenRect::new)
    }

    /// 复制 assign（COM 方法 `CopyAssign`）。
    pub fn copy_assign(&self, src: &ICuiScreenRect) -> bool {
        let args = vec![src.as_variant()];
        self.disp.invoke_method("CopyAssign", args).is_ok()
    }

    /// 偏移（COM 方法 `Offset`）。
    pub fn offset(&self, dx: i32, dy: i32) -> bool {
        let args = vec![
            Variant::from_i64(dx as i64),
            Variant::from_i64(dy as i64),
        ];
        self.disp.invoke_method("Offset", args).is_ok()
    }

    /// 膨胀（COM 方法 `Inflate`）。
    pub fn inflate(&self, l: i32, t: i32, r: i32, b: i32) -> bool {
        let args = vec![
            Variant::from_i64(l as i64),
            Variant::from_i64(t as i64),
            Variant::from_i64(r as i64),
            Variant::from_i64(b as i64),
        ];
        self.disp.invoke_method("Inflate", args).is_ok()
    }

    /// 是否点内部（COM 方法 `IsPointInside`）。
    pub fn is_point_inside(&self, x: i32, y: i32) -> Option<bool> {
        let args = vec![
            Variant::from_i64(x as i64),
            Variant::from_i64(y as i64),
        ];
        self.disp
            .invoke_method("IsPointInside", args)
            .ok()?
            .to_bool()
            .ok()
    }

    // ---- 集合运算 ----

    /// 调用 COM 方法 `Union`。
    pub fn union(&self, other: &ICuiScreenRect) -> Option<ICuiScreenRect> {
        let args = vec![other.as_variant()];
        self.disp
            .invoke_method("Union", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(ICuiScreenRect::new)
    }

    /// intersect（COM 方法 `Intersect`）。
    pub fn intersect(&self, other: &ICuiScreenRect) -> Option<ICuiScreenRect> {
        let args = vec![other.as_variant()];
        self.disp
            .invoke_method("Intersect", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(ICuiScreenRect::new)
    }
}