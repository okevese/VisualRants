
extern crate reqwest;
extern crate serde;
extern crate serde_json;

use std::io::Read;
use reqwest::Client;
use reqwest::Response;


fn main() {
	let algo = Sort::Algo;

	let day = Range::Day;

	let limit: &str = "4";
	let skip: &str = "0";

	get_rants(algo, day, limit, skip);
	
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


struct Rant {
	origin: String,
}

// limit: i32, skip: i32
fn get_rants(sort_type: Sort, range_type: Range, _limit: &str, _skip: &str) {
	let sort_type = sort_type.as_str();
	let range_type = range_type.as_str();

	let client = reqwest::Client::new();
	let res = client.get("https://devrant.com/api/devrant/rants?app=3")
		.query(&[("sort", sort_type), ("range", range_type), ("limit", _limit), ("skip", _skip)])
		.send()
		.unwrap();

	println!("{:?}", res);
}