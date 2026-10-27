//! `IVGTreeNode` —— 单个树节点
//!
//! 每个页面 / 图层 / 群组 / 图形都在树里对应一个节点。
//! 通过 `shape.tree_node()` / `layer.tree_node()` / `page.tree_node()`
//! 取得。

use wincom::{ComObject, Variant};
use windows::Win32::System::Com::IDispatch;

use crate::document::IvgDocument;
use crate::layer::IvgLayer;
use crate::page::IvgPage;
use crate::shape::IvgShape;
use crate::tree::IvgTreeNodes;

/// CorelDRAW `IVGTreeNode` 对象的后期绑定包装（对象树节点）。
///
/// 所有属性与方法通过 `IDispatch::Invoke` 按名称调用，调用失败时一般返回 `None` 或 `false`。
pub struct IvgTreeNode {
    disp: ComObject,
}

impl IvgTreeNode {
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

    // ---------------------------------------------------------
    // 通用工具
    // ---------------------------------------------------------

    fn prop_i64(&self, name: &str) -> Option<i64> {
        self.disp.get_property(name).ok()?.to_i64().ok()
    }

    fn prop_string(&self, name: &str) -> Option<String> {
        self.disp.get_property(name).ok()?.to_string().ok()
    }

    fn prop_bool(&self, name: &str) -> Option<bool> {
        self.disp.get_property(name).ok()?.to_bool().ok()
    }

    fn prop_dispatch(&self, name: &str) -> Option<IDispatch> {
        self.disp.get_property(name).ok()?.to_idispatch().ok()
    }

    fn put_string(&self, name: &str, v: impl Into<String>) -> bool {
        let arg = Variant::from_str(v.into());
        self.disp.set_property(name, vec![arg]).is_ok()
    }

    // ---------------------------------------------------------
    // 类型 / 关联对象
    // ---------------------------------------------------------

    /// `cdrTreeNodeType`
    pub fn node_type(&self) -> Option<i64> { self.prop_i64("Type") }

    /// `cdrShapeType`
    pub fn shape_type(&self) -> Option<i64> { self.prop_i64("ShapeType") }

    /// 关联的图形（如果该节点代表一个图形）。
    pub fn shape(&self) -> Option<IvgShape> {
        self.prop_dispatch("Shape").map(IvgShape::new)
    }

    /// 虚拟图形（用于群组头等）。
    pub fn virtual_shape(&self) -> Option<IvgShape> {
        self.prop_dispatch("VirtualShape").map(IvgShape::new)
    }

    /// 读取页面（`Page`）。
    pub fn page(&self) -> Option<IvgPage> {
        self.prop_dispatch("Page").map(IvgPage::new)
    }

    /// 读取图层（`Layer`）。
    pub fn layer(&self) -> Option<IvgLayer> {
        self.prop_dispatch("Layer").map(IvgLayer::new)
    }

    /// 读取文档（`Document`）。
    pub fn document(&self) -> Option<IvgDocument> {
        self.prop_dispatch("Document").map(IvgDocument::new)
    }

    // ---------------------------------------------------------
    // 遍历
    // ---------------------------------------------------------

    /// 读取下一个（`Next`）。
    pub fn next(&self) -> Option<IvgTreeNode> {
        self.prop_dispatch("Next").map(IvgTreeNode::new)
    }

    /// 读取上一个（`Previous`）。
    pub fn previous(&self) -> Option<IvgTreeNode> {
        self.prop_dispatch("Previous").map(IvgTreeNode::new)
    }

    /// 读取父对象（`Parent`）。
    pub fn parent(&self) -> Option<IvgTreeNode> {
        self.prop_dispatch("Parent").map(IvgTreeNode::new)
    }

    /// 读取第一个子对象（`FirstChild`）。
    pub fn first_child(&self) -> Option<IvgTreeNode> {
        self.prop_dispatch("FirstChild").map(IvgTreeNode::new)
    }

    /// 读取最后一个子对象（`LastChild`）。
    pub fn last_child(&self) -> Option<IvgTreeNode> {
        self.prop_dispatch("LastChild").map(IvgTreeNode::new)
    }

    /// 读取子对象集（`Children`）。
    pub fn children(&self) -> Option<IvgTreeNodes> {
        self.prop_dispatch("Children").map(IvgTreeNodes::new)
    }

    // ---------------------------------------------------------
    // 状态
    // ---------------------------------------------------------

    /// 是否组子对象（`IsGroupChild`）。
    pub fn is_group_child(&self) -> Option<bool> { self.prop_bool("IsGroupChild") }
    /// 读取已选中（`Selected`）。
    pub fn selected(&self) -> Option<bool> { self.prop_bool("Selected") }

    /// 下一个被选中的节点（用于遍历选择）。
    pub fn next_selected(&self) -> Option<IvgTreeNode> {
        self.prop_dispatch("NextSelected").map(IvgTreeNode::new)
    }

    // ---------------------------------------------------------
    // 名称 / 句柄
    // ---------------------------------------------------------

    /// 读取名称（`Name`）。
    pub fn name(&self) -> Option<String> { self.prop_string("Name") }
    /// 设置名称（`Name`）。
    pub fn set_name(&self, v: impl Into<String>) -> bool { self.put_string("Name", v) }

    /// 读取手柄（`Handle`）。
    pub fn handle(&self) -> Option<i64> { self.prop_i64("Handle") }

    // ---------------------------------------------------------
    // 链接操作
    // ---------------------------------------------------------

    /// 断开本节点与父节点的链接。
    pub fn unlink(&self) -> Option<bool> {
        self.disp.invoke_method("UnLink", vec![]).ok()?.to_bool().ok()
    }

    /// 链接到另一个节点*之前**。
    pub fn link_before(&self, other: &IvgTreeNode) -> Option<bool> {
        let args = vec![other.as_variant()];
        self.disp
            .invoke_method("LinkBefore", args)
            .ok()?
            .to_bool()
            .ok()
    }

    /// 链接到另一个节点*之后**。
    pub fn link_after(&self, other: &IvgTreeNode) -> Option<bool> {
        let args = vec![other.as_variant()];
        self.disp
            .invoke_method("LinkAfter", args)
            .ok()?
            .to_bool()
            .ok()
    }

    /// 作为另一个节点的子节点。
    pub fn link_as_child_of(&self, other: &IvgTreeNode) -> Option<bool> {
        let args = vec![other.as_variant()];
        self.disp
            .invoke_method("LinkAsChildOf", args)
            .ok()?
            .to_bool()
            .ok()
    }

    /// 移到同级的第一个位置。
    pub fn move_to_first(&self) -> Option<bool> {
        self.disp.invoke_method("MoveToFirst", vec![]).ok()?.to_bool().ok()
    }

    /// 移到同级的最后一个位置。
    pub fn move_to_last(&self) -> Option<bool> {
        self.disp.invoke_method("MoveToLast", vec![]).ok()?.to_bool().ok()
    }

    /// 移到另一个节点之前。
    pub fn move_before(&self, other: &IvgTreeNode) -> Option<bool> {
        let args = vec![other.as_variant()];
        self.disp
            .invoke_method("MoveBefore", args)
            .ok()?
            .to_bool()
            .ok()
    }

    /// 移到另一个节点之后。
    pub fn move_after(&self, other: &IvgTreeNode) -> Option<bool> {
        let args = vec![other.as_variant()];
        self.disp
            .invoke_method("MoveAfter", args)
            .ok()?
            .to_bool()
            .ok()
    }

    // ---------------------------------------------------------
    // 判断
    // ---------------------------------------------------------

    /// 本节点是否是另一个节点的后代。
    pub fn is_descendent_of(&self, other: &IvgTreeNode) -> Option<bool> {
        let args = vec![other.as_variant()];
        self.disp
            .invoke_method("IsDescendentOf", args)
            .ok()?
            .to_bool()
            .ok()
    }

    // ---------------------------------------------------------
    // 复制 / 交换 / 删除
    // ---------------------------------------------------------

    /// 读取复制（`GetCopy`）。
    pub fn get_copy(&self) -> Option<IvgTreeNode> {
        self.disp
            .invoke_method("GetCopy", vec![])
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgTreeNode::new)
    }

    /// 交换两个节点的位置（保持节点数据不动，只换位置）。
    pub fn swap_data(&self, other: &IvgTreeNode) -> bool {
        let args = vec![other.as_variant()];
        self.disp.invoke_method("SwapData", args).is_ok()
    }

    /// 交换两个群组节点（含子节点数据）。
    pub fn swap_group_data(&self, other: &IvgTreeNode) -> bool {
        let args = vec![other.as_variant()];
        self.disp.invoke_method("SwapGroupData", args).is_ok()
    }

    /// 删除（COM 方法 `Delete`）。
    pub fn delete(&self) -> bool {
        self.disp.invoke_method("Delete", vec![]).is_ok()
    }
}