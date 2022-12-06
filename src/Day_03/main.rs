use std::{fs::File, io::Read};

fn main() {
    let mut file = File::open("src/Day_03/input").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

	let backpacks: u32 = contents
		.lines()
		.map(|line| {
			let(part_one, part_two) = line.split_at(line.len()/2);
			let mut char_val: u32 = 0;
			for char in part_one.chars() {
				if part_two.contains(char) {
					char_val = get_char_value(&char);
					break;
				} else {
					char_val = 0;
				};
			};
			char_val
		})
		.sum();

	println!("{:?}", backpacks)
}

fn get_char_value(char: &char) -> u32 {
    let offset = if char.is_ascii_lowercase() {
        96
    } else {
        64 - 26
    };
    let value = (*char as u32) - offset;
    value
}