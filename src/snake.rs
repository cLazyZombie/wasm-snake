use crate::{
    game::{Direction, WORLD_SIZE},
    renderer,
};

pub(crate) struct Snake {
    pub body: Vec<(i32, i32)>, // head is first index
    pub dir: Direction,
}

impl Snake {
    pub fn new() -> Self {
        Self {
            body: vec![
                (WORLD_SIZE.0 / 2, WORLD_SIZE.1 / 2),
                (WORLD_SIZE.0 / 2, WORLD_SIZE.1 / 2 + 1),
                (WORLD_SIZE.0 / 2, WORLD_SIZE.1 / 2 + 2),
            ],
            dir: Direction::Up,
        }
    }

    pub fn update(&mut self) {
        let (head_x, head_y) = self.body[0];
        let next = match self.dir {
            Direction::Up => (head_x, head_y - 1),
            Direction::Right => (head_x + 1, head_y),
            Direction::Down => (head_x, head_y + 1),
            Direction::Left => (head_x - 1, head_y),
        };
        let next = wrap_logical_pos(next);

        self.body.insert(0, next);
        self.body.pop();

        log::debug!("snake body count: {}", self.body.len());
    }

    pub fn draw(&self, context: &web_sys::CanvasRenderingContext2d) {
        for (x, y) in &self.body {
            renderer::draw_rect(context, (*x, *y), (0, 255, 0));
        }
    }
}

/// (x, y) 가 ((0,0) , (WORLD_SIZE)] 내부에 있도록 wrap 시킨다
// todo! +2 or -2 이상 벗어났을때 처리
pub fn wrap_logical_pos(pos: (i32, i32)) -> (i32, i32) {
    let x = if pos.0 < 0 {
        WORLD_SIZE.0 - 1
    } else if pos.0 >= WORLD_SIZE.0 {
        0
    } else {
        pos.0
    };

    let y = if pos.1 < 0 {
        WORLD_SIZE.1 - 1
    } else if pos.1 >= WORLD_SIZE.1 {
        0
    } else {
        pos.1
    };

    (x, y)
}
