
extern crate structopt;
extern crate visual_rants;

use structopt::StructOpt;
use visual_rants::*;
use visual_rants::params::*;




fn main() {
    
    let algo = Sort::ALGO;
    //let recent = Sort::RECENT;
    let top = Sort::TOP;
    let day = Range::DAY;
    let week = Range::WEEK;
    let month = Range::MONTH;
    let all = Range::ALL;
    


    let limit: &str = "5";      // Number of rants to return
    let skip: &str = "0";       // Number of rants to skip

    
    #[derive(StructOpt, Debug)]
    #[structopt(name = "VisualRants", about = "See a visualization of rants.")]
    struct Cli {
        sort: String,
        range: String,
        limit: String,
        skip: String,
    }


    impl Cli {
        fn create_request_params() -> Cli {
            let args = Cli::from_args();
            match args.sort.as_ref() {
                Sort::RECENT => {
                    println!("YES, A MATCH! YAAAAY");
                    args
                },
                _ => {
                    println!("No match");
                    args
                }
            }
        }
    }
    println!("{:?}", Cli::create_request_params());

    match get_rants(algo, day, limit, skip) {
        Ok(rants) => plot(prepare_data(rants)),
        
        Err(err) => println!(" Error: {:?}", err),
    }
    
}

