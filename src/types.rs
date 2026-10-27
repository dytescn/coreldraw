//! CLSID / IID / ProgID / 常量
//!
//! - **CLSID** 从 `VGCore.dll` 类型库的 co-class 段抄录
//! - **IID** 从各 `IVGXxx` 接口的 `#[uuid]` 抄录（当前走 `IDispatch` 后期绑定，暂未用到，留作备用）

use windows::core::GUID;

// ---------------------------------------------------------------
// ProgID 辅助
// ---------------------------------------------------------------

/// 生成 `CorelDRAW.Application.{version}` 形式的 ProgID
///
/// 例如 `progid_application("24.0")` → `"CorelDRAW.Application.24.0"`
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
/// `IDISPATCH` 接口的 IID。
pub const IID_IDISPATCH: windows::core::GUID =
    windows::core::GUID::from_u128(0x00020400_0000_0000_C000_000000000046);
/// `APPLICATION` 对象的 CLSID。
pub const CLSID_APPLICATION:  GUID = GUID::from_u128(0xde3d0002_fee3_4ec0_9845_53d17354ae24);
/// `DOCUMENT` 对象的 CLSID。
pub const CLSID_DOCUMENT:     GUID = GUID::from_u128(0xde3d0026_fee3_4ec0_9845_53d17354ae24);
/// `DOCUMENTS` 对象的 CLSID。
pub const CLSID_DOCUMENTS:    GUID = GUID::from_u128(0xde3d0027_fee3_4ec0_9845_53d17354ae24);
/// `PAGE` 对象的 CLSID。
pub const CLSID_PAGE:         GUID = GUID::from_u128(0xde3d005d_fee3_4ec0_9845_53d17354ae24);
/// `PAGES` 对象的 CLSID。
pub const CLSID_PAGES:        GUID = GUID::from_u128(0xde3d005e_fee3_4ec0_9845_53d17354ae24);
/// `LAYER` 对象的 CLSID。
pub const CLSID_LAYER:        GUID = GUID::from_u128(0xde3d0051_fee3_4ec0_9845_53d17354ae24);
/// `LAYERS` 对象的 CLSID。
pub const CLSID_LAYERS:       GUID = GUID::from_u128(0xde3d0052_fee3_4ec0_9845_53d17354ae24);
/// `SHAPE` 对象的 CLSID。
pub const CLSID_SHAPE:        GUID = GUID::from_u128(0xde3d0076_fee3_4ec0_9845_53d17354ae24);
/// `SHAPES` 对象的 CLSID。
pub const CLSID_SHAPES:       GUID = GUID::from_u128(0xde3d0078_fee3_4ec0_9845_53d17354ae24);
/// `SHAPE_RANGE` 对象的 CLSID。
pub const CLSID_SHAPE_RANGE:  GUID = GUID::from_u128(0xde3d0077_fee3_4ec0_9845_53d17354ae24);
/// `CURVE` 对象的 CLSID。
pub const CLSID_CURVE:        GUID = GUID::from_u128(0xde3d001c_fee3_4ec0_9845_53d17354ae24);
/// `COLOR` 对象的 CLSID。
pub const CLSID_COLOR:        GUID = GUID::from_u128(0xde3d000f_fee3_4ec0_9845_53d17354ae24);
/// `COLORS` 对象的 CLSID。
pub const CLSID_COLORS:       GUID = GUID::from_u128(0xde3d0015_fee3_4ec0_9845_53d17354ae24);
/// `FILL` 对象的 CLSID。
pub const CLSID_FILL:         GUID = GUID::from_u128(0xde3d0040_fee3_4ec0_9845_53d17354ae24);
/// `OUTLINE` 对象的 CLSID。
pub const CLSID_OUTLINE:      GUID = GUID::from_u128(0xde3d005a_fee3_4ec0_9845_53d17354ae24);
/// `TEXT` 对象的 CLSID。
pub const CLSID_TEXT:         GUID = GUID::from_u128(0xde3d009b_fee3_4ec0_9845_53d17354ae24);
/// `EFFECT` 对象的 CLSID。
pub const CLSID_EFFECT:       GUID = GUID::from_u128(0xde3d002c_fee3_4ec0_9845_53d17354ae24);
/// `VIEW` 对象的 CLSID。
pub const CLSID_VIEW:         GUID = GUID::from_u128(0xde3d00b1_fee3_4ec0_9845_53d17354ae24);
/// `WINDOW` 对象的 CLSID。
pub const CLSID_WINDOW:       GUID = GUID::from_u128(0xde3d00b3_fee3_4ec0_9845_53d17354ae24);
/// `TREE_NODE` 对象的 CLSID。
pub const CLSID_TREE_NODE:    GUID = GUID::from_u128(0xde3d00ad_fee3_4ec0_9845_53d17354ae24);
/// `PROPERTIES` 对象的 CLSID。
pub const CLSID_PROPERTIES:   GUID = GUID::from_u128(0xde3d006b_fee3_4ec0_9845_53d17354ae24);
/// `POINT` 对象的 CLSID。
pub const CLSID_POINT:        GUID = GUID::from_u128(0xde3d00c1_fee3_4ec0_9845_53d17354ae24);
/// `VECTOR` 对象的 CLSID。
pub const CLSID_VECTOR:       GUID = GUID::from_u128(0xde3d00c3_fee3_4ec0_9845_53d17354ae24);
/// `RECT` 对象的 CLSID。
pub const CLSID_RECT:         GUID = GUID::from_u128(0xde3d006f_fee3_4ec0_9845_53d17354ae24);

// 其余 co-class CLSID 按需补：
// CLSID_STRUCT_EXPORT_OPTIONS, CLSID_STRUCT_SAVE_AS_OPTIONS, ...

// ---------------------------------------------------------------
// 接口 IID（备用）
// ---------------------------------------------------------------

/// `IVGAPPLICATION` 接口的 IID。
pub const IID_IVGAPPLICATION: GUID = GUID::from_u128(0xb058000b_9aa4_44fd_9547_4f91eb757ac4);
/// `IVGDOCUMENT` 接口的 IID。
pub const IID_IVGDOCUMENT:    GUID = GUID::from_u128(0xb0580024_9aa4_44fd_9547_4f91eb757ac4);
/// `IVGDOCUMENTS` 接口的 IID。
pub const IID_IVGDOCUMENTS:   GUID = GUID::from_u128(0xb0580025_9aa4_44fd_9547_4f91eb757ac4);
/// `IVGPAGE` 接口的 IID。
pub const IID_IVGPAGE:        GUID = GUID::from_u128(0xb0580048_9aa4_44fd_9547_4f91eb757ac4);
/// `IVGPAGES` 接口的 IID。
pub const IID_IVGPAGES:       GUID = GUID::from_u128(0xb0580049_9aa4_44fd_9547_4f91eb757ac4);
/// `IVGLAYER` 接口的 IID。
pub const IID_IVGLAYER:       GUID = GUID::from_u128(0xb0580040_9aa4_44fd_9547_4f91eb757ac4);
/// `IVGLAYERS` 接口的 IID。
pub const IID_IVGLAYERS:      GUID = GUID::from_u128(0xb0580041_9aa4_44fd_9547_4f91eb757ac4);
/// `IVGSHAPE` 接口的 IID。
pub const IID_IVGSHAPE:       GUID = GUID::from_u128(0xb058005d_9aa4_44fd_9547_4f91eb757ac4);
/// `IVGSHAPES` 接口的 IID。
pub const IID_IVGSHAPES:      GUID = GUID::from_u128(0xb058005f_9aa4_44fd_9547_4f91eb757ac4);
/// `IVGSHAPERANGE` 接口的 IID。
pub const IID_IVGSHAPERANGE:  GUID = GUID::from_u128(0xb058005e_9aa4_44fd_9547_4f91eb757ac4);
/// `IVGCURVE` 接口的 IID。
pub const IID_IVGCURVE:       GUID = GUID::from_u128(0xb0580019_9aa4_44fd_9547_4f91eb757ac4);
/// `IVGCOLOR` 接口的 IID。
pub const IID_IVGCOLOR:       GUID = GUID::from_u128(0xb0580012_9aa4_44fd_9547_4f91eb757ac4);
/// `IVGFILL` 接口的 IID。
pub const IID_IVGFILL:        GUID = GUID::from_u128(0xb0580038_9aa4_44fd_9547_4f91eb757ac4);
/// `IVGOUTLINE` 接口的 IID。
pub const IID_IVGOUTLINE:     GUID = GUID::from_u128(0xb0580045_9aa4_44fd_9547_4f91eb757ac4);
/// `IVGTEXT` 接口的 IID。
pub const IID_IVGTEXT:        GUID = GUID::from_u128(0xb0580071_9aa4_44fd_9547_4f91eb757ac4);
/// `IVGEFFECT` 接口的 IID。
pub const IID_IVGEFFECT:      GUID = GUID::from_u128(0xb0580026_9aa4_44fd_9547_4f91eb757ac4);
/// `IVGVIEW` 接口的 IID。
pub const IID_IVGVIEW:        GUID = GUID::from_u128(0xb058007f_9aa4_44fd_9547_4f91eb757ac4);

/// 向后兼容旧代码的常量名
pub const APP_INTER_IID: GUID = IID_IVGAPPLICATION;