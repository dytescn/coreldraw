//! `IVGOutlineStyle` / `IVGOutlineStyles` —— 轮廓线型

use wincom::{ComObject, Variant};
use windows::Win32::System::Com::IDispatch;

// =============================================================
// IvgOutlineStyle
// =============================================================

/// 单条轮廓线型。
///
/// `DashCount` 决定虚线由多少段实 + 空组成：
/// `DashLength(i)` 是第 i 段划的长度，`GapLength(i)` 是第 i 段空的长度。
pub struct IvgOutlineStyle {
    disp: ComObject,
}

impl IvgOutlineStyle {
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
    // 索引
    // ---------------------------------------------------------

    /// 读取索引（`Index`）。
    pub fn index(&self) -> Option<i64> { self.prop_i64("Index") }

    // ---------------------------------------------------------
    // 虚线结构
    // ---------------------------------------------------------

    /// 虚线由多少段创/ 空组成。
    pub fn dash_count(&self) -> Option<i64> { self.prop_i64("DashCount") }
    /// 设置虚线数量（`DashCount`）。
    pub fn set_dash_count(&self, v: i32) -> bool { self.put_i64("DashCount", v as i64) }

    /// 笑 `index` 段划的长度（1-based）。
    pub fn dash_length(&self, index: i32) -> Option<i64> {
        let args = vec![Variant::from_i64(index as i64)];
        self.disp.invoke_method("DashLength", args).ok()?.to_i64().ok()
    }

    /// 设置虚线长度（`put_DashLength`）。
    pub fn set_dash_length(&self, index: i32, v: i32) -> bool {
        let args = vec![
            Variant::from_i64(index as i64),
            Variant::from_i64(v as i64),
        ];
        self.disp.invoke_method("put_DashLength", args).is_ok()
    }

    /// 笑 `index` 段空的长度（1-based）。
    pub fn gap_length(&self, index: i32) -> Option<i64> {
        let args = vec![Variant::from_i64(index as i64)];
        self.disp.invoke_method("GapLength", args).ok()?.to_i64().ok()
    }

    /// 设置间隙长度（`put_GapLength`）。
    pub fn set_gap_length(&self, index: i32, v: i32) -> bool {
        let args = vec![
            Variant::from_i64(index as i64),
            Variant::from_i64(v as i64),
        ];
        self.disp.invoke_method("put_GapLength", args).is_ok()
    }

    // ---------------------------------------------------------
    // 增强标记
    // ---------------------------------------------------------

    /// 是否使用了增强线型（CorelDRAW 特有的多段虚线）。
    pub fn enhanced(&self) -> Option<bool> { self.prop_bool("Enhanced") }
}

// =============================================================
// IvgOutlineStyles
// =============================================================

/// CorelDRAW `IVGOutlineStyle` 对象的后期绑定包装（轮廓样式集）。
///
/// 所有属性与方法通过 `IDispatch::Invoke` 按名称调用，调用失败时一般返回 `None` 或 `false`。
pub struct IvgOutlineStyles {
    disp: ComObject,
}

impl IvgOutlineStyles {
    /// 由 COM `IDispatch` 构造包装对象。
    pub fn new(disp: IDispatch) -> Self {
        Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) }
    }

    fn prop_i64(&self, name: &str) -> Option<i64> {
        self.disp.get_property(name).ok()?.to_i64().ok()
    }

    /// 读取返回条目数量（`Count`）。
    pub fn count(&self) -> Option<i64> { self.prop_i64("Count") }

    /// 按索引或名称取条目（COM 方法 `Item`）。
    pub fn item(&self, index: i32) -> Option<IvgOutlineStyle> {
        let args = vec![Variant::from_i64(index as i64)];
        self.disp
            .invoke_method("Item", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgOutlineStyle::new)
    }

    /// 新建一个空线型。
    pub fn add(&self) -> Option<IvgOutlineStyle> {
        self.disp
            .invoke_method("Add", vec![])
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgOutlineStyle::new)
    }

    /// 用已有线型新建一个（复制）。
    pub fn add_style(&self, style: &IvgOutlineStyle) -> Option<IvgOutlineStyle> {
        let args = vec![style.as_variant()];
        self.disp
            .invoke_method("AddStyle", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgOutlineStyle::new)
    }

    /// 移除（COM 方法 `Remove`）。
    pub fn remove(&self, index: i32) -> bool {
        let args = vec![Variant::from_i64(index as i64)];
        self.disp.invoke_method("Remove", args).is_ok()
    }

    /// 保存到文件。
    pub fn save(&self) -> bool {
        self.disp.invoke_method("Save", vec![]).is_ok()
    }
}