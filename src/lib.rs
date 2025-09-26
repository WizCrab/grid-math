#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Cell {
    x: u8,
    y: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Grid {
    start: Cell,
    end: Cell,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Cells {
    grid: Grid,
    current: Cell,
    consumed: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Rows {
    grid: Grid,
    current: Cells,
    consumed: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Columns {
    grid: Grid,
    current: Cells,
    consumed: bool,
}

impl Cell {
    pub fn new(x: u8, y: u8) -> Self {
        Self { x, y }
    }

    pub fn within(self, grid: Grid) -> bool {
        (grid.start.x..=grid.end.x).contains(&self.x)
            && (grid.start.y..=grid.end.y).contains(&self.y)
    }

    pub fn x(self) -> u8 {
        self.x
    }

    pub fn y(self) -> u8 {
        self.y
    }

    pub fn width(self, grid: Grid) -> u8 {
        if !self.within(grid) {
            panic!("point is not within given grid!")
        }
        self.x - grid.start.x
    }

    pub fn width_gap(self, grid: Grid) -> u8 {
        if !self.within(grid) {
            panic!("point is not within given grid!")
        }
        grid.end.x - self.x
    }

    pub fn depth(self, grid: Grid) -> u8 {
        if !self.within(grid) {
            panic!("point is not within given grid!")
        }
        self.y - grid.start.y
    }

    pub fn depth_gap(self, grid: Grid) -> u8 {
        if !self.within(grid) {
            panic!("point is not within given grid!")
        }
        grid.end.y - self.y
    }

    pub fn up(self, step: u8) -> Cell {
        if self.y < step {
            panic!(
                "(self.y - step) will overflow u8 bounds! Try using wrapping_up or saturating_up with grid specification"
            );
        }
        Cell {
            x: self.x,
            y: self.y - step,
        }
    }

    pub fn down(self, step: u8) -> Cell {
        if self.y > u8::MAX - step {
            panic!(
                "(self.y + step) will overflow u8 bounds! Try using wrapping_down or saturating_down with grid specification"
            );
        }
        Cell {
            x: self.x,
            y: self.y + step,
        }
    }

    pub fn left(self, step: u8) -> Cell {
        if self.x < step {
            panic!(
                "(self.x - step) will overflow u8 bounds! Try using wrapping_left or saturating_left with grid specification"
            );
        }
        Cell {
            x: self.x - step,
            y: self.y,
        }
    }

    pub fn right(self, step: u8) -> Cell {
        if self.x > u8::MAX - step {
            panic!(
                "(self.x + step) will overflow u8 bounds! Try using wrapping_right or saturating_right with grid specification"
            );
        }
        Cell {
            x: self.x + step,
            y: self.y,
        }
    }

    pub fn will_underflow_depth(self, grid: Grid, step: u8) -> bool {
        if !self.within(grid) {
            panic!("point is not within given grid!")
        }
        self.y < step || self.y - step < grid.start.y
    }

    pub fn will_overflow_depth(self, grid: Grid, step: u8) -> bool {
        if !self.within(grid) {
            panic!("point is not within given grid!")
        }
        self.y > u8::MAX - step || self.y + step > grid.end.y
    }

    pub fn will_underflow_width(self, grid: Grid, step: u8) -> bool {
        if !self.within(grid) {
            panic!("point is not within given grid!")
        }
        self.x < step || self.x - step < grid.start.x
    }

    pub fn will_overflow_width(self, grid: Grid, step: u8) -> bool {
        if !self.within(grid) {
            panic!("point is not within given grid!")
        }
        self.x > u8::MAX - step || self.x + step > grid.end.x
    }

    pub fn saturating_up(self, grid: Grid, step: u8) -> Cell {
        let next_y = if self.will_underflow_depth(grid, step) {
            grid.start.y
        } else {
            self.y - step
        };
        Cell {
            x: self.x,
            y: next_y,
        }
    }

    pub fn saturating_down(self, grid: Grid, step: u8) -> Cell {
        let next_y = if self.will_overflow_depth(grid, step) {
            grid.end.y
        } else {
            self.y + step
        };
        Cell {
            x: self.x,
            y: next_y,
        }
    }

    pub fn saturating_left(self, grid: Grid, step: u8) -> Cell {
        let next_x = if self.will_underflow_width(grid, step) {
            grid.start.x
        } else {
            self.x - step
        };
        Cell {
            x: next_x,
            y: self.y,
        }
    }

    pub fn saturating_right(self, grid: Grid, step: u8) -> Cell {
        let next_x = if self.will_overflow_width(grid, step) {
            grid.end.x
        } else {
            self.x + step
        };
        Cell {
            x: next_x,
            y: self.y,
        }
    }

    pub fn overflowing_up(self, grid: Grid, step: u8) -> (Cell, bool) {
        let underflowed = self.will_underflow_depth(grid, step);
        let next_y = if underflowed {
            grid.end.y - ((step - self.depth(grid) - 1) % grid.depth())
        } else {
            self.y - step
        };
        (
            Cell {
                x: self.x,
                y: next_y,
            },
            underflowed,
        )
    }

    pub fn overflowing_down(self, grid: Grid, step: u8) -> (Cell, bool) {
        let overflowed = self.will_overflow_depth(grid, step);
        let next_y = if overflowed {
            grid.start.y + ((step - self.depth_gap(grid) - 1) % grid.depth())
        } else {
            self.y + step
        };
        (
            Cell {
                x: self.x,
                y: next_y,
            },
            overflowed,
        )
    }

    pub fn overflowing_left(self, grid: Grid, step: u8) -> (Cell, bool) {
        let underflowed = self.will_underflow_width(grid, step);
        let next_x = if underflowed {
            grid.end.x - ((step - self.width(grid) - 1) % grid.width())
        } else {
            self.x - step
        };
        (
            Cell {
                x: next_x,
                y: self.y,
            },
            underflowed,
        )
    }

    pub fn overflowing_right(self, grid: Grid, step: u8) -> (Cell, bool) {
        let overflowed = self.will_overflow_width(grid, step);
        let next_x = if overflowed {
            grid.start.x + ((step - self.width_gap(grid) - 1) % grid.width())
        } else {
            self.x + step
        };
        (
            Cell {
                x: next_x,
                y: self.y,
            },
            overflowed,
        )
    }

    pub fn wrapping_up(self, grid: Grid, step: u8) -> Cell {
        self.overflowing_up(grid, step).0
    }

    pub fn wrapping_down(self, grid: Grid, step: u8) -> Cell {
        self.overflowing_down(grid, step).0
    }

    pub fn wrapping_left(self, grid: Grid, step: u8) -> Cell {
        self.overflowing_left(grid, step).0
    }

    pub fn wrapping_right(self, grid: Grid, step: u8) -> Cell {
        self.overflowing_right(grid, step).0
    }

    pub fn project_up(self, grid: Grid) -> Cell {
        self.saturating_up(grid, u8::MAX)
    }

    pub fn project_down(self, grid: Grid) -> Cell {
        self.saturating_down(grid, u8::MAX)
    }

    pub fn project_left(self, grid: Grid) -> Cell {
        self.saturating_left(grid, u8::MAX)
    }

    pub fn project_right(self, grid: Grid) -> Cell {
        self.saturating_right(grid, u8::MAX)
    }
}

impl Grid {
    pub fn new(width: u8, depth: u8) -> Self {
        Self {
            start: Cell { x: 0, y: 0 },
            end: Cell {
                x: width - 1,
                y: depth - 1,
            },
        }
    }

    pub fn from(start: Cell, end: Cell) -> Self {
        if start.x > end.x || start.y > end.y {
            panic!("start point overflows end point!")
        }
        Self { start, end }
    }

    pub fn member(self, x: u8, y: u8) -> Cell {
        Cell {
            x: self.start.x + x,
            y: self.start.y + y,
        }
    }

    pub fn start(self) -> Cell {
        self.start
    }

    pub fn end(self) -> Cell {
        self.end
    }

    pub fn width(self) -> u8 {
        self.end.x - self.start.x + 1
    }

    pub fn depth(self) -> u8 {
        self.end.y - self.start.y + 1
    }

    pub fn size(self) -> u16 {
        (self.end.x as u16 + 1) * (self.end.y as u16 + 1)
    }

    pub fn cells(self) -> Cells {
        Cells::from(self)
    }

    pub fn rows(self) -> Rows {
        Rows::from(self)
    }

    pub fn columns(self) -> Columns {
        Columns::from(self)
    }
}

impl Cells {
    pub fn from(grid: Grid) -> Self {
        Self {
            grid,
            current: grid.start,
            consumed: false,
        }
    }
}

impl Columns {
    pub fn from(grid: Grid) -> Self {
        Self {
            grid,
            current: Cells::from(Grid {
                start: grid.start,
                end: grid.start.project_down(grid),
            }),
            consumed: false,
        }
    }
}

impl Rows {
    pub fn from(grid: Grid) -> Self {
        Self {
            grid,
            current: Cells::from(Grid {
                start: grid.start,
                end: grid.start.project_right(grid),
            }),
            consumed: false,
        }
    }
}

impl Iterator for Cells {
    type Item = Cell;
    fn next(&mut self) -> Option<Self::Item> {
        if self.consumed {
            return None;
        }
        if self.current == self.grid.end {
            self.consumed = true;
            return Some(self.current);
        }
        let previous = self.current;
        match self.current.overflowing_right(self.grid, 1) {
            (next, true) => self.current = next.wrapping_down(self.grid, 1),
            (next, false) => self.current = next,
        }
        Some(previous)
    }
}

impl Iterator for Columns {
    type Item = Cells;
    fn next(&mut self) -> Option<Self::Item> {
        if self.consumed {
            return None;
        }
        if self.current.grid.end == self.grid.end {
            self.consumed = true;
            return Some(self.current);
        }
        let previous = self.current;
        self.current = Cells::from(Grid {
            start: self.current.grid.start.saturating_right(self.grid, 1),
            end: self.current.grid.end.saturating_right(self.grid, 1),
        });
        Some(previous)
    }
}

impl Iterator for Rows {
    type Item = Cells;
    fn next(&mut self) -> Option<Self::Item> {
        if self.consumed {
            return None;
        }
        if self.current.grid.end == self.grid.end {
            self.consumed = true;
            return Some(self.current);
        }
        let previous = self.current;
        self.current = Cells::from(Grid {
            start: self.current.grid.start.saturating_down(self.grid, 1),
            end: self.current.grid.end.saturating_down(self.grid, 1),
        });
        Some(previous)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cells() {
        let my_grid = Grid::new(5, 5);
        let printed_grid: String = my_grid
            .cells()
            .map(|cell| {
                if cell.x == my_grid.end.x {
                    " [#]\n\n"
                } else {
                    " [#]"
                }
            })
            .collect();
        println!("\nMAP:\n{printed_grid}");
    }

    #[test]
    fn rows() {
        let my_grid = Grid::new(5, 5);
        let rows: Vec<Cells> = my_grid.rows().collect();
        println!("rows: {rows:#?}");
    }

    #[test]
    fn columns() {
        let my_grid = Grid::new(5, 5);
        let columns: Vec<Cells> = my_grid.columns().collect();
        println!("rows: {columns:#?}");
    }
}
