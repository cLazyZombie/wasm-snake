mod game;
mod renderer;
mod snake;

use std::{cell::RefCell, rc::Rc};

use anyhow::{anyhow, Result};
use game::Game;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

#[wasm_bindgen(start)]
pub async fn start() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();
    console_log::init_with_level(log::Level::Debug).expect("could not initialize logger");
    log::debug!("start");

    register_loop().unwrap();

    Ok(())
}

type CallbackClosure = Rc<RefCell<Option<Closure<dyn FnMut(f64)>>>>;

pub fn register_loop() -> Result<()> {
    let g: CallbackClosure = Rc::new(RefCell::new(None));
    let g_cloned = g.clone();

    let mut game = Game::new();

    let callback = Closure::<dyn FnMut(f64)>::new(move |v| {
        game.update(v);
        game.draw();

        web_sys::window()
            .unwrap()
            .request_animation_frame(g_cloned.borrow().as_ref().unwrap().as_ref().unchecked_ref())
            .expect("could not request animation frame");
    });
    *g.borrow_mut() = Some(callback);

    web_sys::window()
        .unwrap()
        .request_animation_frame(g.borrow().as_ref().unwrap().as_ref().unchecked_ref())
        .map_err(|_| anyhow!("could not request animation frame"))?;

    Ok(())
}
