//! 文件过滤器的后缀名映射（Rust 端辅助）

use crate::enums::filter::cdrFilter;

/// 常见文件过滤器的默认后缀（不含点）。
pub fn extension_for(filter: cdrFilter) -> &'static str {
    use cdrFilter::*;
    match filter {
        BMP   => "bmp",
        PCX   => "pcx",
        TGA   => "tga",
        TIFF  => "tif",
        GIF   => "gif",
        JPEG  => "jpg",
        PNG   => "png",
        PSD   => "psd",
        CDR   => "cdr",
        CDX   => "cdx",
        CMX   => "cmx",
        SVG   => "svg",
        PDF   => "pdf",
        AI    => "ai",
        EPS   => "eps",
        WMF   => "wmf",
        EMF   => "emf",
        DXF   => "dxf",
        DWG   => "dwg",
        AutoSense => "",
    }
}

/// 根据后缀名猜测过滤器（不区分大小写）。
pub fn filter_from_extension(ext: &str) -> Option<cdrFilter> {
    use cdrFilter::*;
    match ext.trim_start_matches('.').to_ascii_lowercase().as_str() {
        "bmp"        => Some(BMP),
        "pcx"        => Some(PCX),
        "tga"        => Some(TGA),
        "tif" | "tiff" => Some(TIFF),
        "gif"        => Some(GIF),
        "jpg" | "jpeg" => Some(JPEG),
        "png"        => Some(PNG),
        "psd"        => Some(PSD),
        "cdr"        => Some(CDR),
        "cdx"        => Some(CDX),
        "cmx"        => Some(CMX),
        "svg"        => Some(SVG),
        "pdf"        => Some(PDF),
        "ai"         => Some(AI),
        "eps"        => Some(EPS),
        "wmf"        => Some(WMF),
        "emf"        => Some(EMF),
        "dxf"        => Some(DXF),
        "dwg"        => Some(DWG),
        _            => None,
    }
}