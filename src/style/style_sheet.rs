//! `IVGStyleSheet` 鈥斺€?鏍峰紡琛?

use wincom::{ComObject, Variant};
use windows::Win32::System::Com::IDispatch;

use crate::shape::{IvgShape, IvgShapeRange};
use crate::style::{IvgStyle, IvgStyles};
use crate::text::IvgTextRange;

pub struct IvgStyleSheet {
    disp: ComObject,
}

impl IvgStyleSheet {
    pub fn new(disp: IDispatch) -> Self {
        Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) }
    }

    pub fn from_disp(disp: IDispatch) -> Self {
        Self::new(disp)
    }

    fn prop_dispatch(&self, name: &str) -> Option<IDispatch> {
        self.disp.get_property(name).ok()?.to_idispatch().ok()
    }

    // ---------------------------------------------------------
    // 闆嗗悎瑙嗗浘
    // ---------------------------------------------------------

    pub fn styles(&self) -> Option<IvgStyles> {
        self.prop_dispatch("Styles").map(IvgStyles::new)
    }

    pub fn style_sets(&self) -> Option<IvgStyles> {
        self.prop_dispatch("StyleSets").map(IvgStyles::new)
    }

    pub fn object_defaults(&self) -> Option<IvgStyles> {
        self.prop_dispatch("ObjectDefaults").map(IvgStyles::new)
    }

    pub fn all_styles(&self) -> Option<IvgStyles> {
        self.prop_dispatch("AllStyles").map(IvgStyles::new)
    }

    pub fn all_style_sets(&self) -> Option<IvgStyles> {
        self.prop_dispatch("AllStyleSets").map(IvgStyles::new)
    }

    pub fn all_color_styles(&self) -> Option<crate::color::IvgColors> {
        self.prop_dispatch("AllColorStyles").map(crate::color::IvgColors::new)
    }

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
    // 鍒涘缓鏍峰紡
    // ---------------------------------------------------------

    /// 浠庡浘褰㈠垱寤烘牱寮忋€?
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

    /// 鍒涘缓绌烘牱寮忋€?
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
    // 棰滆壊鏍峰紡
    // ---------------------------------------------------------

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

    pub fn delete_all_color_styles(&self) -> bool {
        self.disp
            .invoke_method("DeleteAllColorStyles", vec![])
            .is_ok()
    }

    pub fn delete_color_style(&self, name: impl Into<String>) -> bool {
        let args = vec![Variant::from_str(name.into())];
        self.disp.invoke_method("DeleteColorStyle", args).is_ok()
    }

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
    // 瀵煎叆 / 瀵煎嚭
    // ---------------------------------------------------------

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