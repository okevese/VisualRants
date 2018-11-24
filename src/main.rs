extern crate reqwest;

use std::io::Read;



fn main() {
	let algo = Sort::Algo;

	let day = Range::Day;

	get_rants(algo, day, 4, 0);
	
}

// How rants are sorted
enum Sort {
	Algo,
	Top,
	Recent,
}

// Time spectrum of rants
enum Range {
	Day,
	Week,
	Month,
	All,
}

// Ensures only correct string values for `get_rants` parameters
impl Sort {
	fn as_str(&self) -> &str {
		match self {
			&Sort::Algo => "algo",
			&Sort::Top => "top",
			&Sort::Recent => "recent"
		}
	}
}

impl Range {
	fn as_str(&self) -> &str {
		match self {
			&Range::Day => "day",
			&Range::Week => "week",
			&Range::Month => "month",
			&Range::All => "all",
		}
	}
}

// limit: i32, skip: i32
fn get_rants(sort_type: Sort, range_type: Range, _limit: i32, _skip: i32) {
	let sort_type = sort_type.as_str();
	let range_type = range_type.as_str();

	// TODO: pass params to this URL
	let url: &str = "https://devrant.com/api/devrant/rants?app=3&sort=top&range=day&limit=6&skip=0";
}