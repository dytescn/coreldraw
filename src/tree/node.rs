//! `IVGTreeNode` 鈥斺€?鍗曚釜鏍戣妭鐐?
//!
//! 姣忎釜椤甸潰 / 鍥惧眰 / 缇ょ粍 / 鍥惧舰閮藉湪鏍戦噷瀵瑰簲涓€涓妭鐐广€?
//! 閫氳繃 `shape.tree_node()` / `layer.tree_node()` / `page.tree_node()`
//! 鍙栧緱銆?

use wincom::{ComObject, Variant};
use windows::Win32::System::Com::IDispatch;

use crate::document::IvgDocument;
use crate::layer::IvgLayer;
use crate::page::IvgPage;
use crate::shape::IvgShape;
use crate::tree::IvgTreeNodes;

pub struct IvgTreeNode {
    disp: ComObject,
}

impl IvgTreeNode {
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
    // 閫氱敤宸ュ叿
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
    // 绫诲瀷 / 鍏宠仈瀵硅薄
    // ---------------------------------------------------------

    /// `cdrTreeNodeType`
    pub fn node_type(&self) -> Option<i64> { self.prop_i64("Type") }

    /// `cdrShapeType`
    pub fn shape_type(&self) -> Option<i64> { self.prop_i64("ShapeType") }

    /// 鍏宠仈鐨勫浘褰紙濡傛灉璇ヨ妭鐐逛唬琛ㄤ竴涓浘褰級銆?
    pub fn shape(&self) -> Option<IvgShape> {
        self.prop_dispatch("Shape").map(IvgShape::new)
    }

    /// 铏氭嫙鍥惧舰锛堢敤浜庣兢缁勫ご绛夛級銆?
    pub fn virtual_shape(&self) -> Option<IvgShape> {
        self.prop_dispatch("VirtualShape").map(IvgShape::new)
    }

    pub fn page(&self) -> Option<IvgPage> {
        self.prop_dispatch("Page").map(IvgPage::new)
    }

    pub fn layer(&self) -> Option<IvgLayer> {
        self.prop_dispatch("Layer").map(IvgLayer::new)
    }

    pub fn document(&self) -> Option<IvgDocument> {
        self.prop_dispatch("Document").map(IvgDocument::new)
    }

    // ---------------------------------------------------------
    // 閬嶅巻
    // ---------------------------------------------------------

    pub fn next(&self) -> Option<IvgTreeNode> {
        self.prop_dispatch("Next").map(IvgTreeNode::new)
    }

    pub fn previous(&self) -> Option<IvgTreeNode> {
        self.prop_dispatch("Previous").map(IvgTreeNode::new)
    }

    pub fn parent(&self) -> Option<IvgTreeNode> {
        self.prop_dispatch("Parent").map(IvgTreeNode::new)
    }

    pub fn first_child(&self) -> Option<IvgTreeNode> {
        self.prop_dispatch("FirstChild").map(IvgTreeNode::new)
    }

    pub fn last_child(&self) -> Option<IvgTreeNode> {
        self.prop_dispatch("LastChild").map(IvgTreeNode::new)
    }

    pub fn children(&self) -> Option<IvgTreeNodes> {
        self.prop_dispatch("Children").map(IvgTreeNodes::new)
    }

    // ---------------------------------------------------------
    // 鐘舵€?
    // ---------------------------------------------------------

    pub fn is_group_child(&self) -> Option<bool> { self.prop_bool("IsGroupChild") }
    pub fn selected(&self) -> Option<bool> { self.prop_bool("Selected") }

    /// 涓嬩竴涓閫変腑鐨勮妭鐐癸紙鐢ㄤ簬閬嶅巻閫夋嫨锛夈€?
    pub fn next_selected(&self) -> Option<IvgTreeNode> {
        self.prop_dispatch("NextSelected").map(IvgTreeNode::new)
    }

    // ---------------------------------------------------------
    // 鍚嶇О / 鍙ユ焺
    // ---------------------------------------------------------

    pub fn name(&self) -> Option<String> { self.prop_string("Name") }
    pub fn set_name(&self, v: impl Into<String>) -> bool { self.put_string("Name", v) }

    pub fn handle(&self) -> Option<i64> { self.prop_i64("Handle") }

    // ---------------------------------------------------------
    // 閾炬帴鎿嶄綔
    // ---------------------------------------------------------

    /// 鏂紑鏈妭鐐逛笌鐖惰妭鐐圭殑閾炬帴銆?
    pub fn unlink(&self) -> Option<bool> {
        self.disp.invoke_method("UnLink", vec![]).ok()?.to_bool().ok()
    }

    /// 閾炬帴鍒板彟涓€涓妭鐐?*涔嬪墠**銆?
    pub fn link_before(&self, other: &IvgTreeNode) -> Option<bool> {
        let args = vec![other.as_variant()];
        self.disp
            .invoke_method("LinkBefore", args)
            .ok()?
            .to_bool()
            .ok()
    }

    /// 閾炬帴鍒板彟涓€涓妭鐐?*涔嬪悗**銆?
    pub fn link_after(&self, other: &IvgTreeNode) -> Option<bool> {
        let args = vec![other.as_variant()];
        self.disp
            .invoke_method("LinkAfter", args)
            .ok()?
            .to_bool()
            .ok()
    }

    /// 浣滀负鍙︿竴涓妭鐐圭殑瀛愯妭鐐广€?
    pub fn link_as_child_of(&self, other: &IvgTreeNode) -> Option<bool> {
        let args = vec![other.as_variant()];
        self.disp
            .invoke_method("LinkAsChildOf", args)
            .ok()?
            .to_bool()
            .ok()
    }

    /// 绉诲埌鍚岀骇鐨勭涓€涓綅缃€?
    pub fn move_to_first(&self) -> Option<bool> {
        self.disp.invoke_method("MoveToFirst", vec![]).ok()?.to_bool().ok()
    }

    /// 绉诲埌鍚岀骇鐨勬渶鍚庝竴涓綅缃€?
    pub fn move_to_last(&self) -> Option<bool> {
        self.disp.invoke_method("MoveToLast", vec![]).ok()?.to_bool().ok()
    }

    /// 绉诲埌鍙︿竴涓妭鐐逛箣鍓嶃€?
    pub fn move_before(&self, other: &IvgTreeNode) -> Option<bool> {
        let args = vec![other.as_variant()];
        self.disp
            .invoke_method("MoveBefore", args)
            .ok()?
            .to_bool()
            .ok()
    }

    /// 绉诲埌鍙︿竴涓妭鐐逛箣鍚庛€?
    pub fn move_after(&self, other: &IvgTreeNode) -> Option<bool> {
        let args = vec![other.as_variant()];
        self.disp
            .invoke_method("MoveAfter", args)
            .ok()?
            .to_bool()
            .ok()
    }

    // ---------------------------------------------------------
    // 鍒ゆ柇
    // ---------------------------------------------------------

    /// 鏈妭鐐规槸鍚︽槸鍙︿竴涓妭鐐圭殑鍚庝唬銆?
    pub fn is_descendent_of(&self, other: &IvgTreeNode) -> Option<bool> {
        let args = vec![other.as_variant()];
        self.disp
            .invoke_method("IsDescendentOf", args)
            .ok()?
            .to_bool()
            .ok()
    }

    // ---------------------------------------------------------
    // 澶嶅埗 / 浜ゆ崲 / 鍒犻櫎
    // ---------------------------------------------------------

    pub fn get_copy(&self) -> Option<IvgTreeNode> {
        self.disp
            .invoke_method("GetCopy", vec![])
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgTreeNode::new)
    }

    /// 浜ゆ崲涓や釜鑺傜偣鐨勪綅缃紙淇濇寔鑺傜偣鏁版嵁涓嶅姩锛屽彧鎹綅缃級銆?
    pub fn swap_data(&self, other: &IvgTreeNode) -> bool {
        let args = vec![other.as_variant()];
        self.disp.invoke_method("SwapData", args).is_ok()
    }

    /// 浜ゆ崲涓や釜缇ょ粍鑺傜偣锛堝惈瀛愯妭鐐规暟鎹級銆?
    pub fn swap_group_data(&self, other: &IvgTreeNode) -> bool {
        let args = vec![other.as_variant()];
        self.disp.invoke_method("SwapGroupData", args).is_ok()
    }

    pub fn delete(&self) -> bool {
        self.disp.invoke_method("Delete", vec![]).is_ok()
    }
}