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

const E: f32 = std::f32::consts::E;

fn sigmoid(x: f32) -> f32 {
    1. / (1. + E.powf(-x))
}

fn cost(w0: f32, w1: f32) -> f32 {
    // total score
    let mut cost = 0.0;
    // i in amount of iterations
    for i in 0..DATASET_SIZE {
        let x0= TEST_DATASET[i][0];
        let x1= TEST_DATASET[i][1];
        let y = x0 * w0 + x1 * w1;
        let diff= y - TEST_DATASET[i][2];
        cost += diff.powf(2.);
    }

    cost /= DATASET_SIZE as f32;
    cost
}

fn main() {
    let mut rng = rand::thread_rng();

    // weight 1
    let mut w0 = rng.gen_range(0.0..=10.0);
    // weight 2
    let mut w1 = rng.gen_range(0.0..=10.0);
    let iterations = 1000*100;

    let modi = 1e-3;
    let lrate = 1e-3;

    // println!("weight 1 = {}, weight 2 = {}, cost = {}", w0, w1, cost(w0, w1));

    for i in 0..iterations {
        let c = cost(w0, w1);

        let dw1 = (cost(w0 + modi, w1) - c) / modi;
        let dw2 = (cost(w0, w1 + modi) - c) / modi;

        w0 -= lrate * dw1;
        w1 -= lrate * dw2;

        //println!("cost: {}", cost(w0, w1));
    }
    println!("final cost: {}", cost(w0, w1));
    println!("-------------");
    println!("0 and 0: {}", sigmoid(TEST_DATASET[0][0] * w0 + TEST_DATASET[0][1] * w1));
    println!("1 and 0: {}", sigmoid(TEST_DATASET[1][0] * w0 + TEST_DATASET[1][1] * w1));
    println!("0 and 1: {}", sigmoid(TEST_DATASET[2][0] * w0 + TEST_DATASET[2][1] * w1));
    println!("1 and 1: {}", sigmoid(TEST_DATASET[3][0] * w0 + TEST_DATASET[3][1] * w1));
}
