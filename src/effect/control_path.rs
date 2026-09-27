//! `IVGEffectControlPath` 鈥斺€?鎺у埗璺緞
//!
//! RIDL 閲屽彧鏈?`Application` / `Parent` / `Effects` 涓変釜灞炴€э紝
//! 娌℃湁瀹為檯涓氬姟鏂规硶銆備繚鐣欐渶灏忛鏋躲€?

use wincom::ComObject;
use windows::Win32::System::Com::IDispatch;

use crate::effect::IvgEffects;

pub struct IvgEffectControlPath {
    disp: ComObject,
}

impl IvgEffectControlPath {
    pub fn new(disp: IDispatch) -> Self {
        Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) }
    }

    fn prop_dispatch(&self, name: &str) -> Option<IDispatch> {
        self.disp.get_property(name).ok()?.to_idispatch().ok()
    }

    /// 涓庢湰鎺у埗璺緞鍏宠仈鐨勬晥鏋滈泦鍚堛€?
    pub fn effects(&self) -> Option<IvgEffects> {
        self.prop_dispatch("Effects").map(IvgEffects::new)
    }
}