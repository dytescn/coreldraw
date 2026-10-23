//! 把 SDK 对象转成 JSON 描述，供各接口复用。

use cdrsdk::prelude::*;
use serde_json::{json, Value};

pub fn color_info(color: &IvgColor) -> Value {
    json!({
        "type": color.color_type(),
        "rgb": {
            "r": color.rgb_red(),
            "g": color.rgb_green(),
            "b": color.rgb_blue(),
        },
        "hex": color.hex_value(),
        "string": color.to_color_string(),
        "is_white": color.is_white(),
        "is_spot": color.is_spot(),
    })
}

pub fn fill_info(fill: &IvgFill) -> Value {
    json!({
        "type": fill.fill_type(),
        "uniform": fill.uniform_color().map(|c| color_info(&c)),
        "string": fill.to_fill_string(),
    })
}

pub fn outline_info(outline: &IvgOutline) -> Value {
    json!({
        "type": outline.outline_type(),
        "width": outline.width(),
        "size": outline.size(),
        "color": outline.color().map(|c| color_info(&c)),
    })
}

pub fn text_info(text: &IvgText) -> Value {
    let story = text.story();
    let content = story.as_ref().and_then(|s| s.text());
    json!({
        "type": text.text_type(),
        "is_artistic": text.is_artistic_text(),
        "overflow": text.overflow(),
        "frames_in_link": text.frames_in_link(),
        "content": content,
        "length": story.as_ref().and_then(|s| s.length()),
    })
}

pub fn shape_info(shape: &IvgShape) -> Value {
    let text = if shape.shape_type() == Some(6) {
        shape.text().map(|t| text_info(&t))
    } else {
        None
    };

    json!({
        "name": shape.name(),
        "type": shape.shape_type(),
        "static_id": shape.static_id(),
        "selected": shape.selected(),
        "locked": shape.locked(),
        "visible": shape.visible(),
        "position": {
            "x": shape.position_x(),
            "y": shape.position_y(),
        },
        "size": {
            "width": shape.size_width(),
            "height": shape.size_height(),
        },
        "bounds": {
            "left": shape.left_x(),
            "top": shape.top_y(),
            "right": shape.right_x(),
            "bottom": shape.bottom_y(),
        },
        "rotation": {
            "angle": shape.rotation_angle(),
            "center_x": shape.rotation_center_x(),
            "center_y": shape.rotation_center_y(),
        },
        "fill": shape.fill().map(|f| fill_info(&f)),
        "outline": shape.outline().map(|o| outline_info(&o)),
        "text": text,
    })
}

pub fn shapes_list(shapes: &IvgShapes) -> Value {
    let items: Vec<Value> = shapes.all().iter().map(shape_info).collect();
    json!({
        "count": items.len(),
        "items": items,
    })
}

pub fn page_info(page: &IvgPage) -> Value {
    json!({
        "name": page.name(),
        "index": page.index(),
        "paper": page.paper(),
        "size": {
            "width": page.size_width(),
            "height": page.size_height(),
        },
        "resolution": page.resolution(),
        "orientation": page.orientation(),
        "bleed": page.bleed(),
        "background": page.background(),
        "bounds": {
            "left": page.left_x(),
            "top": page.top_y(),
            "right": page.right_x(),
            "bottom": page.bottom_y(),
        },
        "center": {
            "x": page.center_x(),
            "y": page.center_y(),
        },
        "layers": page.layers().and_then(|l| l.count()),
        "shapes": page.shapes().and_then(|s| s.count()),
    })
}

pub fn pages_list(pages: &IvgPages) -> Value {
    let items: Vec<Value> = pages.all().iter().map(page_info).collect();
    json!({
        "count": items.len(),
        "items": items,
    })
}

pub fn layer_info(layer: &IvgLayer) -> Value {
    json!({
        "name": layer.name(),
        "index": layer.index(),
        "absolute_index": layer.absolute_index(),
        "visible": layer.visible(),
        "printable": layer.printable(),
        "editable": layer.editable(),
        "master": layer.master(),
        "is_guides_layer": layer.is_guides_layer(),
        "is_desktop_layer": layer.is_desktop_layer(),
        "is_grid_layer": layer.is_grid_layer(),
        "shapes": layer.shapes().and_then(|s| s.count()),
    })
}

pub fn layers_list(layers: &IvgLayers) -> Value {
    let items: Vec<Value> = layers.all().iter().map(layer_info).collect();
    json!({
        "count": items.len(),
        "items": items,
    })
}

pub fn documents_list(docs: &IvgDocuments) -> Value {
    let items: Vec<Value> = docs
        .all()
        .iter()
        .enumerate()
        .map(|(i, d)| {
            json!({
                "index": i as i64 + 1,
                "name": d.name(),
                "file_name": d.file_name(),
                "dirty": d.dirty(),
                "saved": d.saved(),
            })
        })
        .collect();
    json!({
        "count": docs.count(),
        "items": items,
    })
}

pub fn doc_info(doc: &IvgDocument) -> Value {
    json!({
        "name": doc.name(),
        "file_name": doc.file_name(),
        "full_file_name": doc.full_file_name(),
        "file_path": doc.file_path(),
        "dirty": doc.dirty(),
        "saved": doc.saved(),
        "read_only": doc.read_only(),
        "version": doc.version(),
        "unit": doc.unit(),
        "reference_point": doc.reference_point(),
        "world_scale": doc.world_scale(),
        "pages": doc.pages().and_then(|p| p.count()),
        "shapes": doc.active_page().and_then(|p| p.shapes()).and_then(|s| s.count()),
        "active_page": doc.active_page().and_then(|p| p.name()),
        "active_layer": doc.active_layer().and_then(|l| l.name()),
    })
}
