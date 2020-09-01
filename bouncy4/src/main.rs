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
        // println!("{}, {}", self.vertical_position, self.horizontal_position);

        if self.horizontal_position == 0 {
            self.horizontal_direction = HorizontalDirection::Right
        }

        if self.horizontal_position == frame.width - 1 {
            self.horizontal_direction = HorizontalDirection::Left
        }

        if self.vertical_position == 0 {
            self.vertical_direction = VerticalDirection::Down
        }

        if self.vertical_position == frame.height - 1 {
            self.vertical_direction = VerticalDirection::Up
        }
    }

    fn move_(&mut self) {
        match self.vertical_direction {
            VerticalDirection::Down => self.vertical_position += 1,
            VerticalDirection::Up => self.vertical_position -= 1,
        }

        match self.horizontal_direction {
            HorizontalDirection::Left => self.horizontal_position -= 1,
            HorizontalDirection::Right => self.horizontal_position += 1,
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
    fn new(frame: Frame) -> Game {
        Game {
            frame,
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
    fn fmt(&self, fmt: &mut Formatter<'_>) -> std::fmt::Result {
        let horizontal_border = |fmt: &mut Formatter| {
            write!(fmt, "+")?;
            for _ in 0..self.frame.width {
                write!(fmt, "-")?;
            }
            write!(fmt, "+\n")
        };

        let ball_position = (self.ball.horizontal_position, self.ball.vertical_position);
        horizontal_border(fmt)?;
        for row in 0..self.frame.height {
            write!(fmt, "|")?;
            for col in 0..self.frame.width {
                let c = match (row, col) {
                    (row, col) if row == ball_position.1 && col == ball_position.0 => "o",
                    (_, _) => " ",
                };
                write!(fmt, "{}", c)?;
            }
            write!(fmt, "|\n")?;
        }
        horizontal_border(fmt)
    }
}

fn main() {
    let window = pancurses::initscr();
    let (max_y, max_x) = window.get_max_yx();
    let frame = Frame{
        width: (max_x - 4) as u32,
        height: (max_y - 4) as u32,
    };

    let mut game = Game::new(frame);
    let sleep_duration = std::time::Duration::from_millis(33);
    loop {
        window.clear();
        window.printw(game.to_string());
        window.refresh();
        game.step();
        std::thread::sleep(sleep_duration);
    }
}
