use wasm_bindgen::JsValue;
use web_sys::CanvasRenderingContext2d;

use crate::game::WORLD_SIZE;

pub(crate) fn draw_rect(context: &CanvasRenderingContext2d, pos: (i32, i32), color: (u8, u8, u8)) {
    let color_str = format!("rgb({}, {}, {})", color.0, color.1, color.2);
    context.set_fill_style(&JsValue::from_str(&color_str));

    let world_real_size = (
        context.canvas().unwrap().width(),
        context.canvas().unwrap().height(),
    );
    let unit_size = (
        world_real_size.0 as f64 / WORLD_SIZE.0 as f64,
        world_real_size.1 as f64 / WORLD_SIZE.1 as f64,
    );
    log::debug!("unit_size: {:?}", unit_size);

    let left_top = (pos.0 as f64 * unit_size.0, pos.1 as f64 * unit_size.1);
    let right_top = (left_top.0 + unit_size.0, left_top.1);
    let left_bottom = (left_top.0, left_top.1 + unit_size.1);
    let right_bottom = (left_top.0 + unit_size.0, left_top.1 + unit_size.1);

    context.move_to(left_top.0, left_top.1);
    context.begin_path();
    context.line_to(right_top.0, right_top.1);
    context.line_to(right_bottom.0, right_bottom.1);
    context.line_to(left_bottom.0, left_bottom.1);
    context.line_to(left_top.0, left_top.1);
    context.close_path();
    context.stroke();
    context.fill();
}

pub(crate) fn clear(context: &CanvasRenderingContext2d) {
    let world_real_size = (
        context.canvas().unwrap().width(),
        context.canvas().unwrap().height(),
    );
    context.clear_rect(0.0, 0.0, world_real_size.0 as f64, world_real_size.1 as f64);
}
