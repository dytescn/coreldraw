use dyteslogs::logs::LogError;
use windows::Win32::System::Ole;
use windows::Win32::System::Com;
use windows::Win32::System::Variant::VARIANT;
use windows::core;
use wincom::dispatch::ComObject;
use wincom::utils::VariantExt;
use crate::sdk::types::*;

pub struct IvgSelectInfo {
    disp:ComObject
}

impl IvgSelectInfo{
    pub fn new(disp:Com::IDispatch)->Self {
        let obj: ComObject =  ComObject::clone_from(disp, DOC_INTER_IID).expect("init core error");
        Self{
            disp:obj
        }
    }
    fn get_Application(){}
    fn get_Parent(){}
    pub fn get_Count(&self)->i32{
        let vart_res = self.disp.get_property("count");
        if vart_res.is_err(){
            return 0;
        }
        vart_res.unwrap().to_i32().unwrap()
    }
    fn get_FirstShape(){

    }
    fn get_SecondShape(){

    }
    fn get_BlendTopShape(){

    }
    fn get_BlendBottomShape(){

    }
    fn get_BlendPath(){

    }
    fn get_CanCreateBlend(){

    }
    fn get_DistortionShape(){

    }
    fn get_DistortionType(){

    }
    fn get_ExtrudeFaceShape(){

    }
    fn get_ExtrudeGroup(){

    }
    fn get_ExtrudeBevelGroup(){

    }
    fn get_ContourControlShape(){

    }
    fn get_ContourGroup(){

    }
    fn get_DropShadowControlShape(){

    }
    fn get_DropShadowGroup(){

    }
    fn get_DimensionControlShape(){

    }
    fn get_DimensionGroup(){

    }
    fn get_ConnectorLines(){

    }
    fn get_FittedTextControlShape(){

    }
    fn get_FittedText(){

    }
    fn get_FirstShapeWithOutline(){

    }
    fn get_FirstShapeWithFill(){

    }
    fn get_NaturalMediaControlShape(){

    }
    fn get_NaturalMediaGroup(){
        
    }
    fn get_CanPrint(){}
    fn get_IsEditingText(){}
    fn get_IsTextSelection(){}
    fn get_IsOnPowerClipContents(){}
    fn get_IsEditingRollOver(){}
    fn get_CanApplyFillOutline(){}
    fn get_IsControlSelected(){}
    fn get_CanDeleteControl(){}
    fn get_IsGroup(){}
    fn get_IsRegularShape(){}
    fn get_IsControlShape(){}
    fn get_IsBlendControl(){}
    fn get_IsBlendGroup(){}
    fn get_IsCloneControl(){}
    fn get_IsContourControl(){}
    fn get_IsContourGroup(){}
    fn get_IsDropShadowControl(){}
    fn get_IsDropShadowGroup(){}
    fn get_IsDimensionControl(){}
    fn get_IsExtrudeControl(){}
    fn get_IsExtrudeGroup(){}
    fn get_IsBevelGroup(){}
    fn get_HasAutoLabelText(){}
    fn get_IsEnvelope(){}
    fn get_IsPerspective(){}
    fn get_IsDistortion(){}
    fn get_IsConnectorLine(){}
    fn get_IsConnector(){}
    fn get_IsFittedText(){}
    fn get_IsFittedTextControl(){}
    fn get_IsNaturalMediaControl(){}
    fn get_IsNaturalMediaGroup(){}
    fn get_IsSecondExtrudeControl(){}
    fn get_IsSecondContourControl(){}
    fn get_IsSecondDropShadowControl(){}
    fn get_IsSecondNaturalMediaControl(){}
    fn get_IsArtisticTextSelected(){}
    fn get_IsParagraphTextSelected(){}
    fn get_IsTextSelected(){}
    fn get_IsOLESelected(){}
    fn get_IsBitmapSelected(){}
    fn get_IsBitmapPresent(){}
    fn get_IsLensPresent(){}
    fn get_IsMaskedBitmapPresent(){}
    fn get_IsGroupSelected(){}
    fn get_CanUngroup(){}
    fn get_IsLinkGroupSelected(){}
    fn get_IsLinkControlSelected(){}
    fn get_IsAttachedToDimension(){}
    fn get_IsFittedTextSelected(){}
    fn get_IsConnectorLineSelected(){}
    fn get_IsConnectorSelected(){}
    fn get_IsPerspectivePresent(){}
    fn get_IsEnvelopePresent(){}
    fn get_IsDistortionPresent(){}
    fn get_IsGuidelineSelected(){}
    fn get_IsInternetObjectSelected(){}
    fn get_IsSoundObjectSelected(){}
    fn get_IsExternalBitmapSelected(){}
    fn get_IsNonExternalBitmapSelected(){}
    fn get_IsMeshFillSelected(){}
    fn get_IsMeshFillPresent(){}
    fn get_IsRollOverSelected(){}
    fn get_ContainsRollOverParent(){}
    fn get_CanClone(){}
    fn get_CanApplyBlend(){}
    fn get_CanApplyContour(){}
    fn get_CanApplyFill(){}
    fn get_CanApplyOutline(){}
    fn get_CanApplyTransparency(){}
    fn get_CanAssignURL(){}
    fn get_CanApplyDistortion(){}
    fn get_CanApplyEnvelope(){}
    fn get_CanCopyBlend(){}
    fn get_CanCloneBlend(){}
    fn get_CanCopyExtrude(){}
    fn get_CanCloneExtrude(){}
    fn get_CanCopyContour(){}
    fn get_CanCloneContour(){}
    fn get_CanCopyDropShadow(){}
    fn get_CanCloneDropShadow(){}
    fn get_CanCopyLens(){}
    fn get_CanCopyPerspective(){}
    fn get_CanCopyEnvelope(){}
    fn get_CanCopyPowerclip(){}
    fn get_CanCopyDistortion(){}
    fn get_CanLockShapes(){}
    fn get_CanUnlockShapes(){}
}