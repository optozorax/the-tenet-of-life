use the_tenet_of_life::{rules_invert_second_step, rules_calc_tenet};

fn main() {
	//let rules = vec![15,14,13,3,11,5,6,1,7,9,10,2,12,4,8,0]; // critters
	let rules = vec![15,1,2,3,4,5,6,7,8,9,10,11,12,13,14,0]; // tron
	dbg!(rules_invert_second_step(&rules));
	dbg!(rules_calc_tenet(&rules));

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
