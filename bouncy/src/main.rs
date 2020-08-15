use std::fmt::{Display, Formatter};

enum VerticalDirection {
    Up,
    Down,
}

enum HorizontalDirection {
    Left,
    Right,
}

struct Ball {
    vertical_position: u32,
    horizontal_position: u32,
    vertical_direction: VerticalDirection,
    horizontal_direction: HorizontalDirection,
}

impl Ball {
    fn bounce(&mut self, frame: &Frame) {
        if self.horizontal_position == 0 {
            self.horizontal_direction = HorizontalDirection::Right
        }

        if self.horizontal_position == frame.width - 1 {
            self.horizontal_direction = HorizontalDirection::Left
        }

        if self.vertical_position == 0 {
            self.vertical_direction = VerticalDirection::Up
        }

        if self.vertical_position == frame.height - 1 {
            self.vertical_direction = VerticalDirection::Down
        }
    }

    fn move_(&mut self) {
        match self.vertical_direction {
            VerticalDirection::Down => self.vertical_position -= 1,
            VerticalDirection::Up => self.vertical_position += 1,
        }

        match self.horizontal_direction {
            HorizontalDirection::Left => self.vertical_position -= 1,
            HorizontalDirection::Right => self.vertical_position += 1,
        }
    }
}

struct Frame {
    width: u32,
    height: u32,
}

struct Game {
    frame: Frame,
    ball: Ball,
}

impl Game {
    fn new() -> Game {
        Game {
            frame: Frame {
                width: 60,
                height: 30,
            },
            ball: Ball {
                vertical_position: 2,
                horizontal_position: 4,
                vertical_direction: VerticalDirection::Up,
                horizontal_direction: HorizontalDirection::Left,
            },
        }
    }

    fn step(&mut self) {
        self.ball.bounce(&self.frame);
        self.ball.move_()
    }
}

impl Display for Game {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        unimplemented!()
    }
}

fn main() {
    println!("{}", Game::new());
}
