//! 页面相关
//!
//! - [`IvgPage`] / [`IvgPages`]            : 页面 / 页面集合
//! - [`IvgPageSize`] / [`IvgPageSizes`]    : 命名页面尺寸 / 集合
//! - [`IvgSpread`] / [`IvgSpreads`]        : 跨页 / 跨页集合

pub mod page;
pub mod pages;
pub mod page_size;
pub mod spread;

pub use page::IvgPage;
pub use pages::IvgPages;
pub use page_size::{IvgPageSize, IvgPageSizes};
pub use spread::{IvgSpread, IvgSpreads};