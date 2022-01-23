//! Snake properties module for Battlesnake description
//!
//! The module contains a struct that handles required
//! battlesnake settings like name, version, appearance etc.
//!
//! TODO:
//!  - [ ] Add option for bulk settings e.g from json

use serde::{Deserialize, Serialize};

/// Snake Properties as defined in [BattleSnake API Reference](https://docs.battlesnake.com/references/api#get)
///
/// Check your BattleSnake profile for available cosmetics
#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct SnakeProps<'a> {
	/// Apiversion must be set to "1" atm.
	apiversion: &'a str,
	/// Optional author field, by default taken from toml file
	author: &'a str,
	/// Optional Color field
	color: &'a str,
	/// Optional head cosmetics
	head: &'a str,
	/// Optional tail cosmetics
	tail: &'a str,
	/// Optional snake version, by default taken from toml file
	version: &'a str,
}

impl<'a> SnakeProps<'a> {
	/// Same as SnakeProps::default
	pub fn new() -> Self {
		Self::default()
	}

	/// Apiversion setter. Disabled in release version
	#[cfg(debug_assertions)]
	pub fn set_apiversion(&mut self, apiversion: &'a str) {
		self.apiversion = apiversion;
	}
	/// Author setter
	pub fn set_author(&mut self, author: &'a str) {
		self.author = author;
	}
	/// Color setter
	pub fn set_color(&mut self, color: &'a str) {
		self.color = color;
	}
	/// Head cosmetics setter
	pub fn set_head(&mut self, head: &'a str) {
		self.head = head;
	}
	/// Tail cosmetics setter
	pub fn set_tail(&mut self, tail: &'a str) {
		self.tail = tail;
	}
	/// Version setter
	pub fn set_version(&mut self, version: &'a str) {
		self.version = version;
	}

	/// JSON serializer
	///
	/// Serializes the struct into a JSON string.
	/// Example:
	/// ```
	/// 	let snake_props = SnakeProps::default();
	/// 	dbg!(snake_props.json())
	/// ```
	pub fn json(&self) -> String {
		serde_json::to_string(&self).unwrap()
	}
}

impl<'a> Default for SnakeProps<'a> {
	/// Sets apiversion to 1, loads author and version from Cargo.toml, rest is left empty.
	fn default() -> Self {
		Self {
			apiversion: "1",
			author: env!("CARGO_PKG_AUTHORS"),
			color: "",
			head: "",
			tail: "",
			version: env!("CARGO_PKG_VERSION"),
		}
	}
}
