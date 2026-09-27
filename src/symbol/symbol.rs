//! `IVGSymbol` 鈥斺€?绗﹀彿瀹炰緥
//!
//! 鐢?`shape.symbol()` 鍙栧緱銆備竴涓鍙峰疄渚嬪湪鏂囨。閲岃〃鐜颁负涓€涓浘褰紝
//! 浣嗗畠寮曠敤鐨勬槸鏌愪釜 [`IvgSymbolDefinition`]銆?

use wincom::{ComObject, Variant};
use windows::Win32::System::Com::IDispatch;

use crate::shape::IvgShapeRange;
use crate::symbol::IvgSymbolDefinition;

pub struct IvgSymbol {
    disp: ComObject,
}

impl IvgSymbol {
    pub fn new(disp: IDispatch) -> Self {
        Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) }
    }

    pub fn from_disp(disp: IDispatch) -> Self {
        Self::new(disp)
    }

    pub fn as_variant(&self) -> Variant {
        self.disp.as_variant()
    }

    pub fn raw(&self) -> &ComObject {
        &self.disp
    }

    fn prop_dispatch(&self, name: &str) -> Option<IDispatch> {
        self.disp.get_property(name).ok()?.to_idispatch().ok()
    }

    /// 鏈疄渚嬪紩鐢ㄧ殑绗﹀彿瀹氫箟銆?
    pub fn definition(&self) -> Option<IvgSymbolDefinition> {
        self.prop_dispatch("Definition").map(IvgSymbolDefinition::new)
    }

    /// 鎶婄鍙峰疄渚嬭繕鍘熸垚涓€缁勫浘褰紙鏂紑涓庡畾涔夌殑閾炬帴锛夈€?
    pub fn revert_to_shapes(&self) -> Option<IvgShapeRange> {
        self.disp
            .invoke_method("RevertToShapes", vec![])
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgShapeRange::new)
    }
}