![GitHub](https://img.shields.io/github/license/louisna/rust-wsp) ![Crates.io](https://img.shields.io/crates/v/wsp) ![docs.rs](https://img.shields.io/docsrs/wsp)

# rust-wsp

A rust implementation of the WSP space filling algorithm.

This is based on the paper from J. Santiago _et al_:
```raw
[1] Santiago, J., Claeys-Bruno, M., & Sergent, M. (2012). Construction of space-filling designs using WSP algorithm for high dimensional spaces. Chemometrics and Intelligent Laboratory Systems, 113, 26-31.
```

---

## Usage

Add the following line to the `Cargo.toml` file:
```toml
[dependencies]
wsp = "0.1.2"
```
## Use cases

A space-filling algorithm enables to remove points to close to each other in a given space. Given a minimal distance `d_min` and an initial set of points, wsp builds a subset where all remaining points are at least `d_min` distant from each other.

wsp also provides an alternative version of the classical WSP algorithm. Certain scenarios do not have any clue about the `d_min` to choose, but require a given number of remaining points in the subset. `adaptive_wsp` search the best `d_min` to create a subset of a target number of points.

## Example

### WSP

The following example generates an initial set of 1000 points from a uniform random distribution, in a 20-dimensions space. The generation uses the seed 51. The minimal distance is arbitrarily set to 3.0.

```rust
// Generates the initial set
let mut points = wsp::PointSet::init_from_random(1000, 20, 51);

// Only keep distant enough points
let d_min = 3.0;
wsp::wsp(&mut points, d_min);

// Iterate over the remaining points
for valid_point in points.get_remaining() {
    println!("{:?}", valid_point);
}
```

### Adaptive WSP

The next example uses the `adaptive_wsp` function with verbose mode. The initial set is similar to the previous example thanks to the seed. We aim to find a minimal distance such that the resulting set only contains 100 points.

```rust
// Generates the initial set
let mut points = wsp::PointSet::init_from_random(1000, 20, 51);

// Adaptive WSP makes a binary search to reach the target
// number of remaining points
let objective_nb: usize = 100;
wsp::adaptive_wsp(&mut points, objective_nb, false);

// Save the result in a CSV file
if let Err(err) = points.save_in_csv("wsp.csv") {
    println!("Error writing in CSV: {}", err);
    std::process::exit(1);
}
```