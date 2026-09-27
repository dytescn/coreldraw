//! 效果（Effect）相关
//!
//! - [`IvgEffect`]        : 效果对象（`IVGEffect`）—— 通用入口
//! - [`IvgEffects`]       : 效果集合（`IVGEffects`）
//! - [`IvgCustomEffect`]  : 自定义效果（`IVGCustomEffect`）
//!
//! 具体子效果：
//! - [`IvgEffectBlend`]           : 调和
//! - [`IvgEffectContour`]         : 轮廓
//! - [`IvgEffectControlPath`]     : 控制路径
//! - [`IvgEffectDistortion`]      : 变形（含 push-pull / zipper / twister / custom）
//! - [`IvgEffectDropShadow`]      : 阴影
//! - [`IvgEffectEnvelope`]        : 封套
//! - [`IvgEffectExtrude`]         : 立体化
//! - [`IvgEffectLens`]            : 透镜
//! - [`IvgEffectPerspective`]     : 透视
//! - [`IvgEffectTextOnPath`]      : 文本适配路径

pub mod effect;
pub mod effects;
pub mod blend;
pub mod contour;
pub mod control_path;
pub mod distortion;
pub mod drop_shadow;
pub mod envelope;
pub mod extrude;
pub mod lens;
pub mod perspective;
pub mod text_on_path;

pub use effect::{IvgEffect, IvgCustomEffect};
pub use effects::IvgEffects;
pub use blend::IvgEffectBlend;
pub use contour::IvgEffectContour;
pub use control_path::IvgEffectControlPath;
pub use distortion::{
    IvgEffectDistortion,
    IvgEffectPushPullDistortion,
    IvgEffectZipperDistortion,
    IvgEffectTwisterDistortion,
    IvgEffectCustomDistortion,
};
pub use drop_shadow::IvgEffectDropShadow;
pub use envelope::IvgEffectEnvelope;
pub use extrude::{IvgEffectExtrude, IvgExtrudeVanishingPoint};
pub use lens::IvgEffectLens;
pub use perspective::IvgEffectPerspective;
pub use text_on_path::IvgEffectTextOnPath;