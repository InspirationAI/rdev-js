#![deny(clippy::all)]

mod keys;

#[macro_use]
extern crate napi_derive;

use std::{thread, time};
use std::time::SystemTime;
use napi::{Error, JsFunction};
use napi::threadsafe_function::{ErrorStrategy, ThreadSafeCallContext, ThreadsafeFunction, ThreadsafeFunctionCallMode};
use rdev::{Button, Event, EventType, Key, listen, simulate};
use rdev::EventType::{ButtonPress, ButtonRelease, KeyPress, KeyRelease, MouseMove, Wheel};
use crate::keys::{index, scan_code};

const ZERO: SystemTime = SystemTime::UNIX_EPOCH;

pub enum RdevEventType {
    MouseUp,
    MouseDown,
    MouseMove,
    KeyUp,
    KeyDown,
    Wheel,
}

#[napi(object)]
pub struct RdevEventValue {
    pub x: Option<f64>,
    pub y: Option<f64>,
    pub delta_x: Option<i64>,
    pub delta_y: Option<i64>,
    pub key: Option<String>,
    pub button: Option<String>,
}

#[napi(object)]
pub struct RdevEvent {
    pub _type: String,
    pub code: Option<String>,
    pub value: RdevEventValue,
    pub time: u32,
}

fn pre_handle_event(event: Event) -> Option<RdevEvent> {
    let time = event.time.duration_since(ZERO).unwrap().as_millis();

    match event.event_type {
        MouseMove { x, y } => {
            Some(RdevEvent {
                _type: "mousemove".to_string(),
                code: None,
                value: RdevEventValue {
                    x: Some(x),
                    y: Some(y),
                    delta_x: None,
                    delta_y: None,
                    key: None,
                    button: None,
                },
                time: time as u32,
            })
        }

        Wheel { delta_x, delta_y } => {
            Some(RdevEvent {
                _type: "wheel".to_string(),
                code: None,
                value: RdevEventValue {
                    x: None,
                    y: None,
                    delta_x: Some(delta_x),
                    delta_y: Some(delta_y),
                    key: None,
                    button: None,
                },
                time: time as u32,
            })
        }

        KeyPress(_key) => {
            Some(RdevEvent {
                _type: "keydown".to_string(),
                code: Some(scan_code(_key).to_string()),
                value: RdevEventValue {
                    x: None,
                    y: None,
                    delta_x: None,
                    delta_y: None,
                    key: Some(format!("{:?}", _key)),
                    button: None,
                },
                time: time as u32,
            })
        }

        KeyRelease(_key) => {
            Some(RdevEvent {
                _type: "keyup".to_string(),
                code: Some(scan_code(_key).to_string()),
                value: RdevEventValue {
                    x: None,
                    y: None,
                    delta_x: None,
                    delta_y: None,
                    key: Some(format!("{:?}", _key)),
                    button: None,
                },
                time: time as u32,
            })
        }

        ButtonPress(_button) => {
            Some(RdevEvent {
                _type: "mousedown".to_string(),
                code: Some(index(_button).to_string()),
                value: RdevEventValue {
                    x: None,
                    y: None,
                    delta_x: None,
                    delta_y: None,
                    key: None,
                    button: Some(format!("{:?}", _button)),
                },
                time: time as u32,
            })
        }

        ButtonRelease(_button) => {
            Some(RdevEvent {
                _type: "mouseup".to_string(),
                code: Some(index(_button).to_string()),
                value: RdevEventValue {
                    x: None,
                    y: None,
                    delta_x: None,
                    delta_y: None,
                    key: None,
                    button: Some(format!("{:?}", _button)),
                },
                time: time as u32,
            })
        }
    }
}

#[napi(js_name = "rdev", ts_args_type = "callback: (err: null | Error, event: RdevEvent) => void")]
fn rdev_hook(callback: JsFunction, is_thread_safe: Option<bool>) -> Result<(), Error> {
    let ts_function: ThreadsafeFunction<Option<RdevEvent>, ErrorStrategy::CalleeHandled> = callback
        .create_threadsafe_function(0, |ctx: ThreadSafeCallContext<Option<RdevEvent>>| {
            let value = ctx.value;
            Ok(vec![value])
            // ctx.env.create_object().map(|v| vec![v])
        })?;

    let curry_event = move |event: Event| {
        let action = pre_handle_event(event);
        // println!("Rust Log: {:?}", action);
        ts_function.call(Ok(action), ThreadsafeFunctionCallMode::NonBlocking);
    };

    if is_thread_safe.unwrap_or(true) {
        use std::thread;
        thread::spawn(move || {
            if let Err(error) = listen(curry_event) {
                println!("Error: {:?}", error)
            }
        });
    } else if let Err(error) = listen(curry_event) {
        println!("Error: {:?}", error)
    }

    Ok(())
}

fn send(event_type: &EventType) {
    let delay = time::Duration::from_millis(20);
    match simulate(event_type) {
        Ok(()) => (),
        Err(_simulate_error) => {
            println!("We could not send {:?}", event_type);
        }
    }
    // Let ths OS catchup (at least MacOS)
    thread::sleep(delay);
}

#[napi]
pub fn test_press_s_key() -> Result<(), Error> {
    send(&KeyPress(Key::KeyS));

    Ok(())
}

#[napi]
pub fn test_release_s_key() -> Result<(), Error> {
    send(&KeyRelease(Key::KeyS));

    Ok(())
}

#[napi]
pub fn test_move_mouse(x: f64, y: f64) -> Result<(), Error> {
    send(&MouseMove { x, y });

    Ok(())
}

#[napi]
pub fn test_left_mouse_button() -> Result<(), Error> {
    send(&ButtonPress(Button::Left));

    Ok(())
}

#[napi]
pub fn test_release_left_mouse_button() -> Result<(), Error> {
    send(&ButtonRelease(Button::Left));

    Ok(())
}

#[napi]
pub fn test_right_mouse_button() -> Result<(), Error> {
    send(&ButtonPress(Button::Right));

    Ok(())
}

#[napi]
pub fn test_release_right_mouse_button() -> Result<(), Error> {
    send(&ButtonRelease(Button::Right));

    Ok(())
}

#[napi]
pub fn test_wheel(delta_x: i64, delta_y: i64) -> Result<(), Error> {
    send(&Wheel { delta_x, delta_y });

    Ok(())
}