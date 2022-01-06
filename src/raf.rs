use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast; // Appears to enable casting (e.g. for Element -> HTMLCanvasElement)

// Adapted from https://github.com/rustwasm/gloo/blob/master/crates/render/src/lib.rs
pub struct AnimationFrame {
    render_id: i32,
    _closure: Closure<dyn Fn(JsValue)>,
    callback_wrapper: Rc<RefCell<Option<CallbackWrapper>>>,
}

impl Drop for AnimationFrame {
    fn drop(&mut self) {
        if self.callback_wrapper.borrow_mut().is_some() {
            web_sys::window()
                .unwrap()
                .cancel_animation_frame(self.render_id)
                .unwrap();
        }
    }
}

struct CallbackWrapper(Box<dyn FnOnce(f64) + 'static>);

pub fn request_animation_frame<F>(callback_once: F) -> AnimationFrame
where
    F: FnOnce(f64) + 'static,
{
    let callback_wrapper = Rc::new(RefCell::new(Some(CallbackWrapper(Box::new(callback_once)))));
    let callback: Closure<dyn Fn(JsValue)> = {
        let callback_wrapper = Rc::clone(&callback_wrapper);
        Closure::wrap(Box::new(move |v: JsValue| {
            let time: f64 = v.as_f64().unwrap_or(0.0);
            let callback = callback_wrapper.borrow_mut().take().unwrap().0;
            callback(time)
        }))
    };

    let render_id = web_sys::window()
        .unwrap()
        .request_animation_frame(callback.as_ref().unchecked_ref())
        .unwrap();

    AnimationFrame {
        render_id,
        _closure: callback,
        callback_wrapper,
    }
}
