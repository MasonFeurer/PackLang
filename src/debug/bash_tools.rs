use std::fmt::{Display, Formatter};

pub const BLACK:u8 = 30;
pub const RED:u8 = 31;
pub const GREEN:u8 = 32;
pub const YELLOW:u8 = 33;
pub const BLUE:u8 = 34;
pub const PURPLE:u8 = 35;
pub const CYAN:u8 = 36;
pub const WHITE:u8 = 37;
pub const LIGHT_BLACK:u8 = 90;
pub const LIGHT_RED:u8 = 91;
pub const LIGHT_GREEN:u8 = 92;
pub const LIGHT_YELLOW:u8 = 93;
pub const LIGHT_BLUE:u8 = 94;
pub const LIGHT_PURPLE:u8 = 95;
pub const LIGHT_CYAN:u8 = 96;
pub const LIGHT_WHITE:u8 = 97;

pub const PLAIN:u8 = 0;
pub const BOLD:u8 = 1;
pub const DIM:u8 = 2;
pub const UNDERLINE:u8 = 4;
pub const BLINKING:u8 = 5;
pub const REVERSED:u8 = 7;

#[derive(Clone, Copy)]
pub enum Fmt {
	Decor(u8),
	Color(u8),
	DecorColor(u8, u8),
	Reset,
}
impl Display for Fmt {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::Decor(decor) => f.write_str(format!(
				"\u{001b}[{}m", decor
			).as_str()),
			Self::Color(color) => f.write_str(format!(
				"\u{001b}[;{}m", color
			).as_str()),
			Self::DecorColor(decor, color) => f.write_str(format!(
				"\u{001b}[{};{}m", decor, color
			).as_str()),
			Self::Reset => f.write_str("\u{001b}[0;0m")
		}
	}
}
