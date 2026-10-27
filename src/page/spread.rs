//! `IVGSpread` / `IVGSpreads` —— 跨页

use wincom::{ComObject, Variant};
use windows::Win32::System::Com::IDispatch;

use crate::geometry::IvgRect;
use crate::layer::{IvgLayer, IvgLayers};
use crate::page::{IvgPage, IvgPages};
use crate::shape::IvgShapes;
use crate::tree::IvgTreeNode;

// =============================================================
// IvgSpreads
// =============================================================

/// CorelDRAW `IVGSpread` 对象的后期绑定包装（跨页集）。
///
/// 所有属性与方法通过 `IDispatch::Invoke` 按名称调用，调用失败时一般返回 `None` 或 `false`。
pub struct IvgSpreads {
    disp: ComObject,
}

impl IvgSpreads {
    /// 由 COM `IDispatch` 构造包装对象。
    pub fn new(disp: IDispatch) -> Self {
        Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) }
    }

    /// 从 COM `IDispatch` 构造包装对象，等价于 `new`。
    pub fn from_disp(disp: IDispatch) -> Self {
        Self::new(disp)
    }

    /// 读取返回条目数量（`Count`）。
    pub fn count(&self) -> Option<i64> {
        self.disp.get_property("Count").ok()?.to_i64().ok()
    }

    /// 按索引或名称取条目（COM 方法 `Item`）。
    pub fn item(&self, index: i32) -> Option<IvgSpread> {
        let args = vec![Variant::from_i64(index as i64)];
        self.disp
            .invoke_method("Item", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgSpread::new)
    }

    /// 读取返回第一个条目（`First`）。
    pub fn first(&self) -> Option<IvgSpread> {
        self.disp
            .get_property("First")
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgSpread::new)
    }

    /// 读取返回最后一个条目（`Last`）。
    pub fn last(&self) -> Option<IvgSpread> {
        self.disp
            .get_property("Last")
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgSpread::new)
    }

    /// 返回全部条目。
    pub fn all(&self) -> Vec<IvgSpread> {
        let n = self.count().unwrap_or(0) as i32;
        (1..=n).filter_map(|i| self.item(i)).collect()
    }
}

// =============================================================
// IvgSpread
// =============================================================

/// CorelDRAW `IVGSpread` 对象的后期绑定包装（跨页）。
///
/// 所有属性与方法通过 `IDispatch::Invoke` 按名称调用，调用失败时一般返回 `None` 或 `false`。
pub struct IvgSpread {
    disp: ComObject,
}

impl IvgSpread {
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

    fn prop_i64(&self, name: &str) -> Option<i64> {
        self.disp.get_property(name).ok()?.to_i64().ok()
    }

    fn prop_f64(&self, name: &str) -> Option<f64> {
        self.disp.get_property(name).ok()?.to_f64().ok()
    }

    fn prop_dispatch(&self, name: &str) -> Option<IDispatch> {
        self.disp.get_property(name).ok()?.to_idispatch().ok()
    }

    // ---------------------------------------------------------
    // 基本信息
    // ---------------------------------------------------------

    /// 读取索引（`Index`）。
    pub fn index(&self) -> Option<i64> { self.prop_i64("Index") }

    /// 读取尺寸宽度（`SizeWidth`）。
    pub fn size_width(&self) -> Option<f64> { self.prop_f64("SizeWidth") }
    /// 读取尺寸高度（`SizeHeight`）。
    pub fn size_height(&self) -> Option<f64> { self.prop_f64("SizeHeight") }

    /// 读取左 X 坐标（`LeftX`）。
    pub fn left_x(&self) -> Option<f64> { self.prop_f64("LeftX") }
    /// 读取右 X 坐标（`RightX`）。
    pub fn right_x(&self) -> Option<f64> { self.prop_f64("RightX") }
    /// 读取顶部 Y 坐标（`TopY`）。
    pub fn top_y(&self) -> Option<f64> { self.prop_f64("TopY") }
    /// 读取底部 Y 坐标（`BottomY`）。
    pub fn bottom_y(&self) -> Option<f64> { self.prop_f64("BottomY") }
    /// 读取中心 X 坐标（`CenterX`）。
    pub fn center_x(&self) -> Option<f64> { self.prop_f64("CenterX") }
    /// 读取中心 Y 坐标（`CenterY`）。
    pub fn center_y(&self) -> Option<f64> { self.prop_f64("CenterY") }

    /// 读取边界框（`BoundingBox`）。
    pub fn bounding_box(&self) -> Option<IvgRect> {
        self.prop_dispatch("BoundingBox").map(IvgRect::new)
    }

    /// 读取边界框。
    pub fn get_bounding_box(&self) -> Option<(f64, f64, f64, f64)> {
        Some((
            self.left_x()?,
            self.top_y()?,
            self.size_width()?,
            self.size_height()?,
        ))
    }

    /// 读取下一个（`Next`）。
    pub fn next(&self) -> Option<IvgSpread> {
        self.prop_dispatch("Next").map(IvgSpread::new)
    }

    /// 读取上一个（`Previous`）。
    pub fn previous(&self) -> Option<IvgSpread> {
        self.prop_dispatch("Previous").map(IvgSpread::new)
    }

    // ---------------------------------------------------------
    // 页面
    // ---------------------------------------------------------

    /// 读取页面集（`Pages`）。
    pub fn pages(&self) -> Option<IvgPages> {
        self.prop_dispatch("Pages").map(IvgPages::new)
    }

    // ---------------------------------------------------------
    // 图层
    // ---------------------------------------------------------

    /// 读取图层集（`Layers`）。
    pub fn layers(&self) -> Option<IvgLayers> {
        self.prop_dispatch("Layers").map(IvgLayers::new)
    }

    /// 读取全部图层集（`AllLayers`）。
    pub fn all_layers(&self) -> Option<IvgLayers> {
        self.prop_dispatch("AllLayers").map(IvgLayers::new)
    }

    /// 读取活动图层（`ActiveLayer`）。
    pub fn active_layer(&self) -> Option<IvgLayer> {
        self.prop_dispatch("ActiveLayer").map(IvgLayer::new)
    }

    /// 读取参考线集图层（`GuidesLayer`）。
    pub fn guides_layer(&self) -> Option<IvgLayer> {
        self.prop_dispatch("GuidesLayer").map(IvgLayer::new)
    }

    /// 读取桌面图层（`DesktopLayer`）。
    pub fn desktop_layer(&self) -> Option<IvgLayer> {
        self.prop_dispatch("DesktopLayer").map(IvgLayer::new)
    }

    /// 读取网格图层（`GridLayer`）。
    pub fn grid_layer(&self) -> Option<IvgLayer> {
        self.prop_dispatch("GridLayer").map(IvgLayer::new)
    }

    /// 创建图层（COM 方法 `CreateLayer`）。
    pub fn create_layer(&self, layer_name: impl Into<String>) -> Option<IvgLayer> {
        let args = vec![Variant::from_str(layer_name.into())];
        self.disp
            .invoke_method("CreateLayer", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgLayer::new)
    }

    // ---------------------------------------------------------
    // 图形
    // ---------------------------------------------------------

    /// 读取图形集（`Shapes`）。
    pub fn shapes(&self) -> Option<IvgShapes> {
        self.prop_dispatch("Shapes").map(IvgShapes::new)
    }

    /// 读取可选择图形集（`SelectableShapes`）。
    pub fn selectable_shapes(&self) -> Option<IvgShapes> {
        self.prop_dispatch("SelectableShapes").map(IvgShapes::new)
    }

    // ---------------------------------------------------------
    // 辅助级/ 样
    // ---------------------------------------------------------

    /// `cdrGuideType`
    pub fn guides(&self, guide_type: i32) -> Option<crate::shape::IvgShapeRange> {
        let args = vec![Variant::from_i64(guide_type as i64)];
        self.disp
            .invoke_method("Guides", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(crate::shape::IvgShapeRange::new)
    }

    /// 读取对象树节点（`TreeNode`）。
    pub fn tree_node(&self) -> Option<IvgTreeNode> {
        self.prop_dispatch("TreeNode").map(IvgTreeNode::new)
    }

    // 供外部便利
    /// 第一个页面。
    pub fn first_page(&self) -> Option<IvgPage> {
        self.pages()?.first()
    }

    /// 最后一个页面。
    pub fn last_page(&self) -> Option<IvgPage> {
        self.pages()?.last()
    }
}