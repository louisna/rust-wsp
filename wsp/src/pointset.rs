use rand::Rng;

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
    /// For each point, the idx in the idx_sort of the closest
    /// active point
    idx_active: Vec<u32>,
}

impl PointSet {
    pub fn init_from_preset(points: Vec<Vec<f64>>) -> PointSet {
        // First compute the distance matrix, then move "points" to the
        // output structure
        let mut p = PointSet {
            distance_matrix: PointSet::compute_distance_matrix(&points),
            active: vec![true; points.len() as usize],
            nb_active: points.len() as u32,
            idx_sort: Vec::with_capacity(points.len()),
            // Start at 1 because closest is itself
            idx_active: vec![1; points.len()],
            points,
        };
        p.compute_closest_idx();
        p
    }

    pub fn init_from_random(nb_points: u32, nb_dim: usize) -> PointSet {

        let mut points: Vec<Vec<f64>> = Vec::with_capacity(nb_points as usize);

        // TODO: add seed
        let mut rng = rand::thread_rng();

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

    fn compute_closest_idx(&mut self) -> () {
        for i in 0..self.nb_active as usize {
            let mut idxs: Vec<usize> = (0..self.nb_active as usize).collect();
            idxs.sort_by(|&a, &b| self.distance_matrix[i][a].partial_cmp(&self.distance_matrix[i][b]).unwrap());
            self.idx_sort.push(idxs);
        }
    }

    pub fn print_from_idx(&self, i: u32) -> () {
        let point: &Vec<f64> = &self.points[i as usize];
        println!("Vec#{}: {:?}", i, point);
    }

    fn compute_distance_matrix(points: &Vec<Vec<f64>>) -> Vec<Vec<f64>> {
        let nb_points = points.len();
        let mut distance_matrix = vec![vec![0.0f64; nb_points]; nb_points];
        for i in 0..nb_points {
            for j in i+1..nb_points {
                distance_matrix[i][j] = distance_sq(&points[i], &points[j]);
                distance_matrix[j][i] = distance_matrix[i][j]; // Primitive type copy
            }
        }
        distance_matrix
    }
}

pub fn distance_sq(p1: &Vec<f64>, p2: &Vec<f64>) -> f64 {
    let mut dist: f64 = 0.0;
    for i in 0..p1.len() {
        dist += (&p1[i] - &p2[i]) * (&p1[i] - &p2[i]);
    }
    dist
}

fn wsp_loop_std(set: &mut PointSet, d_min: f64, mut origin: u32) -> () {
    // Repeat loop...
    loop {
        let mut unchanged = true;
        let mut closest_valid_dist = 3.0;
        let mut closest_valid_idx: u32 = 0;

        for i in 0..set.points.len() {
            // Self point
            if i as u32 == origin {
                continue;
            } else if !set.active[i] { // Already dead point
                continue;
            } else {
                // Step 4: eliminate points in the circle
                let p: &Vec<f64> = &set.points[i];
                let dist = distance_sq(&set.points[origin as usize], p);
                if dist < d_min {
                    set.active[i] = false;
                    set.nb_active -= 1;
                    unchanged = false;
                } else if dist < closest_valid_dist {
                    closest_valid_dist = dist;
                    closest_valid_idx = i as u32;
                }
            }
        }
        // ... until no change
        if unchanged {
            println!("Break");
            break;
        }

        // Step 5: Origin is replaced with closest neighbor
        origin = closest_valid_idx;
    }
}

fn wsp_loop_fast(set: &mut PointSet, d_min: f64, mut origin: usize) {
    loop {
        let mut unchanged = true;
        let idxs_this_origin = &mut set.idx_sort[origin as usize];

        // Iterate over all "active" points closest to the current origin
        // We may iterate over inactive points due to previous loop
        // We stop iterating once we find the next closest point
        // that is 1) active and 2) at a higher distance than *d_min*
        let mut closest_origin: usize = set.idx_active[origin as usize] as usize;
        loop {
            if closest_origin >= set.points.len() {
                panic!("A computation error seems to have occured");
            }
            let point_idx = idxs_this_origin[closest_origin];
            if !set.active[point_idx as usize] {
                // Not active point
                closest_origin += 1;
                continue;
            } else if set.distance_matrix[origin as usize][point_idx] < d_min {
                // Point too close to the origin => kill
                set.active[point_idx] = false;
                set.nb_active -= 1;
                closest_origin += 1;
                unchanged = true; // We remove another point
            } else {
                // Closest active point remaining is far enough from the origin
                // Stop the loop and this point is the next origin
                // Update the closest_origin of the current origin just in case
                set.idx_active[origin as usize] = closest_origin as u32;
                origin = closest_origin;
                break; // Further points will always be at a higher distance
            }
        }
        if unchanged {
            break; // Next iteration won't update the set of points
        }
    }
}

pub fn wsp(set: &mut PointSet, d_min: f64) -> () {
    // Step 1: generate initial set
    // DONE

    // Step 2: Compute distance matrix of the points
    // DONE

    // Step 3: chose random point
    println!("Commence");
    let mut rng = rand::thread_rng();
    let mut origin: u32 = rng.gen::<u32>() % set.points.len() as u32;

    // Step 4, 5, 6: call specific algorithm for speed
    wsp_loop_std(set, d_min, origin);
    // wsp_loop_fast(set, d_min, origin as usize);

    
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
    fn test_iterative_std_1() {
        let p1 = vec![0.0, 0.0];
        let p2 = vec![1.0, 0.1];
        let p3 = vec![1.0, 1.0];
        let p4 = vec![2.0, 1.0];
        let mut pointset = PointSet::init_from_preset(vec![p1, p2, p3, p4]);

        wsp_loop_std(&mut pointset, 1.0, 1);

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
}
