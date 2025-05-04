use crate::{Direction, HexGrid};

pub struct Values<'a, T> {
    pub(crate) iter: DirectionIterator<'a, T>,
}

impl<'a, T> Iterator for Values<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|(_, v)| v)
    }
}

pub struct Keys<'a, T> {
    pub(crate) iter: DirectionIterator<'a, T>,
}

impl<'a, T> Iterator for Keys<'a, T> {
    type Item = &'a (u32, u32);

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|(k, _)| k)
    }
}

pub struct ValuesMut<'a, T> {
    pub(crate) iter: DirectionIteratorMut<'a, T>,
}

impl<'a, T> Iterator for ValuesMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|(_, v)| v)
    }
}

pub struct DirectionIterator<'a, T> {
    grid: &'a HexGrid<T>,
    direction: Direction,
    current_x: u32,
    current_y: u32,
    done: bool,
}

impl<'a, T> DirectionIterator<'a, T> {
    pub fn new(grid: &'a HexGrid<T>, direction: Direction, current_x: u32, current_y: u32) -> Self {
        DirectionIterator {
            grid,
            direction,
            current_x,
            current_y,
            done: false,
        }
    }
}

impl<'a, T> Iterator for DirectionIterator<'a, T> {
    type Item = (&'a (u32, u32), &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            return None;
        }
        let item = self.grid.get_key_value(self.current_x, self.current_y)?;

        if let Some((current_x, current_y)) =
            self.direction.apply_next(self.current_x, self.current_y)
        {
            self.current_x = current_x;
            self.current_y = current_y;
        } else {
            self.done = true
        }

        Some(item)
    }
}

pub struct DirectionIteratorMut<'a, T> {
    grid: &'a mut HexGrid<T>,
    direction: Direction,
    current_x: u32,
    current_y: u32,
    done: bool,
}

impl<'a, T> DirectionIteratorMut<'a, T> {
    pub fn new(
        grid: &'a mut HexGrid<T>,
        direction: Direction,
        current_x: u32,
        current_y: u32,
    ) -> Self {
        DirectionIteratorMut {
            grid,
            direction,
            current_x,
            current_y,
            done: false,
        }
    }
}

impl<'a, T> Iterator for DirectionIteratorMut<'a, T> {
    type Item = ((u32, u32), &'a mut T);

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            return None;
        }

        // ps: this is from chatgpt
        let grid_ptr = self.grid as *mut HexGrid<T>;
        let key = (self.current_x, self.current_y);

        // SAFETY: We're the exclusive owner of `self.grid`, and we yield only one &mut at a time
        let item = unsafe { (*grid_ptr).data.get_mut(&key) }?;
        // end ps

        if let Some((current_x, current_y)) =
            self.direction.apply_next(self.current_x, self.current_y)
        {
            self.current_x = current_x;
            self.current_y = current_y;
        } else {
            self.done = true
        }

        Some((key, item))
    }
}

impl<'a, T> IntoIterator for HexGrid<T> {
    type Item = ((u32, u32), T);

    type IntoIter = std::collections::btree_map::IntoIter<(u32, u32), T>;

    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter()
    }
}

#[cfg(test)]
use collection_literals::btree;

#[test]
fn values_mut_test() {
    let mut grid = HexGrid::from(btree! {
        (0, 0) => 1,
        (1, 0) => 2,
        (2, 0) => 3,
        (3, 0) => 4,
        (4, 0) => 5,
    });

    for value in grid.values_direction_mut(Direction::Right, 0, 0) {
        *value += 1;
    }

    let expected = HexGrid::from(btree! {
        (0, 0) => 2,
        (1, 0) => 3,
        (2, 0) => 4,
        (3, 0) => 5,
        (4, 0) => 6,
    });

    assert_eq!(expected, grid);
}

#[test]
fn iter_mut_test() {
    let mut grid = HexGrid::from(btree! {
        (0, 0) => 1,
        (1, 0) => 2,
        (2, 0) => 3,
        (3, 0) => 4,
        (4, 0) => 5,
    });

    let mut keys = Vec::new();
    for (key, value) in grid.iter_direction_mut(Direction::Right, 0, 0) {
        *value += 1;
        keys.push(key)
    }

    let expected = HexGrid::from(btree! {
        (0, 0) => 2,
        (1, 0) => 3,
        (2, 0) => 4,
        (3, 0) => 5,
        (4, 0) => 6,
    });

    assert_eq!(expected, grid);
    assert_eq!(vec![(0, 0), (1, 0), (2, 0), (3, 0), (4, 0)], keys);
}
