use aoc_derive::impl_day;

#[inline]
fn split_input_line(line: &str) -> (char, i32) {
    let bytes = line.as_bytes();
    let command = bytes[0] as char;
    let arg = std::str::from_utf8(&bytes[1..])
        .ok()
        .and_then(|s| s.parse::<i32>().ok())
        .unwrap();
    (command, arg)
}

#[inline]
fn convert_degree(arg: i32) -> u16 {
    if arg >= 0 {
        (arg % 360) as u16
    } else {
        (360 + (arg % 360)) as u16
    }
}

#[derive(Default)]
struct Ship {
    /// only used for p1
    facing_degree: u16,

    /// negative for west
    east: i32,
    /// negative for south
    north: i32,
}

impl Ship {
    fn forward(&mut self, distance: i32) {
        assert_eq!(self.facing_degree % 90, 0);

        match self.facing_degree / 90 {
            0 => self.move_ship(distance, 0),
            1 => self.move_ship(0, distance),
            2 => self.move_ship(-distance, 0),
            3 => self.move_ship(0, -distance),
            _ => unreachable!(),
        }
    }

    #[inline]
    fn move_ship(&mut self, east: i32, north: i32) {
        self.east += east;
        self.north += north;
    }

    fn process_input(&mut self, lines: &[String]) {
        for l in lines {
            let (command, arg) = split_input_line(l);

            match command {
                'N' => self.move_ship(0, arg),
                'S' => self.move_ship(0, -arg),
                'E' => self.move_ship(arg, 0),
                'W' => self.move_ship(-arg, 0),
                'L' => self.facing_degree = (self.facing_degree + convert_degree(arg)) % 360,
                'R' => self.facing_degree = (self.facing_degree + convert_degree(-arg)) % 360,
                'F' => self.forward(arg),
                _ => unreachable!(),
            }
        }
    }

    fn get_manhattan_distance(&self) -> u32 {
        self.east.abs() as u32 + self.north.abs() as u32
    }
}

struct WaypointWithShip {
    ship: Ship,

    /// `east` and `north` are position relative to the ship
    /// negative for west
    east: i32,
    /// `east` and `north` are position relative to the ship
    /// negative for south
    north: i32,
}

impl Default for WaypointWithShip {
    fn default() -> Self {
        Self {
            ship: Ship::default(),
            north: 1,
            east: 10,
        }
    }
}

impl WaypointWithShip {
    fn move_waypoint(&mut self, east: i32, north: i32) {
        self.east += east;
        self.north += north;
    }

    fn rotate_waypoint(&mut self, degree: i32) {
        // convert from signed to unsigned degree
        let degree = convert_degree(degree);
        assert_eq!(degree % 90, 0);

        match degree / 90 {
            0 => {}
            1 => {
                let tmp_east = self.east;
                self.east = -self.north;
                self.north = tmp_east;
            }
            2 => {
                self.east = -self.east;
                self.north = -self.north;
            }
            3 => {
                let tmp_east = self.east;
                self.east = self.north;
                self.north = -tmp_east;
            }
            _ => unreachable!(),
        }
    }

    fn process_input(&mut self, lines: &[String]) {
        for l in lines {
            let (command, arg) = split_input_line(l);

            match command {
                'N' => self.move_waypoint(0, arg),
                'S' => self.move_waypoint(0, -arg),
                'E' => self.move_waypoint(arg, 0),
                'W' => self.move_waypoint(-arg, 0),
                'L' => self.rotate_waypoint(arg),
                'R' => self.rotate_waypoint(-arg),
                'F' => self.ship.move_ship(self.east * arg, self.north * arg),
                _ => unreachable!(),
            }
        }
    }

    fn ship(&self) -> &Ship {
        &self.ship
    }
}

impl_day!(12, |reader| {
    let lines: Vec<String> = reader
        .lines()
        .filter_map(|l| l.ok())
        .take_while(|l| !l.is_empty())
        .collect();

    let mut ship = Ship::default();
    ship.process_input(&lines);
    let p1 = ship.get_manhattan_distance();

    let mut waypoint_with_ship = WaypointWithShip::default();
    waypoint_with_ship.process_input(&lines);
    let p2 = waypoint_with_ship.ship().get_manhattan_distance();

    println!("Part1: {}", p1);
    println!("Part2: {}", p2);
});
