
extern crate reqwest;
extern crate serde;
extern crate serde_json;
extern crate plotlib;

#[macro_use]
extern crate serde_derive;

use std::io::Error;
use std::io::Read;
use reqwest::Client;

use plotlib::scatter::Scatter;
use plotlib::scatter;
use plotlib::style::{Marker, Point};
use plotlib::view::View;
use plotlib::page::Page;




// How rants are sorted
pub enum Sort {
	Algo,
	Top,
	Recent,
}

// Time spectrum of rants
pub enum Range {
	Day,
	Week,
	Month,
	All,
}

// Ensures only correct string values for `get_rants` parameters
impl Sort {
	pub fn as_str(&self) -> &str {
		match self {
			&Sort::Algo => "algo",
			&Sort::Top => "top",
			&Sort::Recent => "recent"
		}
	}
}

impl Range {
	pub fn as_str(&self) -> &str {
		match self {
			&Range::Day => "day",
			&Range::Week => "week",
			&Range::Month => "month",
			&Range::All => "all",
		}
	}
}


#[derive(Serialize, Deserialize, Debug)]
pub struct RantData {
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
pub struct Rant {
	success: bool,
	rants: Vec<RantData>,
}


// Get rants from API
pub fn get_rants(sort_type: Sort, range_type: Range, _limit: &str, _skip: &str) -> Result<Rant, Error> {
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


// Holds lists of tuples for each data set
#[derive(Debug)]
pub struct Points {
	user_rants: Vec<(f64, f64)>,
	user_comments: Vec<(f64, f64)>,
}

impl Points {
	fn new() -> Points {
		Points {
			user_rants: Vec::with_capacity(50),
			user_comments: Vec::with_capacity(50),
		}
	}
}


// Extracts data from each rant of `Rant` and adds to `Point`, a struct containing a list of tuples for each data set
pub fn prepare_data(rant: Rant) {

	let mut all_points = Points::new();

	for i in rant.rants {

		// Add tuple of number of rant upvotes and user upvote count to vector
		let user_rant: (f64, f64) = (i.score.into(), i.user_score.into()); 
		all_points.user_rants.push(user_rant);

		// Add tuple of number of comments and user upvote count to vector
		let user_comment: (f64, f64) = (i.num_comments.into(), i.user_score.into());
		all_points.user_comments.push(user_comment);
	}
	println!("{:?}", &all_points);
	plot(&all_points)
}



fn plot(points: &Points) {

	// Create the scatter plot for user - rant upvotes 
	let s1 = Scatter::from_vec(&points.user_rants)
		.style(scatter::Style::new()
			.marker(Marker::Cross)
			.colour("#DD3355"));

	// Create the scatter plot for user - number of comments
	let s2 = Scatter::from_vec(&points.user_comments)
		.style(scatter::Style::new()
			.colour("#35C788"));


	// The views describe the set of data drawn
	let v1 = View::new()
		.add(&s1)
		.x_range(-10., 100.)			
		.y_range(1., 10000.)			
		.x_label("Rant upvotes")
		.y_label("User upvote count");

	let v2 = View::new()
		.add(&s2)
		.x_range(-10., 50.)				
		.y_range(1., 10000.)
		.x_label("Number of comments")	
		.y_label("User upvote count");	

	// Pages with a single view saved to an SVG file in the root folder
	Page::single(&v1).save("user_rant.svg");

	Page::single(&v2).save("user_comments.svg");
}