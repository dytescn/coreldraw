//! 对象树（Object Manager Tree）
//!
//! - [`IvgTreeNode`]    : 单个树节点（`IVGTreeNode`）
//! - [`IvgTreeNodes`]   : 子节点集合（`IVGTreeNodes`）
//! - [`IvgTreeManager`] : 树管理器（`IVGTreeManager`），
//!   由 `document.tree_manager()` 取得

pub mod node;
pub mod nodes;
pub mod manager;

pub use node::IvgTreeNode;
pub use nodes::IvgTreeNodes;
pub use manager::IvgTreeManager;