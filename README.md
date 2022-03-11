![GitHub](https://img.shields.io/github/license/louisna/rust-wsp) ![Crates.io](https://img.shields.io/crates/v/wsp) ![docs.rs](https://img.shields.io/docsrs/wsp)

# rust-wsp

A rust implementation of the WSP space filling algorithm.

This is based on the paper from J. Santiago _et al_:
```raw
[1] Santiago, J., Claeys-Bruno, M., & Sergent, M. (2012). Construction of space-filling designs using WSP algorithm for high dimensional spaces. Chemometrics and Intelligent Laboratory Systems, 113, 26-31.
```

---

## Crate usage

Add the following line to the `Cargo.toml` file:
```toml
[dependencies]
wsp = "0.1.5"
```
### Use cases

A space-filling algorithm enables to remove points to close to each other in a given space. Given a minimal distance `d_min` and an initial set of points, wsp builds a subset where all remaining points are at least `d_min` distant from each other.

wsp also provides an alternative version of the classical WSP algorithm. Certain scenarios do not have any clue about the `d_min` to choose, but require a given number of remaining points in the subset. `adaptive_wsp` search the best `d_min` to create a subset of a target number of points.

### Example

#### WSP

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

#### Adaptive WSP

The next example uses the `adaptive_wsp` function with verbose mode. The initial set is similar to the previous example thanks to the seed. We aim to find a minimal distance such that the resulting set only contains 100 points.

```rust
// Generates the initial set
let mut points = wsp::PointSet::init_from_random(1000, 20, 51);

// Adaptive WSP makes a binary search to reach the target
// number of remaining points
let objective_nb: usize = 100;
wsp::adaptive_wsp(&mut points, objective_nb, false);

// Save the result in a CSV file
if let Err(err) = points.save_in_csv("wsp.csv", false) {
    println!("Error writing in CSV: {}", err);
    std::process::exit(1);
}
```

## Binary usage

Use `cargo install wsp` to install a binary version of the rust-wsp crate. Both `wsp()` and `adaptive_wsp` are available through the command line.

### Classic WSP

You may run the classic WSP version with

```bash
wsp -n 1000 -m 20 -d 0.5
```

This will run the WSP algorithm with 1000 initial points. Each point has a dimension of 10. The minimal distance between each point, as detailed in [1], is set to 0.5. For now, the algorithm uses the l1 (Manhattan) distance, as it provides better separation in high dimensional space than the l2 (euclidian) distance. The result is stored in a file named `wsp.csv`. Each row represents a point in a 20 dimensions space that is far enough from its other neighbours. You may change the output file with the `-o` option.

### Adaptive WSP

```bash
$ wsp -n 1000 -m 20 --adaptive 100
```

Similarly to the previous command, the initial space if filled with 1000 points of 20 dimensions. However, now, the user does not need to specify the minimal distance between points, but instead the _desired_ number of points in the resulting set. The algorithm will perform a binary search on the minimal distance until 1) the resulting set contains the desired number of points or 2) there is not distance that can be found to reach this quantity. In the second scenario, rust-wsp uses the minimal distance resulting in a space where the number of points is _as close as possible_ to the desired value.

Consider the example below, where we want 200 points in a space of 20 dimensions, initially filled with 1000 points:
```bash
$ wsp -n 1000 -m 20 --adaptive 200 -v
Iter #1: distance=7.430897367982144, nb_active=5
Iter #2: distance=5.028120018334874, nb_active=98
Iter #3: distance=3.826731343511239, nb_active=591
Iter #4: distance=4.427425680923057, nb_active=270
Iter #5: distance=4.727772849628966, nb_active=168
Iter #6: distance=4.577599265276012, nb_active=209
Iter #7: distance=4.652686057452489, nb_active=170
Iter #8: distance=4.615142661364251, nb_active=195
Iter #9: distance=4.5963709633201315, nb_active=207
Iter #10: distance=4.605756812342191, nb_active=194
Iter #11: distance=4.601063887831161, nb_active=191
Iter #12: distance=4.5987174255756464, nb_active=206
Iter #13: distance=4.599890656703404, nb_active=206
Iter #14: distance=4.600477272267282, nb_active=201
Iter #15: distance=4.600770580049222, nb_active=194
Iter #16: distance=4.600623926158252, nb_active=194
Iter #17: distance=4.600550599212767, nb_active=197
[...]
Iter #53: distance=4.60049404718639, nb_active=201
Iter #54: distance=4.600494047186391, nb_active=197
Last iter: best approximation is distance=4.600477272267282, nb_active=201
Nb active: 201
```

The algorithm performs 54 iterations until the minimal distance search space is completely explored. It will recompute the space (if needed) qith the minimal distance resulting in the best approximation of the target number of active points in the set. Here, it is 201, with an error of 1 compared to the objective. The resulting matrix is also stored in a file named `wsp.csv` by default.

### More help

Run `wsp -h` or `wsp --help` for more information about the arguments.