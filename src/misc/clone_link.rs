//! `IVGCloneLink` —— 图形克隆链

use wincom::{ComObject, Variant};
use windows::Win32::System::Com::IDispatch;

use crate::shape::IvgShape;

/// CorelDRAW `IVGCloneLink` 对象的后期绑定包装（克隆链接）。
///
/// 所有属性与方法通过 `IDispatch::Invoke` 按名称调用，调用失败时一般返回 `None` 或 `false`。
pub struct IvgCloneLink {
    disp: ComObject,
}

impl IvgCloneLink {
    /// 由 COM `IDispatch` 构造包装对象。
    pub fn new(disp: IDispatch) -> Self {
        Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) }
    }

    /// 从 COM `IDispatch` 构造包装对象，等价于 `new`。
    pub fn from_disp(disp: IDispatch) -> Self {
        Self::new(disp)
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

    /// 读取克隆父对象（`CloneParent`）。
    pub fn clone_parent(&self) -> Option<IvgShape> {
        self.prop_dispatch("CloneParent").map(IvgShape::new)
    }

    /// 读取填充已链接（`FillLinked`）。
    pub fn fill_linked(&self) -> Option<bool> { self.prop_bool("FillLinked") }
    /// 设置填充已链接（`FillLinked`）。
    pub fn set_fill_linked(&self, v: bool) -> bool { self.put_bool("FillLinked", v) }

    /// 读取轮廓已链接（`OutlineLinked`）。
    pub fn outline_linked(&self) -> Option<bool> { self.prop_bool("OutlineLinked") }
    /// 设置轮廓已链接（`OutlineLinked`）。
    pub fn set_outline_linked(&self, v: bool) -> bool {
        self.put_bool("OutlineLinked", v)
    }

    /// 读取图形已链接（`ShapeLinked`）。
    pub fn shape_linked(&self) -> Option<bool> { self.prop_bool("ShapeLinked") }
    /// 设置图形已链接（`ShapeLinked`）。
    pub fn set_shape_linked(&self, v: bool) -> bool { self.put_bool("ShapeLinked", v) }

    /// 读取变换已链接（`TransformLinked`）。
    pub fn transform_linked(&self) -> Option<bool> { self.prop_bool("TransformLinked") }
    /// 设置变换已链接（`TransformLinked`）。
    pub fn set_transform_linked(&self, v: bool) -> bool {
        self.put_bool("TransformLinked", v)
    }

    /// 读取 COM 属性 `BitmapColorMaskLinked`。
    pub fn bitmap_color_mask_linked(&self) -> Option<bool> {
        self.prop_bool("BitmapColorMaskLinked")
    }
    /// 设置 COM 属性 `BitmapColorMaskLinked`。
    pub fn set_bitmap_color_mask_linked(&self, v: bool) -> bool {
        self.put_bool("BitmapColorMaskLinked", v)
    }

    /// 还原全部链接（COM 方法 `RestoreAllLinks`）。
    pub fn restore_all_links(&self) -> bool {
        self.disp.invoke_method("RestoreAllLinks", vec![]).is_ok()
    }
}