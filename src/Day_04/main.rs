use std::{fs::File, io::Read};



fn main () {
	let mut file = File::open("src/Day_04/input").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

	let mut contains_counter = 0;
	let mut overlap_counter = 0;

	contents.lines().for_each(|line| {
		let parsed_line = parse_section(line);
		if contains(&parsed_line[0], &parsed_line[1]) {
			contains_counter += 1;
		}
		if overlap(&parsed_line[0], &parsed_line[1]) {
			overlap_counter += 1;
		}
	});

	println!("Contains: {}", contains_counter);
	println!("Overlap: {}", overlap_counter);
}

// "16-82,81-82" -> [[16, 82][81, 82]]

fn parse_section(data_line: &str) -> Vec<Vec<u32>> {
	// "16-82,81-82"
	let start_end: Vec<&str> = data_line.split(',').collect::<Vec<&str>>();

	// ["16-82", "81-82"]
	let start_end: Vec<Vec<&str>> = start_end
		.iter()
		.map(|section| section.split('-').collect::<Vec<&str>>())
		.collect::<Vec<Vec<&str>>>();

	let res: Vec<Vec<u32>> = start_end
		.iter()
		.map(|section| {
			let start: u32 = section[0].parse::<u32>().unwrap();
			let end: u32 = section[1].parse::<u32>().unwrap();
			vec![start, end]
		})
		.collect::<Vec<Vec<u32>>>();
	res
}

fn contains(vec_1: &Vec<u32>, vec_2: &Vec<u32>) -> bool {
	let mut res = false;
	if vec_1[0] >= vec_2[0] && vec_1[1] <= vec_2[1] {
		// vec1 is inside vec2
		res = true;
	}
	if vec_2[0] >= vec_1[0] && vec_2[1] <= vec_1[1] {
		// vec2 is inside vec1
		res = true;
	}
	res
}

fn overlap(vec_1: &Vec<u32>, vec_2: &Vec<u32>) -> bool {
	let mut res = false;
	if vec_1[0] >= vec_2[0] && vec_1[0] <= vec_2[1] {
		// vec1 start is inside vec2
		res = true;
	}
	if vec_1[1] >= vec_2[0] && vec_1[1] <= vec_2[1] {
		// vec1 end is inside vec2
		res = true;
	}
	if vec_2[0] >= vec_1[0] && vec_2[0] <= vec_1[1] {
		// vec2 start is inside vec1
		res = true;
	}
	if vec_2[1] >= vec_1[0] && vec_2[1] <= vec_1[1] {
		// vec2 end is inside vec1
		res = true;
	}
	res
}