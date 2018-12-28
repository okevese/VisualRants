extern crate structopt;
extern crate visual_rants;
#[macro_use]
extern crate diesel;
extern crate dotenv;

use structopt::StructOpt;
use visual_rants::params::*;
use visual_rants::*;

mod db;

fn main() {
    /// See visuals of rants based on input values
    #[derive(StructOpt, Debug)]
    #[structopt(name = "VisualRants", about = "See a visualization of rants.")]
    struct Cli {
        /// How rants are sorted
        #[structopt(long = "sort", short = "s")]
        sort: String,

        /// Time period to fetch rants from
        #[structopt(long = "range", short = "r")]
        range: String,

        /// Number of rants to fetch
        #[structopt(long = "limit", short = "l")]
        limit: String,

        /// Number of rants to skip
        #[structopt(long = "skip", default_value = "0")]
        skip: String,
    }

    let mut sort_type = "";
    let mut range_type = "";

    let args = Cli::from_args();

    let mut compare_query_values = |args: &Cli| {
        match args.sort.as_ref() {
            Sort::RECENT => sort_type = Sort::RECENT,
            Sort::ALGO => sort_type = Sort::ALGO,
            Sort::TOP => sort_type = Sort::TOP,
            _ => {
                panic!("No sort match");
            }
        }

        match args.range.as_ref() {
            Range::DAY => range_type = Range::DAY,
            Range::WEEK => range_type = Range::WEEK,
            Range::MONTH => range_type = Range::MONTH,
            Range::ALL => range_type = Range::ALL,
            _ => {
                panic!("No range match");
            }
        }
    };

    compare_query_values(&args);

    let limit: &str = &args.limit.as_ref();
    let skip: &str = &args.skip.as_ref();

    match get_rants(sort_type, range_type, limit, skip) {
        Ok(rants) => plot(prepare_data(rants)),

        Err(err) => eprintln!(" Error: {:?}", err),
    }

    db::establish_connection();
}
