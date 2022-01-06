## cli_gen_csv

### Classic WSP

Here are a few examples of how to use rust-wsp.

After cloning the project, the algorithm can be run as follows:

```bash
$ cargo run -- -n 1000 -m 20 -d 0.5
```

This will run the WSP algorithm with 1000 initial points. Each point has a dimension of 10. The minimal distance between each point, as detailed in [1], is set to 0.5. For now, the algorithm uses the l1 (Manhattan) distance, as it provides better separation in high dimensional space than the l2 (euclidian) distance.

### Adaptive WSP

The initial objective of WSP is to use it as a space filling algorithm, where we require points to have a given minimal with each other. The number of points in the resulting space is directly dependent of the distance.

In an experimental situation, it may be hard to choose the best _distance_ between points, and the user generally has a better idea of _how many_ points he wants in the space. As a result, rust-wsp also implements an _adaptive_ method, which can be run as follows:

```bash
$ cargo run -- -n 1000 -m 20 --adaptive 100
```

Similarly to the previous command, the initial space if filled with 1000 points of 20 dimensions. However, now, the user does not need to specify the minimal distance between points, but instead the _desired_ number of points in the resulting set. The algorithm will perform a binary search on the minimal distance until 1) the resulting set contains the desired number of points or 2) there is not distance that can be found to reach this quantity. In the second scenario, rust-wsp uses the minimal distance resulting in a space where the number of points is _as close as possible_ to the desired value.

Consider the example below, where we want 200 points in a space of 20 dimensions, initially filled with 1000 points:
```bash
$ cargo run -- -n 1000 -m 20 --adaptive 200
    Finished dev [unoptimized + debuginfo] target(s) in 0.02s
     Running `target/debug/wsp -n 1000 -m 20 --adaptive 200`
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

The algorithm performs 54 iterations until the minimal distance search space is completely explored. It will recompute the space (if needed) qith the minimal distance resulting in the best approximation of the target number of active points in the set. Here, it is 201, with an error of 1 compared to the objective.

### CLI arguments

Use `--help` to display the description of the command-line options.