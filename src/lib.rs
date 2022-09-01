/* TODO
*/

pub mod node;
pub mod pathfinder;

pub use node::*;
pub use pathfinder::*;

pub(crate) fn distance(a: (f32, f32), b: (f32, f32)) -> f32 {
	f32::sqrt(f32::powf(a.0 - b.0, 2.0) + f32::powf(a.1 - b.1, 2.0))
}