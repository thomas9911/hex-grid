use crate::to_3d_coordinate;
use crate::{Direction, HexGrid};

pub struct Values<'a, T> {
    pub(crate) iter: AroundIterator<'a, T>,
}

impl<'a, T> Iterator for Values<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|(_, v)| v)
    }
}

pub struct Keys<'a, T> {
    pub(crate) iter: AroundIterator<'a, T>,
}

impl<'a, T> Iterator for Keys<'a, T> {
    type Item = &'a (i32, i32);

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|(k, _)| k)
    }
}

// pub struct ValuesMut<'a, T> {
//     pub(crate) iter: AroundIteratorMut<'a, T>,
// }

// impl<'a, T> Iterator for ValuesMut<'a, T> {
//     type Item = &'a mut T;

//     fn next(&mut self) -> Option<Self::Item> {
//         self.iter.next().map(|(_, v)| v)
//     }
// }

#[derive(Debug)]
pub struct AroundIterator<'a, T> {
    grid: &'a HexGrid<T>,
    distance: i32,
    mid_point_x: i32,
    mid_point_y: i32,
    start_x: i32,
    start_y: i32,
    current_x: i32,
    current_y: i32,
    done: bool,
    previous_direction: Direction,
}

impl<'a, T> Iterator for AroundIterator<'a, T> {
    type Item = (&'a (i32, i32), &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            return None;
        }

        let current_item = self.grid.get_key_value(self.current_x, self.current_y);
        // dbg!((self.current_x, self.current_y));
        let (next_x, next_y) = self.determine_next();
        // dbg!((next_x, next_y));
        if (next_x, next_y) == (self.start_x, self.start_y) {
            self.done = true;
        }
        self.current_x = next_x;
        self.current_y = next_y;

        if let Some(item) = current_item {
            return Some(item);
        }

        self.next()
    }
}

impl<'a, T> AroundIterator<'a, T> {
    pub fn new(grid: &'a HexGrid<T>, distance: i32, mid_point: (i32, i32)) -> Self {
        let start_point = determine_start(distance, mid_point.0, mid_point.1);

        AroundIterator {
            grid,
            distance,
            mid_point_x: mid_point.0,
            mid_point_y: mid_point.1,
            start_x: start_point.0,
            start_y: start_point.1,
            current_x: start_point.0,
            current_y: start_point.1,
            done: false,
            previous_direction: Direction::Right,
        }
    }

    fn determine_next(&mut self) -> (i32, i32) {
        let coord = (
            self.current_x - self.mid_point_x,
            self.current_y - self.mid_point_y,
        );

        if let Some(might_be_next) = self.previous_direction.apply_next(coord.0, coord.1) {
            let oke = to_3d_coordinate(might_be_next.0, might_be_next.1);
            if max(oke) > self.distance {
                self.previous_direction = self.previous_direction.rotate_clockwise();
                return self.determine_next();
            }

            (
                might_be_next.0 + self.mid_point_x,
                might_be_next.1 + self.mid_point_y,
            )
        } else {
            // probably not what you want
            self.previous_direction = self.previous_direction.rotate_clockwise();
            return self.determine_next();
        }
    }
}

fn determine_start(distance: i32, mid_point_x: i32, mid_point_y: i32) -> (i32, i32) {
    (mid_point_x + distance, mid_point_y)
}

fn max(tuple: (i32, i32, i32)) -> i32 {
    tuple.0.abs().max(tuple.1.abs()).max(tuple.2.abs())
}

#[test]
fn determine_next_test() {
    fn do_it(iter: &mut AroundIterator<i32>) -> (i32, i32) {
        let next = iter.determine_next();
        iter.current_x = next.0;
        iter.current_y = next.1;
        return next;
    }

    let grid = HexGrid::<i32>::new();
    let mut iter = AroundIterator::new(&grid, 2, (0, 0));

    // loops successfull with distance 2
    assert_eq!(do_it(&mut iter), (1, 1));
    assert_eq!(do_it(&mut iter), (0, 2));
    assert_eq!(do_it(&mut iter), (-1, 2));
    assert_eq!(do_it(&mut iter), (-2, 2));
    assert_eq!(do_it(&mut iter), (-2, 1));
    assert_eq!(do_it(&mut iter), (-2, 0));
    assert_eq!(do_it(&mut iter), (-1, -1));
    assert_eq!(do_it(&mut iter), (0, -2));
    assert_eq!(do_it(&mut iter), (1, -2));
    assert_eq!(do_it(&mut iter), (2, -2));
    assert_eq!(do_it(&mut iter), (2, -1));
    assert_eq!(do_it(&mut iter), (2, 0));
    assert_eq!(do_it(&mut iter), (1, 1));
}

#[cfg(test)]
use collection_literals::btree;

#[test]
fn around_iterator_test() {
    //
    //     A B C
    //      D
    //   F E
    let grid = HexGrid::from(btree! {
        (0, 0) => "A",
        (1, 0) => "B",
        (2, 0) => "C",
        (0, 1) => "D",
        (-1, 2) => "E",
        (-2, 2) => "F",
    });

    let iter_a = AroundIterator::new(&grid, 1, (0, 0));
    let out: Vec<_> = iter_a.collect();
    assert_eq!(out, vec![(&(1, 0), &"B"), (&(0, 1), &"D")]);

    let iter_b = AroundIterator::new(&grid, 2, (0, 0));
    let out: Vec<_> = iter_b.collect();
    assert_eq!(
        out,
        vec![(&(2, 0), &"C"), (&(-1, 2), &"E"), (&(-2, 2), &"F")]
    );

    let iter_c = AroundIterator::new(&grid, 1, (1, 0));
    let out: Vec<_> = iter_c.collect();
    assert_eq!(out, vec![(&(2, 0), &"C"), (&(0, 1), &"D"), (&(0, 0), &"A")]);
}
