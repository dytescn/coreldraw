//! `IVGImage` / `IVGImageTile` / `IVGImageTiles` —— 像素级图像数据

use wincom::{ComObject, Variant};
use windows::Win32::System::Com::IDispatch;

use crate::color::IvgColor;

// =============================================================
// IvgImage
// =============================================================

/// CorelDRAW `IVGImage` 对象的后期绑定包装（图像）。
///
/// 所有属性与方法通过 `IDispatch::Invoke` 按名称调用，调用失败时一般返回 `None` 或 `false`。
pub struct IvgImage {
    disp: ComObject,
}

impl IvgImage {
    /// 由 COM `IDispatch` 构造包装对象。
    pub fn new(disp: IDispatch) -> Self {
        Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) }
    }

    /// 从 COM `IDispatch` 构造包装对象，等价于 `new`。
    pub fn from_disp(disp: IDispatch) -> Self { Self::new(disp) }

    /// 把底层 COM 对象包装成 `VARIANT`，用于传给其它 COM 调用。
    pub fn as_variant(&self) -> Variant {
        self.disp.as_variant()
    }

    /// 获取底层 `ComObject` 的引用。
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
    /// 读取宽度（`Width`）。
    pub fn width(&self) -> Option<i64> { self.prop_i64("Width") }
    /// 读取高度（`Height`）。
    pub fn height(&self) -> Option<i64> { self.prop_i64("Height") }
    /// 读取读取仅（`ReadOnly`）。
    pub fn read_only(&self) -> Option<bool> { self.prop_bool("ReadOnly") }

    /// 像素（COM 方法 `Pixel`）。
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

    /// 读取复制（`GetCopy`）。
    pub fn get_copy(&self) -> Option<IvgImage> {
        self.disp
            .invoke_method("GetCopy", vec![])
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgImage::new)
    }

    /// 读取 COM 属性 `Tiles`。
    pub fn tiles(&self) -> Option<IvgImageTiles> {
        self.prop_dispatch("Tiles").map(IvgImageTiles::new)
    }

    /// 调用 COM 方法 `Blit`。
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

    /// 填充区域（COM 方法 `FillArea`）。
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

    /// 翻转区域（COM 方法 `FlipArea`）。
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

/// CorelDRAW `IVGImage` 对象的后期绑定包装（图像平铺块）。
///
/// 所有属性与方法通过 `IDispatch::Invoke` 按名称调用，调用失败时一般返回 `None` 或 `false`。
pub struct IvgImageTiles {
    disp: ComObject,
}

impl IvgImageTiles {
    /// 由 COM `IDispatch` 构造包装对象。
    pub fn new(disp: IDispatch) -> Self {
        Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) }
    }

    fn prop_i64(&self, name: &str) -> Option<i64> {
        self.disp.get_property(name).ok()?.to_i64().ok()
    }

    fn prop_dispatch(&self, name: &str) -> Option<IDispatch> {
        self.disp.get_property(name).ok()?.to_idispatch().ok()
    }

    /// 读取返回条目数量（`Count`）。
    pub fn count(&self) -> Option<i64> { self.prop_i64("Count") }

    /// 按索引或名称取条目（COM 方法 `Item`）。
    pub fn item(&self, index: i32) -> Option<IvgImageTile> {
        let args = vec![Variant::from_i64(index as i64)];
        self.disp
            .invoke_method("Item", args)
            .ok()?
            .to_idispatch()
            .ok()
            .map(IvgImageTile::new)
    }

    /// 读取返回第一个条目（`First`）。
    pub fn first(&self) -> Option<IvgImageTile> {
        self.prop_dispatch("First").map(IvgImageTile::new)
    }

    /// 读取返回最后一个条目（`Last`）。
    pub fn last(&self) -> Option<IvgImageTile> {
        self.prop_dispatch("Last").map(IvgImageTile::new)
    }
}

// =============================================================
// IvgImageTile
// =============================================================

/// CorelDRAW `IVGImage` 对象的后期绑定包装（图像平铺）。
///
/// 所有属性与方法通过 `IDispatch::Invoke` 按名称调用，调用失败时一般返回 `None` 或 `false`。
pub struct IvgImageTile {
    disp: ComObject,
}

impl IvgImageTile {
    /// 由 COM `IDispatch` 构造包装对象。
    pub fn new(disp: IDispatch) -> Self {
        Self { disp: ComObject::from_dispatch(disp, crate::types::IID_IDISPATCH) }
    }

    fn prop_i64(&self, name: &str) -> Option<i64> {
        self.disp.get_property(name).ok()?.to_i64().ok()
    }

    fn prop_bool(&self, name: &str) -> Option<bool> {
        self.disp.get_property(name).ok()?.to_bool().ok()
    }

    /// 读取左（`Left`）。
    pub fn left(&self) -> Option<i64> { self.prop_i64("Left") }
    /// 读取顶（`Top`）。
    pub fn top(&self) -> Option<i64> { self.prop_i64("Top") }
    /// 读取右（`Right`）。
    pub fn right(&self) -> Option<i64> { self.prop_i64("Right") }
    /// 读取底（`Bottom`）。
    pub fn bottom(&self) -> Option<i64> { self.prop_i64("Bottom") }
    /// 读取宽度（`Width`）。
    pub fn width(&self) -> Option<i64> { self.prop_i64("Width") }
    /// 读取高度（`Height`）。
    pub fn height(&self) -> Option<i64> { self.prop_i64("Height") }
    /// 读取 COM 属性 `BytesPerTile`。
    pub fn bytes_per_tile(&self) -> Option<i64> { self.prop_i64("BytesPerTile") }
    /// 读取 COM 属性 `BytesPerLine`。
    pub fn bytes_per_line(&self) -> Option<i64> { self.prop_i64("BytesPerLine") }
    /// 读取 COM 属性 `BytesPerPixel`。
    pub fn bytes_per_pixel(&self) -> Option<i64> { self.prop_i64("BytesPerPixel") }
    /// 读取读取仅（`ReadOnly`）。
    pub fn read_only(&self) -> Option<bool> { self.prop_bool("ReadOnly") }

    /// 像素数据（`SAFEARRAY`）。
    pub fn pixel_data(&self) -> Option<Variant> {
        self.disp.get_property("PixelData").ok()
    }

    /// 设置像素数据（`PixelData`）。
    pub fn set_pixel_data(&self, v: Variant) -> bool {
        self.disp.set_property("PixelData", vec![v]).is_ok()
    }
}