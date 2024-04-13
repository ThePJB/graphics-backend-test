use super::*;

pub fn run_wasm<T: Game + 'static>(mut game: T) {
    let (gl, shader_version) = {
        use wasm_bindgen::JsCast;
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
        (gl, "#version 300 es")
    };
}