extern crate visual_rants;

use visual_rants::*;



fn main() {

	let algo = Sort::Algo;
	let recent = Sort::Recent;
	let top = Sort::Top;
	let day = Range::Day;
	let week = Range::Week;
	let month = Range::Month;
	let all = Range::All;
	


	let limit: &str = "5"; 		// Number of rants to return
	let skip: &str = "0";		// Number of rants to skip


	match get_rants(algo, day, limit, skip) {
		Ok(rants) => plot(prepare_data(rants)),
		
		Err(err) => println!(" Error: {:?}", err),
	}
	
}

