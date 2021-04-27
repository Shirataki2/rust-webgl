use glow::Context as InnerContext;
#[cfg(not(target_arch = "wasm32"))]
use sdl2::{EventPump, video::{Window, GLContext}};
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::JsCast;




pub struct Context {
    inner: InnerContext,
    version: String,
    #[cfg(not(target_arch = "wasm32"))]
    window: Window,
    #[cfg(not(target_arch = "wasm32"))]
    event_loop: EventPump,
    #[cfg(not(target_arch = "wasm32"))]
    gl_context: GLContext,
}

impl Default for Context {
    fn default() -> Context {
        Context::new()
    }
}

impl Context {
    #[cfg(target_arch = "wasm32")]
    pub fn new() -> Context {
        let (inner, version) = Self::create_context();
        Context { inner, version }
    }

    #[cfg(target_arch = "wasm32")]
    fn create_context() -> (InnerContext, String) {
        let canvas = web_sys::window()
            .unwrap()
            .document()
            .unwrap()
            .get_element_by_id("canvas")
            .unwrap()
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .unwrap();
        let webgl2_context = canvas
            .get_context("webgl2")
            .unwrap()
            .unwrap()
            .dyn_into::<web_sys::WebGl2RenderingContext>()
            .unwrap();
        let gl = glow::Context::from_webgl2_context(webgl2_context);
        (gl, "#version 300 es".to_string())
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn new() -> Context {
        let (inner, version, window, event_loop, gl_context) = Self::create_context();
        Context {
            inner, version, window, event_loop, gl_context
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn create_context() -> (InnerContext, String, Window, EventPump, GLContext) {
        let sdl = sdl2::init().unwrap();
        let video = sdl.video().unwrap();
        let gl_attr = video.gl_attr();
        gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
        gl_attr.set_context_version(3, 0);
        let window = video
            .window("Hello triangle!", 1024, 769)
            .opengl()
            .resizable()
            .build()
            .unwrap();
        let gl_context = window.gl_create_context().unwrap();
        let gl =
            unsafe { glow::Context::from_loader_function(|s| video.gl_get_proc_address(s) as *const _) };
        let event_loop = sdl.event_pump().unwrap();
        (gl, "#version 410".to_string(), window, event_loop, gl_context)
    }

    pub fn get_context(&self) -> &InnerContext {
        &self.inner
    }

    pub fn get_version(&self) -> String {
        self.version.clone()
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn get_eventloop(&mut self) -> &mut EventPump {
        &mut self.event_loop
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn get_window(&self) -> &Window {
        &self.window
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn get_gl_context(&self) -> &GLContext {
        &self.gl_context
    }
}