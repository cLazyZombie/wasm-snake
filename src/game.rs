use crate::{input::KeyState, renderer};
use crate::{
    input::{self, KeyPress},
    snake::Snake,
};
use futures::channel::mpsc::UnboundedReceiver;
use wasm_bindgen::JsCast;

pub const WORLD_SIZE: (i32, i32) = (20, 20);

#[derive(Debug, Copy, Clone)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    pub fn is_opposite(&self, other: Direction) -> bool {
        matches!(
            (*self, other),
            (Direction::Up, Direction::Down)
                | (Direction::Right, Direction::Left)
                | (Direction::Down, Direction::Up)
                | (Direction::Left, Direction::Right)
        )
    }
}

pub struct Game {
    prev_time: Option<f64>, // 최근 update 한 시각
    elapsed_time: f64,      // 한 프레임이 지난 이후 시간
    frame_dt: f64,          // 한 프레임의 시간
    key_state: KeyState,
    key_input: UnboundedReceiver<KeyPress>,
    snake: Snake,
}

impl Game {
    pub fn new() -> Self {
        let key_state = KeyState::new();
        let key_input = input::prepare_input().unwrap();

        Self {
            prev_time: None,
            elapsed_time: 0.0,
            frame_dt: 300.0,
            key_state,
            key_input,
            snake: Snake::new(),
        }
    }

    pub fn update(&mut self, now: f64) {
        self.process_input();

        // update time
        let dt = if let Some(prev_time) = self.prev_time {
            now - prev_time
        } else {
            0.0
        };
        self.prev_time = Some(now);
        self.elapsed_time += dt;

        // update frame
        while self.elapsed_time > self.frame_dt {
            self.elapsed_time -= self.frame_dt;
            self.update_frame();
        }
    }

    fn process_input(&mut self) {
        loop {
            match self.key_input.try_next() {
                Ok(None) => break,
                Err(_err) => break,
                Ok(Some(key_press)) => match dbg!(key_press) {
                    KeyPress::KeyDown(evt) => self.key_state.set_pressed(&evt.code(), evt),
                    KeyPress::KeyUp(evt) => self.key_state.set_released(&evt.code()),
                },
            }
        }
    }

    fn update_frame(&mut self) {
        if let Some(dir) = self.key_state.take_direction() {
            self.snake.set_direction(dir);
        }
        self.snake.update();
    }

    pub fn draw(&self) {
        let win = web_sys::window().unwrap();
        let doc = win.document().unwrap();
        let canvas = doc
            .get_element_by_id("canvas")
            .unwrap()
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .unwrap();
        let (w, h) = (canvas.width(), canvas.height());
        log::debug!("canvas size: {:?}", (w, h));

        let context = canvas
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<web_sys::CanvasRenderingContext2d>()
            .unwrap();

        renderer::clear(&context);

        self.snake.draw(&context);
    }
}
