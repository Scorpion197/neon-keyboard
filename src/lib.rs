use neon::prelude::*;
use rdev::listen;
use rdev::Event;
use rdev::EventType;
use std::sync::{Arc, Mutex};
use lazy_static::lazy_static;

lazy_static! {
    static ref KEYBOARD_STROKES: Arc<Mutex<f64>> = Arc::new(Mutex::new(0.0));
}

fn record_keyboard_strokes(mut cx: FunctionContext) -> JsResult<JsUndefined> {

    let inc_count = move |e: Event| {
        let et = e.event_type;
        match et {
            EventType::KeyPress(_) => {
                let mut count = KEYBOARD_STROKES.lock().unwrap();
                *count += 1.0;

            }
            _ => {}
        }
    };

    std::thread::spawn(move || {
        if let Err(error) = listen(inc_count) {
            println!("Error: {:?}", error)
        }
    });

    Ok(cx.undefined())
}

fn get_keyboard_strokes_count(mut cx: FunctionContext) -> JsResult<JsNumber> {
    let count = KEYBOARD_STROKES.lock().unwrap();
    Ok(cx.number(*count))
}

fn reset_keyboard_count(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let mut count = KEYBOARD_STROKES.lock().unwrap();
    *count = 0.0;
    Ok(cx.undefined())
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("record_keyboard_strokes", record_keyboard_strokes)?;
    cx.export_function("get_keyboard_strokes_count", get_keyboard_strokes_count)?;
    cx.export_function("reset_keyboard_count", reset_keyboard_count)?;
    Ok(())
}