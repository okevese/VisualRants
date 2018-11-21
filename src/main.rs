extern crate reqwest;
extern crate select;

use select::document::Document;
use select::predicate::Name;


fn main() {
    rant_feed("https://devrant.com/feed");
}


fn rant_feed(url: &str) {
	let res = reqwest::get(url).unwrap();
	assert!(res.status().is_success());

	Document::from_read(res)
		.unwrap()
		.find(Name("a"))
		.filter_map(|n| n.attr("href"))
		.for_each(|x| println!("{:?}", x)); // TODO: filter for rant tags here <==

}