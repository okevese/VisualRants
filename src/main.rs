
extern crate reqwest;
extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

use std::io::Read;
use reqwest::Client;
use reqwest::Response;


fn main() {
	// TODO: Pass these variables as command line arguments when running visual_rants
	let algo = Sort::Algo;

	let day = Range::Day;

	let limit: &str = "1";
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




#[derive(Serialize, Deserialize, Debug)]
struct RantData {
	id: i32,
	text: String,
	score: i32,
	created_time: i32,
	num_comments: i32,
	tags: Vec<String>,
	vote_state: i32,
	edited: bool,
	user_username: String,
	user_score: i32,
}


#[derive(Serialize, Deserialize, Debug)]
struct Rant {
	success: bool,
	rants: Vec<RantData>,
}


// Get rants from API
fn get_rants(sort_type: Sort, range_type: Range, _limit: &str, _skip: &str) {
	let sort_type = sort_type.as_str();
	let range_type = range_type.as_str();

	let client = Client::new();
	let mut body = String::new();
	let mut res = client.get("https://devrant.com/api/devrant/rants?app=3")
		.query(&[("sort", sort_type), ("range", range_type), ("limit", _limit), ("skip", _skip)])
		.send()
		.unwrap();
	res.read_to_string(&mut body).unwrap();

	// Deserializes JSON. Parses string of data into `Rant` object
	let data: Rant = serde_json::from_str(&body).unwrap();

	println!("Body:\n{:?}", data);
}