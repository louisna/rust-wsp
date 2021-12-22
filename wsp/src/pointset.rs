use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};
use serde::Serialize;
use std::error::Error;

#[derive(Debug, Serialize)]
struct Record {
    point: Vec<f64>,
}

pub struct PointSet {
    /// Points of the initial set
    pub points: Vec<Vec<f64>>,
    /// All ditances between all points
    pub distance_matrix: Vec<Vec<f64>>,
    /// If true, the point is still in the set
    pub active: Vec<bool>,
    pub nb_active: u32,
    /// For each point, the idx sorted increasingly with distance
    /// to improve performance
    idx_sort: Vec<Vec<usize>>,
    /// For each point, the idx in the idx_sort of the closest active point
    idx_active: Vec<usize>,
    /// Visited point to avoid looping over the same point several times => ensures that we clear all the space
    visited: Vec<bool>,
}

impl PointSet {
    pub fn init_from_preset(points: Vec<Vec<f64>>) -> PointSet {
        // First compute the distance matrix, then move "points" to the
        // output structure
        let mut p = PointSet {
            distance_matrix: PointSet::compute_distance_matrix(&points),
            active: vec![true; points.len()],
            nb_active: points.len() as u32,
            idx_sort: Vec::with_capacity(points.len()),
            // Start at 1 because closest is itself
            idx_active: vec![1; points.len()],
            visited: vec![false; points.len()],
            points,
        };
        p.compute_closest_idx();
        p
    }

    pub fn init_from_random(nb_points: u32, nb_dim: usize, seed: u64) -> PointSet {
        let mut points: Vec<Vec<f64>> = Vec::with_capacity(nb_points as usize);

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

    fn compute_closest_idx(&mut self) {
        for i in 0..self.nb_active as usize {
            let mut idxs: Vec<usize> = (0..self.nb_active as usize).collect();
            idxs.sort_by(|&a, &b| {
                self.distance_matrix[i][a]
                    .partial_cmp(&self.distance_matrix[i][b])
                    .unwrap()
            });
            self.idx_sort.push(idxs);
        }
    }

    pub fn print_from_idx(&self, i: usize) {
        let point: &Vec<f64> = &self.points[i];
        println!("Vec#{}: {:?}", i, point);
    }

    fn compute_distance_matrix(points: &[Vec<f64>]) -> Vec<Vec<f64>> {
        let nb_points = points.len();
        let mut distance_matrix = vec![vec![0.0f64; nb_points]; nb_points];
        for i in 0..nb_points {
            for j in i + 1..nb_points {
                distance_matrix[i][j] = distance_sq(&points[i], &points[j]);
                distance_matrix[j][i] = distance_matrix[i][j]; // Primitive type copy
            }
        }
        distance_matrix
    }

    pub fn save_in_csv(&self, filepath: &str) -> Result<(), Box<dyn Error>> {
        let mut wrt = csv::WriterBuilder::new()
            .has_headers(false)
            .from_path(filepath)?;
        for (i, point) in (&(*self.points)).iter().enumerate() {
            // Use star notation just to show that we understand it
            if !self.active[i] {
                continue;
            }
            wrt.serialize(Record {
                point: point.clone(),
            })?;
        }
        Ok(())
    }
}

pub fn distance_sq(p1: &[f64], p2: &[f64]) -> f64 {
    let mut dist: f64 = 0.0;
    for i in 0..p1.len() {
        dist += (p1[i] - p2[i]) * (p1[i] - p2[i]);
    }
    dist
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

pub fn wsp(set: &mut PointSet, d_min: f64) {
    // Step 1: generate initial set
    // DONE

    // Step 2: Compute distance matrix of the points
    // DONE

    // Step 3: chose random point
    let mut rng = SmallRng::seed_from_u64(10);
    let origin: usize = rng.gen::<usize>() % set.points.len();

    // Step 4, 5, 6: call specific algorithm for speed
    wsp_loop_fast(set, d_min, origin);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_distance_sq() {
        let mut p1: Vec<f64> = vec![1.0, 0.0];
        let mut p2 = vec![0.0, 0.0];
        assert_eq!(distance_sq(&p1, &p2), 1.0);

        p1 = vec![2.0, 2.0];
        p2 = vec![2.0, 9.0];
        assert_eq!(distance_sq(&p1, &p2), 49.0);
    }

    #[test]
    fn test_distance_matrix() {
        let p1 = vec![0.0, 0.0];
        let p2 = vec![4.0, 0.0];
        let p3 = vec![4.0, 3.0];
        let distance_matrix = PointSet::compute_distance_matrix(&vec![p1, p2, p3]);

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
                if points.distance_matrix[i][j] < d_min {
                    println!("{} {} {}", i, j, points.distance_matrix[i][j]);
                }
                assert!(points.distance_matrix[i][j] >= d_min);
            }
        }
    }
}
