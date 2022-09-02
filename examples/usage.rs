use labrat::*;

fn main() {
	let mut pathfinder = Pathfinder::new(
		(0.0, 0.0), // start coords
		(10.0, 8.0), // end coords

		vec![ // map: 0 = nothing, 1 = wall
			vec![0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0],
			vec![0, 0, 1, 0, 1, 1, 1, 1, 1, 0, 0],
			vec![0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0],
			vec![0, 1, 1, 0, 1, 0, 1, 0, 0, 0, 0],
			vec![0, 1, 0, 0, 1, 0, 1, 1, 1, 0, 1],
			vec![0, 1, 1, 1, 1, 0, 1, 0, 0, 0, 0],
			vec![0, 0, 0, 0, 0, 0, 1, 0, 1, 0, 0],
			vec![1, 1, 1, 1, 1, 0, 1, 0, 1, 1, 1],
			vec![0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0],
		],

		999, // The max number of steps the pathfinder is allowed
	);

	pathfinder.find_path(); // creates a Vec<(f32, f32)> of all the coordinates from start to end
	println!("{:?}", pathfinder.path);
	// This get's the next position
	println!("{:?}", pathfinder.path.pop().expect("Empty path"));
}