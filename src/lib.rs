use std::convert::{From, Into};
use std::fmt;

/// `Cell` represents the basic unit of `Grid`.
///
/// Consists of global coordinates `x: u8` and `y: u8`, alongside with methods implementing
/// common mathematical operations for safe interactions with grids and other cells
///
/// Due to low memory size, `Cell` implements `Copy` trait, so all methods take `self` (copy) as first argument
///
/// # Examples
///
/// You can create Cell using new(x, y):
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
/// 'Cell' implements `Display` and `Debug` trait, so you can easily print it out:
/// ```
/// use grid_math::Cell;
///
/// let cell = Cell::new(10, 10);
/// println!("Cell: {cell}"); // Cell: (10, 10)
/// assert_eq!(format!("{cell}"), "(10, 10)");
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

/// `Grid` represents the field of `Cell`
///
/// Consists of `start: Cell` and `end: Cell` fields, alongside with methods implementing
/// common mathematical operations for safe interactions with cells and other grids
///
/// `Grid` has two axis: width, and depth:
///
/// (0,0) [#] [#] [#] [#] (5,0)
///
///  [#]  [#] [#] [#] [#]  [#]
///
///  [#]  [#] [#] [#] [#]  [#]
///
///  [#]  [#] [#] [#] [#]  [#]
///
///  [#]  [#] [#] [#] [#]  [#]
///
/// (0,5) [#] [#] [#] [#] (5,5)
///
/// Due to low memory size, `Grid` implements `Copy` trait, so all methods take `self` (copy) as first argument
///
/// # Examples
///
/// You can create Grid using new(width, depth) or indented(width, depth, indent):
/// ```
/// use grid_math::Grid;
///
/// let grid = Grid::new(5, 5); // new 5x5 grid, starting at (0,0)
/// let grid = Grid::indented(5, 5, (1, 2)); // new 5x5 grid, starting at (1,2)
/// ```
///
/// Or use functionality of implemented `From` and `Into` traits:
/// ```
/// use grid_math::{Grid, Cell};
///
/// let grid = Grid::from(((1, 2), (5, 6))); // new field where `start` is (1,2), `end` is (5,6)
/// let grid: Grid = ((1, 2), (5, 6)).into();
/// //same for (cell, cell):
/// let grid = Grid::from((Cell::new(1, 2), Cell::new(5, 6)));
/// let grid: Grid = (Cell::new(1, 2), Cell::new(5, 6)).into();
/// // backwards:
/// let (start, end) = grid.into();
/// assert_eq!((start, end), (Cell::new(1, 2), Cell::new(5, 6)));
/// let ((x1, y1), (x2, y2)) = grid.into();
/// assert_eq!(((x1, y1), (x2, y2)), ((1, 2), (5, 6)));
/// ```
///
/// Important:
/// When creating `Grid` from cells, you specify `start` and `end` cell, not width and depth
/// This means that if you create grid with `start` (1, 2) and `end` (5, 6),
/// this will be 5x5 grid, not 4x4 as you can think (5 - 1 = 4, 6 - 2 = 4)
/// this is because `start` and `end` bounds included, they are actual members of grid,
/// where the `start` is the first cell on the grid, and `end` is the last cell on grid
///
/// To read `start` and `end` fields, or to calculate other common attributes, use getters:
/// ```
/// use grid_math::{Grid, Cell};
///
/// let grid = Grid::indented(8, 8, (3, 3)); // 8x8 grid, starting at (3,3)
/// let (start, end) = (grid.start(), grid.end());
/// assert_eq!((start, end), (Cell::new(3, 3), Cell::new(10, 10)));
/// let (width, depth) = (grid.width(), grid.depth());
/// assert_eq!((width, depth), (8, 8));
/// let size = grid.size();
/// assert_eq!(size, 64);
/// ```
///
/// 'Grid' implements `Display` and `Debug` trait, so you can easily print it out:
/// ```
/// use grid_math::Grid;
///
/// let grid = Grid::new(10, 10);
/// println!("Grid: {grid}"); // Grid: [(0, 0):(9, 9)]
/// assert_eq!(format!("{grid}"), "[(0, 0):(9, 9)]");
/// ```
///
/// Other advanced operations include interactions with other grids and cells:
///
/// Check if cell or subgrid is within grid:
/// ```
/// use grid_math::{Grid, Cell};
///
/// let grid = Grid::new(10, 10);
/// let cell = Cell::new(3, 4);
/// let subgrid = Grid::indented(5, 5, (2, 2));
/// assert!(subgrid.within(grid));
/// assert!(cell.within(grid));
/// ```
///
/// Get `Cell` from `Grid` by relative position:
/// ```
/// use grid_math::{Grid, Cell};
///
/// let grid = Grid::indented(5, 5, (2, 2));
/// let member = grid.member(2, 2);
///
/// assert_eq!(member, Cell::new(4, 4));
/// ```
///
/// Important:
/// When creating `Grid`, we specify `width` and `depth` in terms of length
/// But when we address member of `Grid`, we specify `width` and `depth` in terms of indexes
/// This means that `Grid` with `width` 5, and start at (0,*), will has `end` at (4,*),
/// because we have (0,*) (1,*) (2,*) (3,*) (4,*), which is 5 elements in total
/// So we used `width` 5 at `Grid` creation and got `Grid` with last cell (4,*)
/// But if we use `width` 5 when indexing member of `Grid`, we will get an error, because indexing starts at 0
///
/// Get subgrid from `Grid` by relative position:
/// ```
/// use grid_math::{Grid, Cell};
///
/// let grid = Grid::indented(5, 5, (2, 2));
/// assert_eq!(format!("{grid}"), "[(2, 2):(6, 6)]");
/// // get subgrid starting at current grid start, with specified width and depth:
/// let area = grid.area(3, 3);
/// assert_eq!(format!("{area}"), "[(2, 2):(4, 4)]");
/// // get subgrid starting at indent from current grid start and specified width and depth:
/// let slice = grid.slice(3, 3, (1, 1));
/// assert_eq!(format!("{slice}"), "[(3, 3):(5, 5)]");
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
/// Perform some actually usefull operations, using `Iterator` functionality:
/// ```
/// use grid_math::{Cell, Grid};
///
/// let grid = Grid::new(3, 3);
/// let grid_string = grid
///     .rows()
///     .map(|row| {
///         row.cells().map(|_| " [#]")
///             .chain(std::iter::once("\n\n"))
///             .collect::<String>()
///     })
///     .collect::<String>();
/// assert_eq!(grid_string,
/// " \
///  [#] [#] [#]
///
///  [#] [#] [#]
///
///  [#] [#] [#]
///
/// "
/// );
/// ```
///
/// To get more examples, look at `Cell` and `Grid` methods documentation.
///
///
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Grid {
    start: Cell,
    end: Cell,
}

/// `Cells` is an iterator over every `Cell` on `Grid`
///
/// # Examples
///
/// Get every `Cell` on `width` and `depth` axis:
/// ```
/// use grid_math::{Cell, Grid};
///
/// let grid = Grid::new(3, 3);
///
/// let axis_cells: Vec<Cell> = grid
///     .cells()
///     .filter(|cell| {
///         cell.x() == grid.start().x() || cell.y() == grid.start().y()
///     })
///     .collect();
/// assert_eq!(axis_cells, vec![
///     Cell::new(0, 0),
///     Cell::new(1, 0),
///     Cell::new(2, 0),
///     Cell::new(0, 1),
///     Cell::new(0, 2),
/// ]);
/// ```
///
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Cells {
    grid: Grid,
    current: Cell,
    consumed: bool,
}

/// `Rows` is an iterator over every row of `Cell` on `Grid`
///
/// Every element of 'Rows' returns `Grid`
///
/// # Examples
///
/// Print out `Grid` in custom format:
/// ```
/// use grid_math::{Cell, Grid};
///
/// let grid = Grid::new(3, 3);
/// let grid_string = grid
///     .rows()
///     .map(|row| {
///         row.cells().map(|_| " [#]")
///             .chain(std::iter::once("\n\n"))
///             .collect::<String>()
///     })
///     .collect::<String>();
/// assert_eq!(grid_string,
/// " \
///  [#] [#] [#]
///
///  [#] [#] [#]
///
///  [#] [#] [#]
///
/// "
/// );
/// ```
///
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Rows {
    grid: Grid,
    current: Grid,
    consumed: bool,
}

/// `Columns` is an iterator over every column of `Cell` on `Grid`
///
/// Every element of 'Columns' returns `Grid`
///
/// # Examples
///
/// Get every `Cell` on the first column of `Grid`:
/// ```
/// use grid_math::{Cell, Grid};
///
/// let grid = Grid::new(3, 3);
///
/// let first_column_cells: Vec<Cell> = grid
///     .columns()
///     .next()
///     .unwrap()
///     .cells()
///     .collect();
///
/// assert_eq!(first_column_cells, vec![
///     Cell::new(0, 0),
///     Cell::new(0, 1),
///     Cell::new(0, 2),
/// ]);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Columns {
    grid: Grid,
    current: Grid,
    consumed: bool,
}

impl Cell {
    /// Creates new `Cell` with specified `x: u8` and `y: u8` global position
    ///
    /// # Examples
    ///
    /// ```
    /// use grid_math::Cell;
    ///
    /// let cell = Cell::new(10, 15);
    /// ```
    pub fn new(x: u8, y: u8) -> Self {
        Self { x, y }
    }

    /// Checks if the `Cell` is within the given `Grid`
    ///
    /// # Examples
    ///
    /// ```
    /// use grid_math::{Cell, Grid};
    ///
    /// let grid = Grid::new(10, 10);
    /// let cell = Cell::new(5, 5);
    /// assert!(cell.within(grid));
    ///
    /// let cell = Cell::new(9, 15);
    /// assert!(!cell.within(grid));
    /// ```
    pub fn within(self, grid: Grid) -> bool {
        (grid.start.x..=grid.end.x).contains(&self.x)
            && (grid.start.y..=grid.end.y).contains(&self.y)
    }

    /// Checks if the `Cell` is within the given `Grid`
    ///
    /// # Panics
    /// Panics if the `Cell` is not within the given `Grid`
    ///
    /// # Examples
    ///
    /// ```should_panic
    /// use grid_math::{Cell, Grid};
    ///
    /// let grid = Grid::new(10, 10);
    /// let cell = Cell::new(9, 15);
    /// cell.within_panic(grid);
    /// ```
    pub fn within_panic(self, grid: Grid) {
        if !self.within(grid) {
            panic!("cell is not within given grid! cell:{self}, grid:{grid}")
        }
    }

    /// Returns `x` field of `Cell`
    ///
    /// # Examples
    ///
    /// ```
    /// use grid_math::Cell;
    ///
    /// let cell = Cell::new(8, 8);
    /// let x = cell.x();
    /// assert_eq!(x, 8);
    /// ```
    pub fn x(self) -> u8 {
        self.x
    }

    /// Returns `y` field of `Cell`
    ///
    /// # Examples
    ///
    /// ```
    /// use grid_math::Cell;
    ///
    /// let cell = Cell::new(8, 8);
    /// let y = cell.y();
    /// assert_eq!(y, 8);
    /// ```
    pub fn y(self) -> u8 {
        self.y
    }

    /// Calculates the `width` of the `Cell` relative to the given `Grid`
    /// `width` here means position / index / x of `Cell` on width axis
    ///
    /// # Panics
    /// Panics if the `Cell` is not within the given `Grid`
    ///
    /// # Examples
    ///
    /// ```
    /// use grid_math::{Cell, Grid};
    ///
    /// let cell = Cell::new(8, 8);
    /// let grid = Grid::indented(7, 7, (4, 4)); // 7x7 grid starting at (4,4)
    /// let width = cell.width(grid); // width = 4
    /// assert_eq!(width, 4);
    /// ```
    pub fn width(self, grid: Grid) -> u8 {
        self.within_panic(grid);
        self.x - grid.start.x
    }

    /// Calculates the gap between the `width` of `Cell` and the `width` of `Grid`
    ///
    /// # Panics
    /// Panics if the `Cell` is not within the given `Grid`
    ///
    /// # Examples
    ///
    /// ```
    /// use grid_math::{Cell, Grid};
    ///
    /// let cell = Cell::new(8, 8);
    /// let grid = Grid::indented(7, 7, (4, 4)); // 7x7 grid starting at (4,4)
    /// let width_gap = cell.width_gap(grid); // width_gap = 2
    /// assert_eq!(width_gap, 2);
    /// ```
    pub fn width_gap(self, grid: Grid) -> u8 {
        self.within_panic(grid);
        grid.end.x - self.x
    }

    /// Calculates the `depth` of `Cell` relative to the given `Grid`
    /// `depth` here means position / index / y of `Cell` on depth axis
    ///
    /// # Panics
    /// Panics if the `Cell` is not within the given `Grid`
    ///
    /// # Examples
    ///
    /// ```
    /// use grid_math::{Cell, Grid};
    ///
    /// let cell = Cell::new(8, 8);
    /// let grid = Grid::indented(7, 7, (4, 4)); // 7x7 grid starting at (4,4)
    /// let depth = cell.depth(grid); // depth = 4
    /// assert_eq!(depth, 4);
    /// ```
    pub fn depth(self, grid: Grid) -> u8 {
        self.within_panic(grid);
        self.y - grid.start.y
    }

    /// Calculates the gap between the `depth` of `Cell` and the `depth` of `Grid`
    ///
    /// # Panics
    /// Panics if the `Cell` is not within the given `Grid`
    ///
    /// # Examples
    ///
    /// ```
    /// use grid_math::{Cell, Grid};
    ///
    /// let cell = Cell::new(8, 8);
    /// let grid = Grid::indented(7, 7, (4, 4)); // 7x7 grid starting at (4,4)
    /// let depth_gap = cell.depth_gap(grid); // depth_gap = 2
    /// assert_eq!(depth_gap, 2);
    /// ```
    pub fn depth_gap(self, grid: Grid) -> u8 {
        self.within_panic(grid);
        grid.end.y - self.y
    }

    /// Checks if the `up` operation on `Cell` will violate the given `Grid` upper border
    ///
    /// # Panics
    /// Panics if the `Cell` is not within the given `Grid`
    ///
    /// # Examples
    ///
    /// ```
    /// use grid_math::{Cell, Grid};
    ///
    /// let grid = Grid::new(10, 10);
    /// let cell = Cell::new(2, 2);
    /// assert!(cell.will_underflow_depth(grid, 3));
    /// assert!(!cell.will_underflow_depth(grid, 2));
    /// ```
    pub fn will_underflow_depth(self, grid: Grid, step: u8) -> bool {
        self.within_panic(grid);
        self.y < step || self.y - step < grid.start.y
    }

    /// Checks if the `down` operation on `Cell` will violate the given `Grid` lower border
    ///
    /// # Panics
    /// Panics if the `Cell` is not within the given `Grid`
    ///
    /// # Examples
    ///
    /// ```
    /// use grid_math::{Cell, Grid};
    ///
    /// let grid = Grid::new(10, 10);
    /// let cell = Cell::new(7, 7);
    /// assert!(cell.will_overflow_depth(grid, 3));
    /// assert!(!cell.will_overflow_depth(grid, 2));
    /// ```
    pub fn will_overflow_depth(self, grid: Grid, step: u8) -> bool {
        self.within_panic(grid);
        self.y > u8::MAX - step || self.y + step > grid.end.y
    }

    /// Checks if the `left` operation on `Cell` will violate the given `Grid` left border
    ///
    /// # Panics
    /// Panics if the `Cell` is not within the given `Grid`
    ///
    /// # Examples
    ///
    /// ```
    /// use grid_math::{Cell, Grid};
    ///
    /// let grid = Grid::new(10, 10);
    /// let cell = Cell::new(2, 2);
    /// assert!(cell.will_underflow_width(grid, 3));
    /// assert!(!cell.will_underflow_width(grid, 2));
    /// ```
    pub fn will_underflow_width(self, grid: Grid, step: u8) -> bool {
        self.within_panic(grid);
        self.x < step || self.x - step < grid.start.x
    }

    /// Checks if the `right` operation on `Cell` will violate the given `Grid` right border
    ///
    /// # Panics
    /// Panics if the `Cell` is not within the given `Grid`
    ///
    /// # Examples
    ///
    /// ```
    /// use grid_math::{Cell, Grid};
    ///
    /// let grid = Grid::new(10, 10);
    /// let cell = Cell::new(7, 7);
    /// assert!(cell.will_overflow_width(grid, 3));
    /// assert!(!cell.will_overflow_width(grid, 2));
    /// ```
    pub fn will_overflow_width(self, grid: Grid, step: u8) -> bool {
        self.within_panic(grid);
        self.x > u8::MAX - step || self.x + step > grid.end.x
    }

    /// Moves current `Cell` upwards by `step` relative to the given `Grid`
    /// This operation does not mutate current `Cell` fields,
    /// instead it calculates new position and returns new `Cell`
    ///
    /// # Panics
    /// Panics if the `Cell` is not within the given `Grid`
    /// Panics if this operation will violate the given `Grid` upper border
    ///
    /// # Examples
    ///
    /// ```
    /// use grid_math::{Cell, Grid};
    ///
    /// let grid = Grid::new(10, 10);
    /// let cell = Cell::new(2, 2);
    /// let next = cell.up(grid, 2);
    /// assert_eq!(next, Cell::new(2, 0));
    /// ```
    ///
    /// ```should_panic
    /// use grid_math::{Cell, Grid};
    ///
    /// let grid = Grid::new(10, 10);
    /// let cell = Cell::new(2, 2);
    /// let next = cell.up(grid, 3); // panic!
    /// ```
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

    /// Moves current `Cell` downwards by `step` relative to the given `Grid`
    /// This operation does not mutate current `Cell` fields,
    /// instead it calculates new position and returns new `Cell`
    ///
    /// # Panics
    /// Panics if the `Cell` is not within the given `Grid`
    /// Panics if this operation will violate the given `Grid` lower border
    ///
    /// # Examples
    ///
    /// ```
    /// use grid_math::{Cell, Grid};
    ///
    /// let grid = Grid::new(10, 10);
    /// let cell = Cell::new(7, 7);
    /// let next = cell.down(grid, 2);
    /// assert_eq!(next, Cell::new(7, 9));
    /// ```
    ///
    /// ```should_panic
    /// use grid_math::{Cell, Grid};
    ///
    /// let grid = Grid::new(10, 10);
    /// let cell = Cell::new(7, 7);
    /// let next = cell.down(grid, 3); // panic!
    /// ```
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

    /// Moves current `Cell` to the left by `step` relative to the given `Grid`
    /// This operation does not mutate current `Cell` fields,
    /// instead it calculates new position and returns new `Cell`
    ///
    /// # Panics
    /// Panics if the `Cell` is not within the given `Grid`
    /// Panics if this operation will violate the given `Grid` left border
    ///
    /// # Examples
    ///
    /// ```
    /// use grid_math::{Cell, Grid};
    ///
    /// let grid = Grid::new(10, 10);
    /// let cell = Cell::new(2, 2);
    /// let next = cell.left(grid, 2);
    /// assert_eq!(next, Cell::new(0, 2));
    /// ```
    ///
    /// ```should_panic
    /// use grid_math::{Cell, Grid};
    ///
    /// let grid = Grid::new(10, 10);
    /// let cell = Cell::new(2, 2);
    /// let next = cell.left(grid, 3); // panic!
    /// ```
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

    /// Moves current `Cell` to the right by `step` relative to the given `Grid`
    /// This operation does not mutate current `Cell` fields,
    /// instead it calculates new position and returns new `Cell`
    ///
    /// # Panics
    /// Panics if the `Cell` is not within the given `Grid`
    /// Panics if this operation will violate the given `Grid` right border
    ///
    /// # Examples
    ///
    /// ```
    /// use grid_math::{Cell, Grid};
    ///
    /// let grid = Grid::new(10, 10);
    /// let cell = Cell::new(7, 7);
    /// let next = cell.right(grid, 2);
    /// assert_eq!(next, Cell::new(9, 7));
    /// ```
    ///
    /// ```should_panic
    /// use grid_math::{Cell, Grid};
    ///
    /// let grid = Grid::new(10, 10);
    /// let cell = Cell::new(7, 7);
    /// let next = cell.right(grid, 3); // panic!
    /// ```
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

    /// Moves current `Cell` upwards by `step` relative to the given `Grid`
    ///
    /// This operation does not mutate current `Cell` fields,
    /// instead it calculates new position and returns new `Cell`
    ///
    /// If this operation will cross `Grid` upper border,
    /// returns `Cell` with `depth` = `Grid` upper depth limit
    ///
    /// # Panics
    /// Panics if the `Cell` is not within the given `Grid`
    ///
    /// # Examples
    ///
    /// ```
    /// use grid_math::{Cell, Grid};
    ///
    /// let grid = Grid::new(10, 10);
    /// let cell = Cell::new(2, 2);
    /// let next = cell.saturating_up(grid, 2);
    /// assert_eq!(next, Cell::new(2, 0));
    /// let next = cell.saturating_up(grid, 5);
    /// assert_eq!(next, Cell::new(2, 0));
    /// ```
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

    /// Moves current `Cell` downwards by `step` relative to the given `Grid`
    ///
    /// This operation does not mutate current `Cell` fields,
    /// instead it calculates new position and returns new `Cell`
    ///
    /// If this operation will cross `Grid` lower border,
    /// returns `Cell` with `depth` = `Grid` lower depth limit
    ///
    /// # Panics
    /// Panics if the `Cell` is not within the given `Grid`
    ///
    /// # Examples
    ///
    /// ```
    /// use grid_math::{Cell, Grid};
    ///
    /// let grid = Grid::new(10, 10);
    /// let cell = Cell::new(7, 7);
    /// let next = cell.saturating_down(grid, 2);
    /// assert_eq!(next, Cell::new(7, 9));
    /// let next = cell.saturating_down(grid, 5);
    /// assert_eq!(next, Cell::new(7, 9));
    /// ```
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

    /// Moves current `Cell` to the left by `step` relative to the given `Grid`
    ///
    /// This operation does not mutate current `Cell` fields,
    /// instead it calculates new position and returns new `Cell`
    ///
    /// If this operation will cross `Grid` left border,
    /// returns `Cell` with `width` = `Grid` left width limit
    ///
    /// # Panics
    /// Panics if the `Cell` is not within the given `Grid`
    ///
    /// # Examples
    ///
    /// ```
    /// use grid_math::{Cell, Grid};
    ///
    /// let grid = Grid::new(10, 10);
    /// let cell = Cell::new(2, 2);
    /// let next = cell.saturating_left(grid, 2);
    /// assert_eq!(next, Cell::new(0, 2));
    /// let next = cell.saturating_left(grid, 5);
    /// assert_eq!(next, Cell::new(0, 2));
    /// ```
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

    /// Moves current `Cell` to the right by `step` relative to the given `Grid`
    ///
    /// This operation does not mutate current `Cell` fields,
    /// instead it calculates new position and returns new `Cell`
    ///
    /// If this operation will cross `Grid` right border,
    /// returns `Cell` with `width` = `Grid` right width limit
    ///
    /// # Panics
    /// Panics if the `Cell` is not within the given `Grid`
    ///
    /// # Examples
    ///
    /// ```
    /// use grid_math::{Cell, Grid};
    ///
    /// let grid = Grid::new(10, 10);
    /// let cell = Cell::new(7, 7);
    /// let next = cell.saturating_right(grid, 2);
    /// assert_eq!(next, Cell::new(9, 7));
    /// let next = cell.saturating_right(grid, 5);
    /// assert_eq!(next, Cell::new(9, 7));
    /// ```
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

    /// Moves current `Cell` upwards by `step` relative to the given `Grid`
    ///
    /// This operation does not mutate current `Cell` fields,
    /// instead it calculates new position and returns new `Cell` and `bool`
    ///
    /// This operation is similar to the overflowing operations on integer types
    /// It returns new `Cell` and 'bool' signaling that overflow happened
    ///
    /// # Panics
    /// Panics if the `Cell` is not within the given `Grid`
    ///
    /// # Examples
    ///
    /// ```
    /// use grid_math::{Cell, Grid};
    ///
    /// let grid = Grid::new(10, 10);
    /// let cell = Cell::new(2, 2);
    /// let (next, overflowed) = cell.overflowing_up(grid, 2);
    /// assert_eq!((next, overflowed), (Cell::new(2, 0), false));
    /// let (next, overflowed) = cell.overflowing_up(grid, 5);
    /// assert_eq!((next, overflowed), (Cell::new(2, 7), true));
    /// ```
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

    /// Moves current `Cell` downwards by `step` relative to the given `Grid`
    ///
    /// This operation does not mutate current `Cell` fields,
    /// instead it calculates new position and returns new `Cell` and `bool`
    ///
    /// This operation is similar to the overflowing operations on integer types
    /// It returns new `Cell` and 'bool' signaling that overflow happened
    ///
    /// # Panics
    /// Panics if the `Cell` is not within the given `Grid`
    ///
    /// # Examples
    ///
    /// ```
    /// use grid_math::{Cell, Grid};
    ///
    /// let grid = Grid::new(10, 10);
    /// let cell = Cell::new(7, 7);
    /// let (next, overflowed) = cell.overflowing_down(grid, 2);
    /// assert_eq!((next, overflowed), (Cell::new(7, 9), false));
    /// let (next, overflowed) = cell.overflowing_down(grid, 5);
    /// assert_eq!((next, overflowed), (Cell::new(7, 2), true));
    /// ```
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

    /// Moves current `Cell` to the left by `step` relative to the given `Grid`
    ///
    /// This operation does not mutate current `Cell` fields,
    /// instead it calculates new position and returns new `Cell` and `bool`
    ///
    /// This operation is similar to the overflowing operations on integer types
    /// It returns new `Cell` and 'bool' signaling that overflow happened
    ///
    /// # Panics
    /// Panics if the `Cell` is not within the given `Grid`
    ///
    /// # Examples
    ///
    /// ```
    /// use grid_math::{Cell, Grid};
    ///
    /// let grid = Grid::new(10, 10);
    /// let cell = Cell::new(2, 2);
    /// let (next, overflowed) = cell.overflowing_left(grid, 2);
    /// assert_eq!((next, overflowed), (Cell::new(0, 2), false));
    /// let (next, overflowed) = cell.overflowing_left(grid, 5);
    /// assert_eq!((next, overflowed), (Cell::new(7, 2), true));
    /// ```
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

    /// Moves current `Cell` to the right by `step` relative to the given `Grid`
    ///
    /// This operation does not mutate current `Cell` fields,
    /// instead it calculates new position and returns new `Cell` and `bool`
    ///
    /// This operation is similar to the overflowing operations on integer types
    /// It returns new `Cell` and 'bool' signaling that overflow happened
    ///
    /// # Panics
    /// Panics if the `Cell` is not within the given `Grid`
    ///
    /// # Examples
    ///
    /// ```
    /// use grid_math::{Cell, Grid};
    ///
    /// let grid = Grid::new(10, 10);
    /// let cell = Cell::new(7, 7);
    /// let (next, overflowed) = cell.overflowing_right(grid, 2);
    /// assert_eq!((next, overflowed), (Cell::new(9, 7), false));
    /// let (next, overflowed) = cell.overflowing_right(grid, 5);
    /// assert_eq!((next, overflowed), (Cell::new(2, 7), true));
    /// ```
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
    /// implements display for `Cell`
    ///
    /// # Examples
    ///
    /// ```
    /// use grid_math::Cell;
    ///
    /// let cell = Cell::new(5, 6);
    /// assert_eq!(format!("{cell}"), "(5, 6)");
    /// ```
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({x}, {y})", x = self.x, y = self.y)
    }
}

impl From<(u8, u8)> for Cell {
    /// implements constructor for `Cell` from (u8, u8)
    ///
    /// # Examples
    ///
    /// ```
    /// use grid_math::Cell;
    ///
    /// let pos = (5, 6);
    /// let cell = Cell::from(pos);
    /// assert_eq!((pos.0, pos.1), (cell.x(), cell.y()));
    /// ```
    fn from(value: (u8, u8)) -> Self {
        Self {
            x: value.0,
            y: value.1,
        }
    }
}

#[allow(clippy::from_over_into)]
impl Into<(u8, u8)> for Cell {
    /// implements conversion from `Cell` into (u8, u8)
    ///
    /// # Examples
    ///
    /// ```
    /// use grid_math::Cell;
    ///
    /// let cell = Cell::new(5, 6);
    /// let pos: (u8, u8) = cell.into();
    /// assert_eq!((pos.0, pos.1), (cell.x(), cell.y()));
    /// ```
    fn into(self) -> (u8, u8) {
        (self.x, self.y)
    }
}

impl Grid {
    /// Creates new `Grid` with specified `width: u8` and `depth: u8`, starting at (0,0)
    ///
    /// # Panics
    /// Panics if `width` or `depth` parameters < 1
    ///
    /// # Examples
    ///
    /// ```
    /// use grid_math::Grid;
    ///
    /// let grid = Grid::new(10, 10);
    /// assert_eq!(format!("{grid}"), "[(0, 0):(9, 9)]");
    /// ```
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

    /// Creates new `Grid` with specified `width: u8` and `depth: u8`, starting at indent
    ///
    /// # Panics
    /// Panics if `width` or `depth` parameters < 1
    ///
    /// # Examples
    ///
    /// ```
    /// use grid_math::{Grid, Cell};
    ///
    /// let grid = Grid::indented(5, 5, (2, 2));
    /// assert_eq!(format!("{grid}"), "[(2, 2):(6, 6)]");
    ///
    /// // use `Cell` as indent:
    /// let cell = Cell::new(2, 2);
    /// let grid = Grid::indented(5, 5, cell.into());
    /// assert_eq!(format!("{grid}"), "[(2, 2):(6, 6)]");
    /// ```
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

    /// Checks if the `Grid` is within the another `Grid`
    ///
    /// # Examples
    ///
    /// ```
    /// use grid_math::Grid;
    ///
    /// let grid = Grid::new(10, 10);
    /// let subgrid = grid.area(5, 5);
    /// assert!(subgrid.within(grid));
    ///
    /// let subgrid = Grid::new(10, 12);
    /// assert!(!subgrid.within(grid));
    /// ```
    pub fn within(self, grid: Grid) -> bool {
        self.start.within(grid) && self.end.within(grid)
    }

    /// Checks if the `Grid` is within the another `Grid`
    ///
    /// # Panics
    /// Panics if the `Grid` is not within the another `Grid`
    ///
    /// # Examples
    ///
    /// ```should_panic
    /// use grid_math::Grid;
    ///
    /// let grid = Grid::new(10, 10);
    /// let subgrid = Grid::new(10, 12);
    /// subgrid.within_panic(grid);
    /// ```
    pub fn within_panic(self, grid: Grid) {
        if !self.within(grid) {
            panic!("subgrid is not within given grid! subgrid:{self}, grid:{grid}")
        }
    }

    /// Returns new `Cell` by `width: u8` and `depth: u8` relative to the current `Grid`
    ///
    /// # Panics
    /// Panics if `width` or `depth` of the requested member exceeds borders of the current `Grid`
    ///
    /// # Examples
    ///
    /// ```
    /// use grid_math::{Grid, Cell};
    ///
    /// let grid = Grid::indented(5, 5, (2, 2)); // 5x5 grid, starting at (2,2)
    /// let member = grid.member(4, 4);
    /// assert_eq!(member, Cell::new(6, 6));
    /// ```
    pub fn member(self, width: u8, depth: u8) -> Cell {
        self.start.right(self, width).down(self, depth)
    }

    /// Returns new `Grid` with `width: u8` and `depth: u8`, which is a subgrid
    /// of current `Grid`, starting at current `Grid` start
    ///
    /// # Panics
    /// Panics if `width` or `depth` parameters < 1
    /// Panics if `width` or `depth` of the requested area exceeds borders of the current `Grid`
    ///
    /// # Examples
    ///
    /// ```
    /// use grid_math::{Grid, Cell};
    ///
    /// let grid = Grid::indented(5, 5, (2, 2)); // 5x5 grid, starting at (2,2)
    /// let area = grid.area(3, 3);
    /// assert_eq!(format!("{area}"), "[(2, 2):(4, 4)]");
    /// ```
    pub fn area(self, width: u8, depth: u8) -> Grid {
        if width < 1 || depth < 1 {
            panic!("can't create grid with width < 0 or depth < 0!")
        }
        Grid {
            start: self.start,
            end: self.start.right(self, width - 1).down(self, depth - 1),
        }
    }

    /// Returns new `Grid` with `width: u8` and `depth: u8`, which is a subgrid
    /// of current `Grid`, starting at current `Grid` start + indent
    ///
    /// # Panics
    /// Panics if `width` or `depth` parameters < 1
    /// Panics if `width` or `depth` of the requested slice exceeds borders of the current `Grid`
    /// Panics if `indent` of the requested slice exceeds borders of the current `Grid`
    ///
    /// # Examples
    ///
    /// ```
    /// use grid_math::{Grid, Cell};
    ///
    /// let grid = Grid::new(10, 10);
    /// let slice = grid.slice(3, 3, (2, 2));
    /// assert_eq!(format!("{slice}"), "[(2, 2):(4, 4)]");
    ///
    /// // use `Cell` as indent:
    /// let cell = Cell::new(2, 2);
    /// let slice = grid.slice(3, 3, cell.into());
    /// assert_eq!(format!("{slice}"), "[(2, 2):(4, 4)]");
    /// ```
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

    /// Returns `start` cell of `Grid`
    ///
    /// # Examples
    ///
    /// ```
    /// use grid_math::{Grid, Cell};
    ///
    /// let grid = Grid::new(10, 10);
    /// let start = grid.start();
    /// assert_eq!(start, Cell::new(0, 0));
    /// ```
    pub fn start(self) -> Cell {
        self.start
    }

    /// Returns `end` cell of `Grid`
    ///
    /// # Examples
    ///
    /// ```
    /// use grid_math::{Grid, Cell};
    ///
    /// let grid = Grid::new(10, 10);
    /// let end = grid.end();
    /// assert_eq!(end, Cell::new(9, 9));
    /// ```
    pub fn end(self) -> Cell {
        self.end
    }

    /// Calculates `width` of `Grid`
    ///
    /// # Examples
    ///
    /// ```
    /// use grid_math::Grid;
    ///
    /// let grid = Grid::new(10, 10);
    /// let width = grid.width();
    /// assert_eq!(width, 10);
    /// ```
    pub fn width(self) -> u8 {
        self.end.x - self.start.x + 1
    }

    /// Calculates `depth` of `Grid`
    ///
    /// # Examples
    ///
    /// ```
    /// use grid_math::Grid;
    ///
    /// let grid = Grid::new(10, 10);
    /// let depth = grid.depth();
    /// assert_eq!(depth, 10);
    /// ```
    pub fn depth(self) -> u8 {
        self.end.y - self.start.y + 1
    }

    /// Calculates `size: u16` of `Grid`
    ///
    /// # Examples
    ///
    /// ```
    /// use grid_math::Grid;
    ///
    /// let grid = Grid::new(10, 10);
    /// let size = grid.size();
    /// assert_eq!(size, 100);
    /// ```
    pub fn size(self) -> u16 {
        self.width() as u16 * self.depth() as u16
    }

    /// Returns `Cells`, which is an iterator over every cell of the `Grid`
    ///
    /// # Examples
    ///
    /// Get every `Cell` on `width` and `depth` axis:
    /// ```
    /// use grid_math::{Cell, Grid};
    ///
    /// let grid = Grid::new(3, 3);
    ///
    /// let axis_cells: Vec<Cell> = grid
    ///     .cells()
    ///     .filter(|cell| {
    ///         cell.x() == grid.start().x() || cell.y() == grid.start().y()
    ///     })
    ///     .collect();
    /// assert_eq!(axis_cells, vec![
    ///     Cell::new(0, 0),
    ///     Cell::new(1, 0),
    ///     Cell::new(2, 0),
    ///     Cell::new(0, 1),
    ///     Cell::new(0, 2),
    /// ]);
    /// ```
    pub fn cells(self) -> Cells {
        Cells::from(self)
    }

    /// Returns `Rows`, which is an iterator over every row of the `Grid`
    ///
    /// # Examples
    ///
    /// Print out `Grid` in custom format:
    /// ```
    /// use grid_math::{Cell, Grid};
    ///
    /// let grid = Grid::new(3, 3);
    /// let grid_string = grid
    ///     .rows()
    ///     .map(|row| {
    ///         row.cells().map(|_| " [#]")
    ///             .chain(std::iter::once("\n\n"))
    ///             .collect::<String>()
    ///     })
    ///     .collect::<String>();
    /// assert_eq!(grid_string,
    /// " \
    ///  [#] [#] [#]
    ///
    ///  [#] [#] [#]
    ///
    ///  [#] [#] [#]
    ///
    /// "
    /// );
    /// ```
    pub fn rows(self) -> Rows {
        Rows::from(self)
    }

    /// Returns `Columns`, which is an iterator over every column of the `Grid`
    ///
    /// # Examples
    ///
    /// Get every `Cell` on the first column of `Grid`:
    /// ```
    /// use grid_math::{Cell, Grid};
    ///
    /// let grid = Grid::new(3, 3);
    ///
    /// let first_column_cells: Vec<Cell> = grid
    ///     .columns()
    ///     .next()
    ///     .unwrap()
    ///     .cells()
    ///     .collect();
    ///
    /// assert_eq!(first_column_cells, vec![
    ///     Cell::new(0, 0),
    ///     Cell::new(0, 1),
    ///     Cell::new(0, 2),
    /// ]);
    /// ```
    pub fn columns(self) -> Columns {
        Columns::from(self)
    }
}

impl From<(Cell, Cell)> for Grid {
    /// implements constructor for `Grid` from (Cell, Cell)
    ///
    /// # Examples
    ///
    /// ```
    /// use grid_math::{Cell, Grid};
    ///
    /// let cells = (Cell::new(2, 2), Cell::new(5, 5));
    /// let grid = Grid::from(cells);
    /// assert_eq!((cells.0, cells.1), (grid.start(), grid.end()));
    /// ```
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
    /// implements conversion from `Grid` into (Cell, Cell)
    ///
    /// # Examples
    ///
    /// ```
    /// use grid_math::{Cell, Grid};
    ///
    /// let grid = Grid::new(5, 5);
    /// let cells: (Cell, Cell) = grid.into();
    /// assert_eq!((cells.0, cells.1), (grid.start(), grid.end()));
    /// ```
    fn into(self) -> (Cell, Cell) {
        (self.start, self.end)
    }
}

impl From<((u8, u8), (u8, u8))> for Grid {
    /// implements constructor for `Grid` from ((u8, u8), (u8, u8))
    ///
    /// # Examples
    ///
    /// ```
    /// use grid_math::{Cell, Grid};
    ///
    /// let vals = ((2, 2), (5, 5));
    /// let grid = Grid::from(vals);
    /// assert_eq!((Cell::from(vals.0), Cell::from(vals.1)), (grid.start(), grid.end()));
    /// ```
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
    /// implements conversion from `Grid` into ((u8, u8), (u8, u8))
    ///
    /// # Examples
    ///
    /// ```
    /// use grid_math::{Cell, Grid};
    ///
    /// let grid = Grid::new(5, 5);
    /// let vals: ((u8, u8), (u8, u8)) = grid.into();
    /// assert_eq!((Cell::from(vals.0), Cell::from(vals.1)), (grid.start(), grid.end()));
    /// ```
    fn into(self) -> ((u8, u8), (u8, u8)) {
        (self.start.into(), self.end.into())
    }
}

impl fmt::Display for Grid {
    /// implements display for `Grid`
    ///
    /// # Examples
    ///
    /// ```
    /// use grid_math::Grid;
    ///
    /// let grid = Grid::new(5, 6);
    /// assert_eq!(format!("{grid}"), "[(0, 0):(4, 5)]");
    /// ```
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{start}:{end}]", start = self.start, end = self.end)
    }
}

impl Cells {
    /// Creates new iterator over every `Cell` on the `Grid`
    ///
    /// # Examples:
    ///
    /// ```
    /// use grid_math::{Grid, Cells};
    ///
    /// let grid = Grid::new(5, 5);
    /// let cells = Cells::from(grid);
    /// ```
    pub fn from(grid: Grid) -> Self {
        Self {
            grid,
            current: grid.start,
            consumed: false,
        }
    }
}

impl Columns {
    /// Creates new iterator over every column on the `Grid`
    ///
    /// # Examples:
    ///
    /// ```
    /// use grid_math::{Grid, Columns};
    ///
    /// let grid = Grid::new(5, 5);
    /// let columns = Columns::from(grid);
    /// ```
    pub fn from(grid: Grid) -> Self {
        Self {
            grid,
            current: Grid {
                start: grid.start,
                end: grid.start.project_down(grid),
            },
            consumed: false,
        }
    }
}

impl Rows {
    /// Creates new iterator over every row on the `Grid`
    ///
    /// # Examples:
    ///
    /// ```
    /// use grid_math::{Grid, Rows};
    ///
    /// let grid = Grid::new(5, 5);
    /// let rows = Rows::from(grid);
    /// ```
    pub fn from(grid: Grid) -> Self {
        Self {
            grid,
            current: Grid {
                start: grid.start,
                end: grid.start.project_right(grid),
            },
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
    type Item = Grid;
    fn next(&mut self) -> Option<Self::Item> {
        if self.consumed {
            return None;
        }
        if self.current.end == self.grid.end {
            self.consumed = true;
            return Some(self.current);
        }
        let previous = self.current;
        self.current = Grid {
            start: self.current.start.saturating_right(self.grid, 1),
            end: self.current.end.saturating_right(self.grid, 1),
        };
        Some(previous)
    }
}

impl Iterator for Rows {
    type Item = Grid;
    fn next(&mut self) -> Option<Self::Item> {
        if self.consumed {
            return None;
        }
        if self.current.end == self.grid.end {
            self.consumed = true;
            return Some(self.current);
        }
        let previous = self.current;
        self.current = Grid {
            start: self.current.start.saturating_down(self.grid, 1),
            end: self.current.end.saturating_down(self.grid, 1),
        };
        Some(previous)
    }
}
