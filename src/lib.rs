mod lexer;
mod parser;
mod token;
mod utils;
//#[cfg(target_arch = "wasm32")]
mod wasm;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlTextAreaElement;

use token::Tokens;
use token::memory::Memory;
use lexer::tokenize;
use parser::parse;

//#[cfg(target_arch = "wasm32")]
use wasm::draw_canvas;

use std::rc::Rc;
use std::cell::RefCell;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(a: &str);
}

//#[wasm_bindgen]
//pub struct Wasm {
//    memory: Rc<RefCell<Memory>>,
//}
//
//#[wasm_bindgen]
//impl Wasm {
//    pub fn static_data(&self) -> *const u8 {
//        self.memory.borrow().static_data().as_ptr()
//    }
//}

//static mut STATIC_DATA_MEMORY: Option<Rc<RefCell<Vec<u8>>>> = None;
//fn get_static_data() -> Rc<RefCell<Vec<u8>>> {
//    unsafe { Rc::clone(&STATIC_DATA_MEMORY.as_ref().unwrap()) }
//}

#[wasm_bindgen]
pub fn wasm_run() {
    let mut tokens: Tokens = Tokens::new();
    let mut memory = Memory::default();
    let mut codes: Vec<(String, String)> = Vec::new();
    //let memory = Rc::new(RefCell::new(Memory::default()));
    //let wasm = Wasm {
    //    memory: Rc::clone(&memory),
    //};

    utils::set_panic_hook();

    //let reader = FileReader::new().unwrap();
    //let src_file = document
    //    .get_element_by_id("src_file")
    //    .unwrap()
    //    .dyn_into::<HtmlInputElement>()
    //    .map_err(|_|())
    //    .unwrap();
    let src = document()
        .get_element_by_id("src")
        .unwrap()
        .dyn_into::<HtmlTextAreaElement>()
        .map_err(|_|())
        .unwrap();
    codes.push(("textarea".to_string(), src.value()));

    //if let Some(files) = src_file.files() {
    //    if files.length() == 0 {
    //        log("error: required file");
    //        return;
    //    }
    //    for idx in 0..files.length() {
    //        let file = files.get(idx).unwrap();
    //        reader.read_as_text(&file).unwrap();
    //        file.text().then(&Closure::wrap(Box::new(|js_value: JsValue| {
    //            log(&js_value.as_string().unwrap());
    //            //codes.push((file.name(), js_value.as_string().unwrap()));
    //        })));
    //        //if let Err(js_value) = reader.read_as_text(&file) {
    //        //match reader.result() {
    //        //    Ok (js_value) => codes.push((file.name(), js_value.as_string().unwrap())),
    //        //    Err(js_value) => codes.push((file.name(), js_value.as_string().unwrap())),
    //        //}
    //        //if let Ok(js_value) = reader.result() {
    //        //    log("aaa");
    //        //    codes.push((file.name(), js_value.as_string().unwrap()));
    //        //    log("bbb");
    //        //} else {
    //        //    log("ccc");
    //        //    return;
    //        //}
    //    }
    //} else {
    //    log("error: required file");
    //    return;
    //}

    // Join files  =>  Everyone global
    log("tokenize");
    for (filename, code) in codes {
        let mut number_of_lines: u32 = 1;
        tokens.add_file(&filename);
        let lines = code.lines();
        for buf in lines {
            if let Err(e) = tokenize(number_of_lines, 0, &buf, &mut tokens) {
                eprintln!("{}:{}: {}", filename, number_of_lines, e);
            }
            number_of_lines += 1;
        }
    }

    //let static_data_clone = Rc::clone(&memory.static_data);
    //spawn_local(async move {
    //    log("draw");
    //    draw_canvas(static_data_clone, 0, 64, 64, 1, 1);
    //});

    // Display Bitmap
    //let f = Rc::new(RefCell::new(None));
    //let g = f.clone();
    ////let static_data_clone = Rc::clone(&memory.static_data);
    //unsafe { STATIC_DATA_MEMORY = Some(Rc::clone(&memory.static_data)); }
    //*g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
    //    log("draw");
    //    draw_canvas(get_static_data(), 0, 64, 64, 1, 1);
    //    request_animation_frame(f.borrow().as_ref().unwrap());
    //}) as Box<dyn FnMut()>));

    // Execute
    //spawn_local(async move {
        log("parse");
        if let Err(e) = parse(&mut tokens, &mut memory) {
            eprintln!("{}", e);
        }
    //});

    // Display
    //spawn_local(async move {
        //request_animation_frame(g.borrow().as_ref().unwrap());
    //});
    draw_canvas(&memory.static_data(), 0, 64, 64, 1, 1);

    log("end");
}

fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    window()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK");
}

fn window() -> web_sys::Window {
    web_sys::window().expect("no global `window` exists")
}

fn document() -> web_sys::Document {
    window()
        .document()
        .expect("should have a document on window")
}

