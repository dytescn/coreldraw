//! `IVGEffectControlPath` —— 控制路径
//!
//! RIDL 里只有 `Application` / `Parent` / `Effects` 三个属性，
//! 没有实际业务方法。保留最小骨架。

use wincom::ComObject;
use windows::Win32::System::Com::IDispatch;

use crate::effect::IvgEffects;

/// CorelDRAW `IVGEffectControlPath` 对象的后期绑定包装（效果控件路径）。
///
/// 所有属性与方法通过 `IDispatch::Invoke` 按名称调用，调用失败时一般返回 `None` 或 `false`。
pub struct IvgEffectControlPath {
    disp: ComObject,
}

impl IvgEffectControlPath {
    /// 由 COM `IDispatch` 构造包装对象。
    pub fn new(disp: IDispatch) -> Self {
        Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) }
    }

    fn prop_dispatch(&self, name: &str) -> Option<IDispatch> {
        self.disp.get_property(name).ok()?.to_idispatch().ok()
    }

    /// 与本控制路径关联的效果集合。
    pub fn effects(&self) -> Option<IvgEffects> {
        self.prop_dispatch("Effects").map(IvgEffects::new)
    }
}