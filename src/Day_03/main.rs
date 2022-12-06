use std::{fs::File, io::Read};

fn main() {
    let mut file = File::open("src/Day_03/input").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

	// Part - 01
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

	println!("{:?}", backpacks);

	// Part - 02

	let mut group_sum: u32 = 0;
	for n in (0..contents.lines().collect::<Vec<&str>>().len()).step_by(3) {
		let elf1 = contents.lines().collect::<Vec<&str>>()[n];
		let elf2 = contents.lines().collect::<Vec<&str>>()[n+1];
		let elf3 = contents.lines().collect::<Vec<&str>>()[n+2];
		for char in elf1.chars() {
			if elf2.contains(char) && elf3.contains(char) {
				group_sum += get_char_value(&char);
				break;
			};
		};
	}

	println!("{:?}", group_sum)
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