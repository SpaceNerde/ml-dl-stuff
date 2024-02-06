// one neuron with two inputs

// recoded in rust from https://www.youtube.com/watch?v=PGSba51aRYU
// credit Tsoding Daily

use rand::Rng;

const TEST_DATASET: [[f32; 3]; 4] =
[
    [0., 0., 0.],
    [1., 0., 1.],
    [0., 1., 1.],
    [1., 1., 1.],
];
// size of training data
const DATASET_SIZE: usize = TEST_DATASET.len();

fn cost(w0: f32, w1: f32) -> f32 {
    // total score
    let mut cost = 0.0;
    let mut dif = 0.0;
    // i in amount of iterations
    for i in 0..DATASET_SIZE {
        let x0 = TEST_DATASET[i][0];
        let x1 = TEST_DATASET[i][1];
        let y = x0 * w0 + x1 * w1 ;
        dif += y - TEST_DATASET[i][1];
        cost += dif;
    }

    cost /= DATASET_SIZE as f32;
    cost
}

fn main() {

}
