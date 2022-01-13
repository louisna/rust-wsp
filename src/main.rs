use std::process;
use structopt::StructOpt;
use wsp::{adaptive_wsp, wsp, PointSet};

/// Set the parameters of the WSP space filling algorithm
#[derive(StructOpt)]
struct Cli {
    /// Output file where the matrix is stored after WSP
    #[structopt(short = "o", long = "output", default_value = "wsp.csv")]
    output_file: String,
    /// Output file where the matrix is stored before WSP
    #[structopt(short = "i", long = "initial")]
    output_file_before: Option<String>,
    /// Algorithm to generate the initial set of candidate points (low impact)
    #[structopt(short = "a", long = "algo", default_value = "random")]
    _initial_algo: String,
    /// Number of points in the initial set of candidate points (major impact)
    #[structopt(short = "n", long = "nb-initial", default_value = "2000")]
    nb_initial: usize,
    /// Minimal distance desired
    #[structopt(short = "d", long = "distance", default_value = "1.0")]
    d_min: f64,
    /// Dimension of the points
    #[structopt(short = "m", long = "dimension", default_value = "20")]
    dim: usize,
    /// Seed for the origin choice and the initialization
    #[structopt(short = "s", long = "seed", default_value = "51")]
    seed: u64,
    /// Use adaptive algorithm instead of distance input to reach <nb-target> active points in the space
    #[structopt(long = "adaptive")]
    nb_target: Option<usize>,
    /// Display debug information. Only for adaptive WSP
    #[structopt(short = "v", long = "verbose")]
    verbose: bool,
}

fn main() {
    let args = Cli::from_args();

    let mut points: PointSet = PointSet::init_from_random(args.nb_initial, args.dim, args.seed);

    if let Some(filename) = args.output_file_before {
        if let Err(err) = points.save_in_csv(&filename) {
            println!("Error writing in CSV: {}", err);
            process::exit(1);
        }
    }

    match args.nb_target {
        Some(obj_nb) => adaptive_wsp(&mut points, obj_nb, args.verbose),
        None => wsp(&mut points, args.d_min),
    }

    if let Err(err) = points.save_in_csv(&args.output_file) {
        println!("Error writing in CSV: {}", err);
        process::exit(1);
    }

    println!("Nb active: {}", points.nb_active);
}
