//! `IPrnVBAPrintLayout` —— 打印排版（出血 / 拼版 / 贴标）

use wincom::{ComObject, Variant};
use windows::Win32::System::Com::IDispatch;

/// CorelDRAW `IPrnVBAPrintLayout` 对象的后期绑定包装（排版）。
///
/// 所有属性与方法通过 `IDispatch::Invoke` 按名称调用，调用失败时一般返回 `None` 或 `false`。
pub struct IvgPrnLayout {
    disp: ComObject,
}

impl IvgPrnLayout {
    /// 由 COM `IDispatch` 构造包装对象。
    pub fn new(disp: IDispatch) -> Self {
        Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) }
    }

    fn prop_i64(&self, name: &str) -> Option<i64> {
        self.disp.get_property(name).ok()?.to_i64().ok()
    }

    fn prop_f64(&self, name: &str) -> Option<f64> {
        self.disp.get_property(name).ok()?.to_f64().ok()
    }

    fn prop_bool(&self, name: &str) -> Option<bool> {
        self.disp.get_property(name).ok()?.to_bool().ok()
    }

    fn put_i64(&self, name: &str, v: i64) -> bool {
        let arg = Variant::from_i64(v);
        self.disp.set_property(name, vec![arg]).is_ok()
    }

    fn put_f64(&self, name: &str, v: f64) -> bool {
        let arg = Variant::from_f64(v);
        self.disp.set_property(name, vec![arg]).is_ok()
    }

    fn put_bool(&self, name: &str, v: bool) -> bool {
        let arg = Variant::from_bool(v);
        self.disp.set_property(name, vec![arg]).is_ok()
    }

    // ---- 出血 ----

    /// 读取使用出血限制（`UseBleedLimit`）。
    pub fn use_bleed_limit(&self) -> Option<bool> { self.prop_bool("UseBleedLimit") }
    /// 设置使用出血限制（`UseBleedLimit`）。
    pub fn set_use_bleed_limit(&self, v: bool) -> bool { self.put_bool("UseBleedLimit", v) }

    /// 读取出血限制（`BleedLimit`）。
    pub fn bleed_limit(&self) -> Option<f64> { self.prop_f64("BleedLimit") }
    /// 设置出血限制（`BleedLimit`）。
    pub fn set_bleed_limit(&self, v: f64) -> bool { self.put_f64("BleedLimit", v) }

    // ---- 平铺 ----

    /// 读取 COM 属性 `PrintTiledPages`。
    pub fn print_tiled_pages(&self) -> Option<bool> { self.prop_bool("PrintTiledPages") }
    /// 设置 COM 属性 `PrintTiledPages`。
    pub fn set_print_tiled_pages(&self, v: bool) -> bool {
        self.put_bool("PrintTiledPages", v)
    }

    /// 读取 COM 属性 `PrintTilingMarks`。
    pub fn print_tiling_marks(&self) -> Option<bool> { self.prop_bool("PrintTilingMarks") }
    /// 设置 COM 属性 `PrintTilingMarks`。
    pub fn set_print_tiling_marks(&self, v: bool) -> bool {
        self.put_bool("PrintTilingMarks", v)
    }

    /// 读取平铺重叠（`TileOverlap`）。
    pub fn tile_overlap(&self) -> Option<f64> { self.prop_f64("TileOverlap") }
    /// 设置平铺重叠（`TileOverlap`）。
    pub fn set_tile_overlap(&self, v: f64) -> bool { self.put_f64("TileOverlap", v) }

    // ---- 位置 ----

    /// `PrnPlaceType`
    pub fn placement(&self) -> Option<i64> { self.prop_i64("Placement") }
    /// 设置放置（`Placement`）。
    pub fn set_placement(&self, v: i32) -> bool { self.put_i64("Placement", v as i64) }
}