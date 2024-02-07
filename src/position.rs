type Coord = u16;
pub struct Position {
    x: Coord,
    y: Coord,
}

impl Position {
    pub fn new(x: Coord, y: Coord) -> Self {
        Self { x, y }
    }
    pub fn go_right(&mut self) {
        self.x += 1;
    }
    pub fn go_left(&mut self) {
        self.x = 1.max(self.x - 1);
    }
    pub fn go_down(&mut self) {
        self.y += 1;
    }
    pub fn go_up(&mut self) {
        self.y = 1.max(self.y - 1);
    }
    pub fn x(&self) -> Coord {
        self.x
    }
    pub fn y(&self) -> Coord {
        self.y
    }
}
