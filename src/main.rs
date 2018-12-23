
extern crate structopt;
extern crate visual_rants;

use structopt::StructOpt;
use visual_rants::*;
use visual_rants::params::*;




fn main() {

    #[derive(StructOpt, Debug)]
    #[structopt(name = "VisualRants", about = "See a visualization of rants.")]
    struct Cli {
        sort: String,
        range: String,
        limit: String,
        skip: String,
    }


    
    let mut sort_type = "";
    let mut range_type = "";
    let mut limit: &'a str = "";
    let mut skip = "";

    let mut compare_query_values = || {
        let args = Cli::from_args();
        match args.sort.as_ref() {
            Sort::RECENT =>  sort_type = Sort::RECENT,
            Sort::ALGO => sort_type = Sort::ALGO,
            Sort::TOP => sort_type = Sort::TOP,
            _ => println!("No sort match")
        }

        match args.range.as_ref() {
            Range::DAY => range_type = Range::DAY,
            Range::WEEK => range_type = Range::WEEK,
            Range::MONTH => range_type = Range::MONTH,
            Range::ALL => range_type = Range::ALL,
            _ => println!("No range match")
        }

        limit = args.limit.as_ref();
        skip = args.skip.as_ref();
    };


    compare_query_values();

    match get_rants(sort_type, range_type, limit, skip) {
        Ok(rants) => plot(prepare_data(rants)),
        
        Err(err) => println!(" Error: {:?}", err),
    }
    
}

