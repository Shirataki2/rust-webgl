use rust_webgl_bin::entrypoint::{draw, setup};
use rust_webgl_lib::application::Application;

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    let mut app = Application::default();
    setup(&app);
    'running: loop {
        let eventloop = app.get_eventloop();
        for event in eventloop.poll_iter() {
            if let sdl2::event::Event::Quit { .. } = event {
                break 'running;
            }
        }
        app.get_context();
        draw(&app);
        app.get_window().gl_swap_window();
    }
}

#[cfg(target_arch = "wasm32")]
fn main() {}
