
extern crate structopt;
extern crate visual_rants;

use structopt::StructOpt;
use visual_rants::*;
use visual_rants::params::*;




fn main() {

    
    let algo = Sort::Algo;
    let recent = Sort::Recent;
    let top = Sort::Top;
    let day = Range::Day;
    let week = Range::Week;
    let month = Range::Month;
    let all = Range::All;
    


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

    let args = Cli::from_args();
    println!("{:?}", args);
    println!("{:?}", args.range);



    match get_rants(algo, day, limit, skip) {
        Ok(rants) => plot(prepare_data(rants)),
        
        Err(err) => println!(" Error: {:?}", err),
    }
    
}

