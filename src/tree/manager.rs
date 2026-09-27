//! `IVGTreeManager` 鈥斺€?瀵硅薄鏍戠鐞嗗櫒
//!
//! 鐢?`document.tree_manager()` / `application.active_tree_manager()` 鍙栧緱銆?

use wincom::ComObject;
use windows::Win32::System::Com::IDispatch;

use crate::layer::IvgLayer;
use crate::tree::IvgTreeNode;

pub struct IvgTreeManager {
    disp: ComObject,
}

impl IvgTreeManager {
    pub fn new(disp: IDispatch) -> Self {
        Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) }
    }

    pub fn from_disp(disp: IDispatch) -> Self {
        Self::new(disp)
    }

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
    // 铏氭嫙灞?
    // ---------------------------------------------------------

    /// 褰撳墠铏氭嫙灞傦紙Virtual Layer锛夈€?
    pub fn virtual_layer(&self) -> Option<IvgLayer> {
        self.prop_dispatch("VirtualLayer").map(IvgLayer::new)
    }

    // ---------------------------------------------------------
    // 閫夋嫨
    // ---------------------------------------------------------

    /// 褰撳墠閫変腑鐨勮妭鐐规暟銆?
    pub fn selected_node_count(&self) -> Option<i64> { self.prop_i64("SelectedNodeCount") }

    /// 绗竴涓閫変腑鐨勮妭鐐广€?
    pub fn first_selected_node(&self) -> Option<IvgTreeNode> {
        self.prop_dispatch("FirstSelectedNode").map(IvgTreeNode::new)
    }

    // ---------------------------------------------------------
    // 缇ょ粍鑺傜偣鎿嶄綔
    // ---------------------------------------------------------

    /// 鍒涘缓涓€涓┖缇ょ粍鑺傜偣锛堜笉鏀瑰彉鍥惧舰灞傜骇锛屼粎鏍戠粨鏋勶級銆?
    pub fn create_group_node(&self) -> Option<IvgTreeNode> {
        self.disp
            .invoke_method("CreateGroupNode", vec![])
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgTreeNode::new)
    }

    /// 娓呯悊缇ょ粍鑺傜偣锛堟妸鍗曞眰瀛愯妭鐐规彁鍗囧埌鐖剁骇锛夈€?
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