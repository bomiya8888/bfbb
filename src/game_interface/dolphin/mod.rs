//! Dolphin backend for [GameInterface](crate::game_interface::GameInterface)

mod data_member;
mod dolphin_interface;

pub(self) use data_member::DataMember;
pub use dolphin_interface::{DolphinInterface, Error};
