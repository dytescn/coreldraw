//! `IVGTreeManager` —— 对象树管理器
//!
//! 由 `document.tree_manager()` / `application.active_tree_manager()` 取得。

use wincom::ComObject;
use windows::Win32::System::Com::IDispatch;

use crate::layer::IvgLayer;
use crate::tree::IvgTreeNode;

/// CorelDRAW `IVGTreeManager` 对象的后期绑定包装（对象树管理器）。
///
/// 所有属性与方法通过 `IDispatch::Invoke` 按名称调用，调用失败时一般返回 `None` 或 `false`。
pub struct IvgTreeManager {
    disp: ComObject,
}

impl IvgTreeManager {
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

    fn prop_dispatch(&self, name: &str) -> Option<IDispatch> {
        self.disp.get_property(name).ok()?.to_idispatch().ok()
    }

    // ---------------------------------------------------------
    // 虚拟层
    // ---------------------------------------------------------

    /// 当前虚拟层（Virtual Layer）。
    pub fn virtual_layer(&self) -> Option<IvgLayer> {
        self.prop_dispatch("VirtualLayer").map(IvgLayer::new)
    }

    // ---------------------------------------------------------
    // 选择
    // ---------------------------------------------------------

    /// 当前选中的节点数。
    pub fn selected_node_count(&self) -> Option<i64> { self.prop_i64("SelectedNodeCount") }

    /// 第一个被选中的节点。
    pub fn first_selected_node(&self) -> Option<IvgTreeNode> {
        self.prop_dispatch("FirstSelectedNode").map(IvgTreeNode::new)
    }

    // ---------------------------------------------------------
    // 群组节点操作
    // ---------------------------------------------------------

    /// 创建一个空群组节点（不改变图形层级，仅树结构）。
    pub fn create_group_node(&self) -> Option<IvgTreeNode> {
        self.disp
            .invoke_method("CreateGroupNode", vec![])
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgTreeNode::new)
    }

    /// 清理群组节点（把单层子节点提升到父级）。
    pub fn clean_group_node(&self, group_node: &IvgTreeNode) -> Option<IvgTreeNode> {
        let args = vec![group_node.as_variant()];
        self.disp
            .invoke_method("CleanGroupNode", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgTreeNode::new)
    }
}