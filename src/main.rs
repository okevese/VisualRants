
extern crate reqwest;
extern crate serde;
extern crate serde_json;
extern crate plotlib;

#[macro_use]
extern crate serde_derive;

use std::io::Error;
use std::io::Read;
use reqwest::Client;
use reqwest::Response;
use plotlib::scatter::Scatter;
use plotlib::scatter;
use plotlib::style::{Marker, Point};
use plotlib::view::View;
use plotlib::page::Page;


fn main() {
	// TODO: Pass these variables as command line arguments when running visual_rants
	let algo = Sort::Algo;
	let recent = Sort::Recent;
	let day = Range::Day;

	let limit: &str = "2"; 		// Number of rants to return
	let skip: &str = "1";		// Number of rants to skip

	match get_rants(recent, day, limit, skip) {
		Ok(rants) => graph_data(rants),
		Err(err) => println!(" Error: {:?}", err),
	}
	
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
fn get_rants(sort_type: Sort, range_type: Range, _limit: &str, _skip: &str) -> Result<Rant, Error> {
	let sort_type = sort_type.as_str();
	let range_type = range_type.as_str();

	let client = Client::new();
	let mut body = String::new();
	let mut res = client.get("https://devrant.com/api/devrant/rants?app=3")
		.query(&[("sort", sort_type), ("range", range_type), ("limit", _limit), ("skip", _skip)])
		.send()
		.unwrap();
	assert!(res.status().is_success());
	res.read_to_string(&mut body)?;

	// Deserializes JSON. Parses string of data into `Rant` object
	let rant_data: Rant = serde_json::from_str(&body)?;


	Ok(rant_data)
}


fn graph_data(rant: Rant) {
	let mut points: Vec<(f64, f64)> = Vec::new();

	// Adds the user upvote count and rant upvotes to an array of tuples for plotting
	for i in rant.rants {

		// Creates tuple and casts `i32` of user and rant counts into `f64`
		let user_rant: (f64, f64) = (i.score.into(), i.user_score.into()); 
		points.push(user_rant);
	}
	println!("{:?}", points);
	plot(points)
}



fn plot(points: Vec<(f64, f64)>) {

	// Create the scatter plot from the data
	let s1 = Scatter::from_vec(&points)
		.style(scatter::Style::new()
			.marker(Marker::Square)
			.colour("#DD3355"));


	// The view describes the set of data drawn
	let v = View::new()
		.add(&s1)
		.x_range(1., 100.)
		.y_range(100., 2000.)
		.x_label("rant upvotes")
		.y_label("User upvote count");

	// Page with a single view saved to an SVG file
	Page::single(&v).save("rant_stat.svg");
}