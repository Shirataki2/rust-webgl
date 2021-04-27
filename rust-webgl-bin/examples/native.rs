use rust_webgl_bin::entrypoint::{draw, setup};
use rust_webgl_lib::context::Context;

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    let mut ctx = Context::default();
    {
        let gl = ctx.get_context();
        setup(&ctx, gl);
    }
    'running: loop {
        {
            let eventloop = ctx.get_eventloop();
            for event in eventloop.poll_iter() {
                if let sdl2::event::Event::Quit { .. } = event {
                    break 'running;
                }
            }
        }
        let gl = ctx.get_context();
        draw(&ctx, gl);
        ctx.get_window().gl_swap_window();
    }
}

#[cfg(target_arch = "wasm32")]
fn main() {}
