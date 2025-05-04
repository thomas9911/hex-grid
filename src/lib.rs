use std::collections::BTreeMap;

pub mod iter;

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
    data: BTreeMap<(u32, u32), T>,
}

impl<T> HexGrid<T> {
    pub fn new() -> Self {
        HexGrid {
            data: BTreeMap::new(),
        }
    }

    pub fn get(&self, x: u32, y: u32) -> Option<&T> {
        self.get_by_point(&(x, y))
    }

    pub fn get_key_value(&self, x: u32, y: u32) -> Option<(&(u32, u32), &T)> {
        self.data.get_key_value(&(x, y))
    }

    pub fn get_mut(&mut self, x: u32, y: u32) -> Option<&mut T> {
        self.get_by_point_mut(&(x, y))
    }

    pub fn get_by_point(&self, point: &(u32, u32)) -> Option<&T> {
        self.data.get(point)
    }

    pub fn get_by_point_mut<'a>(&'a mut self, point: &(u32, u32)) -> Option<&'a mut T> {
        self.data.get_mut(point)
    }

    pub fn set(&mut self, x: u32, y: u32, item: T) -> Option<T> {
        self.insert((x, y), item)
    }

    pub fn insert(&mut self, point: (u32, u32), item: T) -> Option<T> {
        self.data.insert(point, item)
    }

    pub fn iter_direction(
        &self,
        direction: Direction,
        start_x: u32,
        start_y: u32,
    ) -> iter::DirectionIterator<T> {
        iter::DirectionIterator::new(self, direction, start_x, start_y)
    }

    pub fn iter_direction_mut(
        &mut self,
        direction: Direction,
        start_x: u32,
        start_y: u32,
    ) -> iter::DirectionIteratorMut<T> {
        iter::DirectionIteratorMut::new(self, direction, start_x, start_y)
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
        start_x: u32,
        start_y: u32,
    ) -> iter::Values<T> {
        iter::Values {
            iter: self.iter_direction(direction, start_x, start_y),
        }
    }

    pub fn values_direction_mut(
        &mut self,
        direction: Direction,
        start_x: u32,
        start_y: u32,
    ) -> iter::ValuesMut<T> {
        iter::ValuesMut {
            iter: self.iter_direction_mut(direction, start_x, start_y),
        }
    }

    pub fn keys_direction(
        &self,
        direction: Direction,
        start_x: u32,
        start_y: u32,
    ) -> iter::Keys<T> {
        iter::Keys {
            iter: self.iter_direction(direction, start_x, start_y),
        }
    }

    pub fn into_keys(self) -> std::collections::btree_map::IntoKeys<(u32, u32), T> {
        self.data.into_keys()
    }

    pub fn into_values(self) -> std::collections::btree_map::IntoValues<(u32, u32), T> {
        self.data.into_values()
    }
}

pub enum Direction {
    Right,
    Left,
    DownRight,
    DownLeft,
    UpRight,
    UpLeft,
}

impl Direction {
    fn apply_next(&self, current_x: u32, current_y: u32) -> Option<(u32, u32)> {
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
}

impl<T> From<BTreeMap<(u32, u32), T>> for HexGrid<T> {
    fn from(data: BTreeMap<(u32, u32), T>) -> Self {
        HexGrid { data }
    }
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
}
