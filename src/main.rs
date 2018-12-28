extern crate visual_rants;
#[macro_use]
extern crate diesel;
extern crate dotenv;

use visual_rants::*;

mod db;

fn main() {
	/*let algo = Sort::Algo;
	let recent = Sort::Recent;
	let top = Sort::Top;
	let day = Range::Day;
	let week = Range::Week;
	let month = Range::Month;
	let all = Range::All;



	let limit: &str = "5"; 		// Number of rants to return
	let skip: &str = "0";		// Number of rants to skip */

	db::connect();
}
