mod pointset;

use std::env;
use structopt::StructOpt;
use pointset::{PointSet, wsp};
use csv;

/// Set the parameters of the WSP space filling algorithm
#[derive(StructOpt)]
struct CLI {
    /// Output file where the matrix is stored
    #[structopt(short = "o", long = "output", default_value = "wsp.csv")]
    output_file: String,
    /// Algorithm to generate the initial set of candidate points (low impact)
    #[structopt(short = "a", long = "algo", default_value = "random")]
    initial_algo: String,
    /// Number of points in the initial set of candidate points (major impact)
    #[structopt(short = "n", long = "nb-initial", default_value = "2000")]
    nb_initial: u32,
}

fn main() {
    let args = CLI::from_args();
    println!("Before");
    let mut points: PointSet = PointSet::init_from_random(args.nb_initial, 4);
    for i in 0..5 {
        points.print_from_idx(i);
    }

    wsp(&mut points, 0.75);
    println!("{}", points.nb_active);


}