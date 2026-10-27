//! `IVGTextFrame` / `IVGTextFrames` —— 文本框

use wincom::{ComObject, Variant};
use windows::Win32::System::Com::IDispatch;

use crate::fill::IvgFill;
use crate::outline::IvgOutline;
use crate::shape::IvgShape;
use crate::text::IvgTextRange;

// =============================================================
// IvgTextFrames
// =============================================================

/// CorelDRAW `IVGTextFrame` 对象的后期绑定包装（文本文本框集）。
///
/// 所有属性与方法通过 `IDispatch::Invoke` 按名称调用，调用失败时一般返回 `None` 或 `false`。
pub struct IvgTextFrames {
    disp: ComObject,
}

impl IvgTextFrames {
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

    fn prop_dispatch(&self, name: &str) -> Option<IDispatch> {
        self.disp.get_property(name).ok()?.to_idispatch().ok()
    }

    /// 读取返回条目数量（`Count`）。
    pub fn count(&self) -> Option<i64> { self.prop_i64("Count") }

    /// 按索引或名称取条目（COM 方法 `Item`）。
    pub fn item(&self, index: i32) -> Option<IvgTextFrame> {
        let args = vec![Variant::from_i64(index as i64)];
        self.disp
            .invoke_method("Item", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgTextFrame::new)
    }

    /// 读取返回第一个条目（`First`）。
    pub fn first(&self) -> Option<IvgTextFrame> {
        self.prop_dispatch("First").map(IvgTextFrame::new)
    }

    /// 读取返回最后一个条目（`Last`）。
    pub fn last(&self) -> Option<IvgTextFrame> {
        self.prop_dispatch("Last").map(IvgTextFrame::new)
    }

    /// 按索引范围取文本范围。
    pub fn range(&self, index: i32, count: i32) -> Option<IvgTextRange> {
        let args = vec![
            Variant::from_i64(index as i64),
            Variant::from_i64(count as i64),
        ];
        self.disp
            .invoke_method("Range", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgTextRange::new)
    }

    /// 所有框的文本范围。
    pub fn all(&self) -> Option<IvgTextRange> {
        self.prop_dispatch("All").map(IvgTextRange::new)
    }
}

// =============================================================
// IvgTextFrame
// =============================================================

/// CorelDRAW `IVGTextFrame` 对象的后期绑定包装（文本文本框）。
///
/// 所有属性与方法通过 `IDispatch::Invoke` 按名称调用，调用失败时一般返回 `None` 或 `false`。
pub struct IvgTextFrame {
    disp: ComObject,
}

impl IvgTextFrame {
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

    // fn prop_f64(&self, name: &str) -> Option<f64> {
    //     self.disp.get_property(name).ok()?.to_f64().ok()
    // }

    fn prop_bool(&self, name: &str) -> Option<bool> {
        self.disp.get_property(name).ok()?.to_bool().ok()
    }

    fn prop_dispatch(&self, name: &str) -> Option<IDispatch> {
        self.disp.get_property(name).ok()?.to_idispatch().ok()
    }

    fn put_i64(&self, name: &str, v: i64) -> bool {
        let arg = Variant::from_i64(v);
        self.disp.set_property(name, vec![arg]).is_ok()
    }

    fn put_dispatch(&self, name: &str, v: Variant) -> bool {
        self.disp.set_property(name, vec![v]).is_ok()
    }

    // ---- 文本范围 ----

    /// 读取范围（`Range`）。
    pub fn range(&self) -> Option<IvgTextRange> {
        self.prop_dispatch("Range").map(IvgTextRange::new)
    }

    // ---- 上下桥----

    /// 读取上一个（`Previous`）。
    pub fn previous(&self) -> Option<IvgTextFrame> {
        self.prop_dispatch("Previous").map(IvgTextFrame::new)
    }

    /// 读取下一个（`Next`）。
    pub fn next(&self) -> Option<IvgTextFrame> {
        self.prop_dispatch("Next").map(IvgTextFrame::new)
    }

    // ---- 状态----

    /// 读取索引（`Index`）。
    pub fn index(&self) -> Option<i64> { self.prop_i64("Index") }
    /// 是否空（`Empty`）。
    pub fn is_empty(&self) -> Option<bool> { self.prop_bool("Empty") }
    /// 是否第一个（`IsFirst`）。
    pub fn is_first(&self) -> Option<bool> { self.prop_bool("IsFirst") }
    /// 是否最后一个（`IsLast`）。
    pub fn is_last(&self) -> Option<bool> { self.prop_bool("IsLast") }
    /// 是否内部容器（`IsInsideContainer`）。
    pub fn is_inside_container(&self) -> Option<bool> {
        self.prop_bool("IsInsideContainer")
    }
    /// 是否已适配路径（`IsFittedToPath`）。
    pub fn is_fitted_to_path(&self) -> Option<bool> { self.prop_bool("IsFittedToPath") }

    // ---- 垂直对齐 ----

    /// `cdrVerticalAlignment`
    pub fn vertical_alignment(&self) -> Option<i64> { self.prop_i64("VerticalAlignment") }
    /// 设置垂直对齐（`VerticalAlignment`）。
    pub fn set_vertical_alignment(&self, v: i32) -> bool {
        self.put_i64("VerticalAlignment", v as i64)
    }

    // ---- 分栏 ----

    /// 读取 COM 属性 `Multicolumn`。
    pub fn multicolumn(&self) -> Option<bool> { self.prop_bool("Multicolumn") }
    /// 读取分栏数量（`ColumnCount`）。
    pub fn column_count(&self) -> Option<i64> { self.prop_i64("ColumnCount") }

    /// 分栏宽度（COM 方法 `ColumnWidth`）。
    pub fn column_width(&self, index: i32) -> Option<f64> {
        let args = vec![Variant::from_i64(index as i64)];
        self.disp.invoke_method("ColumnWidth", args).ok()?.to_f64().ok()
    }

    /// 调用 COM 方法 `ColumnGutter`。
    pub fn column_gutter(&self, index: i32) -> Option<f64> {
        let args = vec![Variant::from_i64(index as i64)];
        self.disp.invoke_method("ColumnGutter", args).ok()?.to_f64().ok()
    }

    /// `WidthsAndGutters` 是 `SAFEARRAY`。
    pub fn set_columns(
        &self,
        num_columns: i32,
        equal_columns: bool,
        widths_and_gutters: Variant,
    ) -> bool {
        let args = vec![
            Variant::from_i64(num_columns as i64),
            Variant::from_bool(equal_columns),
            widths_and_gutters,
        ];
        self.disp.invoke_method("SetColumns", args).is_ok()
    }

    // ---- 链接 ----

    /// 把本框链接到另一个图形。
    pub fn link_to(&self, shape: &IvgShape) -> bool {
        let args = vec![shape.as_variant()];
        self.disp.invoke_method("LinkTo", args).is_ok()
    }

    /// 断开链接。
    pub fn unlink(&self) -> bool {
        self.disp.invoke_method("UnLink", vec![]).is_ok()
    }

    // ---- 容器 / 路径 ----

    /// 读取容器（`Container`）。
    pub fn container(&self) -> Option<IvgShape> {
        self.prop_dispatch("Container").map(IvgShape::new)
    }

    /// 读取路径（`Path`）。
    pub fn path(&self) -> Option<IvgShape> {
        self.prop_dispatch("Path").map(IvgShape::new)
    }

    /// 读取文本框图形（`FrameShape`）。
    pub fn frame_shape(&self) -> Option<IvgShape> {
        self.prop_dispatch("FrameShape").map(IvgShape::new)
    }

    // ---- 填充 / 轮廓 ----

    /// 读取填充（`Fill`）。
    pub fn fill(&self) -> Option<IvgFill> {
        self.prop_dispatch("Fill").map(IvgFill::new)
    }
    /// 设置填充（`Fill`）。
    pub fn set_fill(&self, f: &IvgFill) -> bool {
        self.put_dispatch("Fill", f.as_variant())
    }

    /// 读取轮廓（`Outline`）。
    pub fn outline(&self) -> Option<IvgOutline> {
        self.prop_dispatch("Outline").map(IvgOutline::new)
    }
    /// 设置轮廓（`Outline`）。
    pub fn set_outline(&self, o: &IvgOutline) -> bool {
        self.put_dispatch("Outline", o.as_variant())
    }
}