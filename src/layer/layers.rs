//! `IVGLayers` —— 图层集合

use wincom::{ComObject, Variant};
use windows::Win32::System::Com::IDispatch;

use crate::layer::IvgLayer;

/// CorelDRAW `IVGLayers` 对象的后期绑定包装（图层集）。
///
/// 所有属性与方法通过 `IDispatch::Invoke` 按名称调用，调用失败时一般返回 `None` 或 `false`。
pub struct IvgLayers {
    disp: ComObject,
}

impl IvgLayers {
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
    // 基本信息
    // ---------------------------------------------------------

    /// 读取返回条目数量（`Count`）。
    pub fn count(&self) -> Option<i64> {
        self.disp.get_property("Count").ok()?.to_i64().ok()
    }

    /// 读取应用（`Application`）。
    pub fn application(&self) -> Option<crate::app::IvgApplication> {
        self.disp
            .get_property("Application")
            .ok()?
            .to_idispatch()
            .ok()
            .map(crate::app::IvgApplication::from_disp)
    }

    // ---------------------------------------------------------
    // 索引访问
    // ---------------------------------------------------------

    /// 通过索引＂-based）或名称访问。
    pub fn item(&self, index_or_name: Variant) -> Option<IvgLayer> {
        // `Item` 是带参数的属性，必须用 DISPATCH_PROPERTYGET。
        let args = vec![index_or_name];
        self.disp
            .get_property_with("Item", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgLayer::new)
    }

    /// 读取条目索引（`Item`）。
    pub fn item_by_index(&self, i: i32) -> Option<IvgLayer> {
        let v = self
            .disp
            .get_property_with("Item", vec![Variant::from_i64(i as i64)])
            .or_else(|_| self.disp.invoke_method("Item", vec![Variant::from_i64(i as i64)]))
            .ok()?;
        v.to_idispatch().ok().map(IvgLayer::new)
    }

    /// 读取条目名称（`Item`）。
    pub fn item_by_name(&self, name: impl Into<String>) -> Option<IvgLayer> {
        let name = name.into();
        let v = self
            .disp
            .get_property_with("Item", vec![Variant::from_str(name.clone())])
            .or_else(|_| self.disp.invoke_method("Item", vec![Variant::from_str(name)]))
            .ok()?;
        v.to_idispatch().ok().map(IvgLayer::new)
    }

    /// 直接按名称查找（与 `item_by_name` 等效，但走 COM 的 `Find`）。
    pub fn find(&self, layer_name: impl Into<String>) -> Option<IvgLayer> {
        let args = vec![Variant::from_str(layer_name.into())];
        self.disp
            .invoke_method("Find", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgLayer::new)
    }

    /// 最顶层图层。
    pub fn top(&self) -> Option<IvgLayer> {
        self.disp
            .get_property("Top")
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgLayer::new)
    }

    /// 最底层图层。
    pub fn bottom(&self) -> Option<IvgLayer> {
        self.disp
            .get_property("Bottom")
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgLayer::new)
    }

    // ---------------------------------------------------------
    // 遍历辅助
    // ---------------------------------------------------------

    /// 返回所有图层（按索引 1..Count 顺序）。
    pub fn all(&self) -> Vec<IvgLayer> {
        // `Layers.Item` 在 IDispatch 下索引行为不稳定（VT_I8 报 “Index 无效”，
        // VT_I4 对 0/1 返回同一图层），改用 Bottom + Above 遍历（自底向上）。
        let mut out = Vec::new();
        let mut current = self.bottom();
        while let Some(layer) = current {
            current = layer.above(true);
            out.push(layer);
        }
        out
    }
}