//! Trait module for grid
//!

use crate::api::GameState;
use crate::utils::status::Result;

/// Grid trait
///
/// Allowes various grid implementations
pub trait Grid {
	type D; // Directions
	type P; // Point

	/// Function to get valid directions from a point.
	///
	/// This can be further generalized, but the goal is to have a function
	/// that returns all *legal* move directions within the grid from a given
	/// point.
	fn valid_directions(&self, point: &P) -> Vec<D>;

	fn from_api(&self, game: &GameState) -> Result<Self>;
}
