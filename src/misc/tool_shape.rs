//! `IVGToolShape` / `IVGToolShapeAttributes` —— 工具图形

use wincom::{ComObject, Variant};
use windows::Win32::System::Com::IDispatch;

use crate::layer::IvgLayer;
use crate::misc::IvgProperties;
use crate::style::IvgStyle;

// =============================================================
// IvgToolShapeAttributes
// =============================================================

/// CorelDRAW `IVGToolShape` 对象的后期绑定包装（工具图形属性）。
///
/// 所有属性与方法通过 `IDispatch::Invoke` 按名称调用，调用失败时一般返回 `None` 或 `false`。
pub struct IvgToolShapeAttributes {
    disp: ComObject,
}

impl IvgToolShapeAttributes {
    /// 由 COM `IDispatch` 构造包装对象。
    pub fn new(disp: IDispatch) -> Self {
        Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) }
    }

    /// 把底层 COM 对象包装成 `VARIANT`，用于传给其它 COM 调用。
    pub fn as_variant(&self) -> Variant {
        self.disp.as_variant()
    }

    fn set_bool(&self, method: &str, v: bool) -> bool {
        let args = vec![Variant::from_bool(v)];
        self.disp.invoke_method(method, args).is_ok()
    }

    fn set_string(&self, method: &str, v: impl Into<String>) -> bool {
        let args = vec![Variant::from_str(v.into())];
        self.disp.invoke_method(method, args).is_ok()
    }

    /// 设置是否可以 resize。
    pub fn set_can_resize(&self, v: bool) -> bool { self.set_bool("SetCanResize", v) }
    /// 设置是否可以 rotate。
    pub fn set_can_rotate(&self, v: bool) -> bool { self.set_bool("SetCanRotate", v) }
    /// 设置是否可以倾斜。
    pub fn set_can_skew(&self, v: bool) -> bool { self.set_bool("SetCanSkew", v) }
    /// 设置 COM 属性。
    pub fn set_can_size_disproportionally(&self, v: bool) -> bool {
        self.set_bool("SetCanSizeDisproportionally", v)
    }
    /// 设置 COM 属性。
    pub fn set_can_apply_nonlinear_transforms(&self, v: bool) -> bool {
        self.set_bool("SetCanApplyNonlinearTransforms", v)
    }
    /// 设置 COM 属性。
    pub fn set_regenerate_on_transform(&self, v: bool) -> bool {
        self.set_bool("SetRegenerateOnTransform", v)
    }
    /// 设置 COM 属性。
    pub fn set_regenerate_on_style_change(&self, v: bool) -> bool {
        self.set_bool("SetRegenerateOnStyleChange", v)
    }

    /// 设置属性栏 GUID。
    pub fn set_property_bar_guid(&self, v: impl Into<String>) -> bool {
        self.set_string("SetPropertyBarGuid", v)
    }
    /// 设置上下文菜单 GUID。
    pub fn set_context_menu_guid(&self, v: impl Into<String>) -> bool {
        self.set_string("SetContextMenuGuid", v)
    }
    /// 设置对象管理器位图 GUID。
    pub fn set_object_manager_bitmap_guid(&self, v: impl Into<String>) -> bool {
        self.set_string("SetObjectManagerBitmapGuid", v)
    }
    /// 设置编辑状态 GUID。
    pub fn set_edit_state_guid(&self, v: impl Into<String>) -> bool {
        self.set_string("SetEditStateGuid", v)
    }
    /// 设置默认值图形名称。
    pub fn set_default_shape_name(&self, v: impl Into<String>) -> bool {
        self.set_string("SetDefaultShapename", v)
    }
}

// =============================================================
// IvgToolShape
// =============================================================

/// 工具图形回调。
///
/// ⚠️ 由 CorelDRAW 调用，需要 `IDispatch` 桥接。
pub struct IvgToolShape {
    disp: ComObject,
}

impl IvgToolShape {
    /// 由 COM `IDispatch` 构造包装对象。
    pub fn new(disp: IDispatch) -> Self {
        Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) }
    }

    /// 调用 COM 方法 `OnGenerateShape`。
    #[allow(clippy::too_many_arguments)]
    pub fn on_generate_shape(
        &self,
        parent: &IvgLayer,
        object_properties: &IvgProperties,
        style_attributes: &IvgStyle,
        transformation: &crate::geometry::IvgTransformMatrix,
        is_preview_only: bool,
    ) -> bool {
        let args = vec![
            parent.as_variant(),
            object_properties.as_variant(),
            style_attributes.as_variant(),
            transformation.as_variant(),
            Variant::from_bool(is_preview_only),
        ];
        self.disp.invoke_method("OnGenerateShape", args).is_ok()
    }
}