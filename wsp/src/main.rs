mod pointset;

use pointset::{wsp, PointSet};
use std::process;
use structopt::StructOpt;

/// Set the parameters of the WSP space filling algorithm
#[derive(StructOpt)]
struct Cli {
    /// Output file where the matrix is stored
    #[structopt(short = "o", long = "output", default_value = "wsp.csv")]
    output_file: String,
    /// Algorithm to generate the initial set of candidate points (low impact)
    #[structopt(short = "a", long = "algo", default_value = "random")]
    _initial_algo: String,
    /// Number of points in the initial set of candidate points (major impact)
    #[structopt(short = "n", long = "nb-initial", default_value = "2000")]
    nb_initial: u32,
    /// Minimal distance desired
    #[structopt(short = "d", long = "distance", default_value = "1.0")]
    d_min: f64,
    /// Dimension of the points
    #[structopt(short = "m", long = "dimension", default_value = "20")]
    dim: usize,
    /// Seed for the origin choice and the initialization
    #[structopt(short = "s", long = "seed", default_value = "51")]
    seed: u64,
}

fn main() {
    let args = Cli::from_args();

    let mut points: PointSet = PointSet::init_from_random(args.nb_initial, args.dim, args.seed);
    for i in 0..5 {
        points.print_from_idx(i);
    }
    if let Err(err) = points.save_in_csv("initial.csv") {
        println!("Error writing in CSV: {}", err);
        process::exit(1);
    }

    wsp(&mut points, args.d_min);
    println!("Active: {}", points.nb_active);

    if let Err(err) = points.save_in_csv(&args.output_file) {
        println!("Error writing in CSV: {}", err);
        process::exit(1);
    }
}
