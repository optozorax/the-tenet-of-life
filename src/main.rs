use std::collections::HashMap;
use std::str::FromStr;
use std::convert::TryInto;
use the_tenet_of_life::color2::*;
use the_tenet_of_life::color3::*;
use fntools::value::*;

fn main() {
	let critters_array = [
		0b1111, 0b1110, 0b1101, 0b0011,
		0b1011, 0b0101, 0b0110, 0b0001,
		0b0111, 0b1001, 0b1010, 0b0010,
		0b1100, 0b0100, 0b1000, 0b0000,
	]
		.iter()
		.enumerate()
		.map(|(index, value)| (Square2::new(index.try_into().unwrap()), Square2::new(*value)))
		.collect::<Vec<_>>()
		.apply(Rules2::from_pairs);

	let inverted_critters_step1 = critters_array
		.pairs()
		.into_iter()
		.map(|(pos, value)| (pos, value.invert()))
		.collect::<Vec<_>>()
		.apply(Rules2::from_pairs);

	let inverted_critters_step2 = critters_array
		.pairs()
		.into_iter()
		.map(|(pos, value)| (pos.invert(), value))
		.collect::<Vec<_>>()
		.apply(Rules2::from_pairs);

	let mut result_step1 = Vec::new();
	let mut result_step2 = Vec::new();

	inverted_critters_step1
		.pairs()
		.into_iter()
		.map(|(pos, value)| {
			(
				Square3::from(SquareColors2::from(pos).to_blue()),
				Square3::from(SquareColors2::from(value).to_blue()),
			)
		})
		.apply(|iter| result_step1.extend(iter));

	inverted_critters_step2
		.pairs()
		.into_iter()
		.map(|(pos, value)| {
			(
				Square3::from(SquareColors2::from(pos).to_blue()),
				Square3::from(SquareColors2::from(value).to_blue()),
			)
		})
		.apply(|iter| result_step2.extend(iter));

	inverted_critters_step2
		.pairs()
		.into_iter()
		.filter(|(_, value)| *value != Square2::new(0))
		.map(|(pos, value)| {
			(
				Square3::from(SquareColors2::from(value).to_red()),
				Square3::from(SquareColors2::from(pos).to_red()),
			)
		})
		.apply(|iter| result_step1.extend(iter));

	inverted_critters_step1
		.pairs()
		.into_iter()
		.filter(|(_, value)| *value != Square2::new(0))
		.filter(|(pos, value)| {
			if *pos == Square2::new(0) {
				dbg!(Square3::from(SquareColors2::from(*value).to_red()));
			}
			true
		})
		.map(|(pos, value)| {
			(
				Square3::from(SquareColors2::from(value).to_red()),
				Square3::from(SquareColors2::from(pos).to_red()),
			)
		})
		.apply(|iter| result_step2.extend(iter));

	let red_blue_up   = ["1112", "1211", "1221", "1122", "1212", "1222", "2212"];
	let red_blue_down = ["2111", "1121", "2112", "2211", "2121", "2221", "2122"];

	let red_blue_white_2_up   = ["1002", "0210", "0012", "2100", "2010", "0201"];
	let red_blue_white_2_down = ["2001", "0120", "0021", "1200", "1020", "0102"];

	let red_blue_white_1_up   = ["0211", "0112", "0121", "0212", "0221", "0122", "1012", "2012", "2011", "1021", "1022", "2021"];
	let red_blue_white_1_down = ["1120", "2110", "1210", "2120", "1220", "2210", "2101", "2102", "1102", "1201", "2201", "1202"];

	red_blue_up
		.iter()
		.zip(red_blue_down.iter())
		.map(|(a, b)| (Square3::from_str(a).unwrap(), Square3::from_str(b).unwrap()))
		.map(|(a, b)| vec![(a, b), (b, a)].into_iter())
		.flatten()
		.apply(|iter| {
			result_step1.extend(iter.clone());
			result_step2.extend(iter);
		});

	red_blue_white_2_up
		.iter()
		.zip(red_blue_white_2_down.iter())
		.map(|(a, b)| (Square3::from_str(a).unwrap(), Square3::from_str(b).unwrap()))
		.map(|(a, b)| vec![(a, b), (b, a)].into_iter())
		.flatten()
		.apply(|iter| {
			result_step1.extend(iter.clone());
			result_step2.extend(iter);
		});

	red_blue_white_1_up
		.iter()
		.zip(red_blue_white_1_down.iter())
		.map(|(a, b)| (Square3::from_str(a).unwrap(), Square3::from_str(b).unwrap()))
		.map(|(a, b)| vec![(a, b), (b, a)].into_iter())
		.flatten()
		.apply(|iter| {
			result_step1.extend(iter.clone());
			result_step2.extend(iter);
		});

	let rules_step1 = Rules3::from_pairs(result_step1);
	let rules_step2 = Rules3::from_pairs(result_step2);

	let symmetric1 = rules_step2
		.pairs()
		.into_iter()
		.map(|(pos, value)| (value.invert(), pos.invert()))
		.collect::<HashMap<_, _>>()
		.eq(&rules_step1.pairs().into_iter().collect::<HashMap<_, _>>());

	let symmetric2 = rules_step1
		.pairs()
		.into_iter()
		.map(|(pos, value)| (value.invert(), pos.invert()))
		.collect::<HashMap<_, _>>()
		.eq(&rules_step2.pairs().into_iter().collect::<HashMap<_, _>>());

	dbg!(symmetric1, symmetric2);

	/*
		inverted critters step 1
		inverted critters step 2

		inverted backward critters step 1 из inverted critters step 2
			inverted backward critters step 2 из inverted critters step 3

		красно-синие разворотом на 180 градусов

		два белых и красный и синий как нарисовал

		один белый и в сумме красный и синий хз как

		проверить симметрии:
			* step 1 точно такой же как step 2 в обратную сторону, но с инвертированными красными и синими
			* step 2 точно такой же как step 1 в обратную сторону, но с инвертированными красными и синими
			* количество клеток сохраняется
			* правила работают одинаково во все стороны
	*/
}
