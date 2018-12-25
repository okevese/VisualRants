
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

    let args = Cli::from_args();

    let mut compare_query_values = |args: &Cli| {
        
        match args.sort.as_ref() {
            Sort::RECENT =>  sort_type = Sort::RECENT,
            Sort::ALGO => sort_type = Sort::ALGO,
            Sort::TOP => sort_type = Sort::TOP,
            _ => eprintln!("No sort match")
        }

        match args.range.as_ref() {
            Range::DAY => range_type = Range::DAY,
            Range::WEEK => range_type = Range::WEEK,
            Range::MONTH => range_type = Range::MONTH,
            Range::ALL => range_type = Range::ALL,
            _ => eprintln!("No range match")
        }
    };

    compare_query_values(&args);

    let limit = &args.limit.as_ref();
    let skip = &args.skip.as_ref();

    match get_rants(sort_type, range_type, limit, skip) {
        Ok(rants) => plot(prepare_data(rants)),
        
        Err(err) => eprintln!(" Error: {:?}", err),
    }
    
}

