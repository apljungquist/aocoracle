use std::fs;

use hashbrown::HashMap;

type Tile = u64;
type Room = u64;
type Amphipod = u64;

const NUM_ROOM: usize = 4;

type Rooms = [Room; NUM_ROOM];
type Hallway = u64;
type Paths = HashMap<(Room, Tile), Vec<Tile>>;

const EXISTENCE_MASK: u64 = 0b100;
const VALUE_MASK: u64 = 0b011;
const MASK_WIDTH: u64 = 3;

fn _push(vs: u64, v: u64) -> u64 {
    (vs << MASK_WIDTH) | EXISTENCE_MASK | v
}

fn _pop(vs: u64) -> (u64, u64) {
    let v = vs & VALUE_MASK;
    (vs >> MASK_WIDTH, v)
}

fn _insert(vs: u64, k: u64, v: u64) -> u64 {
    vs | (v | EXISTENCE_MASK) << (MASK_WIDTH * k)
}

fn _get(vs: u64, k: u64) -> u64 {
    let value_mask = VALUE_MASK << (MASK_WIDTH * k);
    (vs & value_mask) >> (MASK_WIDTH * k)
}

fn _remove(vs: u64, k: u64) -> (u64, u64) {
    let v = _get(vs, k);
    let erasure_mask = !((EXISTENCE_MASK | VALUE_MASK) << (MASK_WIDTH * k));
    (vs & erasure_mask, v)
}

fn _is_empty(vs: u64) -> bool {
    vs == 0
}

fn _contains(vs: u64, k: u64) -> bool {
    let mask = EXISTENCE_MASK << (MASK_WIDTH * k);
    vs & mask == mask
}

fn _fmt(vs: u64, len: u64) -> String {
    (0..len)
        .map(|i| match _contains(vs, i) {
            false => '.',
            true => {
                let (_, v) = _remove(vs, i);
                _fmt_amphipod(v)
            }
        })
        .collect()
}

fn _rooms(input: &str) -> Rooms {
    let mut lines = input.lines();
    lines.next();
    lines.next();
    let mut result = [0; NUM_ROOM];
    for line in lines.rev() {
        for (occupant, room) in line
            .trim()
            .chars()
            .filter(|ch| *ch != '#')
            .zip([0, 1, 2, 3])
        {
            result[room] = _push(
                result[room],
                match occupant {
                    'A' => 0,
                    'B' => 1,
                    'C' => 2,
                    'D' => 3,
                    _ => panic!("Unexpected occupant '{}'", occupant),
                },
            );
        }
    }
    result
}

const TILES: [Tile; 7] = [0, 1, 3, 5, 7, 9, 10];

fn _paths() -> Paths {
    let nogo = [2, 4, 6, 8];

    let mut result = HashMap::new();
    for room in 0..NUM_ROOM as u64 {
        let src = _tile(room);
        for dst in TILES {
            let mut path = Vec::new();
            if src < dst {
                for step in src..=dst {
                    if !nogo.contains(&step) {
                        path.push(step);
                    }
                }
            } else {
                for step in dst..=src {
                    if !nogo.contains(&step) {
                        path.insert(0, step);
                    }
                }
            }

            result.insert((room, dst), path);
        }
    }
    result
}

fn _accessible_tiles(paths: &Paths, hallway: Hallway, src: Room) -> Vec<Tile> {
    let mut result = Vec::new();
    for step in paths.get(&(src, 0)).unwrap() {
        if _contains(hallway, *step) {
            break;
        }
        result.push(*step);
    }
    for step in paths.get(&(src, 10)).unwrap() {
        if _contains(hallway, *step) {
            break;
        }
        result.push(*step);
    }
    result
}

fn _room_is_accessible(paths: &Paths, hallway: Hallway, src: Tile, dst: Room) -> bool {
    for step in paths.get(&(dst, src)).unwrap() {
        if src == *step {
            continue;
        }
        if _contains(hallway, *step) {
            return false;
        }
    }
    true
}

fn _room_contains_only(vs: u64, v: u64) -> bool {
    for k in 0.. {
        if !_contains(vs, k) {
            return true;
        }
        if _get(vs, k) != v {
            return false;
        }
    }
    panic!(
        "Ran out of spots to check for {}",
        _fmt(vs, NUM_ROOM as u64)
    );
}

fn _move_from_hallway(paths: &Paths, hallway: &mut Hallway, rooms: &mut Rooms) {
    let mut making_progress = true;
    while making_progress {
        making_progress = false;
        for src in TILES {
            if _contains(*hallway, src) {
                let dst = _get(*hallway, src);
                if !_room_is_accessible(paths, *hallway, src, dst) {
                    continue;
                }
                if !_room_contains_only(rooms[dst as usize], dst) {
                    continue;
                }
                let tmp = _remove(*hallway, src);
                *hallway = tmp.0;
                rooms[dst as usize] = _push(rooms[dst as usize], dst);
                making_progress = true;
            }
        }
    }
}

fn _tile(room: Room) -> Tile {
    room * 2 + 2
}

fn _multiplier(amphipod: Amphipod) -> u64 {
    10_u64.pow(amphipod as u32)
}

fn _distance(left: u64, right: u64) -> u64 {
    if left < right {
        right - left
    } else {
        left - right
    }
}

fn _room_hallway_distance(room: Room, location: Tile, amphipod: Amphipod) -> u64 {
    (_distance(_tile(room), location) + _distance(_tile(amphipod), location)) as u64
}

fn _fmt_amphipod(amphipod: Amphipod) -> char {
    match amphipod {
        0 => 'A',
        1 => 'B',
        2 => 'C',
        3 => 'D',
        _ => panic!("Unkown amphipod '{}'", amphipod),
    }
}

fn _print_all(title: &str, hallway: Hallway, rooms: Rooms) {
    println!("{}", title);
    println!("{}", _fmt(hallway, 11));
    for room in rooms {
        println!("{}", _fmt(room, 5));
    }
}

fn _min_downstream_cost(rooms: Rooms) -> u64 {
    let mut result = 0;
    for (src, room) in rooms.iter().enumerate() {
        for j in 0.. {
            if !_contains(*room, j) {
                break;
            }
            let dst = _get(*room, j);
            if src as u64 != dst {
                result += _distance(_tile(src as u64), _tile(dst)) * _multiplier(dst);
            }
        }
    }
    result
}

fn _min_cost_from_hallway(
    cache: &mut HashMap<(Hallway, Rooms), Option<u64>>,
    paths: &Paths,
    mut hallway: Hallway,
    mut rooms: Rooms,
    upstream_cost: u64,
    mut best_total_cost: u64,
) -> Option<u64> {
    let key = (hallway, rooms);
    if cache.contains_key(&key) {
        return *cache.get(&key).unwrap();
    }

    _move_from_hallway(paths, &mut hallway, &mut rooms);

    if _is_empty(hallway)
        && (0..NUM_ROOM as u64)
            .all(|i| !_is_empty(rooms[i as usize]) && _room_contains_only(rooms[i as usize], i))
    {
        cache.insert(key, Some(0));
        return Some(0);
    }

    let mut ok = false;
    let mut best_cost = std::u64::MAX / 2;
    for src in 0..NUM_ROOM as u64 {
        if _room_contains_only(rooms[src as usize], src) {
            continue;
        }

        let mut new_rooms = rooms;
        let (new_room, amphipod) = _pop(rooms[src as usize]);
        new_rooms[src as usize] = new_room;

        for dst in _accessible_tiles(paths, hallway, src) {
            let marginal_cost = _room_hallway_distance(src, dst, amphipod) * _multiplier(amphipod);

            let min_downstream_cost = _min_downstream_cost(new_rooms);
            let min_total_cost = upstream_cost + marginal_cost + min_downstream_cost;
            if best_total_cost <= min_total_cost {
                continue;
            }

            let new_hallway = _insert(hallway, dst, amphipod);

            let downstream_cost = match _min_cost_from_hallway(
                cache,
                paths,
                new_hallway,
                new_rooms,
                upstream_cost + marginal_cost,
                best_total_cost,
            ) {
                Some(cost) => cost,
                None => continue,
            };
            let cost = marginal_cost + downstream_cost;
            if cost < best_cost {
                ok = true;
                best_cost = cost;
                best_total_cost = upstream_cost + cost;
            } else {
            }
        }
    }
    if ok {
        cache.insert(key, Some(best_cost));
        return Some(best_cost);
    }
    cache.insert(key, None);
    None
}

fn _departure_penalty(room: Room, room_num: u64) -> u64 {
    let mut result = 0;
    for i in 0.. {
        if !_contains(room, i) {
            return result;
        }
        for j in i.. {
            if !_contains(room, j) {
                break;
            }
            if _get(room, j) != room_num {
                result += (i + 1) * _multiplier(_get(room, i));
                break;
            }
        }
    }
    panic!("Oups")
}

fn _arrival_penalty(room: Room, room_num: u64) -> u64 {
    let mut result = 0;
    for i in 0.. {
        if !_contains(room, i) {
            return result;
        }
        for j in i.. {
            if !_contains(room, j) {
                break;
            }
            if _get(room, j) != room_num {
                result += (i + 1) * _multiplier(room_num);
                break;
            }
        }
    }
    panic!("Oups")
}

fn _penalty(rooms: Rooms) -> u64 {
    (0..NUM_ROOM)
        .map(|i| _departure_penalty(rooms[i], i as u64) + _arrival_penalty(rooms[i], i as u64))
        .sum()
}

fn _part_x(rooms: Rooms) -> u64 {
    let paths = _paths();
    let mut cache = HashMap::new();
    let from_hallway =
        _min_cost_from_hallway(&mut cache, &paths, 0, rooms, 0, std::u64::MAX).unwrap();
    let from_rooms = _penalty(rooms);
    from_hallway + from_rooms
}

pub fn part_1(input: &str) -> u64 {
    let rooms = _rooms(input);
    _part_x(rooms)
}

pub fn part_2(input: &str) -> u64 {
    let mut rooms = _rooms(input);
    let mut tmp = _pop(rooms[0]);
    rooms[0] = _push(tmp.0, 3);
    rooms[0] = _push(rooms[0], 3);
    rooms[0] = _push(rooms[0], tmp.1);

    tmp = _pop(rooms[1]);
    rooms[1] = _push(tmp.0, 1);
    rooms[1] = _push(rooms[1], 2);
    rooms[1] = _push(rooms[1], tmp.1);

    tmp = _pop(rooms[2]);
    rooms[2] = _push(tmp.0, 0);
    rooms[2] = _push(rooms[2], 1);
    rooms[2] = _push(rooms[2], tmp.1);

    tmp = _pop(rooms[3]);
    rooms[3] = _push(tmp.0, 2);
    rooms[3] = _push(rooms[3], 0);
    rooms[3] = _push(rooms[3], tmp.1);

    _print_all("Start", 0, rooms);
    _part_x(rooms)
}

fn _from_file<F, T>(func: F, stem: &str) -> T
where
    F: Fn(&str) -> T,
{
    func(&fs::read_to_string(format!("day/23/{}.txt", stem)).unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_works_on_example() {
        assert_eq!(_from_file(part_1, "example"), 12521);
    }

    #[test]
    fn part_1_works_on_input() {
        assert_eq!(_from_file(part_1, "input"), 14510);
    }

    #[test]
    fn part_1_works_on_input_1() {
        assert_eq!(_from_file(part_1, "input_1"), 11332);
    }

    #[test]
    fn part_2_works_on_example() {
        assert_eq!(_from_file(part_2, "example"), 44169);
    }

    #[test]
    fn part_2_works_on_input() {
        assert_eq!(_from_file(part_2, "input"), 49180);
    }

    #[test]
    fn part_2_works_on_input_1() {
        assert_eq!(_from_file(part_2, "input_1"), 49936);
    }

    #[test]
    fn push_pop() {
        let vs = 0;
        assert_eq!(_fmt(vs, 4), "....");
        let vs = _push(vs, 0);
        assert_eq!(_fmt(vs, 4), "A...");
        let vs = _push(vs, 0);
        assert_eq!(_fmt(vs, 4), "AA..");
        let vs = _push(vs, 1);
        assert_eq!(_fmt(vs, 4), "BAA.");
        let (vs, v) = _pop(vs);
        assert_eq!(_fmt(vs, 4), "AA..");
        assert_eq!(_fmt_amphipod(v), 'B');
        let vs = _insert(vs, 2, 3);
        assert_eq!(_fmt(vs, 4), "AAD.");
        let (vs, v) = _pop(vs);
        assert_eq!(_fmt(vs, 4), "AD..");
        assert_eq!(_fmt_amphipod(v), 'A');
        let vs = _insert(vs, 3, 2);
        assert_eq!(_fmt(vs, 4), "AD.C");
        let (vs, v) = _remove(vs, 1);
        assert_eq!(_fmt(vs, 4), "A..C");
        assert_eq!(_fmt_amphipod(v), 'D');
        assert!(!_is_empty(vs));
        let (vs, v) = _remove(vs, 3);
        assert_eq!(_fmt(vs, 4), "A...");
        assert_eq!(_fmt_amphipod(v), 'C');
        assert!(!_is_empty(vs));
        let (vs, v) = _pop(vs);
        assert_eq!(_fmt(vs, 4), "....");
        assert_eq!(_fmt_amphipod(v), 'A');
        assert!(_is_empty(vs));
    }
}
