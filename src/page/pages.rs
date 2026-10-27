//! `IVGPages` —— 页面集合

use wincom::{ComObject, Variant};
use windows::Win32::System::Com::IDispatch;

use crate::page::IvgPage;

/// CorelDRAW `IVGPages` 对象的后期绑定包装（页面集）。
///
/// 所有属性与方法通过 `IDispatch::Invoke` 按名称调用，调用失败时一般返回 `None` 或 `false`。
pub struct IvgPages {
    disp: ComObject,
}

impl IvgPages {
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

    /// 读取按索引或名称取条目（`Item`）。
    pub fn item(&self, index: i32) -> Option<IvgPage> {
        // `Item` 在类型库里可能注册为带参数的属性，也可能是方法。
        let v = self
            .disp
            .get_property_with("Item", vec![Variant::from_i64(index as i64)])
            .or_else(|_| self.disp.invoke_method("Item", vec![Variant::from_i64(index as i64)]))
            .ok()?;
        v.to_idispatch().ok().map(IvgPage::new)
    }

    /// 读取返回第一个条目（`First`）。
    pub fn first(&self) -> Option<IvgPage> {
        self.disp
            .get_property("First")
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgPage::new)
    }

    /// 读取返回最后一个条目（`Last`）。
    pub fn last(&self) -> Option<IvgPage> {
        self.disp
            .get_property("Last")
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgPage::new)
    }

    /// 遍历所有页面。
    pub fn all(&self) -> Vec<IvgPage> {
        let n = self.count().unwrap_or(0) as i32;
        (1..=n).filter_map(|i| self.item(i)).collect()
    }
}