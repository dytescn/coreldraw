//! `IVGLayers` 鈥斺€?鍥惧眰闆嗗悎

use wincom::{ComObject, Variant};
use windows::Win32::System::Com::IDispatch;

use crate::layer::IvgLayer;

pub struct IvgLayers {
    disp: ComObject,
}

impl IvgLayers {
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

    // ---------------------------------------------------------
    // 鍩烘湰淇℃伅
    // ---------------------------------------------------------

    pub fn count(&self) -> Option<i64> {
        self.disp.get_property("Count").ok()?.to_i64().ok()
    }

    pub fn application(&self) -> Option<crate::app::IvgApplication> {
        self.disp
            .get_property("Application")
            .ok()?
            .to_idispatch()
            .ok()
            .map(crate::app::IvgApplication::from_disp)
    }

    // ---------------------------------------------------------
    // 绱㈠紩璁块棶
    // ---------------------------------------------------------

    /// 閫氳繃绱㈠紩锛?-based锛夋垨鍚嶇О璁块棶銆?
    pub fn item(&self, index_or_name: Variant) -> Option<IvgLayer> {
        let args = vec![index_or_name];
        self.disp
            .invoke_method("Item", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgLayer::new)
    }

    pub fn item_by_index(&self, i: i32) -> Option<IvgLayer> {
        self.item(Variant::from_i64(i as i64))
    }

    pub fn item_by_name(&self, name: impl Into<String>) -> Option<IvgLayer> {
        self.item(Variant::from_str(name.into()))
    }

    /// 鐩存帴鎸夊悕绉版煡鎵撅紙涓?`item_by_name` 绛夋晥锛屼絾璧?COM 鐨?`Find`锛夈€?
    pub fn find(&self, layer_name: impl Into<String>) -> Option<IvgLayer> {
        let args = vec![Variant::from_str(layer_name.into())];
        self.disp
            .invoke_method("Find", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgLayer::new)
    }

    /// 鏈€椤跺眰鍥惧眰銆?
    pub fn top(&self) -> Option<IvgLayer> {
        self.disp
            .get_property("Top")
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgLayer::new)
    }

    /// 鏈€搴曞眰鍥惧眰銆?
    pub fn bottom(&self) -> Option<IvgLayer> {
        self.disp
            .get_property("Bottom")
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgLayer::new)
    }

    // ---------------------------------------------------------
    // 閬嶅巻杈呭姪
    // ---------------------------------------------------------

    /// 杩斿洖鎵€鏈夊浘灞傦紙鎸夌储寮?1..Count 椤哄簭锛夈€?
    pub fn all(&self) -> Vec<IvgLayer> {
        let n = self.count().unwrap_or(0) as i32;
        (1..=n).filter_map(|i| self.item_by_index(i)).collect()
    }
}