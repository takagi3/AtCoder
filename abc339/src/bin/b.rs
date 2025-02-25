use proconio::input;

#[derive(Debug, Clone, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn turn_left(&self) -> Self {
        match self {
            Direction::Up => Direction::Left,
            Direction::Left => Direction::Down,
            Direction::Down => Direction::Right,
            Direction::Right => Direction::Up,
        }
    }

    fn turn_right(&self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }
}

struct Grid {
    height: usize,
    width: usize,
    colors: Vec<Vec<u8>>,
    point_h: usize,
    point_w: usize,
    direction: Direction,
}

impl Grid {
    fn new(height: usize, width: usize) -> Self {
        Self {
            height,
            width,
            colors: vec![vec![0; width]; height],
            point_h: 0,
            point_w: 0,
            direction: Direction::Up,
        }
    }

    fn move_point(&mut self) {
        let (h, w) = (self.point_h, self.point_w);
        let cell_color = &mut self.colors[h][w];
        self.direction = if *cell_color == 0 {
            *cell_color = 1;
            self.direction.turn_right()
        } else {
            *cell_color = 0;
            self.direction.turn_left()
        };
        self.update_position();
    }

    fn update_position(&mut self) {
        match self.direction {
            Direction::Up => self.point_h = (self.point_h + self.height - 1) % self.height,
            Direction::Down => self.point_h = (self.point_h + 1) % self.height,
            Direction::Left => self.point_w = (self.point_w + self.width - 1) % self.width,
            Direction::Right => self.point_w = (self.point_w + 1) % self.width,
        }
    }

    fn print_grid(&self) {
        for row in &self.colors {
            for &cell in row {
                print!("{}", if cell == 0 { '.' } else { '#' });
            }
            println!();
        }
    }
}

fn main() {
    input! {
        h: usize,
        w: usize,
        n: usize,
    }

    let mut grid = Grid::new(h, w);
    for _ in 0..n {
        grid.move_point();
    }

    grid.print_grid();
}
