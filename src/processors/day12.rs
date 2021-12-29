use std::fmt::*;

fn parse_input() -> Vec<Room> {
    let mut rooms: Vec<Room> = Vec::new();
    for line in include_str!("../../data/day12.txt").lines() {
        let mut split = line.split('-');
        let (room1, room2) = (split.next().unwrap(), split.next().unwrap());
        parse_room(&mut rooms, room1, room2);
        parse_room(&mut rooms, room2, room1);
    }

    rooms
}

fn parse_room(rooms: &mut Vec<Room>, name: &'static str, link: &'static str) {
    let search = rooms.iter_mut().find(|r| r.name == name);
    if search.is_none() {
        rooms.push(Room {
            name,
            links: vec![link],
            visits: 0,
        });
    } else {
        search.unwrap().add_link(link);
    }
}

pub fn processor() -> (usize, usize) {
    let cave = parse_input();
    let mut output = (0, 0);

    let start_room = cave.iter().find(|r| r.name == "start").unwrap();
    process_path(&Path::new(), start_room, &cave, false, &mut output.0);
    process_path(&Path::new(), start_room, &cave, true, &mut output.1);

    output
}

fn process_path(
    hist: &Path,
    room: &Room,
    cave: &Vec<Room>,
    allow_duplicate: bool,
    path_count: &mut usize,
) {
    let mut path = hist.clone();
    if path.try_add(room, allow_duplicate) {
        if path.is_complete() {
            *path_count += 1;
            return;
        }

        for link in &room.links {
            process_path(
                &path,
                cave.iter().find(|r| &r.name == link).unwrap(),
                cave,
                allow_duplicate,
                path_count,
            );
        }
    }
}

#[derive(Clone)]
struct Room {
    name: &'static str,
    links: Vec<&'static str>,
    visits: usize,
}

impl Room {
    fn add_link(&mut self, link: &'static str) {
        self.links.push(link);
    }

    fn is_big(&self) -> bool {
        self.name.to_uppercase() == self.name
    }

    fn is_small(&self) -> bool {
        self.name.to_lowercase() == self.name
    }

    fn is_endpoint(&self) -> bool {
        self.name == "start" || self.name == "end"
    }
}

impl Debug for Room {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{} => {:?}", self.name, self.links)
    }
}

impl PartialEq for Room {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

#[derive(Clone)]
struct Path {
    sequence: Vec<&'static str>,
    rooms: Vec<Room>,
}

impl Path {
    fn new() -> Path {
        Path {
            sequence: Vec::new(),
            rooms: Vec::new(),
        }
    }

    fn try_add(&mut self, room: &Room, allow_duplicate: bool) -> bool {
        let is_permissive =
            allow_duplicate && !self.rooms.iter().any(|r| r.is_small() && r.visits > 1);
        let search = self.rooms.iter_mut().find(|r| r.name == room.name);
        if search.is_none() {
            let mut new_room = room.clone();
            new_room.visits = 1;
            self.rooms.push(new_room);
        } else {
            let result = search.unwrap();
            if result.is_big()
                || result.visits == 0
                || result.visits == 1 && is_permissive && !result.is_endpoint()
            {
                result.visits += 1;
            } else {
                return false;
            }
        }

        self.sequence.push(room.name);
        true
    }

    fn is_complete(&self) -> bool {
        !self.sequence.is_empty() && *self.sequence.last().unwrap() == "end"
    }
}

impl Debug for Path {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}", self.sequence.join(","))
    }
}
