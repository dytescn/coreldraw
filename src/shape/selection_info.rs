//! `IVGSelectionInformation` —— 选择信息
//!
//! 由 `document.selection_info()` 取得，是**只读**信息源。
//! 用来判断当前选择里有哪些图形 / 什么类型，配合插件工具使用。

use wincom::ComObject;
use windows::Win32::System::Com::IDispatch;

use crate::shape::IvgShape;

/// CorelDRAW `IVGSelectionInformation` 对象的后期绑定包装（选择信息）。
///
/// 所有属性与方法通过 `IDispatch::Invoke` 按名称调用，调用失败时一般返回 `None` 或 `false`。
pub struct IvgSelectionInformation {
    disp: ComObject,
}

impl IvgSelectionInformation {
    /// 由 COM `IDispatch` 构造包装对象。
    pub fn new(disp: IDispatch) -> Self {
        Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) }
    }

    /// 从 COM `IDispatch` 构造包装对象，等价于 `new`。
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

    // ---- 数量 / 首末 ----

    /// 读取返回条目数量（`Count`）。
    pub fn count(&self) -> Option<i64> { self.prop_i64("Count") }

    /// 读取第一个图形（`FirstShape`）。
    pub fn first_shape(&self) -> Option<IvgShape> {
        self.prop_dispatch("FirstShape").map(IvgShape::new)
    }

    /// 读取第二图形（`SecondShape`）。
    pub fn second_shape(&self) -> Option<IvgShape> {
        self.prop_dispatch("SecondShape").map(IvgShape::new)
    }

    /// 读取第一个图形轮廓（`FirstShapeWithOutline`）。
    pub fn first_shape_with_outline(&self) -> Option<IvgShape> {
        self.prop_dispatch("FirstShapeWithOutline").map(IvgShape::new)
    }

    /// 读取第一个图形填充（`FirstShapeWithFill`）。
    pub fn first_shape_with_fill(&self) -> Option<IvgShape> {
        self.prop_dispatch("FirstShapeWithFill").map(IvgShape::new)
    }

    // ---- 分类 ----

    /// 读取调和顶部图形（`BlendTopShape`）。
    pub fn blend_top_shape(&self) -> Option<IvgShape> {
        self.prop_dispatch("BlendTopShape").map(IvgShape::new)
    }
    /// 读取调和底部图形（`BlendBottomShape`）。
    pub fn blend_bottom_shape(&self) -> Option<IvgShape> {
        self.prop_dispatch("BlendBottomShape").map(IvgShape::new)
    }
    /// 读取调和路径（`BlendPath`）。
    pub fn blend_path(&self) -> Option<IvgShape> {
        self.prop_dispatch("BlendPath").map(IvgShape::new)
    }

    /// 读取变形图形（`DistortionShape`）。
    pub fn distortion_shape(&self) -> Option<IvgShape> {
        self.prop_dispatch("DistortionShape").map(IvgShape::new)
    }
    /// 读取变形类型（`DistortionType`）。
    pub fn distortion_type(&self) -> Option<i64> { self.prop_i64("DistortionType") }

    /// 读取立体化字面图形（`ExtrudeFaceShape`）。
    pub fn extrude_face_shape(&self) -> Option<IvgShape> {
        self.prop_dispatch("ExtrudeFaceShape").map(IvgShape::new)
    }
    /// 读取立体化组（`ExtrudeGroup`）。
    pub fn extrude_group(&self) -> Option<IvgShape> {
        self.prop_dispatch("ExtrudeGroup").map(IvgShape::new)
    }
    /// 读取立体化斜角组（`ExtrudeBevelGroup`）。
    pub fn extrude_bevel_group(&self) -> Option<IvgShape> {
        self.prop_dispatch("ExtrudeBevelGroup").map(IvgShape::new)
    }

    /// 读取轮廓控件图形（`ContourControlShape`）。
    pub fn contour_control_shape(&self) -> Option<IvgShape> {
        self.prop_dispatch("ContourControlShape").map(IvgShape::new)
    }
    /// 读取轮廓组（`ContourGroup`）。
    pub fn contour_group(&self) -> Option<IvgShape> {
        self.prop_dispatch("ContourGroup").map(IvgShape::new)
    }

    /// 读取投影阴影控件图形（`DropShadowControlShape`）。
    pub fn drop_shadow_control_shape(&self) -> Option<IvgShape> {
        self.prop_dispatch("DropShadowControlShape").map(IvgShape::new)
    }
    /// 读取投影阴影组（`DropShadowGroup`）。
    pub fn drop_shadow_group(&self) -> Option<IvgShape> {
        self.prop_dispatch("DropShadowGroup").map(IvgShape::new)
    }

    /// 读取尺寸控件图形（`DimensionControlShape`）。
    pub fn dimension_control_shape(&self) -> Option<IvgShape> {
        self.prop_dispatch("DimensionControlShape").map(IvgShape::new)
    }
    /// 读取尺寸组（`DimensionGroup`）。
    pub fn dimension_group(&self) -> Option<IvgShape> {
        self.prop_dispatch("DimensionGroup").map(IvgShape::new)
    }

    /// 读取连接器线（`ConnectorLines`）。
    pub fn connector_lines(&self) -> Option<IvgShape> {
        self.prop_dispatch("ConnectorLines").map(IvgShape::new)
    }

    /// 读取已适配文本控件图形（`FittedTextControlShape`）。
    pub fn fitted_text_control_shape(&self) -> Option<IvgShape> {
        self.prop_dispatch("FittedTextControlShape").map(IvgShape::new)
    }
    /// 读取已适配文本（`FittedText`）。
    pub fn fitted_text(&self) -> Option<IvgShape> {
        self.prop_dispatch("FittedText").map(IvgShape::new)
    }

    /// 读取自然媒体控件图形（`NaturalMediaControlShape`）。
    pub fn natural_media_control_shape(&self) -> Option<IvgShape> {
        self.prop_dispatch("NaturalMediaControlShape").map(IvgShape::new)
    }
    /// 读取自然媒体组（`NaturalMediaGroup`）。
    pub fn natural_media_group(&self) -> Option<IvgShape> {
        self.prop_dispatch("NaturalMediaGroup").map(IvgShape::new)
    }

    // ---- 能力判断 ----

    /// 是否可以创建调和（`CanCreateBlend`）。
    pub fn can_create_blend(&self) -> Option<bool> { self.prop_bool("CanCreateBlend") }
    /// 是否可以打印（`CanPrint`）。
    pub fn can_print(&self) -> Option<bool> { self.prop_bool("CanPrint") }
    /// 是否可以应用填充轮廓（`CanApplyFillOutline`）。
    pub fn can_apply_fill_outline(&self) -> Option<bool> {
        self.prop_bool("CanApplyFillOutline")
    }
    /// 是否可以删除控件（`CanDeleteControl`）。
    pub fn can_delete_control(&self) -> Option<bool> { self.prop_bool("CanDeleteControl") }
    /// 是否可以取消群组（`CanUngroup`）。
    pub fn can_ungroup(&self) -> Option<bool> { self.prop_bool("CanUngroup") }
    /// 是否可以克隆（`CanClone`）。
    pub fn can_clone(&self) -> Option<bool> { self.prop_bool("CanClone") }
    /// 是否可以应用调和（`CanApplyBlend`）。
    pub fn can_apply_blend(&self) -> Option<bool> { self.prop_bool("CanApplyBlend") }
    /// 是否可以应用轮廓（`CanApplyContour`）。
    pub fn can_apply_contour(&self) -> Option<bool> { self.prop_bool("CanApplyContour") }
    /// 是否可以应用填充（`CanApplyFill`）。
    pub fn can_apply_fill(&self) -> Option<bool> { self.prop_bool("CanApplyFill") }
    /// 是否可以应用轮廓（`CanApplyOutline`）。
    pub fn can_apply_outline(&self) -> Option<bool> { self.prop_bool("CanApplyOutline") }
    /// 是否可以应用透明度（`CanApplyTransparency`）。
    pub fn can_apply_transparency(&self) -> Option<bool> {
        self.prop_bool("CanApplyTransparency")
    }
    /// 是否可以 assignURL（`CanAssignURL`）。
    pub fn can_assign_url(&self) -> Option<bool> { self.prop_bool("CanAssignURL") }
    /// 是否可以应用变形（`CanApplyDistortion`）。
    pub fn can_apply_distortion(&self) -> Option<bool> { self.prop_bool("CanApplyDistortion") }
    /// 是否可以应用封套（`CanApplyEnvelope`）。
    pub fn can_apply_envelope(&self) -> Option<bool> { self.prop_bool("CanApplyEnvelope") }
    /// 是否可以锁定图形集（`CanLockShapes`）。
    pub fn can_lock_shapes(&self) -> Option<bool> { self.prop_bool("CanLockShapes") }
    /// 是否可以解锁图形集（`CanUnlockShapes`）。
    pub fn can_unlock_shapes(&self) -> Option<bool> { self.prop_bool("CanUnlockShapes") }

    // ---- 状态----

    /// 是否编辑中文本（`IsEditingText`）。
    pub fn is_editing_text(&self) -> Option<bool> { self.prop_bool("IsEditingText") }
    /// 是否文本选择（`IsTextSelection`）。
    pub fn is_text_selection(&self) -> Option<bool> { self.prop_bool("IsTextSelection") }
    /// 是否于幂裁剪内容（`IsOnPowerClipContents`）。
    pub fn is_on_power_clip_contents(&self) -> Option<bool> {
        self.prop_bool("IsOnPowerClipContents")
    }
    /// 读取 COM 属性 `IsEditingRollOver`。
    pub fn is_editing_roll_over(&self) -> Option<bool> {
        self.prop_bool("IsEditingRollOver")
    }
    /// 是否控件已选中（`IsControlSelected`）。
    pub fn is_control_selected(&self) -> Option<bool> { self.prop_bool("IsControlSelected") }
    /// 是否组（`IsGroup`）。
    pub fn is_group(&self) -> Option<bool> { self.prop_bool("IsGroup") }
    /// 读取 COM 属性 `IsRegularShape`。
    pub fn is_regular_shape(&self) -> Option<bool> { self.prop_bool("IsRegularShape") }
    /// 是否控件图形（`IsControlShape`）。
    pub fn is_control_shape(&self) -> Option<bool> { self.prop_bool("IsControlShape") }
    /// 是否调和控件（`IsBlendControl`）。
    pub fn is_blend_control(&self) -> Option<bool> { self.prop_bool("IsBlendControl") }
    /// 是否调和组（`IsBlendGroup`）。
    pub fn is_blend_group(&self) -> Option<bool> { self.prop_bool("IsBlendGroup") }
    /// 是否克隆控件（`IsCloneControl`）。
    pub fn is_clone_control(&self) -> Option<bool> { self.prop_bool("IsCloneControl") }
    /// 是否轮廓控件（`IsContourControl`）。
    pub fn is_contour_control(&self) -> Option<bool> { self.prop_bool("IsContourControl") }
    /// 是否轮廓组（`IsContourGroup`）。
    pub fn is_contour_group(&self) -> Option<bool> { self.prop_bool("IsContourGroup") }
    /// 是否投影阴影控件（`IsDropShadowControl`）。
    pub fn is_drop_shadow_control(&self) -> Option<bool> {
        self.prop_bool("IsDropShadowControl")
    }
    /// 是否投影阴影组（`IsDropShadowGroup`）。
    pub fn is_drop_shadow_group(&self) -> Option<bool> { self.prop_bool("IsDropShadowGroup") }
    /// 是否尺寸控件（`IsDimensionControl`）。
    pub fn is_dimension_control(&self) -> Option<bool> { self.prop_bool("IsDimensionControl") }
    /// 是否立体化控件（`IsExtrudeControl`）。
    pub fn is_extrude_control(&self) -> Option<bool> { self.prop_bool("IsExtrudeControl") }
    /// 是否立体化组（`IsExtrudeGroup`）。
    pub fn is_extrude_group(&self) -> Option<bool> { self.prop_bool("IsExtrudeGroup") }
    /// 是否斜角组（`IsBevelGroup`）。
    pub fn is_bevel_group(&self) -> Option<bool> { self.prop_bool("IsBevelGroup") }
    /// 读取 COM 属性 `HasAutoLabelText`。
    pub fn has_auto_label_text(&self) -> Option<bool> { self.prop_bool("HasAutoLabelText") }
    /// 是否封套（`IsEnvelope`）。
    pub fn is_envelope(&self) -> Option<bool> { self.prop_bool("IsEnvelope") }
    /// 是否透视（`IsPerspective`）。
    pub fn is_perspective(&self) -> Option<bool> { self.prop_bool("IsPerspective") }
    /// 是否变形（`IsDistortion`）。
    pub fn is_distortion(&self) -> Option<bool> { self.prop_bool("IsDistortion") }
    /// 是否连接器线（`IsConnectorLine`）。
    pub fn is_connector_line(&self) -> Option<bool> { self.prop_bool("IsConnectorLine") }
    /// 是否连接器（`IsConnector`）。
    pub fn is_connector(&self) -> Option<bool> { self.prop_bool("IsConnector") }
    /// 是否已适配文本（`IsFittedText`）。
    pub fn is_fitted_text(&self) -> Option<bool> { self.prop_bool("IsFittedText") }
    /// 是否已适配文本控件（`IsFittedTextControl`）。
    pub fn is_fitted_text_control(&self) -> Option<bool> {
        self.prop_bool("IsFittedTextControl")
    }
    /// 是否自然媒体控件（`IsNaturalMediaControl`）。
    pub fn is_natural_media_control(&self) -> Option<bool> {
        self.prop_bool("IsNaturalMediaControl")
    }
    /// 是否自然媒体组（`IsNaturalMediaGroup`）。
    pub fn is_natural_media_group(&self) -> Option<bool> {
        self.prop_bool("IsNaturalMediaGroup")
    }
    /// 是否第二立体化控件（`IsSecondExtrudeControl`）。
    pub fn is_second_extrude_control(&self) -> Option<bool> {
        self.prop_bool("IsSecondExtrudeControl")
    }
    /// 是否第二轮廓控件（`IsSecondContourControl`）。
    pub fn is_second_contour_control(&self) -> Option<bool> {
        self.prop_bool("IsSecondContourControl")
    }
    /// 是否第二投影阴影控件（`IsSecondDropShadowControl`）。
    pub fn is_second_drop_shadow_control(&self) -> Option<bool> {
        self.prop_bool("IsSecondDropShadowControl")
    }
    /// 是否第二自然媒体控件（`IsSecondNaturalMediaControl`）。
    pub fn is_second_natural_media_control(&self) -> Option<bool> {
        self.prop_bool("IsSecondNaturalMediaControl")
    }
    /// 是否艺术文本已选中（`IsArtisticTextSelected`）。
    pub fn is_artistic_text_selected(&self) -> Option<bool> {
        self.prop_bool("IsArtisticTextSelected")
    }
    /// 是否段落文本已选中（`IsParagraphTextSelected`）。
    pub fn is_paragraph_text_selected(&self) -> Option<bool> {
        self.prop_bool("IsParagraphTextSelected")
    }
    /// 是否文本已选中（`IsTextSelected`）。
    pub fn is_text_selected(&self) -> Option<bool> { self.prop_bool("IsTextSelected") }
    /// 是否 OLE 已选中（`IsOLESelected`）。
    pub fn is_ole_selected(&self) -> Option<bool> { self.prop_bool("IsOLESelected") }
    /// 是否位图已选中（`IsBitmapSelected`）。
    pub fn is_bitmap_selected(&self) -> Option<bool> { self.prop_bool("IsBitmapSelected") }
    /// 是否位图存在（`IsBitmapPresent`）。
    pub fn is_bitmap_present(&self) -> Option<bool> { self.prop_bool("IsBitmapPresent") }
    /// 是否透镜存在（`IsLensPresent`）。
    pub fn is_lens_present(&self) -> Option<bool> { self.prop_bool("IsLensPresent") }
    /// 读取 COM 属性 `IsMaskedBitmapPresent`。
    pub fn is_masked_bitmap_present(&self) -> Option<bool> {
        self.prop_bool("IsMaskedBitmapPresent")
    }
    /// 是否组已选中（`IsGroupSelected`）。
    pub fn is_group_selected(&self) -> Option<bool> { self.prop_bool("IsGroupSelected") }
    /// 是否链接组已选中（`IsLinkGroupSelected`）。
    pub fn is_link_group_selected(&self) -> Option<bool> {
        self.prop_bool("IsLinkGroupSelected")
    }
    /// 是否链接控件已选中（`IsLinkControlSelected`）。
    pub fn is_link_control_selected(&self) -> Option<bool> {
        self.prop_bool("IsLinkControlSelected")
    }
    /// 读取 COM 属性 `IsAttachedToDimension`。
    pub fn is_attached_to_dimension(&self) -> Option<bool> {
        self.prop_bool("IsAttachedToDimension")
    }
    /// 是否透视存在（`IsPerspectivePresent`）。
    pub fn is_perspective_present(&self) -> Option<bool> {
        self.prop_bool("IsPerspectivePresent")
    }
    /// 是否封套存在（`IsEnvelopePresent`）。
    pub fn is_envelope_present(&self) -> Option<bool> { self.prop_bool("IsEnvelopePresent") }
    /// 是否变形存在（`IsDistortionPresent`）。
    pub fn is_distortion_present(&self) -> Option<bool> {
        self.prop_bool("IsDistortionPresent")
    }
    /// 是否参考线已选中（`IsGuidelineSelected`）。
    pub fn is_guideline_selected(&self) -> Option<bool> {
        self.prop_bool("IsGuidelineSelected")
    }
    /// 是否互联网对象已选中（`IsInternetObjectSelected`）。
    pub fn is_internet_object_selected(&self) -> Option<bool> {
        self.prop_bool("IsInternetObjectSelected")
    }
    /// 读取 COM 属性 `IsSoundObjectSelected`。
    pub fn is_sound_object_selected(&self) -> Option<bool> {
        self.prop_bool("IsSoundObjectSelected")
    }
    /// 读取 COM 属性 `IsExternalBitmapSelected`。
    pub fn is_external_bitmap_selected(&self) -> Option<bool> {
        self.prop_bool("IsExternalBitmapSelected")
    }
    /// 读取 COM 属性 `IsNonExternalBitmapSelected`。
    pub fn is_non_external_bitmap_selected(&self) -> Option<bool> {
        self.prop_bool("IsNonExternalBitmapSelected")
    }
    /// 是否网格填充已选中（`IsMeshFillSelected`）。
    pub fn is_mesh_fill_selected(&self) -> Option<bool> {
        self.prop_bool("IsMeshFillSelected")
    }
    /// 是否网格填充存在（`IsMeshFillPresent`）。
    pub fn is_mesh_fill_present(&self) -> Option<bool> { self.prop_bool("IsMeshFillPresent") }
    /// 读取 COM 属性 `IsRollOverSelected`。
    pub fn is_roll_over_selected(&self) -> Option<bool> {
        self.prop_bool("IsRollOverSelected")
    }
    /// 读取 COM 属性 `ContainsRollOverParent`。
    pub fn contains_roll_over_parent(&self) -> Option<bool> {
        self.prop_bool("ContainsRollOverParent")
    }
}