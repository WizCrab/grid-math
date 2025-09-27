use std::convert::{From, Into};
use std::fmt;

/// `Cell` represents the basic unit of `Grid`.
///
/// Consists of coordinates x: u8 and y: u8, alongside with methods implementing
/// common mathematical operations for safe interactions with grids and other cells
///
/// Due to low memory size implements `Copy` trait, so all methods take `self` (copy) as first argument
///
/// # Examples
///
/// You can create Cell using new():
/// ```
/// use grid_math::Cell;
///
/// let cell = Cell::new(10, 15);
/// ```
///
/// Or use functionality of implemented `From` and `Into` traits:
/// ```
/// use grid_math::Cell;
///
/// let cell = Cell::from((9, 9));
/// let cell: Cell = (6, 7).into();
/// ```
///
/// To read x or y values, use getters:
/// ```
/// use grid_math::Cell;
///
/// let cell = Cell::new(10, 10);
/// let x = cell.x();
/// let y = cell.y();
/// ```
/// Or use `into()` provided by `Into` trait:
/// ```
/// use grid_math::Cell;
///
/// let cell = Cell::new(10, 10);
/// let (x, y): (u8, u8) = cell.into();
/// ```
///
/// Other methods involve interactions with `Grid`
///
/// `Cell` is designed to not mutate it's contents.
/// Instead, all operations return new instances of `Cell`
///
/// Also worth noting, that all operations on `Cell` are verified to be logically correct,
/// otherwise logically incorrect operations will be met with panic!
///
/// Here is a brief overview of `Cell` and `Grid` interactions:
///
/// Check if `Cell` is within the `Grid`:
/// ```
/// use grid_math::{Cell, Grid};
///
/// let cell = Cell::new(3, 4);
/// let grid = Grid::new(10, 10); // 10x10 grid starting at (0,0)
/// assert!(cell.within(grid));
/// ```
///
/// Get relative to the `Grid` position of `Cell`:
/// (`Grid` can start not only from (0,0))
/// ```
/// use grid_math::{Cell, Grid};
///
/// let cell = Cell::new(3, 4);
/// let grid = Grid::indented(8, 8, (2, 1)); // 8x8 grid starting at (2,1)
/// let (width, depth) = (cell.width(grid), cell.depth(grid));
/// // cell's width on grid = cell.x - grid.start.x
/// // cell's depth on grid = cell.y - grid.start.y
/// assert_eq!((width, depth), (1, 3));
/// // get gaps between width and depth grid borders and cell:
/// let (width_gap, depth_gap) = (cell.width_gap(grid), cell.depth_gap(grid));
/// assert_eq!((width_gap, depth_gap), (6, 4));
/// // get member of grid by relative position:
/// let member = grid.member(width, depth);
/// assert_eq!(cell, member);
/// ```
///
/// Perform some move calculations of `Cell` on `Grid`:
/// ```
/// use grid_math::{Cell, Grid};
///
/// let grid = Grid::new(10, 10);
/// let cell = grid.start(); // get grid's first cell
/// let next = cell.right(grid, 3); // move to the right by 3, panics if grid bounds overflow occures
/// assert_eq!(next, Cell::new(3, 0));
/// let next = cell.saturating_down(grid, 15); // move down by 15, returns grid bound if overflow occures
/// assert_eq!(next, Cell::new(0, 9));
/// let next = cell.wrapping_right(grid, 5).left(grid, 2).project_down(grid); // chain of movements
/// assert_eq!(next, Cell::new(3, 9));
/// ```
///
/// To get more examples, look at `Cell` and `Grid` methods documentation.
///
///
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

    pub fn within_panic(self, grid: Grid) {
        if !self.within(grid) {
            panic!("cell is not within given grid! cell:{self}, grid:{grid}")
        }
    }

    pub fn x(self) -> u8 {
        self.x
    }

    pub fn y(self) -> u8 {
        self.y
    }

    pub fn width(self, grid: Grid) -> u8 {
        self.within_panic(grid);
        self.x - grid.start.x
    }

    pub fn width_gap(self, grid: Grid) -> u8 {
        self.within_panic(grid);
        grid.end.x - self.x
    }

    pub fn depth(self, grid: Grid) -> u8 {
        self.within_panic(grid);
        self.y - grid.start.y
    }

    pub fn depth_gap(self, grid: Grid) -> u8 {
        self.within_panic(grid);
        grid.end.y - self.y
    }

    pub fn will_underflow_depth(self, grid: Grid, step: u8) -> bool {
        self.within_panic(grid);
        self.y < step || self.y - step < grid.start.y
    }

    pub fn will_overflow_depth(self, grid: Grid, step: u8) -> bool {
        self.within_panic(grid);
        self.y > u8::MAX - step || self.y + step > grid.end.y
    }

    pub fn will_underflow_width(self, grid: Grid, step: u8) -> bool {
        self.within_panic(grid);
        self.x < step || self.x - step < grid.start.x
    }

    pub fn will_overflow_width(self, grid: Grid, step: u8) -> bool {
        self.within_panic(grid);
        self.x > u8::MAX - step || self.x + step > grid.end.x
    }

    pub fn up(self, grid: Grid, step: u8) -> Cell {
        if self.will_underflow_depth(grid, step) {
            panic!(
                "this operation will violate grid upper bounds! cell:{self}, grid:{grid}, step:{step}"
            );
        }
        Cell {
            x: self.x,
            y: self.y - step,
        }
    }

    pub fn down(self, grid: Grid, step: u8) -> Cell {
        if self.will_overflow_depth(grid, step) {
            panic!(
                "this operation will violate grid lower bounds! cell:{self}, grid:{grid}, step:{step}"
            );
        }
        Cell {
            x: self.x,
            y: self.y + step,
        }
    }

    pub fn left(self, grid: Grid, step: u8) -> Cell {
        if self.will_underflow_width(grid, step) {
            panic!(
                "this operation will violate grid left bounds! cell:{self}, grid:{grid}, step:{step}"
            );
        }
        Cell {
            x: self.x - step,
            y: self.y,
        }
    }

    pub fn right(self, grid: Grid, step: u8) -> Cell {
        if self.will_overflow_width(grid, step) {
            panic!(
                "this operation will violate grid right bounds! cell:{self}, grid:{grid}, step:{step}"
            );
        }
        Cell {
            x: self.x + step,
            y: self.y,
        }
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

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({x}, {y})", x = self.x, y = self.y)
    }
}

impl From<(u8, u8)> for Cell {
    fn from(value: (u8, u8)) -> Self {
        Self {
            x: value.0,
            y: value.1,
        }
    }
}

#[allow(clippy::from_over_into)]
impl Into<(u8, u8)> for Cell {
    fn into(self) -> (u8, u8) {
        (self.x, self.y)
    }
}

impl Grid {
    pub fn new(width: u8, depth: u8) -> Self {
        if width < 1 || depth < 1 {
            panic!("can't create grid with width < 0 or depth < 0!")
        }
        Self {
            start: Cell { x: 0, y: 0 },
            end: Cell {
                x: width - 1,
                y: depth - 1,
            },
        }
    }

    pub fn indented(width: u8, depth: u8, indent: (u8, u8)) -> Self {
        if width < 1 || depth < 1 {
            panic!("can't create grid with width < 0 or depth < 0!")
        }
        Self {
            start: Cell {
                x: indent.0,
                y: indent.1,
            },
            end: Cell {
                x: indent.0 + width - 1,
                y: indent.1 + depth - 1,
            },
        }
    }

    pub fn within(self, grid: Grid) -> bool {
        self.start.within(grid) && self.end.within(grid)
    }

    pub fn member(self, width: u8, depth: u8) -> Cell {
        self.start.right(self, width).down(self, depth)
    }

    pub fn area(self, width: u8, depth: u8) -> Grid {
        if width < 1 || depth < 1 {
            panic!("can't create grid with width < 0 or depth < 0!")
        }
        Grid {
            start: self.start,
            end: self.start.right(self, width - 1).down(self, depth - 1),
        }
    }

    pub fn slice(self, width: u8, depth: u8, indent: (u8, u8)) -> Grid {
        if width < 1 || depth < 1 {
            panic!("can't create grid with width < 0 or depth < 0!")
        }
        Grid {
            start: self.start.right(self, indent.0).down(self, indent.1),
            end: self
                .start
                .right(self, indent.0 + width - 1)
                .down(self, indent.1 + depth - 1),
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

impl From<(Cell, Cell)> for Grid {
    fn from(value: (Cell, Cell)) -> Self {
        let (start, end) = value;
        if start.x > end.x || start.y > end.y {
            panic!("start cell overflows end cell! start:{start}, end:{end}")
        }
        Self { start, end }
    }
}

#[allow(clippy::from_over_into)]
impl Into<(Cell, Cell)> for Grid {
    fn into(self) -> (Cell, Cell) {
        (self.start, self.end)
    }
}

impl From<((u8, u8), (u8, u8))> for Grid {
    fn from(value: ((u8, u8), (u8, u8))) -> Self {
        let (start, end): (Cell, Cell) = (value.0.into(), value.1.into());
        if start.x > end.x || start.y > end.y {
            panic!("start cell overflows end cell! start:{start}, end:{end}")
        }
        Self { start, end }
    }
}

#[allow(clippy::from_over_into)]
impl Into<((u8, u8), (u8, u8))> for Grid {
    fn into(self) -> ((u8, u8), (u8, u8)) {
        (self.start.into(), self.end.into())
    }
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{start}:{end}]", start = self.start, end = self.end)
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
    fn basic_cell() {
        let cell = Cell::new(10, 12);
        println!("Cell: {cell}");
        assert_eq!(cell.x(), 10);
        assert_eq!(cell.y(), 12);
        let cell_1 = Cell::from((5, 6));
        let cell_2: Cell = (5, 6).into();
        assert_eq!(cell_1, cell_2);
        let (x, y): (u8, u8) = cell.into();
        assert_eq!(x, 10);
        assert_eq!(y, 12);
    }

    #[test]
    fn cell_on_grid() {
        let grid = Grid::new(11, 18);
        println!("Grid: {grid}");
        assert_eq!(grid.width(), 11);
        assert_eq!(grid.depth(), 18);
        let cell = grid.member(3, 2);
        let area = grid.area(10, 9);
        let slice_from_area = area.slice(5, 5, cell.into());
        println!("5x5 slice from area with indent = {cell}:  {slice_from_area}");
        // let other_grid = Grid::from(Cell::new(2, 3), Cell::new(7, 7));
        // let slice = grid.slice(other_grid);
        // let slice_member = slice.member(Cell::new(3, 4));
        // println!("{slice_member}");
    }

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
        println!("Grid: {my_grid}");
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
