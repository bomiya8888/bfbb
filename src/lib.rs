//! A library for creating tools and mods for *SpongeBob SquarePants: Battle for Bikini Bottom (2003)*.
//!
//! # Feature flags
//! Some functionality is gated behind feature flags to allow you to pick and choose what functionality you need for your project.
//!
//! `serde`: Implements `serde`'s `Serialize`/`Deserialize` for the core data types.
//!
//! `game-interface`: Enables the [`game_interface`] module.
//!
#![deny(missing_docs)]
#![warn(rust_2018_idioms)]

#[cfg(feature = "game-interface")]
pub mod game_interface;

mod level;
pub use level::Level;
mod spatula;
pub use spatula::Spatula;
