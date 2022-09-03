use crate::*;

#[derive(Clone, PartialEq)]
pub struct Pathfinder {
	pub start: (f32, f32),
	pub end: (f32, f32),

	// 0 is walkable, 1 is wall
	pub world: Vec<Vec<usize>>,
	pub nodes: Vec<Node>,
	pub path: Vec<(f32, f32)>,

	pub max_steps: u32,
}

impl Pathfinder {
	pub fn new(start: (f32, f32), end: (f32, f32), world: Vec<Vec<usize>>, max_steps: u32) -> Self {
		Self {
			start,
			end,

			world,
			nodes: vec![],
			path: vec![],

			max_steps,
		}
	}

	fn get_neighbors(&mut self, node: usize) -> Vec<usize> {
		let mut result = vec![];

		let position = self.nodes[node].position;
		for y in (position.1 - 1.0) as usize..=(position.1 + 1.0) as usize {
			for x in (position.0 - 1.0) as usize..=(position.0 + 1.0) as usize {
				let neighbor_position = (x as f32, y as f32);
				if neighbor_position == position
				|| y >= self.world.len()
				|| x >= self.world[0].len()
				|| self.world[y][x] == 1 {
					continue;
				}

				let mut already_created = false;
				for i in 0..self.nodes.len() {
					if self.nodes[i].position == neighbor_position {
						result.push(i);
						already_created = true;
						break;
					}
				}

				if !already_created {
					self.nodes.push(
						Node {
							position: neighbor_position,
							from: node,

							g: (distance(neighbor_position, self.start) * 10.0) as u32,
							h: (distance(neighbor_position, self.end) * 10.0) as u32,
						}
					);

					result.push(self.nodes.len() - 1);
				}
			}
		}

		result
	}

	pub fn find_path(&mut self) {
		// If the ending point is inside a wall, there's no point in looking for a way through it
		if self.world[self.end.1 as usize][self.end.0 as usize] == 1 {
			return;
		}

		// Check the immediate area around the end to see if it's enclosed
		let mut enclosed = true;
		(|| {
			for y in self.end.1 as usize - 1..=self.end.1 as usize + 1 {
				for x in self.end.0 as usize - 1..=self.end.0 as usize + 1 {
					if (y == self.end.1 as usize
					&& x == self.end.0 as usize)
					|| y >= self.world.len()
					|| x >= self.world[0].len() {
						continue;
					}

					if self.world[y][x] == 0 {
						enclosed = false;
						return;
					}
				}
			}
		})();

		if enclosed {
			return;
		}

		let mut open: Vec<usize> = vec![];
		let mut closed: Vec<usize> = vec![];

		self.nodes.push(
			Node {
				position: self.start,
				from: 0,

				g: 0,
				h: (distance(self.start, self.end) * 10.0) as u32,
			}
		);

		open.push(0);

		let mut steps = self.max_steps;
		while open.len() > 0
		&& steps > 0 {
			steps -= 1;

			let mut current = usize::MAX;
			for i in open.iter() {
				if current == usize::MAX
				|| self.nodes[*i].f() < self.nodes[current].f()
				|| self.nodes[*i].f() == self.nodes[current].f()
				&& self.nodes[*i].h < self.nodes[current].h {
					current = *i;
				}
			}

			for i in 0..open.len() {
				if open[i] == current {
					open.remove(i);
					break;
				}
			}
			closed.push(current);

			if self.nodes[current].position == self.end {
				while self.nodes[current].position != self.start {
					self.path.push(self.nodes[current].position);
					current = self.nodes[current].from;
				}

				return;
			}

			for neighbor in self.get_neighbors(current) {
				if closed.contains(&neighbor) {
					continue;
				}

				let new_movement_cost = self.nodes[current].g + (distance(self.nodes[current].position, self.nodes[neighbor].position) * 10.0) as u32;
				if new_movement_cost < self.nodes[neighbor].g
				|| !open.contains(&neighbor) {
					self.nodes[neighbor].g = new_movement_cost;
					self.nodes[neighbor].from = current;

					if !open.contains(&neighbor) {
						open.push(neighbor);
					}
				}
			}
		}
	}
}