use crate::renderer;
use crate::snake::Snake;
use wasm_bindgen::JsCast;

pub const WORLD_SIZE: (i32, i32) = (20, 20);

#[derive(Debug)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

pub struct Game {
    prev_time: Option<f64>, // 최근 update 한 시각
    elapsed_time: f64,      // 한 프레임이 지난 이후 시간
    frame_dt: f64,          // 한 프레임의 시간
    snake: Snake,
}

impl Game {
    pub fn new() -> Self {
        Self {
            prev_time: None,
            elapsed_time: 0.0,
            frame_dt: 500.0,
            snake: Snake::new(),
        }
    }

    pub fn update(&mut self, now: f64) {
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

    fn update_frame(&mut self) {
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

        // renderer::draw_rect(&context, (100.0, 100.0), (20.0, 20.0), (255, 0, 0));
    }
}
