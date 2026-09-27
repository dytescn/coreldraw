//! `IVGToolShape` / `IVGToolShapeAttributes` 鈥斺€?宸ュ叿鍥惧舰

use wincom::{ComObject, Variant};
use windows::Win32::System::Com::IDispatch;

use crate::layer::IvgLayer;
use crate::misc::IvgProperties;
use crate::style::IvgStyle;

// =============================================================
// IvgToolShapeAttributes
// =============================================================

pub struct IvgToolShapeAttributes {
    disp: ComObject,
}

impl IvgToolShapeAttributes {
    pub fn new(disp: IDispatch) -> Self {
        Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) }
    }

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

    pub fn set_can_resize(&self, v: bool) -> bool { self.set_bool("SetCanResize", v) }
    pub fn set_can_rotate(&self, v: bool) -> bool { self.set_bool("SetCanRotate", v) }
    pub fn set_can_skew(&self, v: bool) -> bool { self.set_bool("SetCanSkew", v) }
    pub fn set_can_size_disproportionally(&self, v: bool) -> bool {
        self.set_bool("SetCanSizeDisproportionally", v)
    }
    pub fn set_can_apply_nonlinear_transforms(&self, v: bool) -> bool {
        self.set_bool("SetCanApplyNonlinearTransforms", v)
    }
    pub fn set_regenerate_on_transform(&self, v: bool) -> bool {
        self.set_bool("SetRegenerateOnTransform", v)
    }
    pub fn set_regenerate_on_style_change(&self, v: bool) -> bool {
        self.set_bool("SetRegenerateOnStyleChange", v)
    }

    pub fn set_property_bar_guid(&self, v: impl Into<String>) -> bool {
        self.set_string("SetPropertyBarGuid", v)
    }
    pub fn set_context_menu_guid(&self, v: impl Into<String>) -> bool {
        self.set_string("SetContextMenuGuid", v)
    }
    pub fn set_object_manager_bitmap_guid(&self, v: impl Into<String>) -> bool {
        self.set_string("SetObjectManagerBitmapGuid", v)
    }
    pub fn set_edit_state_guid(&self, v: impl Into<String>) -> bool {
        self.set_string("SetEditStateGuid", v)
    }
    pub fn set_default_shape_name(&self, v: impl Into<String>) -> bool {
        self.set_string("SetDefaultShapename", v)
    }
}

// =============================================================
// IvgToolShape
// =============================================================

/// 宸ュ叿鍥惧舰鍥炶皟銆?
///
/// 鈿狅笍 鐢?CorelDRAW 璋冪敤锛岄渶瑕?`IDispatch` 妗ユ帴銆?
pub struct IvgToolShape {
    disp: ComObject,
}

impl IvgToolShape {
    pub fn new(disp: IDispatch) -> Self {
        Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) }
    }

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