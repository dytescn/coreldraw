use dyteslogs::logs::LogError;
use windows::Win32::System::Ole;
use windows::Win32::System::Com;
use windows::Win32::System::Variant::VARIANT;
use windows::core;
use wincom::dispatch::ComObject;
use wincom::utils::VariantExt;
use crate::sdk::types::*;

use super::fill::IvgFill;
use super::outline::IvgOutLine;
use super::transparency::IVGTransparency;

pub struct IvgShape {
    disp:ComObject
}

impl IvgShape{
   pub fn new(disp:Com::IDispatch)->Self {
        let obj: ComObject =  ComObject::clone_from(disp, DOC_INTER_IID).expect("init core error");
        Self{
            disp:obj
        }
    }
    fn get_Application(){}
    fn get_Parent(){}
    fn get_StaticID(){}
    fn ConvertToCurves(){}
    pub fn get_Name(&self)->String{
        let app_vers = self.disp.get_property("name").log_error("got error").unwrap();
        app_vers.to_string().log_error("got error").unwrap()
    }
    fn put_Name(){}
    fn get_Shapes(){}
    fn get_Rectangle(){}
    fn get_PositionX(){}
    fn put_PositionX(){}
    fn get_PositionY(){}
    fn put_PositionY(){}
    fn get_SizeWidth(){}
    fn put_SizeWidth(){}
    fn get_SizeHeight(){}
    fn put_SizeHeight(){}
    fn get_Ellipse(){}
    fn get_Polygon(){}
    fn get_Curve(){}
    fn get_Bitmap(){}
    fn get_Type(){}
    pub fn get_Outline(&self)->Option<IvgOutLine>{
        let doc_res = self.disp.get_property("outline");
        match doc_res {
            Ok(doc)=>{
                match doc.to_idispatch(){
                    Ok(disp)=>{
                        let ivgcolor: IvgOutLine = IvgOutLine::new(disp.clone());
                        return Some(ivgcolor);
                    }
                    Err(e)=>{
                        return None
                    }
                }
            }
            Err(err)=>{
                return None
            }
        }
    }
    pub fn get_Fill(&self)->Option<IvgFill>{
        let doc_res = self.disp.get_property("fill");
        match doc_res {
            Ok(doc)=>{
                match doc.to_idispatch(){
                    Ok(disp)=>{
                        let ivgcolor: IvgFill = IvgFill::new(disp.clone());
                        return Some(ivgcolor);
                    }
                    Err(e)=>{
                        return None
                    }
                }
            }
            Err(err)=>{
                return None
            }
        }
    }
    fn get_Text(){}
    fn Delete(){}
    fn Duplicate(){}
    fn Skew(){}
    fn Move(){}
    fn get_RotationAngle(){}
    fn put_RotationAngle(){}
    fn get_RotationCenterX(){}
    fn put_RotationCenterX(){}
    fn get_RotationCenterY(){}
    fn put_RotationCenterY(){}
    fn Rotate(){}
    fn ConvertToBitmap(){}
    fn Group(){}
    fn Ungroup(){}
    fn UngroupAll(){}
    fn OrderToFront(){}
    fn OrderToBack(){}
    fn OrderForwardOne(){}
    fn OrderBackOne(){}
    fn OrderFrontOf(){}
    fn OrderBackOf(){}
    fn OrderIsInFrontOf(){}
    fn AddToSelection(){}
    fn RemoveFromSelection(){}
    fn Separate(){}
    fn get_Layer(){}
    fn put_Layer(){}
    fn get_SnapPoints(){}
    fn get_Connector(){}
    fn IsOnShape(){}
    fn CreateArrowHead(){}
    fn Copy(){}
    fn Cut(){}
    fn Clone(){}
    fn Stretch(){}
    fn SetPosition(){}
    fn SetSize(){}
    fn GetPosition(){}
    fn GetSize(){}
    fn get_Properties(){}
    fn OrderReverse(){}
    fn Combine(){}
    fn BreakApart(){}
    fn put_Fill(){}
    fn Weld(){}
    fn Trim(){}
    fn Intersect(){}
    fn get_Effects(){}
    fn get_Effect(){}
    fn CreateDropShadow(){}
    fn CreateBlend(){}
    fn CreateExtrude(){}
    fn CreateEnvelope(){}
    fn Flip(){}
    fn get_Locked(){}
    fn put_Locked(){}
    fn get_OriginalWidth(){}
    fn get_OriginalHeight(){}
    fn get_Selected(&self)->bool{
        let app_vers = self.disp.get_property("selected").log_error("got error").unwrap();
        app_vers.to_bool().log_error("got error").unwrap()
    }
    pub fn put_Selected(&self,param:bool){
        let mut vart_vec = Vec::new();
        let vart_name = VARIANT::from_bool(param);
        vart_vec.push(vart_name);
        self.disp.set_property("selected", vart_vec);
    }
    fn CreateLens(){}
    fn CreatePerspective(){}
    fn CreateContour(){}
    fn CreatePushPullDistortion(){}
    fn CreateZipperDistortion(){}
    fn CreateTwisterDistortion(){}
    fn get_Guide(){}
    fn AddToPowerClip(){}
    fn RemoveFromContainer(){}
    fn get_PowerClip(){}
    fn get_PowerClipParent(){}
    fn get_DrapeFill(){}
    fn put_DrapeFill(){}
    fn get_OverprintFill(){}
    fn put_OverprintFill(){}
    fn get_OverprintOutline(){}
    fn put_OverprintOutline(){}
    fn get_URL(){}
    fn get_ObjectData(){}
    fn get_CloneLink(){}
    fn get_Clones(){}
    fn get_AbsoluteHScale(){}
    fn get_AbsoluteVScale(){}
    fn get_AbsoluteSkew(){}
    pub fn get_Transparency(&self)->Option<IVGTransparency>{
        let doc_res = self.disp.get_property("Transparency");
        match doc_res {
            Ok(doc)=>{
                match doc.to_idispatch(){
                    Ok(disp)=>{
                        let ivgcolor: IVGTransparency = IVGTransparency::new(disp.clone());
                        return Some(ivgcolor);
                    }
                    Err(e)=>{
                        return None
                    }
                }
            }
            Err(err)=>{
                return None
            }
        }
    }
    fn GetMatrix(){}
    fn SetMatrix(){}
    fn ConvertToBitmapEx(){}
    fn SkewEx(){}
    fn RotateEx(){}
    fn get_ParentGroup(){}
    fn SetBoundingBox(){}
    fn CreateSelection(){}
    fn SetRotationCenter(){}
    fn ClearEffect(){}
    fn get_Next(){}
    fn get_Previous(){}
    fn StretchEx(){}
    fn SetSizeEx(){}
    fn GetBoundingBox(){}
    fn UngroupEx(){}
    fn UngroupAllEx(){}
    fn BreakApartEx(){}
    fn ApplyStyle(){}
    fn get_WrapText(){}
    fn put_WrapText(){}
    fn get_TextWrapOffset(){}
    fn put_TextWrapOffset(){}
    fn PlaceTextInside(){}
    fn get_DisplayCurve(){}
    fn CustomCommand(){}
    fn get_Custom(){}
    fn CreateCustomEffect(){}
    fn CreateCustomDistortion(){}
    fn AlignToShape(){}
    fn AlignToShapeRange(){}
    fn AlignToPage(){}
    fn AlignToPageCenter(){}
    fn AlignToGrid(){}
    fn AlignToPoint(){}
    fn get_Dimension(){}
    fn get_Symbol(){}
    fn ConvertToSymbol(){}
    fn get_OLE(){}
    fn DuplicateAsRange(){}
    fn CloneAsRange(){}
    fn MoveToLayer(){}
    fn CopyToLayer(){}
    fn CopyToLayerAsRange(){}
    fn ClearTransformations(){}
    fn Distribute(){}
    fn CompareTo(){}
    fn get_Selectable(){}
    fn ApplyEffectInvert(){}
    fn ApplyEffectPosterize(){}
    fn ApplyEffectBCI(){}
    fn ApplyEffectColorBalance(){}
    fn ApplyEffectGamma(){}
    fn ApplyEffectHSL(){}
    fn TransformMatrix(){}
    fn AffineTransform(){}
    fn get_TreeNode(){}
    fn ReplaceWith(){}
    fn get_Virtual(){}
    fn get_CanHaveFill(){}
    fn get_CanHaveOutline(){}
    fn get_IsSimpleShape(){}
    fn Fillet(){}
    fn Chamfer(){}
    fn Scallop(){}
    fn get_FillMode(){}
    fn put_FillMode(){}
    fn get_LeftX(){}
    fn get_RightX(){}
    fn get_TopY(){}
    fn get_BottomY(){}
    fn StepAndRepeat(){}
    fn get_OverprintBitmap(){}
    fn put_OverprintBitmap(){}
    fn IsTypeAnyOf(){}
    fn GetLinkedShapes(){}
    fn CreateEnvelopeFromShape(){}
    fn CreateEnvelopeFromCurve(){}
    fn get_EPS(){}
    fn Evaluate(){}
    fn get_BoundingBox(){}
    fn GetPositionEx(){}
    fn SetPositionEx(){}
    fn get_CenterX(){}
    fn put_CenterX(){}
    fn get_CenterY(){}
    fn put_CenterY(){}
    fn put_LeftX(){}
    fn put_RightX(){}
    fn put_TopY(){}
    fn put_BottomY(){}
    fn get_ZOrder(){}
    fn CompareToEx(){}
    fn CopyPropertiesFrom(){}
    fn GetOverprintFillState(){}
    fn GetOverprintOutlineState(){}
    fn get_Page(){}
    fn SnapPointsOfType(){}
    fn FindSnapPoint(){}
    fn get_Spread(){}
    fn get_PixelAlignedRendering(){}
    fn put_PixelAlignedRendering(){}
    fn get_BSpline(){}
    fn CreateDocumentFrom(){}
    fn AlignAndDistribute(){}
    fn get_Style(){}
    fn CreateBoundary(){}
    fn EqualDivide(){}
    fn Project(){}
    fn Unproject(){}
    fn get_TransformationMatrix(){}
    fn put_TransformationMatrix(){}
    fn ApplyTransformMatrix(){}
    fn get_Visible(){}
    fn put_Visible(){}
    fn ModifyToolShapeProperties(){}
    fn GetToolShapeGuid(){}
    fn CreateParallelCurves(){}
    fn FindShapeAtPoint(){}
    fn GetColorTypes(){}
    fn GetColors(){}
    fn FlattenEffects(){}
   
}