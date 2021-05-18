pub mod color2;
pub mod color3;

use crate::color2::*;
use crate::color3::*;
use fntools::value::*;

use std::collections::HashSet;
use std::collections::HashMap;
use std::str::FromStr;

pub fn rules_invert_second_step(vec: &[usize]) -> (Vec<usize>, Vec<usize>) {
	let critters_array = Rules2::new(&vec.iter().map(|x| *x as u8).collect::<Vec<u8>>());

	(critters_array.inverted_step1().to_simple_vec(), critters_array.inverted_step2().to_simple_vec())
}

pub fn rules_calc_tenet(vec: &[usize]) -> (Vec<usize>, Vec<usize>) {
	let critters_array = Rules2::new(&vec.iter().map(|x| *x as u8).collect::<Vec<u8>>());

	let inverted_critters_step1 = critters_array.inverted_step1();
	let inverted_critters_step2 = critters_array.inverted_step2();

	let mut result_step1 = HashSet::new();
	let mut result_step2 = HashSet::new();

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
		.map(|(pos, value)| {
			(
				Square3::from(SquareColors2::from(value).to_red()),
				Square3::from(SquareColors2::from(pos).to_red()),
			)
		})
		.apply(|iter| result_step2.extend(iter));

	let both_steps_common_rules = {
		let mut result = Vec::new();

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
			.apply(|iter| result.extend(iter));

		red_blue_white_2_up
			.iter()
			.zip(red_blue_white_2_down.iter())
			.map(|(a, b)| (Square3::from_str(a).unwrap(), Square3::from_str(b).unwrap()))
			.map(|(a, b)| vec![(a, b), (b, a)].into_iter())
			.flatten()
			.apply(|iter| result.extend(iter));

		red_blue_white_1_up
			.iter()
			.zip(red_blue_white_1_down.iter())
			.map(|(a, b)| (Square3::from_str(a).unwrap(), Square3::from_str(b).unwrap()))
			.map(|(a, b)| vec![(a, b), (b, a)].into_iter())
			.flatten()
			.apply(|iter| result.extend(iter));

		result
	};

	result_step1.extend(both_steps_common_rules.clone());
	result_step2.extend(both_steps_common_rules);

	let rules_step1 = Rules3::from_pairs(result_step1.into_iter().collect());
	let rules_step2 = Rules3::from_pairs(result_step2.into_iter().collect());

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

	assert!(symmetric1);
	assert!(symmetric2);

	(rules_step1.to_simple_vec(), rules_step2.to_simple_vec())
}