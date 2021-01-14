use wasm_bindgen::prelude::*;
use wasm_bindgen::{Clamped, JsCast};
use wasm_bindgen_futures::JsFuture;
use web_sys::ImageData;
use js_sys::Promise;

use std::rc::Rc;
use std::cell::RefCell;
use crate::token::memory::Memory;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(a: &str);
}

pub fn draw_canvas(
    //static_data: Rc<RefCell<Vec<u8>>>,
    static_data: &[u8],
    base_address: i32,
    canvas_w: usize,
    canvas_h: usize,
    unit_w: usize,
    unit_h: usize)
{
    const CANVAS_ID: &str = "canvas_wasm";
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id(CANVAS_ID).unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();
    let ctx = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();
    let canvas_w = canvas.width()  as usize;
    let canvas_h = canvas.height() as usize;

    //log("draw");
    //loop {
        //log("loop");
        let mut result = extract_data(&static_data, base_address, canvas_w, canvas_h, unit_w, unit_h);
        let data = ImageData::new_with_u8_clamped_array_and_sh(
            Clamped(&mut result),
            canvas.width(),
            canvas.height(),
        );
        if let Ok(data) = data {
            let _ = ctx.put_image_data(&data, 0.0, 0.0);
            //log("success");
        } else {
            log("failed");
        }
    //}
}

#[inline]
fn extract_data(
    data: &[u8],
    base_address: i32,
    canvas_w: usize,
    canvas_h: usize,
    unit_w: usize,
    unit_h: usize,
) -> Vec<u8> {
    let mut canvas = Vec::new();
    let mut address = base_address as usize;
    if data.len() < canvas_w * canvas_h {
        return canvas;
    }
    for _h in 0..canvas_h {
        for _uh in 0..unit_h { // bug
            for _w in 0..canvas_w {
                for _uw in 0..unit_w {
                    canvas.push(data[address+0]);  // R
                    canvas.push(data[address+1]);  // G
                    canvas.push(data[address+2]);  // B
                    canvas.push(255);              // A
                }
                address += 4;
            }
        }
    }
    canvas
}

