use std::{fs::File, io::Read};

pub fn day_one_part_one() {

	let mut file = File::open("src/Day_01/values").expect("File not found");
	let mut contents = String::new();
	file.read_to_string(&mut contents).expect("Error reading file");

	let split_vals: Vec<i32> = contents.split('\n')
		.map(|x| match x.parse::<i32>() {
		Ok(num) => num,
		Err(_) => 0,
	}).collect();

	let mut sum = 0;
	let mut highest = 0;

	for val in split_vals.iter() {

		if val > &0 {
			sum += val;
		} else {
			if sum > highest {
				highest = sum;
			}
			sum = 0;
		}
	}
	println!("{:?}", highest);
}

pub fn day_one_part_two() {
	
	let mut file = File::open("src/Day_01/values").expect("File not found");
	let mut contents = String::new();
	file.read_to_string(&mut contents).expect("Error reading file");

	let split_vals: Vec<i32> = contents.split('\n')
		.map(|x| match x.parse::<i32>() {
		Ok(num) => num,
		Err(_) => 0,
	}).collect();

	let mut sum = 0;
	let mut sums : Vec<i32> = Vec::new();

	for val in split_vals.iter() {

		if val > &0 {
			sum += val;
		} else {
			sums.push(sum);
			sum = 0;
		}
	}

	sums.sort();
	

	let highest_tree = sums[sums.len()-1] + sums[sums.len()-2] + sums[sums.len()-3];

	println!("{:?}", highest_tree);

}