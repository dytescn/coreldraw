//! `IVGSelectionInformation` 鈥斺€?閫夋嫨淇℃伅
//!
//! 鐢?`document.selection_info()` 鍙栧緱锛?*鍙**淇℃伅婧愩€?
//! 鐢ㄦ潵鍒ゆ柇褰撳墠閫夋嫨閲屾湁鍝簺鍥惧舰 / 浠€涔堢被鍨嬶紝閰嶅悎鎻掍欢宸ュ叿浣跨敤銆?

use wincom::ComObject;
use windows::Win32::System::Com::IDispatch;

use crate::shape::IvgShape;

pub struct IvgSelectionInformation {
    disp: ComObject,
}

impl IvgSelectionInformation {
    pub fn new(disp: IDispatch) -> Self {
        Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) }
    }

    pub fn from_disp(disp: IDispatch) -> Self { Self::new(disp) }

    fn prop_i64(&self, name: &str) -> Option<i64> {
        self.disp.get_property(name).ok()?.to_i64().ok()
    }

    fn prop_bool(&self, name: &str) -> Option<bool> {
        self.disp.get_property(name).ok()?.to_bool().ok()
    }

    fn prop_dispatch(&self, name: &str) -> Option<IDispatch> {
        self.disp.get_property(name).ok()?.to_idispatch().ok()
    }

    // ---- 鏁伴噺 / 棣栨湯 ----

    pub fn count(&self) -> Option<i64> { self.prop_i64("Count") }

    pub fn first_shape(&self) -> Option<IvgShape> {
        self.prop_dispatch("FirstShape").map(IvgShape::new)
    }

    pub fn second_shape(&self) -> Option<IvgShape> {
        self.prop_dispatch("SecondShape").map(IvgShape::new)
    }

    pub fn first_shape_with_outline(&self) -> Option<IvgShape> {
        self.prop_dispatch("FirstShapeWithOutline").map(IvgShape::new)
    }

    pub fn first_shape_with_fill(&self) -> Option<IvgShape> {
        self.prop_dispatch("FirstShapeWithFill").map(IvgShape::new)
    }

    // ---- 鍒嗙被 ----

    pub fn blend_top_shape(&self) -> Option<IvgShape> {
        self.prop_dispatch("BlendTopShape").map(IvgShape::new)
    }
    pub fn blend_bottom_shape(&self) -> Option<IvgShape> {
        self.prop_dispatch("BlendBottomShape").map(IvgShape::new)
    }
    pub fn blend_path(&self) -> Option<IvgShape> {
        self.prop_dispatch("BlendPath").map(IvgShape::new)
    }

    pub fn distortion_shape(&self) -> Option<IvgShape> {
        self.prop_dispatch("DistortionShape").map(IvgShape::new)
    }
    pub fn distortion_type(&self) -> Option<i64> { self.prop_i64("DistortionType") }

    pub fn extrude_face_shape(&self) -> Option<IvgShape> {
        self.prop_dispatch("ExtrudeFaceShape").map(IvgShape::new)
    }
    pub fn extrude_group(&self) -> Option<IvgShape> {
        self.prop_dispatch("ExtrudeGroup").map(IvgShape::new)
    }
    pub fn extrude_bevel_group(&self) -> Option<IvgShape> {
        self.prop_dispatch("ExtrudeBevelGroup").map(IvgShape::new)
    }

    pub fn contour_control_shape(&self) -> Option<IvgShape> {
        self.prop_dispatch("ContourControlShape").map(IvgShape::new)
    }
    pub fn contour_group(&self) -> Option<IvgShape> {
        self.prop_dispatch("ContourGroup").map(IvgShape::new)
    }

    pub fn drop_shadow_control_shape(&self) -> Option<IvgShape> {
        self.prop_dispatch("DropShadowControlShape").map(IvgShape::new)
    }
    pub fn drop_shadow_group(&self) -> Option<IvgShape> {
        self.prop_dispatch("DropShadowGroup").map(IvgShape::new)
    }

    pub fn dimension_control_shape(&self) -> Option<IvgShape> {
        self.prop_dispatch("DimensionControlShape").map(IvgShape::new)
    }
    pub fn dimension_group(&self) -> Option<IvgShape> {
        self.prop_dispatch("DimensionGroup").map(IvgShape::new)
    }

    pub fn connector_lines(&self) -> Option<IvgShape> {
        self.prop_dispatch("ConnectorLines").map(IvgShape::new)
    }

    pub fn fitted_text_control_shape(&self) -> Option<IvgShape> {
        self.prop_dispatch("FittedTextControlShape").map(IvgShape::new)
    }
    pub fn fitted_text(&self) -> Option<IvgShape> {
        self.prop_dispatch("FittedText").map(IvgShape::new)
    }

    pub fn natural_media_control_shape(&self) -> Option<IvgShape> {
        self.prop_dispatch("NaturalMediaControlShape").map(IvgShape::new)
    }
    pub fn natural_media_group(&self) -> Option<IvgShape> {
        self.prop_dispatch("NaturalMediaGroup").map(IvgShape::new)
    }

    // ---- 鑳藉姏鍒ゆ柇 ----

    pub fn can_create_blend(&self) -> Option<bool> { self.prop_bool("CanCreateBlend") }
    pub fn can_print(&self) -> Option<bool> { self.prop_bool("CanPrint") }
    pub fn can_apply_fill_outline(&self) -> Option<bool> {
        self.prop_bool("CanApplyFillOutline")
    }
    pub fn can_delete_control(&self) -> Option<bool> { self.prop_bool("CanDeleteControl") }
    pub fn can_ungroup(&self) -> Option<bool> { self.prop_bool("CanUngroup") }
    pub fn can_clone(&self) -> Option<bool> { self.prop_bool("CanClone") }
    pub fn can_apply_blend(&self) -> Option<bool> { self.prop_bool("CanApplyBlend") }
    pub fn can_apply_contour(&self) -> Option<bool> { self.prop_bool("CanApplyContour") }
    pub fn can_apply_fill(&self) -> Option<bool> { self.prop_bool("CanApplyFill") }
    pub fn can_apply_outline(&self) -> Option<bool> { self.prop_bool("CanApplyOutline") }
    pub fn can_apply_transparency(&self) -> Option<bool> {
        self.prop_bool("CanApplyTransparency")
    }
    pub fn can_assign_url(&self) -> Option<bool> { self.prop_bool("CanAssignURL") }
    pub fn can_apply_distortion(&self) -> Option<bool> { self.prop_bool("CanApplyDistortion") }
    pub fn can_apply_envelope(&self) -> Option<bool> { self.prop_bool("CanApplyEnvelope") }
    pub fn can_lock_shapes(&self) -> Option<bool> { self.prop_bool("CanLockShapes") }
    pub fn can_unlock_shapes(&self) -> Option<bool> { self.prop_bool("CanUnlockShapes") }

    // ---- 鐘舵€?----

    pub fn is_editing_text(&self) -> Option<bool> { self.prop_bool("IsEditingText") }
    pub fn is_text_selection(&self) -> Option<bool> { self.prop_bool("IsTextSelection") }
    pub fn is_on_power_clip_contents(&self) -> Option<bool> {
        self.prop_bool("IsOnPowerClipContents")
    }
    pub fn is_editing_roll_over(&self) -> Option<bool> {
        self.prop_bool("IsEditingRollOver")
    }
    pub fn is_control_selected(&self) -> Option<bool> { self.prop_bool("IsControlSelected") }
    pub fn is_group(&self) -> Option<bool> { self.prop_bool("IsGroup") }
    pub fn is_regular_shape(&self) -> Option<bool> { self.prop_bool("IsRegularShape") }
    pub fn is_control_shape(&self) -> Option<bool> { self.prop_bool("IsControlShape") }
    pub fn is_blend_control(&self) -> Option<bool> { self.prop_bool("IsBlendControl") }
    pub fn is_blend_group(&self) -> Option<bool> { self.prop_bool("IsBlendGroup") }
    pub fn is_clone_control(&self) -> Option<bool> { self.prop_bool("IsCloneControl") }
    pub fn is_contour_control(&self) -> Option<bool> { self.prop_bool("IsContourControl") }
    pub fn is_contour_group(&self) -> Option<bool> { self.prop_bool("IsContourGroup") }
    pub fn is_drop_shadow_control(&self) -> Option<bool> {
        self.prop_bool("IsDropShadowControl")
    }
    pub fn is_drop_shadow_group(&self) -> Option<bool> { self.prop_bool("IsDropShadowGroup") }
    pub fn is_dimension_control(&self) -> Option<bool> { self.prop_bool("IsDimensionControl") }
    pub fn is_extrude_control(&self) -> Option<bool> { self.prop_bool("IsExtrudeControl") }
    pub fn is_extrude_group(&self) -> Option<bool> { self.prop_bool("IsExtrudeGroup") }
    pub fn is_bevel_group(&self) -> Option<bool> { self.prop_bool("IsBevelGroup") }
    pub fn has_auto_label_text(&self) -> Option<bool> { self.prop_bool("HasAutoLabelText") }
    pub fn is_envelope(&self) -> Option<bool> { self.prop_bool("IsEnvelope") }
    pub fn is_perspective(&self) -> Option<bool> { self.prop_bool("IsPerspective") }
    pub fn is_distortion(&self) -> Option<bool> { self.prop_bool("IsDistortion") }
    pub fn is_connector_line(&self) -> Option<bool> { self.prop_bool("IsConnectorLine") }
    pub fn is_connector(&self) -> Option<bool> { self.prop_bool("IsConnector") }
    pub fn is_fitted_text(&self) -> Option<bool> { self.prop_bool("IsFittedText") }
    pub fn is_fitted_text_control(&self) -> Option<bool> {
        self.prop_bool("IsFittedTextControl")
    }
    pub fn is_natural_media_control(&self) -> Option<bool> {
        self.prop_bool("IsNaturalMediaControl")
    }
    pub fn is_natural_media_group(&self) -> Option<bool> {
        self.prop_bool("IsNaturalMediaGroup")
    }
    pub fn is_second_extrude_control(&self) -> Option<bool> {
        self.prop_bool("IsSecondExtrudeControl")
    }
    pub fn is_second_contour_control(&self) -> Option<bool> {
        self.prop_bool("IsSecondContourControl")
    }
    pub fn is_second_drop_shadow_control(&self) -> Option<bool> {
        self.prop_bool("IsSecondDropShadowControl")
    }
    pub fn is_second_natural_media_control(&self) -> Option<bool> {
        self.prop_bool("IsSecondNaturalMediaControl")
    }
    pub fn is_artistic_text_selected(&self) -> Option<bool> {
        self.prop_bool("IsArtisticTextSelected")
    }
    pub fn is_paragraph_text_selected(&self) -> Option<bool> {
        self.prop_bool("IsParagraphTextSelected")
    }
    pub fn is_text_selected(&self) -> Option<bool> { self.prop_bool("IsTextSelected") }
    pub fn is_ole_selected(&self) -> Option<bool> { self.prop_bool("IsOLESelected") }
    pub fn is_bitmap_selected(&self) -> Option<bool> { self.prop_bool("IsBitmapSelected") }
    pub fn is_bitmap_present(&self) -> Option<bool> { self.prop_bool("IsBitmapPresent") }
    pub fn is_lens_present(&self) -> Option<bool> { self.prop_bool("IsLensPresent") }
    pub fn is_masked_bitmap_present(&self) -> Option<bool> {
        self.prop_bool("IsMaskedBitmapPresent")
    }
    pub fn is_group_selected(&self) -> Option<bool> { self.prop_bool("IsGroupSelected") }
    pub fn is_link_group_selected(&self) -> Option<bool> {
        self.prop_bool("IsLinkGroupSelected")
    }
    pub fn is_link_control_selected(&self) -> Option<bool> {
        self.prop_bool("IsLinkControlSelected")
    }
    pub fn is_attached_to_dimension(&self) -> Option<bool> {
        self.prop_bool("IsAttachedToDimension")
    }
    pub fn is_perspective_present(&self) -> Option<bool> {
        self.prop_bool("IsPerspectivePresent")
    }
    pub fn is_envelope_present(&self) -> Option<bool> { self.prop_bool("IsEnvelopePresent") }
    pub fn is_distortion_present(&self) -> Option<bool> {
        self.prop_bool("IsDistortionPresent")
    }
    pub fn is_guideline_selected(&self) -> Option<bool> {
        self.prop_bool("IsGuidelineSelected")
    }
    pub fn is_internet_object_selected(&self) -> Option<bool> {
        self.prop_bool("IsInternetObjectSelected")
    }
    pub fn is_sound_object_selected(&self) -> Option<bool> {
        self.prop_bool("IsSoundObjectSelected")
    }
    pub fn is_external_bitmap_selected(&self) -> Option<bool> {
        self.prop_bool("IsExternalBitmapSelected")
    }
    pub fn is_non_external_bitmap_selected(&self) -> Option<bool> {
        self.prop_bool("IsNonExternalBitmapSelected")
    }
    pub fn is_mesh_fill_selected(&self) -> Option<bool> {
        self.prop_bool("IsMeshFillSelected")
    }
    pub fn is_mesh_fill_present(&self) -> Option<bool> { self.prop_bool("IsMeshFillPresent") }
    pub fn is_roll_over_selected(&self) -> Option<bool> {
        self.prop_bool("IsRollOverSelected")
    }
    pub fn contains_roll_over_parent(&self) -> Option<bool> {
        self.prop_bool("ContainsRollOverParent")
    }
}