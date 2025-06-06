use std::collections::BTreeMap;

use crate::around_iterator::AroundIterator;

pub mod around_iterator;
pub mod astar;
pub mod direction_iterator;

// reads: https://www.redblobgames.com/grids/hexagons/

//    ___           ___           ___           ___
//  /     \       /     \       /     \       /     \
// /       \ ___ /       \ ___ /       \ ___ /       \
// \       /     \       /     \       /     \       /
//  \_____/       \_____/       \_____/       \_____/
//  /     \       /     \       /     \       /     \
// /       \ ___ /       \ ___ /       \ ___ /       \
// \       /     \       /     \       /     \       /
//  \_____/       \_____/       \_____/       \_____/
//  /     \       /     \       /     \       /     \
// /       \ ___ /       \ ___ /       \ ___ /       \
// \       /     \       /     \       /     \       /
//  \_____/       \_____/       \_____/       \_____/
//  /     \       /     \       /     \       /     \
// /       \ ___ /       \ ___ /       \ ___ /       \
// \       /     \       /     \       /     \       /
//  \_____/       \_____/       \_____/       \_____/

//    ___           ___           ___           ___
//  /     \       /     \       /     \       /     \
// /  3,9  \ ___ /  5,8  \ ___ /  7,7  \ ___ /  9,6  \
// \       /     \       /     \       /     \       /
//  \_____/  4,9  \_____/  6,8  \_____/  8,7  \_____/
//  /     \       /     \       /     \       /     \
// /  3,10 \ ___ /  5,9  \ ___ /  7,8  \ ___ /  9,7  \
// \       /     \       /     \       /     \       /
//  \_____/  4,10 \_____/  6,9  \_____/  8,8  \_____/
//  /     \       /     \       /     \       /     \
// /  3,11 \ ___ /  5,10 \ ___ /  7,9  \ ___ /  9,8  \
// \       /     \       /     \       /     \       /
//  \_____/  4,11 \_____/  6,10 \_____/  8,9  \_____/
//  /     \       /     \       /     \       /     \
// /  3,12 \ ___ /  5,11 \ ___ /  7,10 \ ___ /  9,9  \
// \       /     \       /     \       /     \       /
//  \_____/       \_____/       \_____/       \_____/

//   / \   / \   / \   / \
//  /   \ /   \ /   \ /   \
// |     |     |     |     |
// |     |     |     |     |
//  \   / \   / \   / \   / \
//   \ /   \ /   \ /   \ /   \
//    |     |     |     |     |
//    |     |     |     |     |
//   / \   / \   / \   / \   /
//  /   \ /   \ /   \ /   \ /
// |     |     |     |     |
// |     |     |     |     |
//  \   / \   / \   / \   / \
//   \ /   \ /   \ /   \ /   \
//    |     |     |     |     |
//    |     |     |     |     |
//   / \   / \   / \   / \   /
//  /   \ /   \ /   \ /   \ /
// |     |     |     |     |
// |     |     |     |     |
//  \   / \   / \   / \   /
//   \ /   \ /   \ /   \ /

//   / \   / \   / \   / \
//  /   \ /   \ /   \ /   \
// | 5,1 | 6,1 | 7,1 | 8,1 |
// |     |     |     |     |
//  \   / \   / \   / \   / \
//   \ /   \ /   \ /   \ /   \
//    | 5,2 | 6,2 | 7,2 | 8,2 |
//    |     |     |     |     |
//   / \   / \   / \   / \   /
//  /   \ /   \ /   \ /   \ /
// | 4,3 | 5,3 | 6,3 | 7,3 |
// |     |     |     |     |
//  \   / \   / \   / \   / \
//   \ /   \ /   \ /   \ /   \
//    | 4,4 | 5,4 | 6,4 | 7,4 |
//    |     |     |     |     |
//   / \   / \   / \   / \   /
//  /   \ /   \ /   \ /   \ /
// | 3,5 | 4,5 | 5,5 | 6,5 |
// |     |     |     |     |
//  \   / \   / \   / \   /
//   \ /   \ /   \ /   \ /

/// Hexgrid struct
///
/// the grid is layed out:
/// ```text
///   / \   / \   / \   / \  
///  /   \ /   \ /   \ /   \
/// | 3,1 | 4,1 | 5,1 | 6,1 |
/// |     |     |     |     |
///  \   / \   / \   / \   / \
///   \ /   \ /   \ /   \ /   \
///    | 3,2 | 4,2 | 5,2 | 6,2 |
///    |     |     |     |     |
///   / \   / \   / \   / \   /
///  /   \ /   \ /   \ /   \ /  
/// | 2,3 | 3,3 | 4,3 | 5,3 |
/// |     |     |     |     |
///  \   / \   / \   / \   / \
///   \ /   \ /   \ /   \ /   \
///    | 2,4 | 3,4 | 4,4 | 5,4 |
///    |     |     |     |     |
///   / \   / \   / \   / \   /
///  /   \ /   \ /   \ /   \ /  
/// | 1,5 | 2,5 | 3,5 | 4,5 |
/// |     |     |     |     |
///  \   / \   / \   / \   /
///   \ /   \ /   \ /   \ /  
/// ```
#[derive(Debug, Default, PartialEq)]
pub struct HexGrid<T> {
    data: BTreeMap<(i32, i32), T>,
}

impl<T> HexGrid<T> {
    pub fn new() -> Self {
        HexGrid {
            data: BTreeMap::new(),
        }
    }

    pub fn get(&self, x: i32, y: i32) -> Option<&T> {
        self.get_by_point(&(x, y))
    }

    pub fn get_key_value(&self, x: i32, y: i32) -> Option<(&(i32, i32), &T)> {
        self.data.get_key_value(&(x, y))
    }

    pub fn get_mut(&mut self, x: i32, y: i32) -> Option<&mut T> {
        self.get_by_point_mut(&(x, y))
    }

    pub fn get_by_point(&self, point: &(i32, i32)) -> Option<&T> {
        self.data.get(point)
    }

    pub fn get_by_point_mut<'a>(&'a mut self, point: &(i32, i32)) -> Option<&'a mut T> {
        self.data.get_mut(point)
    }

    pub fn set(&mut self, x: i32, y: i32, item: T) -> Option<T> {
        self.insert((x, y), item)
    }

    pub fn insert(&mut self, point: (i32, i32), item: T) -> Option<T> {
        self.data.insert(point, item)
    }

    /// calculate the distance between two points, this doesn't check if it is possible in the grid.
    pub fn distance(point_a: &(i32, i32), point_b: &(i32, i32)) -> i32 {
        let a = to_3d_coordinate(point_a.0, point_a.1);
        let b = to_3d_coordinate(point_b.0, point_b.1);

        // abs_max_3d_point((a.0 - b.0, a.1 - b.1, a.2 - b.2))
        Self::distance_3d(&a, &b)
    }

    pub fn distance_3d(a: &(i32, i32, i32), b: &(i32, i32, i32)) -> i32 {
        abs_max_3d_point((a.0 - b.0, a.1 - b.1, a.2 - b.2))
    }

    /// calculates the path between two points using astar
    pub fn astar(&self, point_a: (i32, i32), point_b: (i32, i32)) -> Option<Vec<(i32, i32)>> {
        astar::astar(&self, point_a, point_b)
    }

    pub fn iter_direction(
        &self,
        direction: Direction,
        start_x: i32,
        start_y: i32,
    ) -> direction_iterator::DirectionIterator<T> {
        direction_iterator::DirectionIterator::new(self, direction, start_x, start_y)
    }

    pub fn iter_direction_mut(
        &mut self,
        direction: Direction,
        start_x: i32,
        start_y: i32,
    ) -> direction_iterator::DirectionIteratorMut<T> {
        direction_iterator::DirectionIteratorMut::new(self, direction, start_x, start_y)
    }

    /// Returns an iterator over the grid in a given direction.
    /// The iterator starts at the given coordinates and moves in the specified direction.
    /// The iterator will continue to move in the specified direction until it reaches the edge of the grid.
    /// The iterator yields references to the items in the grid.
    /// # Arguments
    /// * `direction` - The direction in which to iterate over the grid.
    /// * `start_x` - The x-coordinate of the starting point.
    /// * `start_y` - The y-coordinate of the starting point.
    /// # Returns
    /// An iterator over the grid in the specified direction.
    /// # Examples
    /// ```
    /// use hex_grid::{HexGrid, Direction};
    /// let mut grid = HexGrid::new();
    /// grid.set(0, 0, "A");
    /// grid.set(1, 0, "B");
    /// grid.set(1, 1, "C");
    /// grid.set(1, 2, "D");
    /// grid.set(2, 2, "E");
    /// grid.set(3, 2, "F");
    /// grid.set(2, 1, "G");
    /// grid.set(0, 3, "H");
    ///
    /// let data: Vec<_> = grid.values_direction(Direction::Right, 0, 0).collect();
    /// assert_eq!(data, vec![&"A", &"B"]);
    /// let data: Vec<_> = grid.values_direction(Direction::DownRight, 1, 0).collect();
    /// assert_eq!(data, vec![&"B", &"C", &"D"]);
    /// let data: Vec<_> = grid.values_direction(Direction::Left, 2,2).collect();
    /// assert_eq!(data, vec![&"E", &"D"]);
    /// let data: Vec<_> = grid.values_direction(Direction::UpLeft, 1, 2).collect();
    /// assert_eq!(data, vec![&"D", &"C", &"B"]);
    /// let data: Vec<_> = grid.values_direction(Direction::DownLeft, 1, 2).collect();
    /// assert_eq!(data, vec![&"D", &"H"]);
    /// let data: Vec<_> = grid.values_direction(Direction::UpRight, 1, 2).collect();
    /// assert_eq!(data, vec![&"D", &"G"]);
    /// ```
    pub fn values_direction(
        &self,
        direction: Direction,
        start_x: i32,
        start_y: i32,
    ) -> direction_iterator::Values<T> {
        direction_iterator::Values {
            iter: self.iter_direction(direction, start_x, start_y),
        }
    }

    pub fn values_direction_mut(
        &mut self,
        direction: Direction,
        start_x: i32,
        start_y: i32,
    ) -> direction_iterator::ValuesMut<T> {
        direction_iterator::ValuesMut {
            iter: self.iter_direction_mut(direction, start_x, start_y),
        }
    }

    pub fn keys_direction(
        &self,
        direction: Direction,
        start_x: i32,
        start_y: i32,
    ) -> direction_iterator::Keys<T> {
        direction_iterator::Keys {
            iter: self.iter_direction(direction, start_x, start_y),
        }
    }

    pub fn iter_around(&self, distance: i32, mid_point: (i32, i32)) -> AroundIterator<T> {
        AroundIterator::new(&self, distance, mid_point)
    }

    pub fn keys_around(&self, distance: i32, mid_point: (i32, i32)) -> around_iterator::Keys<T> {
        around_iterator::Keys {
            iter: self.iter_around(distance, mid_point),
        }
    }

    pub fn values_around(
        &self,
        distance: i32,
        mid_point: (i32, i32),
    ) -> around_iterator::Values<T> {
        around_iterator::Values {
            iter: self.iter_around(distance, mid_point),
        }
    }

    pub fn into_keys(self) -> std::collections::btree_map::IntoKeys<(i32, i32), T> {
        self.data.into_keys()
    }

    pub fn into_values(self) -> std::collections::btree_map::IntoValues<(i32, i32), T> {
        self.data.into_values()
    }
}

#[derive(Debug)]
pub enum Direction {
    Right,
    Left,
    DownRight,
    DownLeft,
    UpRight,
    UpLeft,
}

impl Direction {
    fn apply_next(&self, current_x: i32, current_y: i32) -> Option<(i32, i32)> {
        match self {
            Direction::Right => current_x.checked_add(1).map(|x| (x, current_y)),
            Direction::Left => current_x.checked_sub(1).map(|x| (x, current_y)),
            Direction::DownRight => current_y.checked_add(1).map(|y| (current_x, y)),
            Direction::UpLeft => current_y.checked_sub(1).map(|y| (current_x, y)),
            Direction::DownLeft => match (current_x.checked_sub(1), current_y.checked_add(1)) {
                (Some(x), Some(y)) => Some((x, y)),
                _ => None,
            },
            Direction::UpRight => match (current_x.checked_add(1), current_y.checked_sub(1)) {
                (Some(x), Some(y)) => Some((x, y)),
                _ => None,
            },
        }
    }

    fn rotate_clockwise(&self) -> Self {
        match self {
            Direction::Right => Direction::DownRight,
            Direction::DownRight => Direction::DownLeft,
            Direction::DownLeft => Direction::Left,
            Direction::Left => Direction::UpLeft,
            Direction::UpLeft => Direction::UpRight,
            Direction::UpRight => Direction::Right,
        }
    }
}

impl<T> From<BTreeMap<(i32, i32), T>> for HexGrid<T> {
    fn from(data: BTreeMap<(i32, i32), T>) -> Self {
        HexGrid { data }
    }
}

pub fn to_3d_coordinate(x: i32, y: i32) -> (i32, i32, i32) {
    (x, y, -x - y)
}

pub(crate) fn abs_max_3d_point(tuple: (i32, i32, i32)) -> i32 {
    tuple.0.abs().max(tuple.1.abs()).max(tuple.2.abs())
}

#[cfg(test)]
mod tests {
    use collection_literals::btree;

    use super::*;

    #[test]
    fn insert_and_set() {
        let mut grid = HexGrid::new();

        grid.insert((0, 1), true);
        grid.set(1, 2, true);

        let expected = HexGrid {
            data: btree! {(0, 1) => true, (1, 2) => true},
        };

        assert_eq!(expected, grid)
    }

    #[test]
    fn get() {
        let mut grid = HexGrid::from(btree! {
            (0, 1) => 3,
            (0, 2) => 4,
            (1, 2) => 5
        });

        assert_eq!(Some(&4), grid.get(0, 2));
        assert_eq!(Some(&5), grid.get_by_point(&(1, 2)));

        if let Some(item) = grid.get_mut(0, 2) {
            *item = 9;
        }

        if let Some(item) = grid.get_by_point_mut(&(1, 2)) {
            *item = 7;
        }

        let expected = HexGrid {
            data: btree! {
                (0, 1) => 3,
                (0, 2) => 9,
                (1, 2) => 7
            },
        };

        assert_eq!(expected, grid)
    }

    #[test]
    fn to_3d_coordinate_test() {
        assert_eq!(to_3d_coordinate(2, 1), (2, 1, -3));
    }

    #[test]
    fn astar_knightsofu_test() {
        // based on https://theknightsofu.com/pathfinding-on-a-hexagonal-grid-a-algorithm-2/
        let mut btree = BTreeMap::new();
        for (x, y) in (-3..6).flat_map(|i| (0..6).map(move |j| (i, j))) {
            // these are the walls
            if [(0, 2), (1, 4), (2, 3), (2, 2)].contains(&(x, y)) {
                continue;
            }
            btree.insert((x, y), 1);
        }

        let grid = HexGrid::from(btree);

        let path = grid.astar((0, 3), (4, 2));

        assert_eq!(
            Some(vec![(0, 3), (1, 2), (2, 1), (3, 1), (3, 2), (4, 2)]),
            path
        );
    }
}
