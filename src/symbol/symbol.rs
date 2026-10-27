//! `IVGSymbol` —— 符号实例
//!
//! 由 `shape.symbol()` 取得。一个符号实例在文档里表现为一个图形，
//! 但它引用的是某个 [`IvgSymbolDefinition`]。

use wincom::{ComObject, Variant};
use windows::Win32::System::Com::IDispatch;

use crate::shape::IvgShapeRange;
use crate::symbol::IvgSymbolDefinition;

/// CorelDRAW `IVGSymbol` 对象的后期绑定包装（符号）。
///
/// 所有属性与方法通过 `IDispatch::Invoke` 按名称调用，调用失败时一般返回 `None` 或 `false`。
pub struct IvgSymbol {
    disp: ComObject,
}

impl IvgSymbol {
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

    fn prop_dispatch(&self, name: &str) -> Option<IDispatch> {
        self.disp.get_property(name).ok()?.to_idispatch().ok()
    }

    /// 本实例引用的符号定义。
    pub fn definition(&self) -> Option<IvgSymbolDefinition> {
        self.prop_dispatch("Definition").map(IvgSymbolDefinition::new)
    }

    /// 把符号实例还原成一组图形（断开与定义的链接）。
    pub fn revert_to_shapes(&self) -> Option<IvgShapeRange> {
        self.disp
            .invoke_method("RevertToShapes", vec![])
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgShapeRange::new)
    }
}