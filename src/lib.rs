#![cfg(windows)]
#![allow(non_camel_case_types, non_snake_case, unused, non_upper_case_globals)]
#![doc = include_str!("../README.md")]

pub mod types;
pub mod prelude;
pub mod enums;
pub mod app;
pub mod document;
pub mod page;
pub mod layer;
pub mod shape;
pub mod curve;
pub mod geometry;
pub mod color;
pub mod fill;
pub mod outline;
pub mod text;
pub mod effect;
pub mod style;
pub mod view;
pub mod tree;
pub mod symbol;
pub mod structs;


pub mod print;
pub mod import_export;
pub mod ui;
pub mod misc;

pub use prelude::*;