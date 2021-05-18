use std::hash::Hash;
use std::convert::TryInto;
use std::convert::TryFrom;
use fntools::value::*;
use crate::color3::*;

#[derive(Debug, Clone, Hash, Copy, Ord, PartialOrd, Eq, PartialEq)]
pub struct Color2(bool);

impl From<Color2> for u8 {
	fn from(color: Color2) -> Self {
		color.0.into()
	}
}

impl TryFrom<u8> for Color2 {
	type Error = ();

	fn try_from(number: u8) -> Result<Self, Self::Error> {
		match number {
			0 => Ok(Color2(false)),
			1 => Ok(Color2(true)),
			_ => Err(()),
		}
	}
} 

impl Color2 {
	pub fn invert(self) -> Color2 {
		Color2(!self.0)
	}

	pub fn to_blue(self) -> Color3 {
		if self.0 {
			Color3::Blue
		} else {
			Color3::White
		}
	}

	pub fn to_red(self) -> Color3 {
		if self.0 {
			Color3::Red
		} else {
			Color3::White
		}
	}
}

#[derive(Debug, Clone, Hash, Copy, Ord, PartialOrd, Eq, PartialEq)]
pub struct Square2(u8);

impl Square2 {
	pub fn new(val: u8) -> Self {
		if val >= 16 {
			panic!("Square2 value must be under 16");
		}
		Square2(val)
	}
}

#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq)]
pub struct SquareColors2([Color2; 4]);

impl From<Square2> for SquareColors2 {
	fn from(mut square: Square2) -> Self {
		if square.0 >= 16 {
			panic!("Square2 value must be under 16, square.0: {}", square.0);
		}

		let mut result = SquareColors2([Color2(false), Color2(false), Color2(false), Color2(false)]);
		let mut position = 4;
		while square.0 > 0 {
			let current = square.0 % 2;
			position -= 1;
			result.0[position] = current.try_into().unwrap();
			square.0 /= 2;
		}
		result
	}
}

impl From<SquareColors2> for Square2 {
	fn from(square_colors: SquareColors2) -> Self {
		if square_colors.0.len() != 4 {
			panic!("SquareColors3 must have len 4");
		}

		square_colors.0
			.iter()
			.enumerate()
			.map(|(index, color)| (3u32-index as u32, u8::from(*color)))
			.map(|(power, apply)| 2u8.pow(power) * apply)
			.sum::<u8>()
			.apply(Square2)
	}
}

impl Square2 {
	pub fn invert(self) -> Square2 {
		let mut result: SquareColors2 = self.into();
		result.0.iter_mut().for_each(|i| *i = i.invert());
		result.into()
	}
}

impl SquareColors2 {
	pub fn to_blue(&self) -> SquareColors3 {
		let mut result = SquareColors3([Color3::White, Color3::White, Color3::White, Color3::White]);
		self.0.iter().enumerate().for_each(|(index, value)| result.0[index] = value.to_blue());
		result
	}

	pub fn to_red(&self) -> SquareColors3 {
		let mut result = SquareColors3([Color3::White, Color3::White, Color3::White, Color3::White]);
		self.0.iter().enumerate().for_each(|(index, value)| result.0[index] = value.to_red());
		result
	}
}

#[derive(Debug, Clone, Hash, Ord, PartialOrd, Eq, PartialEq)]
pub struct Rules2(Vec<Square2>);

impl Rules2 {
	pub fn new(vec: &[u8]) -> Self {
		vec
			.iter()
			.enumerate()
			.map(|(index, value)| (Square2::new(index.try_into().unwrap()), Square2::new(*value)))
			.collect::<Vec<_>>()
			.apply(Rules2::from_pairs)
	}

	pub fn is_correct(&self) -> bool {
		self.0.len() == 16 &&
			{ 
				let mut to_sort = self.0.clone();
				to_sort.sort();
				to_sort.into_iter().eq((0..16).map(Square2))
			}
	}

	pub fn pairs(&self) -> Vec<(Square2, Square2)> {
		if !self.is_correct() {
			panic!("Rules2 is incorrect");
		}
		self.0
			.iter()
			.enumerate()
			.map(|(index, val)| (Square2(index.try_into().unwrap()), *val))
			.collect()
	}

	pub fn from_pairs(pairs: Vec<(Square2, Square2)>) -> Self {
		pairs
			.also_mut(|x| x.sort_by(|a, b| a.0.cmp(&b.0)))
			.into_iter()
			.enumerate()
			.map(|(index, (pos, value))| {
				if index != pos.0.into() {
					panic!("Pairs2 is incorrect");
				}
				value
			})
			.collect::<Vec<_>>()
			.apply(Rules2)
			.also(|rules| {
				if !rules.is_correct() {
					dbg!(&rules);
					panic!("Pairs2 is incorrect");
				}
			})
	}

	pub fn to_simple_vec(&self) -> Vec<usize> {
		self.pairs().into_iter().map(|(_, value)| value.0.into()).collect::<Vec<_>>()
	}

	pub fn inverted_step1(&self) -> Self {
		self
			.pairs()
			.into_iter()
			.map(|(pos, value)| (pos, value.invert()))
			.collect::<Vec<_>>()
			.apply(Rules2::from_pairs)
	}

	pub fn inverted_step2(&self) -> Self {
		self
			.pairs()
			.into_iter()
			.map(|(pos, value)| (pos.invert(), value))
			.collect::<Vec<_>>()
			.apply(Rules2::from_pairs)
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test() {
		assert_eq!(Square2(0), SquareColors2([Color2(false), Color2(false), Color2(false), Color2(false)]).into());
		assert_eq!(Square2(1), SquareColors2([Color2(false), Color2(false), Color2(false), Color2(true)]).into());
		assert_eq!(Square2(2), SquareColors2([Color2(false), Color2(false), Color2(true), Color2(false)]).into());
		assert_eq!(Square2(8), SquareColors2([Color2(true), Color2(false), Color2(false), Color2(false)]).into());
		assert_eq!(Square2(11), SquareColors2([Color2(true), Color2(false), Color2(true), Color2(true)]).into());

		assert_eq!(SquareColors2([Color2(false), Color2(false), Color2(false), Color2(false)]), Square2(0).into());
		assert_eq!(SquareColors2([Color2(false), Color2(false), Color2(false), Color2(true)]), Square2(1).into());
		assert_eq!(SquareColors2([Color2(false), Color2(false), Color2(true), Color2(false)]), Square2(2).into());
		assert_eq!(SquareColors2([Color2(true), Color2(false), Color2(false), Color2(false)]), Square2(8).into());
		assert_eq!(SquareColors2([Color2(true), Color2(false), Color2(true), Color2(true)]), Square2(11).into());
	}
}