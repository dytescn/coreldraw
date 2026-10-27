//! `IVGStyleSheet` —— 样式表

use wincom::{ComObject, Variant};
use windows::Win32::System::Com::IDispatch;

use crate::shape::{IvgShape, IvgShapeRange};
use crate::style::{IvgStyle, IvgStyles};
use crate::text::IvgTextRange;

/// CorelDRAW `IVGStyleSheet` 对象的后期绑定包装（样式表）。
///
/// 所有属性与方法通过 `IDispatch::Invoke` 按名称调用，调用失败时一般返回 `None` 或 `false`。
pub struct IvgStyleSheet {
    disp: ComObject,
}

impl IvgStyleSheet {
    /// 由 COM `IDispatch` 构造包装对象。
    pub fn new(disp: IDispatch) -> Self {
        Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) }
    }

    /// 从 COM `IDispatch` 构造包装对象，等价于 `new`。
    pub fn from_disp(disp: IDispatch) -> Self {
        Self::new(disp)
    }

    fn prop_dispatch(&self, name: &str) -> Option<IDispatch> {
        self.disp.get_property(name).ok()?.to_idispatch().ok()
    }

    // ---------------------------------------------------------
    // 集合视图
    // ---------------------------------------------------------

    /// 读取样式集（`Styles`）。
    pub fn styles(&self) -> Option<IvgStyles> {
        self.prop_dispatch("Styles").map(IvgStyles::new)
    }

    /// 读取 COM 属性 `StyleSets`。
    pub fn style_sets(&self) -> Option<IvgStyles> {
        self.prop_dispatch("StyleSets").map(IvgStyles::new)
    }

    /// 读取 COM 属性 `ObjectDefaults`。
    pub fn object_defaults(&self) -> Option<IvgStyles> {
        self.prop_dispatch("ObjectDefaults").map(IvgStyles::new)
    }

    /// 读取全部样式集（`AllStyles`）。
    pub fn all_styles(&self) -> Option<IvgStyles> {
        self.prop_dispatch("AllStyles").map(IvgStyles::new)
    }

    /// 读取 COM 属性 `AllStyleSets`。
    pub fn all_style_sets(&self) -> Option<IvgStyles> {
        self.prop_dispatch("AllStyleSets").map(IvgStyles::new)
    }

    /// 读取全部颜色样式集（`AllColorStyles`）。
    pub fn all_color_styles(&self) -> Option<crate::color::IvgColors> {
        self.prop_dispatch("AllColorStyles").map(crate::color::IvgColors::new)
    }

    /// 查找样式（COM 方法 `FindStyle`）。
    pub fn find_style(&self, name: impl Into<String>) -> Option<IvgStyle> {
        let args = vec![Variant::from_str(name.into())];
        self.disp
            .invoke_method("FindStyle", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgStyle::new)
    }

    // ---------------------------------------------------------
    // 创建样式
    // ---------------------------------------------------------

    /// 从图形创建样式。
    pub fn create_style_from_shape(
        &self,
        shape: &IvgShape,
        category: impl Into<String>,
        name: impl Into<String>,
        replace_existing: bool,
    ) -> Option<IvgStyles> {
        let args = vec![
            shape.as_variant(),
            Variant::from_str(category.into()),
            Variant::from_str(name.into()),
            Variant::from_bool(replace_existing),
        ];
        self.disp
            .invoke_method("CreateStyleFromShape", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgStyles::new)
    }

    /// 创建样式图形范围（COM 方法 `CreateStyleFromShapeRange`）。
    pub fn create_style_from_shape_range(
        &self,
        range: &IvgShapeRange,
        category: impl Into<String>,
        name: impl Into<String>,
        replace_existing: bool,
    ) -> Option<IvgStyles> {
        let args = vec![
            range.as_variant(),
            Variant::from_str(category.into()),
            Variant::from_str(name.into()),
            Variant::from_bool(replace_existing),
        ];
        self.disp
            .invoke_method("CreateStyleFromShapeRange", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgStyles::new)
    }

    /// 创建样式文本范围（COM 方法 `CreateStyleFromTextRange`）。
    pub fn create_style_from_text_range(
        &self,
        range: &IvgTextRange,
        category: impl Into<String>,
        name: impl Into<String>,
        replace_existing: bool,
    ) -> Option<IvgStyles> {
        let args = vec![
            range.as_variant(),
            Variant::from_str(category.into()),
            Variant::from_str(name.into()),
            Variant::from_bool(replace_existing),
        ];
        self.disp
            .invoke_method("CreateStyleFromTextRange", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgStyles::new)
    }

    /// 创建空样式。
    pub fn create_style(
        &self,
        category: impl Into<String>,
        based_on: impl Into<String>,
        name: impl Into<String>,
        replace_existing: bool,
    ) -> Option<IvgStyle> {
        let args = vec![
            Variant::from_str(category.into()),
            Variant::from_str(based_on.into()),
            Variant::from_str(name.into()),
            Variant::from_bool(replace_existing),
        ];
        self.disp
            .invoke_method("CreateStyle", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgStyle::new)
    }

    /// 创建样式设置（COM 方法 `CreateStyleSet`）。
    pub fn create_style_set(
        &self,
        based_on: impl Into<String>,
        name: impl Into<String>,
        replace_existing: bool,
    ) -> Option<IvgStyle> {
        let args = vec![
            Variant::from_str(based_on.into()),
            Variant::from_str(name.into()),
            Variant::from_bool(replace_existing),
        ];
        self.disp
            .invoke_method("CreateStyleSet", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgStyle::new)
    }

    // ---------------------------------------------------------
    // 颜色样式
    // ---------------------------------------------------------

    /// 创建颜色样式（COM 方法 `CreateColorStyle`）。
    #[allow(clippy::too_many_arguments)]
    pub fn create_color_style(
        &self,
        name: impl Into<String>,
        color: &crate::color::IvgColor,
        harmony_index: i32,
        index_in_harmony: i32,
        replace_existing: bool,
    ) -> Option<crate::color::IvgColor> {
        let args = vec![
            Variant::from_str(name.into()),
            color.as_variant(),
            Variant::from_i64(harmony_index as i64),
            Variant::from_i64(index_in_harmony as i64),
            Variant::from_bool(replace_existing),
        ];
        self.disp
            .invoke_method("CreateColorStyle", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(crate::color::IvgColor::new)
    }

    /// 删除全部颜色样式集（COM 方法 `DeleteAllColorStyles`）。
    pub fn delete_all_color_styles(&self) -> bool {
        self.disp
            .invoke_method("DeleteAllColorStyles", vec![])
            .is_ok()
    }

    /// 删除颜色样式（COM 方法 `DeleteColorStyle`）。
    pub fn delete_color_style(&self, name: impl Into<String>) -> bool {
        let args = vec![Variant::from_str(name.into())];
        self.disp.invoke_method("DeleteColorStyle", args).is_ok()
    }

    /// 调用 COM 方法 `RenameColorStyle`。
    pub fn rename_color_style(
        &self,
        old_name: impl Into<String>,
        new_name: impl Into<String>,
    ) -> bool {
        let args = vec![
            Variant::from_str(old_name.into()),
            Variant::from_str(new_name.into()),
        ];
        self.disp.invoke_method("RenameColorStyle", args).is_ok()
    }

    // ---------------------------------------------------------
    // 导入 / 导出
    // ---------------------------------------------------------

    /// 导出（COM 方法 `Export`）。
    #[allow(clippy::too_many_arguments)]
    pub fn export(
        &self,
        file_name: impl Into<String>,
        styles: bool,
        style_sets: bool,
        object_defaults: bool,
        color_styles: bool,
    ) -> Option<bool> {
        let args = vec![
            Variant::from_str(file_name.into()),
            Variant::from_bool(styles),
            Variant::from_bool(style_sets),
            Variant::from_bool(object_defaults),
            Variant::from_bool(color_styles),
        ];
        self.disp.invoke_method("Export", args).ok()?.to_bool().ok()
    }

    /// 导入（COM 方法 `Import`）。
    #[allow(clippy::too_many_arguments)]
    pub fn import(
        &self,
        file_name: impl Into<String>,
        merge_styles: bool,
        styles: bool,
        style_sets: bool,
        object_defaults: bool,
        color_styles: bool,
    ) -> Option<bool> {
        let args = vec![
            Variant::from_str(file_name.into()),
            Variant::from_bool(merge_styles),
            Variant::from_bool(styles),
            Variant::from_bool(style_sets),
            Variant::from_bool(object_defaults),
            Variant::from_bool(color_styles),
        ];
        self.disp.invoke_method("Import", args).ok()?.to_bool().ok()
    }
}