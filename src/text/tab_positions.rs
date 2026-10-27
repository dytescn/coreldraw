//! `IVGTextTabPosition(s)` —— 制表位

use wincom::{ComObject, Variant};
use windows::Win32::System::Com::IDispatch;

// =============================================================
// IvgTextTabPositions
// =============================================================

/// CorelDRAW 对象的后期绑定包装（文本制表位位置）。
///
/// 所有属性与方法通过 `IDispatch::Invoke` 按名称调用，调用失败时一般返回 `None` 或 `false`。
pub struct IvgTextTabPositions {
    disp: ComObject,
}

impl IvgTextTabPositions {
    /// 由 COM `IDispatch` 构造包装对象。
    pub fn new(disp: IDispatch) -> Self {
        Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) }
    }

    /// 从 COM `IDispatch` 构造包装对象，等价于 `new`。
    pub fn from_disp(disp: IDispatch) -> Self {
        Self::new(disp)
    }

    fn prop_i64(&self, name: &str) -> Option<i64> {
        self.disp.get_property(name).ok()?.to_i64().ok()
    }

    fn prop_string(&self, name: &str) -> Option<String> {
        self.disp.get_property(name).ok()?.to_string().ok()
    }

    fn put_i64(&self, name: &str, v: i64) -> bool {
        let arg = Variant::from_i64(v);
        self.disp.set_property(name, vec![arg]).is_ok()
    }

    fn put_string(&self, name: &str, v: impl Into<String>) -> bool {
        let arg = Variant::from_str(v.into());
        self.disp.set_property(name, vec![arg]).is_ok()
    }

    /// 读取返回条目数量（`Count`）。
    pub fn count(&self) -> Option<i64> { self.prop_i64("Count") }

    /// 按索引或名称取条目（COM 方法 `Item`）。
    pub fn item(&self, index: i32) -> Option<IvgTextTabPosition> {
        let args = vec![Variant::from_i64(index as i64)];
        self.disp
            .invoke_method("Item", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgTextTabPosition::new)
    }

    /// 前导符（leadered）间隔。
    pub fn leader_spacing(&self) -> Option<i64> { self.prop_i64("LeaderSpacing") }
    /// 设置引出线间距（`LeaderSpacing`）。
    pub fn set_leader_spacing(&self, v: i32) -> bool {
        self.put_i64("LeaderSpacing", v as i64)
    }

    /// 读取引出线字符（`LeaderCharacter`）。
    pub fn leader_character(&self) -> Option<String> { self.prop_string("LeaderCharacter") }
    /// 设置引出线字符（`LeaderCharacter`）。
    pub fn set_leader_character(&self, v: impl Into<String>) -> bool {
        self.put_string("LeaderCharacter", v)
    }

    /// 清空全部内容（COM 方法 `Clear`）。
    pub fn clear(&self) -> bool {
        self.disp.invoke_method("Clear", vec![]).is_ok()
    }

    /// 添加制表位。
    pub fn add(
        &self,
        position: f64,
        alignment: i32,
        leadered: bool,
    ) -> Option<IvgTextTabPosition> {
        let args = vec![
            Variant::from_f64(position),
            Variant::from_i64(alignment as i64),
            Variant::from_bool(leadered),
        ];
        self.disp
            .invoke_method("Add", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgTextTabPosition::new)
    }

    /// 等间隔添加。
    pub fn add_every(
        &self,
        position: f64,
        alignment: i32,
        leadered: bool,
    ) -> bool {
        let args = vec![
            Variant::from_f64(position),
            Variant::from_i64(alignment as i64),
            Variant::from_bool(leadered),
        ];
        self.disp.invoke_method("AddEvery", args).is_ok()
    }
}

// =============================================================
// IvgTextTabPosition
// =============================================================

/// CorelDRAW 对象的后期绑定包装（文本制表位位置）。
///
/// 所有属性与方法通过 `IDispatch::Invoke` 按名称调用，调用失败时一般返回 `None` 或 `false`。
pub struct IvgTextTabPosition {
    disp: ComObject,
}

impl IvgTextTabPosition {
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

    /// 读取位置（`Position`）。
    pub fn position(&self) -> Option<f64> { self.prop_f64("Position") }
    /// 设置位置（`Position`）。
    pub fn set_position(&self, v: f64) -> bool { self.put_f64("Position", v) }

    /// 读取索引（`Index`）。
    pub fn index(&self) -> Option<i64> { self.prop_i64("Index") }

    /// `cdrTextTabAlignment`
    pub fn alignment(&self) -> Option<i64> { self.prop_i64("Alignment") }
    /// 设置对齐（`Alignment`）。
    pub fn set_alignment(&self, v: i32) -> bool { self.put_i64("Alignment", v as i64) }

    /// 读取 COM 属性 `Leadered`。
    pub fn leadered(&self) -> Option<bool> { self.prop_bool("Leadered") }
    /// 设置 COM 属性 `Leadered`。
    pub fn set_leadered(&self, v: bool) -> bool { self.put_bool("Leadered", v) }

    /// 删除（COM 方法 `Delete`）。
    pub fn delete(&self) -> bool {
        self.disp.invoke_method("Delete", vec![]).is_ok()
    }
}