use anyhow::Result;
use futures::channel::mpsc::{unbounded, UnboundedReceiver};
use std::{cell::RefCell, collections::HashMap, rc::Rc};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use crate::game::Direction;

pub struct KeyState {
    pressed_keys: HashMap<String, web_sys::KeyboardEvent>,
    dir: Option<Direction>,
}

impl KeyState {
    pub fn new() -> Self {
        KeyState {
            pressed_keys: HashMap::new(),
            dir: None,
        }
    }

    pub fn take_direction(&mut self) -> Option<Direction> {
        self.dir.take()
    }

    #[allow(dead_code)]
    pub fn is_pressed(&self, code: &str) -> bool {
        self.pressed_keys.contains_key(code)
    }

    pub fn set_pressed(&mut self, code: &str, event: web_sys::KeyboardEvent) {
        self.pressed_keys.insert(code.into(), event);

        match code {
            "ArrowUp" | "KeyW" => self.dir = Some(Direction::Up),
            "ArrowRight" | "KeyD" => self.dir = Some(Direction::Right),
            "ArrowDown" | "KeyS" => self.dir = Some(Direction::Down),
            "ArrowLeft" | "KeyA" => self.dir = Some(Direction::Left),
            _ => (),
        }
    }

    pub fn set_released(&mut self, code: &str) {
        self.pressed_keys.remove(code);
    }
}

#[derive(Debug)]
pub(crate) enum KeyPress {
    KeyUp(web_sys::KeyboardEvent),
    KeyDown(web_sys::KeyboardEvent),
}

pub(crate) fn prepare_input() -> Result<UnboundedReceiver<KeyPress>> {
    let (keydown_sender, keyevent_receiver) = unbounded();
    let keydown_sender = Rc::new(RefCell::new(keydown_sender));
    let keyup_sender = Rc::clone(&keydown_sender);
    let onkeydown = Closure::<dyn FnMut(web_sys::KeyboardEvent)>::new(
        move |keycode: web_sys::KeyboardEvent| {
            keydown_sender
                .borrow_mut()
                .start_send(KeyPress::KeyDown(keycode))
                .unwrap();
        },
    );

    let onkeyup = Closure::<dyn FnMut(web_sys::KeyboardEvent)>::new(
        move |keycode: web_sys::KeyboardEvent| {
            keyup_sender
                .borrow_mut()
                .start_send(KeyPress::KeyUp(keycode))
                .unwrap();
        },
    );

    let win = web_sys::window().unwrap();
    let doc = win.document().unwrap();
    let canvas = doc
        .get_element_by_id("canvas")
        .unwrap()
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .unwrap();

    canvas.focus().unwrap();

    canvas.set_onkeydown(Some(onkeydown.as_ref().unchecked_ref()));
    canvas.set_onkeyup(Some(onkeyup.as_ref().unchecked_ref()));
    onkeydown.forget();
    onkeyup.forget();

    Ok(keyevent_receiver)
}
