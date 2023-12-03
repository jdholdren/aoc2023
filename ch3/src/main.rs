use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

/// Process in my head:
/// 1. Get it into a 2d array for easy lookup with coordinates
/// 2. Loop through and record the numbers and their surrounding squares.
/// 3. Loop through the numbers, see if their surroundings have a symbol.
/// 4. Total up those that do

fn main() {
    let schem = get_schematic("./ch3/input.txt");

    for row in schem.0.iter() {
        for c in row {
            print!("{}", c);
        }
        println!();
    }

    let mut map = HashMap::<Coord, Vec<u32>>::new();
    for part in parts(&schem) {
        let gear = part.touches_gear(&schem);
        if gear.is_none() {
            continue;
        }
        let coord = gear.unwrap();
        // Put the value into the map where the gear lies.
        // But make sure the vector is there first
        match map.get_mut(&coord) {
            None => {
                map.insert(coord, vec![part.value]);
            }
            Some(v) => v.push(part.value),
        };
    }

    let mut total = 0u32;
    for (_k, v) in map {
        if v.len() != 2 {
            continue;
        }

        println!("{:?}", v);
        total += v.iter().product::<u32>();
    }

    println!("total: {}", total);
}

struct Schematic(Vec<Vec<char>>);

impl Schematic {
    fn at(&self, coord: &Coord) -> Option<char> {
        if coord.0 < 0 || coord.1 < 0 {
            // checking those bounds
            return None;
        }
        let row = self.0.get(coord.1 as usize)?; // Fetch the row (y) first.
        row.get(coord.0 as usize).copied()
    }
}

#[derive(Debug, Eq, Hash, PartialEq)]
struct Coord(i32, i32);

impl Coord {
    // Generates the surrounding coordinates
    fn surrounding(&self) -> Vec<Coord> {
        vec![
            Coord(self.0 - 1, self.1 - 1),
            Coord(self.0, self.1 - 1),
            Coord(self.0 + 1, self.1 - 1),
            Coord(self.0 - 1, self.1),
            Coord(self.0 + 1, self.1),
            Coord(self.0 + 1, self.1 + 1),
            Coord(self.0 - 1, self.1 + 1),
            Coord(self.0, self.1 + 1),
            Coord(self.0 + 1, self.1 + 1),
        ]
    }
}

#[derive(Debug)]
struct Part {
    value: u32,
    coords: Vec<Coord>,
}

impl Part {
    fn touches_symbol(&self, schem: &Schematic) -> bool {
        // Check each location to see if it's a `.` or another number
        for coord in &self.coords {
            let surrounding = coord.surrounding();
            for coord in surrounding {
                let got = schem.at(&coord);
                if got.is_none() {
                    continue;
                }

                let c = got.unwrap();
                if c == '.' {
                    continue;
                }
                if c.is_numeric() {
                    continue;
                }

                return true;
            }
        }

        false
    }

    fn touches_gear(&self, schem: &Schematic) -> Option<Coord> {
        // Check each location to see if it's a `*`
        for coord in &self.coords {
            let surrounding = coord.surrounding();
            for coord in surrounding {
                let got = schem.at(&coord);
                if got.is_none() {
                    continue;
                }
                let c = got.unwrap();
                if c != '*' {
                    continue;
                }

                return Some(coord);
            }
        }

        None
    }
}

// Takes in the file and makes the 2d vector
fn get_schematic(filename: &str) -> Schematic {
    // Read input
    let f = File::open(filename).unwrap();
    let mut outer = vec![];

    for line in BufReader::new(f).lines() {
        let line = line.unwrap();
        let mut row = vec![];
        for c in line.chars() {
            row.push(c);
        }

        outer.push(row);
    }

    Schematic(outer)
}

// Gets the parts out of a schematic with their locations
fn parts(schem: &Schematic) -> Vec<Part> {
    let mut parts = vec![];

    for (y, line) in schem.0.iter().enumerate() {
        // Buffer to store the number we're building
        let mut buffer = String::new();
        let mut coords: Vec<Coord> = Vec::new();
        for (x, c) in line.iter().enumerate() {
            if !c.is_ascii_digit() && !coords.is_empty() {
                // Need to flush the buffer and coords to a part since the number ended but the
                // line continued.
                parts.push(Part {
                    value: buffer.parse().unwrap(),
                    coords,
                });
                buffer = String::new();
                coords = Vec::new();
                continue;
            }
            if !c.is_ascii_digit() {
                // Nothing being buffered, just another `.`
                continue;
            }

            // Found a digit, add it and the coordinate to the buffer and vec:
            buffer.push(*c);
            coords.push(Coord(x as i32, y as i32));
        }
        if !coords.is_empty() {
            // Hit the end of the line with a part being made
            parts.push(Part {
                value: buffer.parse().unwrap(),
                coords,
            });
        }
    }

    parts
}
