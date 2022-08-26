# labrat.rs
 A simple grid-based A* implementation in Rust.<br>
 And just like a real labrat, it's not very fast and _not_ meant for pathfinding out catacombs if you know what I mean.

# Usage
```TOML
# ./Cargo.toml
labrat = { git = "https://github.com/eboatwright/labrat.rs" }
```
```Rust
// ./main.rs
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
	);

	pathfinder.find_path(); // creates a Vec<(f32, f32)> of all the coordinates from start to end
	println!("{:?}", pathfinder.path);
}
```

# License
 This project is under the MIT license.<br>
 Check the LICENSE file for more information.
