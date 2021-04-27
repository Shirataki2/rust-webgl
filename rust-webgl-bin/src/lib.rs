pub mod entrypoint;
#[cfg(target_arch = "wasm32")]
use crate::entrypoint::{draw as inner_draw, setup as inner_setup};
#[cfg(target_arch = "wasm32")]
use rust_webgl_lib::context::Context;
#[cfg(target_arch = "wasm32")]
use std::{cell::RefCell, rc::Rc};
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::{prelude::*, JsCast};

#[cfg(target_arch = "wasm32")]
fn window() -> web_sys::Window {
    web_sys::window().expect("no global `window` exists")
}

#[cfg(target_arch = "wasm32")]
fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    window()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK");
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn run() -> Result<(), JsValue> {
    let ctx = Context::default();
    {
        let gl = ctx.get_context();
        inner_setup(&ctx, gl);
    }

    let f = Rc::new(RefCell::new(None));
    let g = f.clone();

    *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        let gl = ctx.get_context();
        inner_draw(&ctx, gl);

        request_animation_frame(f.borrow().as_ref().unwrap());
    }) as Box<dyn FnMut()>));

    request_animation_frame(g.borrow().as_ref().unwrap());
    Ok(())
}
