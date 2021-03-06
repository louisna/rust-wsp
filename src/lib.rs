//!
//! A rust implementation of the WSP space filling algorithm.
//!
//! This is based on the paper from J. Santiago _et al_:
//! ```raw
//! [1] Santiago, J., Claeys-Bruno, M., & Sergent, M. (2012). Construction of space-filling designs using WSP algorithm for high dimensional spaces. Chemometrics and Intelligent Laboratory Systems, 113, 26-31.
//! ```
//!
//! ## Usage
//!
//! Add the following line to the `Cargo.toml` file:
//! ```toml
//! [dependencies]
//! wsp = "0.1.6"
//! ```
//! ## Use cases
//!
//! A space-filling algorithm enables to remove points to close to each other in a given space. Given a minimal distance `d_min` and an initial set of points, wsp builds a subset where all remaining points are at least `d_min` distant from each other.
//!
//! wsp also provides an alternative version of the classical WSP algorithm. Certain scenarios do not have any clue about the `d_min` to choose, but require a given number of remaining points in the subset. `adaptive_wsp` search the best `d_min` to create a subset of a target number of points.
//!
//! ## Example
//!
//! ### WSP
//!
//! The following example generates an initial set of 1000 points from a uniform random distribution, in a 20-dimensions space. The generation uses the seed 51. The minimal distance is arbitrarily set to 3.0.
//!
//! ```rust
//! // Generates the initial set
//! let mut points = wsp::PointSet::init_from_random(1000, 20, 51);
//!
//! // Only keep distant enough points
//! let d_min = 3.0;
//! wsp::wsp(&mut points, d_min);
//!
//! // Iterate over the remaining points
//! for valid_point in points.get_remaining() {
//!     println!("{:?}", valid_point);
//! }
//! ```
//!
//! ### Adaptive WSP
//!
//! The next example uses the `adaptive_wsp` function with verbose mode. The initial set is similar to the previous example thanks to the seed. We aim to find a minimal distance such that the resulting set only contains 100 points.
//!
//! ```rust
//! // Generates the initial set
//! let mut points = wsp::PointSet::init_from_random(1000, 20, 51);
//!
//! // Adaptive WSP makes a binary search to reach the target
//! // number of remaining points
//! let objective_nb: usize = 100;
//! wsp::adaptive_wsp(&mut points, objective_nb, false);
//!
//! // Save the result in a CSV file
//! if let Err(err) = points.save_in_csv("wsp.csv", false) {
//!     println!("Error writing in CSV: {}", err);
//!     std::process::exit(1);
//! }
//! ```
//!
//! ## Binary usage

//! Use `cargo install wsp` to install a binary version of the rust-wsp crate. Both `wsp()` and `adaptive_wsp` are available through the command line.
//!
//! ### Classic WSP
//!
//! You may run the classic WSP version with
//!
//! ```bash
//! wsp -n 1000 -m 20 -d 0.5
//! ```
//!
//! This will run the WSP algorithm with 1000 initial points. Each point has a dimension of 10. The minimal distance between each point, as detailed in the paper, is set to 0.5. For now, the algorithm uses the l1 (Manhattan) distance, as it provides better separation in high dimensional space than the l2 (euclidian) distance. The result is stored in a file named `wsp.csv`. Each row represents a point in a 20 dimensions space that is far enough from its other neighbours. You may change the output file with the `-o` option.
//!
//! ### Adaptive WSP
//!
//! ```bash
//! $ wsp -n 1000 -m 20 --adaptive 100
//! ```
//!
//! Similarly to the previous command, the initial space if filled with 1000 points of 20 dimensions. However, now, the user does not need to specify the minimal distance between points, but instead the _desired_ number of points in the resulting set. The algorithm will perform a binary search on the minimal distance until 1) the resulting set contains the desired number of points or 2) there is not distance that can be found to reach this quantity. In the second scenario, rust-wsp uses the minimal distance resulting in a space where the number of points is _as close as possible_ to the desired value.
//!
//! Consider the example below, where we want 200 points in a space of 20 dimensions, initially filled with 1000 points:
//! ```bash
//! $ wsp -n 1000 -m 20 --adaptive 200 -v
//! Iter #1: distance=7.430897367982144, nb_active=5
//! Iter #2: distance=5.028120018334874, nb_active=98
//! Iter #3: distance=3.826731343511239, nb_active=591
//! Iter #4: distance=4.427425680923057, nb_active=270
//! Iter #5: distance=4.727772849628966, nb_active=168
//! Iter #6: distance=4.577599265276012, nb_active=209
//! Iter #7: distance=4.652686057452489, nb_active=170
//! Iter #8: distance=4.615142661364251, nb_active=195
//! Iter #9: distance=4.5963709633201315, nb_active=207
//! Iter #10: distance=4.605756812342191, nb_active=194
//! Iter #11: distance=4.601063887831161, nb_active=191
//! Iter #12: distance=4.5987174255756464, nb_active=206
//! Iter #13: distance=4.599890656703404, nb_active=206
//! Iter #14: distance=4.600477272267282, nb_active=201
//! Iter #15: distance=4.600770580049222, nb_active=194
//! Iter #16: distance=4.600623926158252, nb_active=194
//! Iter #17: distance=4.600550599212767, nb_active=197
//! [...]
//! Iter #53: distance=4.60049404718639, nb_active=201
//! Iter #54: distance=4.600494047186391, nb_active=197
//! Last iter: best approximation is distance=4.600477272267282, nb_active=201
//! Nb active: 201
//! ```
//!
//! The algorithm performs 54 iterations until the minimal distance search space is completely explored. It will recompute the space (if needed) qith the minimal distance resulting in the best approximation of the target number of active points in the set. Here, it is 201, with an error of 1 compared to the objective. The resulting matrix is also stored in a file named `wsp.csv` by default.
//!
//! ### More help
//!
//! Run `wsp -h` or `wsp --help` for more information about the arguments.

use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};
use serde::Serialize;
use std::cmp::Ordering;
use std::error::Error;

#[derive(Debug, Serialize)]
struct Record {
    point: Vec<f64>,
}

/// Internal representation of the WSP algorithm values.
/// It is needed for the computation and to store information about the resulting point set.
pub struct PointSet {
    /// Points of the initial set
    pub points: Vec<Vec<f64>>,
    /// All ditances between all points
    pub distance_matrix: Vec<Vec<f64>>,
    /// If true, the point is still in the set. Otherwise, the point is considered as removed of the point set.
    /// The user MUST only consider points with 'true' values as the only points in the resulting set
    pub active: Vec<bool>,
    /// Number of active points in the set
    pub nb_active: usize,
    /// For each point, the idx sorted increasingly with distance
    /// to improve performance
    idx_sort: Vec<Vec<usize>>,
    /// For each point, the idx in the idx_sort of the closest active point
    idx_active: Vec<usize>,
    /// Visited point to avoid looping over the same point several times => ensures that we clear all the space
    visited: Vec<bool>,
    /// Minimal distance between points in the point set
    d_min: f64,
    /// Maximal distance between points in the point set
    d_max: f64,
}

impl PointSet {
    /// Creates a 'PointSet' from an already initialised vector of points.
    ///
    /// # Arguments
    ///
    /// * `points` - The pre-initialised set of points.
    ///
    /// # Example
    ///
    /// ```
    /// let points: Vec<Vec<f64>> = vec![vec![1.0, 0.0, 1.0], vec![0.5, 0.5, 0.5]];
    /// let poinset = wsp::PointSet::init_from_preset(points); // Give ownership
    /// ```
    pub fn init_from_preset(points: Vec<Vec<f64>>) -> PointSet {
        // First compute the distance matrix, then move "points" to the
        // output structure
        let (distance_matrix, d_min, d_max) = PointSet::compute_distance_matrix(&points, None);

        let mut p = PointSet {
            distance_matrix,
            active: vec![true; points.len()],
            nb_active: points.len(),
            idx_sort: Vec::with_capacity(points.len()),
            // Start at 1 because closest is itself
            idx_active: vec![1; points.len()],
            visited: vec![false; points.len()],
            points,
            d_max,
            d_min,
        };
        p.compute_closest_idx();
        p
    }

    /// Creates a 'PointSet' using a random initialisation of the points following a uniform distribution.
    ///
    /// # Arguments
    ///
    /// * `nb_points` - The number of points in the set before running WSP.
    /// * `nb_dim` - The dimension of the points.
    /// * `seed` - The seed used for the uniform sampling of the coordinates of the points.
    ///
    /// # Example
    ///
    /// The following code snippet creates a PointSet with 100 points of dimension 10. The seed 51 is used for the
    /// generation of the points.
    /// ```
    /// let poinset = wsp::PointSet::init_from_random(100, 10, 51); // Give ownership
    /// ```
    pub fn init_from_random(nb_points: usize, nb_dim: usize, seed: u64) -> PointSet {
        let mut points: Vec<Vec<f64>> = Vec::with_capacity(nb_points);

        let mut rng = SmallRng::seed_from_u64(seed);

        // Generate random points
        for _ in 0..nb_points {
            let mut point: Vec<f64> = Vec::with_capacity(nb_dim);
            for _ in 0..nb_dim {
                point.push(rng.gen::<f64>());
            }
            points.push(point);
        }

        PointSet::init_from_preset(points)
    }

    fn reset_reseach_params(&mut self) {
        self.nb_active = self.points.len();
        self.active = vec![true; self.nb_active];
        self.idx_active = vec![1; self.nb_active];
        self.visited = vec![false; self.nb_active];
    }

    fn compute_closest_idx(&mut self) {
        for i in 0..self.nb_active {
            let mut idxs: Vec<usize> = (0..self.nb_active).collect();
            idxs.sort_by(|&a, &b| {
                self.distance_matrix[i][a]
                    .partial_cmp(&self.distance_matrix[i][b])
                    .unwrap()
            });
            self.idx_sort.push(idxs);
        }
    }

    fn compute_distance_matrix(
        points: &[Vec<f64>],
        distance_algo: Option<&dyn Fn(&[f64], &[f64]) -> f64>,
    ) -> (Vec<Vec<f64>>, f64, f64) {
        let nb_points = points.len();
        let mut distance_matrix = vec![vec![0.0f64; nb_points]; nb_points];
        let mut dmin: f64 = f64::MAX;
        let mut dmax: f64 = 0.0;
        for i in 0..nb_points {
            for j in i + 1..nb_points {
                distance_matrix[i][j] = match distance_algo {
                    Some(algo) => algo(&points[i], &points[j]),
                    None => manhattan_distance(&points[i], &points[j]),
                };

                distance_matrix[j][i] = distance_matrix[i][j]; // Primitive type copy
                dmin = dmin.min(distance_matrix[i][j]);
                dmax = dmax.max(distance_matrix[i][j]);
            }
        }
        (distance_matrix, dmin, dmax)
    }

    /// Stores a PointSet in a CSV file. This will store in a matrix form the active points in the PointSet.
    /// Each row represents an active point, and each column a dimension in the space.
    ///
    /// # Arguments
    ///
    /// * `filepath` - The path to the file where to store the PointSet points.
    /// * `transpose` - Transpose the matrix in the output CSV
    ///
    /// # Example
    ///
    /// The following code snippet stores the PointSet *before* running WSP
    /// ```
    /// let points = wsp::PointSet::init_from_random(100, 10, 51);
    ///
    /// if let Err(err) = points.save_in_csv("wsp.csv", false) {
    ///     eprintln!("Error writing in CSV: {}", err);
    ///     std::process::exit(1);
    /// }
    /// ```
    pub fn save_in_csv(&self, filepath: &str, transpose: bool) -> Result<(), Box<dyn Error>> {
        let mut wrt = csv::WriterBuilder::new()
            .has_headers(false)
            .from_path(filepath)?;

        // Use star notation just to show that we understand it
        let points = if transpose {
            let mut transposed: Vec<Vec<f64>> =
                vec![Vec::with_capacity(self.nb_active); self.points[0].len()];
            for (i, point) in (*self.points).iter().enumerate() {
                if self.active[i] {
                    for i in 0..point.len() {
                        transposed[i].push(point[i]);
                    }
                }
            }
            transposed
        } else {
            self.points
                .iter()
                .enumerate()
                .filter(|(i, _)| self.active[*i])
                .to_owned()
                .unzip::<usize, &Vec<f64>, Vec<usize>, Vec<&Vec<f64>>>()
                .1
                .iter()
                .map(|x| x.to_owned().to_owned())
                .collect()
        };
        for point in points.iter() {
            wrt.serialize(Record {
                point: point.clone(),
            })?;
        }
        Ok(())
    }

    /// Returns a new vector containing only the active points of the PointSet.
    ///
    /// # Example
    ///
    /// The following code snippet stores the PointSet *before* running WSP
    /// ```
    /// // Generates the initial set
    /// let mut points = wsp::PointSet::init_from_random(1000, 20, 51);
    ///
    /// // Only keep distant enough points
    /// let d_min = 3.0;
    /// wsp::wsp(&mut points, d_min);
    ///
    /// // Iterate over the remaining points
    /// for valid_point in points.get_remaining() {
    ///     println!("{:?}", valid_point);
    /// }
    /// ```
    pub fn get_remaining(&self) -> Vec<Vec<f64>> {
        let mut points: Vec<Vec<f64>> = Vec::with_capacity(self.nb_active);
        for i in 0..self.points.len() {
            if self.active[i] {
                points.push(self.points[i].clone());
            }
        }
        points
    }
}

fn _distance_sq(p1: &[f64], p2: &[f64]) -> f64 {
    let mut dist: f64 = 0.0;
    for i in 0..p1.len() {
        dist += (p1[i] - p2[i]) * (p1[i] - p2[i]);
    }
    dist
}

fn manhattan_distance(p1: &[f64], p2: &[f64]) -> f64 {
    p1.iter()
        .zip(p2.iter())
        .fold(0.0, |dist, (d1, d2)| dist + (d1 - d2).abs())
}

fn wsp_loop_fast(set: &mut PointSet, d_min: f64, mut origin: usize) {
    loop {
        let idxs_this_origin = &mut set.idx_sort[origin];

        // Iterate over all "active" points closest to the current origin
        // We may iterate over inactive points due to previous loop
        // We stop iterating once we find the next closest point
        // that is 1) active and 2) at a higher distance than *d_min*
        let mut closest_origin = set.idx_active[origin];
        set.visited[origin] = true;
        loop {
            if closest_origin >= set.points.len() {
                return;
            }
            let point_idx = idxs_this_origin[closest_origin];
            if !set.active[point_idx] {
                // Not active point
                closest_origin += 1;
                continue;
            } else if set.distance_matrix[origin][point_idx] < d_min {
                // Point too close to the origin => kill
                set.active[point_idx] = false;
                set.nb_active -= 1;
                closest_origin += 1;
            } else if set.visited[point_idx] {
                closest_origin += 1;
            } else {
                // Closest active point remaining is far enough from the origin
                // Stop the loop and this point is the next origin
                // Update the closest_origin of the current origin just in case
                set.idx_active[origin] = closest_origin;
                origin = idxs_this_origin[closest_origin];
                break; // Further points will always be at a higher distance
            }
        }
    }
}
/// Returns a new vector containing only the active points of the PointSet.
///
/// # Example
///
/// The following code snippet stores the PointSet *before* running WSP
/// ```
/// // Generates the initial set
/// let mut points = wsp::PointSet::init_from_random(1000, 20, 51);
///
/// // Only keep distant enough points
/// let d_min = 3.0;
/// wsp::wsp(&mut points, d_min);
///
/// // Iterate over the remaining points
/// for valid_point in points.get_remaining() {
///     println!("{:?}", valid_point);
/// }
/// ```
/// Executes the WSP space filling algorithm according to the paper.
/// (Pseudo-)randomly chooses an origin, and removes all points too close to it
/// according to the d_min value of the PointSet structure.
/// Then, the new origin is the closest valid point from the old origin.
/// The algorithm iterates like this until all points have been visited or removed.
///
/// # Arguments
///
/// * `set` - The PointSet instance. `set` is mutably borrowed.
/// * `d_min` - The desired minimal distance between all remaining points in the PointSet.
///
/// # Example
///
/// ```
/// let mut points = wsp::PointSet::init_from_random(1000, 20, 51);
/// let d_min = 3.0;
/// wsp::wsp(&mut points, d_min);
/// ```
pub fn wsp(set: &mut PointSet, d_min: f64) {
    // Step 3: chose random point
    let mut rng = SmallRng::seed_from_u64(10);
    let origin: usize = rng.gen::<usize>() % set.points.len();

    // Step 4, 5, 6: call specific algorithm for speed
    wsp_loop_fast(set, d_min, origin);
}

/// This is an adaptive version of the WSP algorithm.
/// The traditional algorithm requires a d_min and
/// based on that we obtain a set of a given number of points.
/// Here we adaptively change d_min to get (an approximation of)
/// the desired number of points active after the algorithm.
///
/// # Arguments
///
/// * `set` - The PointSet instance. `set` is mutably borrowed.
/// * `obj_nb` - The desired number of points remaining active in the set after the algorithm.
/// * `verbose` - Print running information about the iterations of the algorithm.
///
/// # Example
///
/// ```
/// let mut points = wsp::PointSet::init_from_random(1000, 20, 51);
/// let objective_nb: usize = 100;
/// wsp::adaptive_wsp(&mut points, objective_nb, false);
/// ```
pub fn adaptive_wsp(set: &mut PointSet, obj_nb: usize, verbose: bool) {
    let mut d_min = set.d_min;
    let mut d_max = set.d_max;
    let mut d_search = (d_min + d_max) / 2.0;
    let mut iter = 0;
    let mut best_distance = 0.0;
    let mut best_difference_active = set.nb_active - obj_nb;
    loop {
        iter += 1;
        wsp(set, d_search);

        // Binary search the best d_min
        if verbose {
            println!(
                "Iter #{}: distance={}, nb_active={}",
                iter, d_search, set.nb_active
            );
        }
        match set.nb_active.cmp(&obj_nb) {
            Ordering::Greater => d_min = d_search,
            Ordering::Less => d_max = d_search,
            Ordering::Equal => return,
        };

        // The search space is not continuous.
        // We must also track the best result to recover it afterwards
        if (set.nb_active as i32 - obj_nb as i32).abs() < best_difference_active as i32 {
            best_difference_active = (set.nb_active as i32 - obj_nb as i32).abs() as usize;
            best_distance = d_search;
        }

        // Stop condition if we cannot exactly reach the target number
        let last_d_search = d_search;
        d_search = (d_min + d_max) / 2.0;
        if (last_d_search - d_search).abs() <= f64::EPSILON {
            break;
        }

        // Reset parameters for the next iteration
        set.reset_reseach_params();
    }

    // Recompute a last time if the best distance is not the last computed distance
    if (best_distance - d_search).abs() > f64::EPSILON {
        d_search = best_distance;
        set.reset_reseach_params();
        wsp(set, d_search);
    }
    if verbose {
        println!(
            "Last iter: best approximation is distance={}, nb_active={}",
            d_search, set.nb_active
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_distance_sq() {
        let mut p1: Vec<f64> = vec![1.0, 0.0];
        let mut p2 = vec![0.0, 0.0];
        assert_eq!(_distance_sq(&p1, &p2), 1.0);

        p1 = vec![2.0, 2.0];
        p2 = vec![2.0, 9.0];
        assert_eq!(_distance_sq(&p1, &p2), 49.0);
    }

    #[test]
    fn test_manhattan_distance() {
        let p1 = vec![0.0, 0.0, 0.0];
        let p2 = vec![0.5, 0.5, 1.0];
        let p3 = vec![1.0, 0.0, 0.5];
        assert_eq!(manhattan_distance(&p1, &p2), 2.0);
        assert_eq!(manhattan_distance(&p1, &p3), 1.5);
        assert_eq!(manhattan_distance(&p2, &p3), 1.5);
        assert_eq!(manhattan_distance(&p1, &p1), 0.0);
    }

    #[test]
    fn test_distance_matrix() {
        let p1 = vec![0.0, 0.0];
        let p2 = vec![4.0, 0.0];
        let p3 = vec![4.0, 3.0];
        let (distance_matrix, d_min, d_max) =
            PointSet::compute_distance_matrix(&vec![p1, p2, p3], Some(&_distance_sq));

        let true_distance = vec![
            vec![0.0, 16.0, 25.0],
            vec![16.0, 0.0, 9.0],
            vec![25.0, 9.0, 0.0],
        ];

        for i in 0..3 {
            for j in 0..3 {
                assert_eq!(distance_matrix[i][j], true_distance[i][j]);
            }
        }

        assert_eq!(d_min, 9.0);
        assert_eq!(d_max, 25.0);
    }

    #[test]
    fn test_closest_idx() {
        let p1 = vec![0.0, 0.0];
        let p2 = vec![1.0, 0.1];
        let p3 = vec![1.0, 1.0];
        let p4 = vec![2.0, 1.0];
        let pointset = PointSet::init_from_preset(vec![p1, p2, p3, p4]);

        let true_idxs = vec![
            vec![0, 1, 2, 3],
            vec![1, 2, 0, 3],
            vec![2, 1, 3, 0],
            vec![3, 2, 1, 0],
        ];

        for i in 0..4 {
            for j in 0..4 {
                assert_eq!(pointset.idx_sort[i][j], true_idxs[i][j]);
            }
        }
    }

    #[test]
    fn test_iterative_fast_1() {
        let p1 = vec![0.0, 0.0];
        let p2 = vec![1.0, 0.1];
        let p3 = vec![1.0, 1.0];
        let p4 = vec![2.0, 1.0];
        let mut pointset = PointSet::init_from_preset(vec![p1, p2, p3, p4]);

        wsp_loop_fast(&mut pointset, 1.0, 1);

        // The expected behaviour is
        // 1) * p3 too close => becomes inactive
        //    * p1 far enough => becomes new origin
        // 2) * p1 far from p2 => p2 becomes origin
        //    * no change in the set => stop iteration
        assert_eq!(pointset.active[0], true);
        assert_eq!(pointset.active[1], true);
        assert_eq!(pointset.active[2], false);
        assert_eq!(pointset.active[3], true);

        assert_eq!(pointset.nb_active, 3);
    }

    #[test]
    fn test_all_points_visited() {
        let d_min: f64 = 0.04;
        let mut points = PointSet::init_from_random(1000, 3, 51);
        wsp(&mut points, d_min);

        // All points are either visited or inactive
        for i in 0..1000 {
            assert!(points.visited[i] || !points.active[i]);
        }
    }

    #[test]
    fn test_min_dist_ok() {
        let d_min: f64 = 0.04;
        let mut points = PointSet::init_from_random(1000, 3, 51);
        wsp(&mut points, d_min);

        // All active points have a distance higher or equal to d_min
        for i in 0..999 {
            if !points.active[i] {
                continue;
            }
            for j in i + 1..1000 {
                if !points.active[j] {
                    continue;
                }
                assert!(points.distance_matrix[i][j] >= d_min);
            }
        }
    }
}
