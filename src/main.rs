extern crate visual_rants;

use visual_rants::*;
use std::io::Error;



fn main() {

	let algo = Sort::Algo;
	let recent = Sort::Recent;
	let top = Sort::Top;
	let day = Range::Day;
	let week = Range::Week;
	let month = Range::Month;
	let all = Range::All;


	let limit: &str = "15"; 		// Number of rants to return
	let skip: &str = "1";		// Number of rants to skip


	match get_rants(recent, day, limit, skip) {
		Ok(rants) => prepare_data(rants),
		Err(err) => println!(" Error: {:?}", err),
	}
	
}

