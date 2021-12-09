use std::fs;
struct Position {
    horizontal_pos: i64,
    depth: i64,
}
impl Position {
    fn drive(&mut self, row: &str) {
        let elements: Vec<&str> = row.split_ascii_whitespace().collect();
        let value = elements[1].parse::<i64>().unwrap();
        match elements[0] {
            "forward" => self.horizontal_pos += value,
            "up" => self.depth -= value,
            "down" => self.depth += value,
            _ => panic!("Direction not possible"),
        }
    }
}
struct PositionAndAim {
    aim: i64,
    horizontal_pos: i64,
    depth: i64,
}
impl PositionAndAim {
    fn drive(&mut self, row: &str) {
        let elements: Vec<&str> = row.split_ascii_whitespace().collect();
        let value = elements[1].parse::<i64>().unwrap();
        match elements[0] {
            "forward" => {
                self.horizontal_pos += value;
                self.depth += value * self.aim;
            }
            "up" => self.aim -= value,
            "down" => self.aim += value,
            _ => panic!("Direction not possible"),
        }
    }
}
fn main() {
    let mut pos = Position {
        horizontal_pos: 0,
        depth: 0,
    };
    let mut pos_and_aim = PositionAndAim {
        aim: 0,
        horizontal_pos: 0,
        depth: 0,
    };
    fs::read_to_string("../input.txt")
        .expect("Something wrong with input")
        .lines()
        .for_each(|s| pos.drive(s));
    fs::read_to_string("../input.txt")
        .expect("Something wrong with input")
        .lines()
        .for_each(|s| pos_and_aim.drive(s));

    println!("Task 1: {}", pos.horizontal_pos * pos.depth);
    println!("Task 2: {}", pos_and_aim.horizontal_pos * pos_and_aim.depth);
}