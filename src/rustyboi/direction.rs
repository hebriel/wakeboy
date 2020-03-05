use super::core::*;

#[derive(Copy, Clone)]
pub enum Direction {
	Left,
	Right,
	Unknown,
}

impl std::convert::From<u8> for Direction {
	fn from(bits: u8) -> Self {
		match bits {
			0b0 => Direction::Left,
			0b1 => Direction::Right,
			_ => {
				warn_or_crash(String::from("Program encountered invalid direction denominator"));
				Direction::Unknown
			}
		}
	}
}

impl std::fmt::Display for Direction {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
			Direction::Left => write!(f, "Left"),
			Direction::Right => write!(f, "Right"),
			Direction::Unknown => write!(f, "Unknown Direction"),
		}
    }
}