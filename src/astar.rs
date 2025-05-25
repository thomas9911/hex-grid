use crate::HexGrid;
use crate::to_3d_coordinate;
use std::collections::{BTreeMap, VecDeque};

#[derive(Debug, Clone, PartialEq)]
struct AStarItem {
    f: i32,
    g: i32,
    h: i32,
}

impl AStarItem {
    fn new(g: i32, h: i32) -> Self {
        AStarItem { g, h, f: g + h }
    }
}

fn find_best(
    open_list: &BTreeMap<(i32, i32, i32), AStarItem>,
) -> Option<(&(i32, i32, i32), &AStarItem)> {
    // https://theknightsofu.com/pathfinding-on-a-hexagonal-grid-a-algorithm-2/
    let mut best: Option<(&(i32, i32, i32), &AStarItem)> = None;
    for (coord, item) in open_list.iter() {
        if let Some((_, current_best)) = best {
            match item.f.partial_cmp(&current_best.f) {
                Some(core::cmp::Ordering::Less) => best = Some((coord, item)),
                Some(core::cmp::Ordering::Equal) if item.g > current_best.g => {
                    best = Some((coord, item))
                }
                _ => {}
            }
        } else {
            best = Some((coord, item))
        }
    }

    best
}

fn closed_list_to_path(
    closed_list: VecDeque<((i32, i32, i32), AStarItem)>,
) -> Option<Vec<(i32, i32)>> {
    let destination_index = closed_list.iter().rposition(|(_, item)| item.h == 0)?;
    let mut reverse_index_path = vec![destination_index];
    let mut current_index = destination_index;

    loop {
        let (prev_coord, prev) = &closed_list[current_index];
        let next_item = closed_list
            .iter()
            .filter(|(_, x)| x.g == prev.g.saturating_sub(1))
            .filter(|(a, _)| HexGrid::<()>::distance_3d(a, prev_coord) == 1)
            .min_by_key(|(_, x)| x.h)
            .expect("path is broken");
        let next_index = closed_list
            .iter()
            .position(|x| x == next_item)
            .expect("item is now not found?!");
        reverse_index_path.push(next_index);
        current_index = next_index;

        if next_item.1.g == 0 {
            // done
            break;
        }
    }

    let mut path = Vec::new();

    for pos in reverse_index_path.into_iter().rev() {
        let (a, b, _) = closed_list[pos].0;
        path.push((a, b))
    }

    Some(path)
}

pub fn astar<T>(
    hexgrid: &HexGrid<T>,
    point_a: (i32, i32),
    point_b: (i32, i32),
) -> Option<Vec<(i32, i32)>> {
    let mut open_list: BTreeMap<(i32, i32, i32), AStarItem> = BTreeMap::new();
    let mut closed_list: VecDeque<((i32, i32, i32), AStarItem)> = VecDeque::new();
    let point_a_3d = to_3d_coordinate(point_a.0, point_a.1);
    let point_b_3d = to_3d_coordinate(point_b.0, point_b.1);

    let h_start = HexGrid::<T>::distance_3d(&point_a_3d, &point_b_3d);
    let start = AStarItem::new(0, h_start);
    open_list.insert(point_a_3d, start);

    let mut found = None;
    while let Some((coord, best)) = find_best(&open_list) {
        let best_copy = best.clone();
        let coord_copy = coord.clone();
        for around_coord in hexgrid.keys_around(1, (coord.0, coord.1)) {
            let around_coord_3d = to_3d_coordinate(around_coord.0, around_coord.1);
            if closed_list
                .iter()
                .find(|(c, _)| c == &around_coord_3d)
                .is_some()
            {
                continue;
            };

            let distance = HexGrid::<T>::distance_3d(&around_coord_3d, &point_b_3d);
            let item = AStarItem::new(best_copy.g + 1, distance);
            if distance == 0 {
                found = Some((around_coord_3d, item));
                break;
            }
            open_list.insert(around_coord_3d, item);
        }
        open_list.remove(&coord_copy);
        closed_list.push_back((coord_copy, best_copy));
        if let Some(asdfg) = found.take() {
            closed_list.push_back(asdfg);
        }
    }

    closed_list_to_path(closed_list)
}

#[cfg(test)]
use collection_literals::btree;

#[test]
fn simple_astar() {
    let grid = HexGrid::from(btree! {
        (0, 1) => 3,
        (1, 1) => 5,
        (0, 2) => 4,
        (0, 3) => 5
    });

    let path = grid.astar((0, 1), (0, 3));

    assert_eq!(Some(vec![(0, 1), (0, 2), (0, 3)]), path);
}

#[test]
fn simple_astar_not_found() {
    let grid = HexGrid::from(btree! {
        (0, 1) => 3,
        (1, 1) => 5,
        (0, 2) => 4,
        (0, 3) => 5
    });

    let path = grid.astar((0, 1), (0, 4));

    assert_eq!(None, path);
}
