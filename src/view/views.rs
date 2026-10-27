//! `IVGViews` —— 命名视图集合

use wincom::{ComObject, Variant};
use windows::Win32::System::Com::IDispatch;

use crate::view::IvgView;

/// CorelDRAW `IVGViews` 对象的后期绑定包装（视图集）。
///
/// 所有属性与方法通过 `IDispatch::Invoke` 按名称调用，调用失败时一般返回 `None` 或 `false`。
pub struct IvgViews {
    disp: ComObject,
}

impl IvgViews {
    /// 由 COM `IDispatch` 构造包装对象。
    pub fn new(disp: IDispatch) -> Self {
        Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) }
    }

    /// 从 COM `IDispatch` 构造包装对象，等价于 `new`。
    pub fn from_disp(disp: IDispatch) -> Self {
        Self::new(disp)
    }

    /// 获取底层 `ComObject` 的引用。
    pub fn raw(&self) -> &ComObject {
        &self.disp
    }

    fn prop_i64(&self, name: &str) -> Option<i64> {
        self.disp.get_property(name).ok()?.to_i64().ok()
    }

    /// 读取返回条目数量（`Count`）。
    pub fn count(&self) -> Option<i64> { self.prop_i64("Count") }

    /// 按索引或名称取条目（COM 方法 `Item`）。
    pub fn item(&self, index_or_name: Variant) -> Option<IvgView> {
        let args = vec![index_or_name];
        self.disp
            .invoke_method("Item", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgView::new)
    }

    /// 条目索引。
    pub fn item_by_index(&self, i: i32) -> Option<IvgView> {
        self.item(Variant::from_i64(i as i64))
    }

    /// 条目名称。
    pub fn item_by_name(&self, name: impl Into<String>) -> Option<IvgView> {
        self.item(Variant::from_str(name.into()))
    }

    /// 从当前活动视图创建一个命名视图。
    pub fn add_active_view(&self, name: impl Into<String>) -> Option<IvgView> {
        let args = vec![Variant::from_str(name.into())];
        self.disp
            .invoke_method("AddActiveView", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgView::new)
    }

    /// 返回全部条目。
    pub fn all(&self) -> Vec<IvgView> {
        let n = self.count().unwrap_or(0) as i32;
        (1..=n).filter_map(|i| self.item_by_index(i)).collect()
    }
}