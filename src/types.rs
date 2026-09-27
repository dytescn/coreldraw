//! CLSID / IID / ProgID / 甯搁噺
//!
//! - **CLSID** 浠?`VGCore.dll` 绫诲瀷搴撶殑 co-class 娈垫妱褰?
//! - **IID** 浠庡悇 `IVGXxx` 鎺ュ彛鐨?`#[uuid]` 鎶勫綍锛堝綋鍓嶈蛋 `IDispatch` 鍚庢湡缁戝畾锛屾殏鏈敤鍒帮紝鐣欎綔澶囩敤锛?

use windows::core::GUID;

// ---------------------------------------------------------------
// ProgID 杈呭姪
// ---------------------------------------------------------------

/// 鐢熸垚 `CorelDRAW.Application.{version}` 褰㈠紡鐨?ProgID
///
/// 渚嬪 `progid_application("24.0")` 鈫?`"CorelDRAW.Application.24.0"`
pub fn progid_application(version: &str) -> String {
    if version.is_empty() {
        "CorelDRAW.Application".to_string()
    } else {
        format!("CorelDRAW.Application.{version}")
    }
}

// ---------------------------------------------------------------
// co-class CLSID
// ---------------------------------------------------------------
pub const IID_IDISPATCH: windows::core::GUID =
    windows::core::GUID::from_u128(0x00020400_0000_0000_C000_000000000046);
pub const CLSID_APPLICATION:  GUID = GUID::from_u128(0xde3d0002_fee3_4ec0_9845_53d17354ae24);
pub const CLSID_DOCUMENT:     GUID = GUID::from_u128(0xde3d0026_fee3_4ec0_9845_53d17354ae24);
pub const CLSID_DOCUMENTS:    GUID = GUID::from_u128(0xde3d0027_fee3_4ec0_9845_53d17354ae24);
pub const CLSID_PAGE:         GUID = GUID::from_u128(0xde3d005d_fee3_4ec0_9845_53d17354ae24);
pub const CLSID_PAGES:        GUID = GUID::from_u128(0xde3d005e_fee3_4ec0_9845_53d17354ae24);
pub const CLSID_LAYER:        GUID = GUID::from_u128(0xde3d0051_fee3_4ec0_9845_53d17354ae24);
pub const CLSID_LAYERS:       GUID = GUID::from_u128(0xde3d0052_fee3_4ec0_9845_53d17354ae24);
pub const CLSID_SHAPE:        GUID = GUID::from_u128(0xde3d0076_fee3_4ec0_9845_53d17354ae24);
pub const CLSID_SHAPES:       GUID = GUID::from_u128(0xde3d0078_fee3_4ec0_9845_53d17354ae24);
pub const CLSID_SHAPE_RANGE:  GUID = GUID::from_u128(0xde3d0077_fee3_4ec0_9845_53d17354ae24);
pub const CLSID_CURVE:        GUID = GUID::from_u128(0xde3d001c_fee3_4ec0_9845_53d17354ae24);
pub const CLSID_COLOR:        GUID = GUID::from_u128(0xde3d000f_fee3_4ec0_9845_53d17354ae24);
pub const CLSID_COLORS:       GUID = GUID::from_u128(0xde3d0015_fee3_4ec0_9845_53d17354ae24);
pub const CLSID_FILL:         GUID = GUID::from_u128(0xde3d0040_fee3_4ec0_9845_53d17354ae24);
pub const CLSID_OUTLINE:      GUID = GUID::from_u128(0xde3d005a_fee3_4ec0_9845_53d17354ae24);
pub const CLSID_TEXT:         GUID = GUID::from_u128(0xde3d009b_fee3_4ec0_9845_53d17354ae24);
pub const CLSID_EFFECT:       GUID = GUID::from_u128(0xde3d002c_fee3_4ec0_9845_53d17354ae24);
pub const CLSID_VIEW:         GUID = GUID::from_u128(0xde3d00b1_fee3_4ec0_9845_53d17354ae24);
pub const CLSID_WINDOW:       GUID = GUID::from_u128(0xde3d00b3_fee3_4ec0_9845_53d17354ae24);
pub const CLSID_TREE_NODE:    GUID = GUID::from_u128(0xde3d00ad_fee3_4ec0_9845_53d17354ae24);
pub const CLSID_PROPERTIES:   GUID = GUID::from_u128(0xde3d006b_fee3_4ec0_9845_53d17354ae24);
pub const CLSID_POINT:        GUID = GUID::from_u128(0xde3d00c1_fee3_4ec0_9845_53d17354ae24);
pub const CLSID_VECTOR:       GUID = GUID::from_u128(0xde3d00c3_fee3_4ec0_9845_53d17354ae24);
pub const CLSID_RECT:         GUID = GUID::from_u128(0xde3d006f_fee3_4ec0_9845_53d17354ae24);

// 鍏朵綑 co-class CLSID 鎸夐渶琛ワ細
// CLSID_STRUCT_EXPORT_OPTIONS, CLSID_STRUCT_SAVE_AS_OPTIONS, ...

// ---------------------------------------------------------------
// 鎺ュ彛 IID锛堝鐢級
// ---------------------------------------------------------------

pub const IID_IVGAPPLICATION: GUID = GUID::from_u128(0xb058000b_9aa4_44fd_9547_4f91eb757ac4);
pub const IID_IVGDOCUMENT:    GUID = GUID::from_u128(0xb0580024_9aa4_44fd_9547_4f91eb757ac4);
pub const IID_IVGDOCUMENTS:   GUID = GUID::from_u128(0xb0580025_9aa4_44fd_9547_4f91eb757ac4);
pub const IID_IVGPAGE:        GUID = GUID::from_u128(0xb0580048_9aa4_44fd_9547_4f91eb757ac4);
pub const IID_IVGPAGES:       GUID = GUID::from_u128(0xb0580049_9aa4_44fd_9547_4f91eb757ac4);
pub const IID_IVGLAYER:       GUID = GUID::from_u128(0xb0580040_9aa4_44fd_9547_4f91eb757ac4);
pub const IID_IVGLAYERS:      GUID = GUID::from_u128(0xb0580041_9aa4_44fd_9547_4f91eb757ac4);
pub const IID_IVGSHAPE:       GUID = GUID::from_u128(0xb058005d_9aa4_44fd_9547_4f91eb757ac4);
pub const IID_IVGSHAPES:      GUID = GUID::from_u128(0xb058005f_9aa4_44fd_9547_4f91eb757ac4);
pub const IID_IVGSHAPERANGE:  GUID = GUID::from_u128(0xb058005e_9aa4_44fd_9547_4f91eb757ac4);
pub const IID_IVGCURVE:       GUID = GUID::from_u128(0xb0580019_9aa4_44fd_9547_4f91eb757ac4);
pub const IID_IVGCOLOR:       GUID = GUID::from_u128(0xb0580012_9aa4_44fd_9547_4f91eb757ac4);
pub const IID_IVGFILL:        GUID = GUID::from_u128(0xb0580038_9aa4_44fd_9547_4f91eb757ac4);
pub const IID_IVGOUTLINE:     GUID = GUID::from_u128(0xb0580045_9aa4_44fd_9547_4f91eb757ac4);
pub const IID_IVGTEXT:        GUID = GUID::from_u128(0xb0580071_9aa4_44fd_9547_4f91eb757ac4);
pub const IID_IVGEFFECT:      GUID = GUID::from_u128(0xb0580026_9aa4_44fd_9547_4f91eb757ac4);
pub const IID_IVGVIEW:        GUID = GUID::from_u128(0xb058007f_9aa4_44fd_9547_4f91eb757ac4);

/// 鍚戝悗鍏煎鏃т唬鐮佺殑甯搁噺鍚?
pub const APP_INTER_IID: GUID = IID_IVGAPPLICATION;