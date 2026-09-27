//! `IVGImage` / `IVGImageTile` / `IVGImageTiles` 鈥斺€?鍍忕礌绾у浘鍍忔暟鎹?

use wincom::{ComObject, Variant};
use windows::Win32::System::Com::IDispatch;

use crate::color::IvgColor;

// =============================================================
// IvgImage
// =============================================================

pub struct IvgImage {
    disp: ComObject,
}

impl IvgImage {
    pub fn new(disp: IDispatch) -> Self {
        Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) }
    }

    pub fn from_disp(disp: IDispatch) -> Self { Self::new(disp) }

    pub fn as_variant(&self) -> Variant {
        self.disp.as_variant()
    }

    pub fn raw(&self) -> &ComObject { &self.disp }

    fn prop_i64(&self, name: &str) -> Option<i64> {
        self.disp.get_property(name).ok()?.to_i64().ok()
    }

    fn prop_bool(&self, name: &str) -> Option<bool> {
        self.disp.get_property(name).ok()?.to_bool().ok()
    }

    fn prop_dispatch(&self, name: &str) -> Option<IDispatch> {
        self.disp.get_property(name).ok()?.to_idispatch().ok()
    }

    /// `cdrImageType`
    pub fn image_type(&self) -> Option<i64> { self.prop_i64("Type") }
    pub fn width(&self) -> Option<i64> { self.prop_i64("Width") }
    pub fn height(&self) -> Option<i64> { self.prop_i64("Height") }
    pub fn read_only(&self) -> Option<bool> { self.prop_bool("ReadOnly") }

    pub fn pixel(&self, x: i32, y: i32) -> Option<IvgColor> {
        let args = vec![
            Variant::from_i64(x as i64),
            Variant::from_i64(y as i64),
        ];
        self.disp
            .invoke_method("Pixel", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgColor::new)
    }

    pub fn get_copy(&self) -> Option<IvgImage> {
        self.disp
            .invoke_method("GetCopy", vec![])
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgImage::new)
    }

    pub fn tiles(&self) -> Option<IvgImageTiles> {
        self.prop_dispatch("Tiles").map(IvgImageTiles::new)
    }

    #[allow(clippy::too_many_arguments)]
    pub fn blit(
        &self,
        dest_x: i32, dest_y: i32, dest_w: i32, dest_h: i32,
        src: &IvgImage,
        src_x: i32, src_y: i32, src_w: i32, src_h: i32,
        merge_mode: i32,
    ) -> bool {
        let args = vec![
            Variant::from_i64(dest_x as i64),
            Variant::from_i64(dest_y as i64),
            Variant::from_i64(dest_w as i64),
            Variant::from_i64(dest_h as i64),
            src.as_variant(),
            Variant::from_i64(src_x as i64),
            Variant::from_i64(src_y as i64),
            Variant::from_i64(src_w as i64),
            Variant::from_i64(src_h as i64),
            Variant::from_i64(merge_mode as i64),
        ];
        self.disp.invoke_method("Blit", args).is_ok()
    }

    pub fn fill_area(
        &self,
        x: i32, y: i32, w: i32, h: i32,
        color: &IvgColor,
    ) -> bool {
        let args = vec![
            Variant::from_i64(x as i64),
            Variant::from_i64(y as i64),
            Variant::from_i64(w as i64),
            Variant::from_i64(h as i64),
            color.as_variant(),
        ];
        self.disp.invoke_method("FillArea", args).is_ok()
    }

    pub fn flip_area(
        &self,
        x: i32, y: i32, w: i32, h: i32,
        axes: i32,
    ) -> bool {
        let args = vec![
            Variant::from_i64(x as i64),
            Variant::from_i64(y as i64),
            Variant::from_i64(w as i64),
            Variant::from_i64(h as i64),
            Variant::from_i64(axes as i64),
        ];
        self.disp.invoke_method("FlipArea", args).is_ok()
    }
}

// =============================================================
// IvgImageTiles
// =============================================================

pub struct IvgImageTiles {
    disp: ComObject,
}

impl IvgImageTiles {
    pub fn new(disp: IDispatch) -> Self {
        Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) }
    }

    fn prop_i64(&self, name: &str) -> Option<i64> {
        self.disp.get_property(name).ok()?.to_i64().ok()
    }

    fn prop_dispatch(&self, name: &str) -> Option<IDispatch> {
        self.disp.get_property(name).ok()?.to_idispatch().ok()
    }

    pub fn count(&self) -> Option<i64> { self.prop_i64("Count") }

    pub fn item(&self, index: i32) -> Option<IvgImageTile> {
        let args = vec![Variant::from_i64(index as i64)];
        self.disp
            .invoke_method("Item", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgImageTile::new)
    }

    pub fn first(&self) -> Option<IvgImageTile> {
        self.prop_dispatch("First").map(IvgImageTile::new)
    }

    pub fn last(&self) -> Option<IvgImageTile> {
        self.prop_dispatch("Last").map(IvgImageTile::new)
    }
}

// =============================================================
// IvgImageTile
// =============================================================

pub struct IvgImageTile {
    disp: ComObject,
}

impl IvgImageTile {
    pub fn new(disp: IDispatch) -> Self {
        Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) }
    }

    fn prop_i64(&self, name: &str) -> Option<i64> {
        self.disp.get_property(name).ok()?.to_i64().ok()
    }

    fn prop_bool(&self, name: &str) -> Option<bool> {
        self.disp.get_property(name).ok()?.to_bool().ok()
    }

    pub fn left(&self) -> Option<i64> { self.prop_i64("Left") }
    pub fn top(&self) -> Option<i64> { self.prop_i64("Top") }
    pub fn right(&self) -> Option<i64> { self.prop_i64("Right") }
    pub fn bottom(&self) -> Option<i64> { self.prop_i64("Bottom") }
    pub fn width(&self) -> Option<i64> { self.prop_i64("Width") }
    pub fn height(&self) -> Option<i64> { self.prop_i64("Height") }
    pub fn bytes_per_tile(&self) -> Option<i64> { self.prop_i64("BytesPerTile") }
    pub fn bytes_per_line(&self) -> Option<i64> { self.prop_i64("BytesPerLine") }
    pub fn bytes_per_pixel(&self) -> Option<i64> { self.prop_i64("BytesPerPixel") }
    pub fn read_only(&self) -> Option<bool> { self.prop_bool("ReadOnly") }

    /// 鍍忕礌鏁版嵁锛坄SAFEARRAY`锛夈€?
    pub fn pixel_data(&self) -> Option<Variant> {
        self.disp.get_property("PixelData").ok()
    }

    pub fn set_pixel_data(&self, v: Variant) -> bool {
        self.disp.set_property("PixelData", vec![v]).is_ok()
    }
}