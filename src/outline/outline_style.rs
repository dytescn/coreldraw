//! `IVGOutlineStyle` / `IVGOutlineStyles` 鈥斺€?杞粨绾垮瀷

use wincom::{ComObject, Variant};
use windows::Win32::System::Com::IDispatch;

// =============================================================
// IvgOutlineStyle
// =============================================================

/// 鍗曟潯杞粨绾垮瀷銆?
///
/// `DashCount` 鍐冲畾铏氱嚎鐢卞灏戞"鍒?+ 绌?缁勬垚锛?
/// `DashLength(i)` 鏄 i 娈靛垝鐨勯暱搴︼紝`GapLength(i)` 鏄 i 娈电┖鐨勯暱搴︺€?
pub struct IvgOutlineStyle {
    disp: ComObject,
}

impl IvgOutlineStyle {
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

    // ---------------------------------------------------------
    // 绱㈠紩
    // ---------------------------------------------------------

    pub fn index(&self) -> Option<i64> { self.prop_i64("Index") }

    // ---------------------------------------------------------
    // 铏氱嚎缁撴瀯
    // ---------------------------------------------------------

    /// 铏氱嚎鐢卞灏戞鍒?/ 绌虹粍鎴愩€?
    pub fn dash_count(&self) -> Option<i64> { self.prop_i64("DashCount") }
    pub fn set_dash_count(&self, v: i32) -> bool { self.put_i64("DashCount", v as i64) }

    /// 绗?`index` 娈靛垝鐨勯暱搴︼紙1-based锛夈€?
    pub fn dash_length(&self, index: i32) -> Option<i64> {
        let args = vec![Variant::from_i64(index as i64)];
        self.disp.invoke_method("DashLength", args).ok()?.to_i64().ok()
    }

    pub fn set_dash_length(&self, index: i32, v: i32) -> bool {
        let args = vec![
            Variant::from_i64(index as i64),
            Variant::from_i64(v as i64),
        ];
        self.disp.invoke_method("put_DashLength", args).is_ok()
    }

    /// 绗?`index` 娈电┖鐨勯暱搴︼紙1-based锛夈€?
    pub fn gap_length(&self, index: i32) -> Option<i64> {
        let args = vec![Variant::from_i64(index as i64)];
        self.disp.invoke_method("GapLength", args).ok()?.to_i64().ok()
    }

    pub fn set_gap_length(&self, index: i32, v: i32) -> bool {
        let args = vec![
            Variant::from_i64(index as i64),
            Variant::from_i64(v as i64),
        ];
        self.disp.invoke_method("put_GapLength", args).is_ok()
    }

    // ---------------------------------------------------------
    // 澧炲己鏍囪
    // ---------------------------------------------------------

    /// 鏄惁浣跨敤浜嗗寮虹嚎鍨嬶紙CorelDRAW 鐗规湁鐨勫娈佃櫄绾匡級銆?
    pub fn enhanced(&self) -> Option<bool> { self.prop_bool("Enhanced") }
}

// =============================================================
// IvgOutlineStyles
// =============================================================

pub struct IvgOutlineStyles {
    disp: ComObject,
}

impl IvgOutlineStyles {
    pub fn new(disp: IDispatch) -> Self {
        Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) }
    }

    fn prop_i64(&self, name: &str) -> Option<i64> {
        self.disp.get_property(name).ok()?.to_i64().ok()
    }

    pub fn count(&self) -> Option<i64> { self.prop_i64("Count") }

    pub fn item(&self, index: i32) -> Option<IvgOutlineStyle> {
        let args = vec![Variant::from_i64(index as i64)];
        self.disp
            .invoke_method("Item", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgOutlineStyle::new)
    }

    /// 鏂板缓涓€涓┖绾垮瀷銆?
    pub fn add(&self) -> Option<IvgOutlineStyle> {
        self.disp
            .invoke_method("Add", vec![])
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgOutlineStyle::new)
    }

    /// 鐢ㄥ凡鏈夌嚎鍨嬫柊寤轰竴涓紙澶嶅埗锛夈€?
    pub fn add_style(&self, style: &IvgOutlineStyle) -> Option<IvgOutlineStyle> {
        let args = vec![style.as_variant()];
        self.disp
            .invoke_method("AddStyle", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgOutlineStyle::new)
    }

    pub fn remove(&self, index: i32) -> bool {
        let args = vec![Variant::from_i64(index as i64)];
        self.disp.invoke_method("Remove", args).is_ok()
    }

    /// 淇濆瓨鍒版枃浠躲€?
    pub fn save(&self) -> bool {
        self.disp.invoke_method("Save", vec![]).is_ok()
    }
}