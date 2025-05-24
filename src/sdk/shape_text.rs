use windows::Win32::System::Ole;
use windows::Win32::System::Com;
use windows::Win32::System::Variant::VARIANT;
use windows::core;
use crate::dispatch::dispatch::ComObject;
use crate::coreldraw::types::*;
use crate::dispatch::utils::VariantExt;

pub struct IvgText {
    disp:ComObject
}

impl IvgText{

}