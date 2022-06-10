
pub enum TileEdges {
    TOP,
    BOTTOM,
    LEFT,
    RIGHT
}

pub struct Point {
    pub x: u16,
    pub y: u16
}

pub enum EdgeDirection {
    HORIZONTAL,
    VERTICAL
}

pub struct Edge {

    start: Point,
    end: Point,

    direction: EdgeDirection

}

impl Edge {
    pub fn x(&self) -> u16 {
        match self.direction {
            EdgeDirection::HORIZONTAL => (start.x + end.x) / 2,
            EdgeDirection::VERTICAL => start.x
        }
    }

    pub fn y(&self) -> u16 {
        match self.direction {
            EdgeDirection::VERTICAL => (start.y + end.y) / 2,
            EdgeDirection::HORIZONTAL => start.y
        }
    }
}

pub struct Tile {

    pub top: &Edge,
    pub bottom: &Edge,
    pub left: &Edge,
    pub right: &Edge

}

impl Tile {
    pub fn width(&self) -> u16 {
        self.right.x - self.left.x
    }

    pub fn height(&self) -> u16 {
        self.bottom.y - self.top.y
    }

    pub fn middle(&self) -> Point {
        //This works because horizontal lines return the midpoint as their 'x'
        //Vertical lines return midpoint for 'y'
        Point {
            x: top.x,
            y: left.y
        }
    }
}
