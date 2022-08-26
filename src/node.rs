#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Node {
	pub position: (f32, f32),
	pub from: usize,

	pub g: u32,
	pub h: u32,
}

impl Node {
	pub fn x(&self) -> usize {
		self.position.0 as usize
	}

	pub fn y(&self) -> usize {
		self.position.1 as usize
	}

	pub fn f(&self) -> u32 {
		self.g + self.h
	}
}