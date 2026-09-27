//! `IvgDocument` —— 文档对象
//!
//! CorelDRAW 类型库中 `IVGDocument` 是**空接口**，所有属性/方法都
//! 通过 `IDispatch` 后期绑定访问。本文件按 CorelDRAW VBA 文档
//! 与常见用法整理。

use wincom::{ComObject, Variant};
use windows::Win32::System::Com::IDispatch;

use crate::curve::IvgCurve;
use crate::document::IvgMetadata;
use crate::layer::{IvgLayer, IvgLayers};
use crate::page::{IvgPage, IvgPages, IvgSpread, IvgSpreads};
use crate::shape::{IvgShape, IvgShapeRange, IvgShapes};
use crate::structs::{IvgStructExportOptions, IvgStructSaveAsOptions};

// =============================================================
// IvgDocument
// =============================================================

pub struct IvgDocument {
    disp: ComObject,
}

impl IvgDocument {
    pub fn new(disp: IDispatch) -> Self {
        Self {
            disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH),
        }
    }

    pub fn from_dispatch(disp: IDispatch) -> Self {
        Self::new(disp)
    }

    pub fn from_disp(disp: IDispatch) -> Self {
        Self::new(disp)
    }

    pub fn raw(&self) -> &ComObject {
        &self.disp
    }

    pub fn as_variant(&self) -> Variant {
        self.disp.as_variant()
    }

    // ---------------------------------------------------------
    // 通用工具
    // ---------------------------------------------------------

    fn prop_string(&self, name: &str) -> Option<String> {
        self.disp.get_property(name).ok()?.to_string().ok()
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

    fn prop_dispatch(&self, name: &str) -> Option<IDispatch> {
        self.disp.get_property(name).ok()?.to_idispatch().ok()
    }

    fn put_i64(&self, name: &str, v: i64) -> bool {
        self.disp
            .set_property(name, vec![Variant::from_i64(v)])
            .is_ok()
    }

    fn put_f64(&self, name: &str, v: f64) -> bool {
        self.disp
            .set_property(name, vec![Variant::from_f64(v)])
            .is_ok()
    }

    fn put_bool(&self, name: &str, v: bool) -> bool {
        self.disp
            .set_property(name, vec![Variant::from_bool(v)])
            .is_ok()
    }

    // ---------------------------------------------------------
    // 基本信息
    // ---------------------------------------------------------

    /// 文档名（不含路径）。
    pub fn name(&self) -> Option<String> {
        self.prop_string("Name")
    }

    /// 完整文件路径（含文件名）。
    pub fn file_name(&self) -> Option<String> {
        self.prop_string("FileName")
    }

    /// 完整文件路径（含文件名）。
    pub fn full_file_name(&self) -> Option<String> {
        self.prop_string("FullFileName")
    }

    /// 文件路径（不含文件名）。
    pub fn file_path(&self) -> Option<String> {
        self.prop_string("FilePath")
    }

    /// 文档是否已被修改（未保存）。
    pub fn dirty(&self) -> Option<bool> {
        self.prop_bool("Dirty")
    }

    /// 设置 Dirty 标志。
    ///
    /// ⚠️ `Dirty` 在 RIDL 里是**只读**属性；`Application` 层可能支持写，
    /// 失败会返回 `false`。需要"清空未保存标记"时用 [`reset_dirty`]。
    pub fn set_dirty(&self, v: bool) -> bool {
        self.put_bool("Dirty", v)
    }

    /// 清空未保存标记（把 `Dirty` 设为 `false`）。
    pub fn reset_dirty(&self) -> bool {
        self.put_bool("Dirty", false)
    }

    /// 是否只读。
    pub fn read_only(&self) -> Option<bool> {
        self.prop_bool("ReadOnly")
    }

    /// 是否已保存过。
    pub fn saved(&self) -> Option<bool> {
        self.prop_bool("Saved")
    }

    /// 文档版本号（保存次数）。
    pub fn version(&self) -> Option<i64> {
        self.prop_i64("Version")
    }

    /// 文档世界坐标系的单位（`cdrUnit`）。
    pub fn unit(&self) -> Option<i64> {
        self.prop_i64("Unit")
    }

    pub fn set_unit(&self, unit: i32) -> bool {
        self.put_i64("Unit", unit as i64)
    }

    /// 文档参考点（`cdrReferencePoint`）。
    pub fn reference_point(&self) -> Option<i64> {
        self.prop_i64("ReferencePoint")
    }

    pub fn set_reference_point(&self, v: i32) -> bool {
        self.put_i64("ReferencePoint", v as i64)
    }

    /// 水平标尺原点（世界坐标）。
    pub fn world_scale(&self) -> Option<f64> {
        self.prop_f64("WorldScale")
    }

    pub fn set_world_scale(&self, v: f64) -> bool {
        self.put_f64("WorldScale", v)
    }

    // ---------------------------------------------------------
    // 上层对象
    // ---------------------------------------------------------

    pub fn application(&self) -> Option<crate::app::IvgApplication> {
        self.prop_dispatch("Application")
            .map(crate::app::IvgApplication::from_dispatch)
    }

    // ---------------------------------------------------------
    // 页面
    // ---------------------------------------------------------

    pub fn pages(&self) -> Option<IvgPages> {
        self.prop_dispatch("Pages").map(IvgPages::new)
    }

    pub fn active_page(&self) -> Option<IvgPage> {
        self.prop_dispatch("ActivePage").map(IvgPage::new)
    }

    pub fn spreads(&self) -> Option<IvgSpreads> {
        self.prop_dispatch("Spreads").map(IvgSpreads::new)
    }

    pub fn active_spread(&self) -> Option<IvgSpread> {
        self.prop_dispatch("ActiveSpread").map(IvgSpread::new)
    }

    /// 新建页面。
    pub fn create_page(&self) -> Option<IvgPage> {
        self.disp
            .invoke_method("CreatePage", vec![])
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgPage::new)
    }

    // ---------------------------------------------------------
    // 图层
    //
    // ⚠️ CorelDRAW 的 `Document` **没有** `Layers` 属性——
    //    `Layers` 挂在 `Page` 上。用 `active_page().layers()`
    //    或下面的便捷方法 `active_page_layers()`。
    // ---------------------------------------------------------

    /// 当前激活图层。
    pub fn active_layer(&self) -> Option<IvgLayer> {
        self.prop_dispatch("ActiveLayer").map(IvgLayer::new)
    }

    /// 当前虚拟层（Master Layer）。
    pub fn active_virtual_layer(&self) -> Option<IvgLayer> {
        self.prop_dispatch("ActiveVirtualLayer").map(IvgLayer::new)
    }

    /// 当前页的所有图层。
    ///
    /// 等价于 `doc.active_page()?.layers()`。
    pub fn active_page_layers(&self) -> Option<IvgLayers> {
        self.active_page()?.layers()
    }

    /// 在当前页新建图层。
    ///
    /// 等价于 `doc.active_page()?.create_layer(name)`。
    pub fn create_layer(&self, name: impl Into<String>) -> Option<IvgLayer> {
        self.active_page()?.create_layer(name)
    }

    // ---------------------------------------------------------
    // 图形
    // ---------------------------------------------------------

    /// 当前页面所有图形（集合）。
    pub fn shapes(&self) -> Option<IvgShapes> {
        self.prop_dispatch("Shapes").map(IvgShapes::new)
    }

    /// 当前选中的图形范围。
    pub fn selection_range(&self) -> Option<IvgShapeRange> {
        self.prop_dispatch("SelectionRange").map(IvgShapeRange::new)
    }

    /// 当前活动图形。
    pub fn active_shape(&self) -> Option<IvgShape> {
        self.prop_dispatch("ActiveShape").map(IvgShape::new)
    }

    /// 清空选择。
    pub fn clear_selection(&self) -> bool {
        self.disp.invoke_method("ClearSelection", vec![]).is_ok()
    }

    /// 通过名称/类型查找图形。
    pub fn find_shape(
        &self,
        name: impl Into<String>,
        shape_type: i32,
        static_id: i32,
        recursive: bool,
    ) -> Option<IvgShape> {
        let args = vec![
            Variant::from_str(&name.into()),
            Variant::from_i64(shape_type as i64),
            Variant::from_i64(static_id as i64),
            Variant::from_bool(recursive),
        ];
        self.disp
            .invoke_method("FindShape", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgShape::new)
    }

    /// 通过名称/类型查找多个图形。
    pub fn find_shapes(
        &self,
        name: impl Into<String>,
        shape_type: i32,
        recursive: bool,
    ) -> Option<IvgShapeRange> {
        let args = vec![
            Variant::from_str(&name.into()),
            Variant::from_i64(shape_type as i64),
            Variant::from_bool(recursive),
        ];
        self.disp
            .invoke_method("FindShapes", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgShapeRange::new)
    }

    // ---------------------------------------------------------
    // 曲线工厂
    // ---------------------------------------------------------

    pub fn create_curve(&self) -> Option<IvgCurve> {
        let args = vec![self.as_variant()];
        self.disp
            .invoke_method("CreateCurve", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgCurve::new)
    }

    // ---------------------------------------------------------
    // 单位转换
    // ---------------------------------------------------------

    /// 单位换算。
    ///
    /// ⚠️ 不走 COM——`Application.ConvertUnits` 通过 `IDispatch` 拿到
    /// 的结果不对（COM 方法有出参）。改用本地纯数学实现
    /// [`crate::geometry::units::convert_by_i32`]。
    pub fn convert_units(&self, value: f64, from: i32, to: i32) -> Option<f64> {
        crate::geometry::units::convert_by_i32(value, from, to)
    }

    // ---------------------------------------------------------
    // 数据字段
    // ---------------------------------------------------------

    pub fn data_fields(&self) -> Option<crate::document::IvgDataFields> {
        self.prop_dispatch("DataFields")
            .map(crate::document::IvgDataFields::new)
    }

    // ---------------------------------------------------------
    // 元数据
    // ---------------------------------------------------------

    pub fn metadata(&self) -> Option<IvgMetadata> {
        self.prop_dispatch("Metadata").map(IvgMetadata::new)
    }

    // ---------------------------------------------------------
    // 保存 / 另存为 / 关闭
    // ---------------------------------------------------------

    /// 保存文档。
    ///
    /// ⚠️ 若从未保存过，会弹出"另存为"对话框——**自动化场景应避免**。
    /// 未命名文档请改用 [`save_as`](Self::save_as) / [`save_as_ex`](Self::save_as_ex)。
    pub fn save(&self) -> bool {
        self.disp.invoke_method("Save", vec![]).is_ok()
    }

    /// 另存为（用默认选项）。
    pub fn save_as(&self, file_name: impl Into<String>) -> bool {
        let args = vec![Variant::from_str(&file_name.into())];
        self.disp.invoke_method("SaveAs", args).is_ok()
    }

    /// 另存为（带 `IVGStructSaveAsOptions`）。
    pub fn save_as_ex(
        &self,
        file_name: impl Into<String>,
        options: &IvgStructSaveAsOptions,
    ) -> bool {
        let args = vec![
            Variant::from_str(&file_name.into()),
            options.as_variant(),
        ];
        self.disp.invoke_method("SaveAsEx", args).is_ok()
    }

    /// 关闭文档。
    ///
    /// ⚠️ 若有未保存修改会弹对话框——自动化场景请先用 [`close_without_saving`](Self::close_without_saving)。
    pub fn close(&self) -> bool {
        self.disp.invoke_method("Close", vec![]).is_ok()
    }

    /// 关闭文档并丢弃修改。
    ///
    /// `cdrSaveOptions`: 0 = 保存, 1 = 不保存, 2 = 取消。
    pub fn close_without_saving(&self) -> bool {
        let _ = self.reset_dirty();
        self.disp.invoke_method("Close", vec![]).is_ok()
    }

    // ---------------------------------------------------------
    // 导出 / 打印
    // ---------------------------------------------------------

    /// 导出为指定格式。
    pub fn export(&self, file_name: impl Into<String>, filter: i32) -> bool {
        let args = vec![
            Variant::from_str(&file_name.into()),
            Variant::from_i64(filter as i64),
        ];
        self.disp.invoke_method("Export", args).is_ok()
    }

    /// 导出（带 `IVGStructExportOptions`）。
    pub fn export_ex(
        &self,
        file_name: impl Into<String>,
        filter: i32,
        options: &IvgStructExportOptions,
    ) -> bool {
        let args = vec![
            Variant::from_str(&file_name.into()),
            Variant::from_i64(filter as i64),
            options.as_variant(),
        ];
        self.disp.invoke_method("ExportEx", args).is_ok()
    }

    /// 打印。
    pub fn print_out(&self) -> bool {
        self.disp.invoke_method("PrintOut", vec![]).is_ok()
    }

    /// 取打印任务对象（进一步设置参数）。
    pub fn print_settings(&self) -> Option<crate::print::IvgPrnJob> {
        self.disp
            .invoke_method("PrintSettings", vec![])
            .ok()?
            .to_idispatch()
            .ok()
            .map(crate::print::IvgPrnJob::new)
    }

    // ---------------------------------------------------------
    // PDF 设置
    // ---------------------------------------------------------

    pub fn pdf_settings(&self) -> Option<crate::print::IvgPdfVbaSettings> {
        self.disp
            .invoke_method("PDFSettings", vec![])
            .ok()?
            .to_idispatch()
            .ok()
            .map(crate::print::IvgPdfVbaSettings::new)
    }

    // ---------------------------------------------------------
    // 撤销 / 重做
    // ---------------------------------------------------------

    /// 开始一个可撤销的命名事务。
    pub fn begin_command_group(&self, name: impl Into<String>) -> bool {
        let args = vec![Variant::from_str(&name.into())];
        self.disp.invoke_method("BeginCommandGroup", args).is_ok()
    }

    /// 结束命名事务。
    pub fn end_command_group(&self) -> bool {
        self.disp.invoke_method("EndCommandGroup", vec![]).is_ok()
    }

    /// 清空撤销栈。
    pub fn clear_undo_stack(&self) -> bool {
        self.disp.invoke_method("ClearUndoStack", vec![]).is_ok()
    }

    // ---------------------------------------------------------
    // 视图 / 窗口
    // ---------------------------------------------------------

    pub fn views(&self) -> Option<crate::view::IvgViews> {
        self.prop_dispatch("Views").map(crate::view::IvgViews::new)
    }

    pub fn windows(&self) -> Option<crate::view::IvgWindows> {
        self.prop_dispatch("Windows")
            .map(crate::view::IvgWindows::new)
    }

    // ---------------------------------------------------------
    // 网格 / 标尺
    // ---------------------------------------------------------

    pub fn grid(&self) -> Option<crate::misc::IvgGrid> {
        self.prop_dispatch("Grid").map(crate::misc::IvgGrid::new)
    }

    pub fn rulers(&self) -> Option<crate::misc::IvgRulers> {
        self.prop_dispatch("Rulers").map(crate::misc::IvgRulers::new)
    }

    // ---------------------------------------------------------
    // 样式
    // ---------------------------------------------------------

    pub fn style_sheet(&self) -> Option<crate::style::IvgStyleSheet> {
        self.prop_dispatch("StyleSheet")
            .map(crate::style::IvgStyleSheet::new)
    }

    // ---------------------------------------------------------
    // 对象管理器（Tree）
    // ---------------------------------------------------------

    pub fn tree_manager(&self) -> Option<crate::tree::IvgTreeManager> {
        self.prop_dispatch("ActiveTreeManager")
            .map(crate::tree::IvgTreeManager::new)
    }

    // ---------------------------------------------------------
    // 选择信息
    // ---------------------------------------------------------

    pub fn selection_info(&self) -> Option<crate::shape::IvgSelectionInformation> {
        self.prop_dispatch("SelectionInfo")
            .map(crate::shape::IvgSelectionInformation::new)
    }
}