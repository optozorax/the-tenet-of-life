use std::convert::TryInto;
use std::convert::TryFrom;
use std::str::FromStr;
use fntools::value::*;

#[derive(Debug, Clone, Hash, Copy, Ord, PartialOrd, Eq, PartialEq)]
pub enum Color3 {
	White,
	Blue,
	Red,
}

impl From<Color3> for u8 {
	fn from(color: Color3) -> Self {
		use Color3::*;
		match color {
			White => 0,
			Blue => 1,
			Red => 2,
		}
	}
}

impl TryFrom<u8> for Color3 {
	type Error = ();

	fn try_from(number: u8) -> Result<Self, Self::Error> {
		use Color3::*;
		match number {
			0 => Ok(White),
			1 => Ok(Blue),
			2 => Ok(Red),
			_ => Err(()),
		}
	}
} 

impl Color3 {
	pub fn invert(self) -> Color3 {
		use Color3::*;
		match self {
			White => White,
			Blue => Red,
			Red => Blue,
		}
	}
}

#[derive(Debug, Clone, Hash, Copy, Ord, PartialOrd, Eq, PartialEq)]
pub struct Square3(u8);

#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq)]
pub struct SquareColors3(pub [Color3; 4]);

impl From<Square3> for SquareColors3 {
	fn from(mut square: Square3) -> Self {
		if square.0 >= 81 {
			panic!("Square3 value must be under 81, square.0: {}", square.0);
		}

		use Color3::*;
		let mut result = SquareColors3([White, White, White, White]);
		let mut position = 4;
		while square.0 > 0 {
			let current = square.0 % 3;
			position -= 1;
			result.0[position] = current.try_into().unwrap();
			square.0 /= 3;
		}
		result
	}
}

impl From<SquareColors3> for Square3 {
	fn from(square_colors: SquareColors3) -> Self {
		if square_colors.0.len() != 4 {
			panic!("SquareColors3 must have len 4");
		}

		square_colors.0
			.iter()
			.enumerate()
			.map(|(index, color)| (3u32-index as u32, u8::from(*color)))
			.map(|(power, apply)| 3u8.pow(power) * apply)
			.sum::<u8>()
			.apply(Square3)
	}
}

impl FromStr for Square3 {
	type Err = ();

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		use Color3::*;

		let mut result = SquareColors3([White, White, White, White]);

		s
			.chars()
			.map(|c| {
				match c {
					'0' => White,
					'1' => Blue,
					'2' => Red,
					_ => unreachable!(),
				}
			})
			.enumerate()
			.for_each(|(index, v)| result.0[index] = v);

		Square3::from(result).apply(Ok)
	}
}

impl Square3 {
	pub fn invert(self) -> Square3 {
		let mut result: SquareColors3 = self.into();
		result.0.iter_mut().for_each(|i| *i = i.invert());
		result.into()
	}
}

#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq)]
pub struct Rules3(Vec<Square3>);

impl Rules3 {
	pub fn is_correct(&self) -> bool {
		self.0.len() == 81 &&
			{ 
				let mut to_sort = self.0.clone();
				to_sort.sort();
				to_sort.into_iter().eq((0..81).map(Square3))
			}
	}

	pub fn pairs(&self) -> Vec<(Square3, Square3)> {
		if !self.is_correct() {
			panic!("Rules3 is incorrect");
		}
		self.0
			.iter()
			.enumerate()
			.map(|(index, val)| (Square3(index.try_into().unwrap()), *val))
			.collect()
	}

	pub fn from_pairs(pairs: Vec<(Square3, Square3)>) -> Self {
		pairs
			.also_mut(|x| x.sort_by(|a, b| a.0.cmp(&b.0)))
			.into_iter()
			.enumerate()
			.map(|(index, (pos, value))| {
				if index != pos.0.into() {
					panic!("Pairs3 is incorrect, {} != {}", index, pos.0);
				}
				value
			})
			.collect::<Vec<_>>()
			.apply(Rules3)
			.also_mut(|rules| {
				if !rules.is_correct() {
					panic!("Pairs3 is incorrect");
				}
			})
	}
}