// one neuron with one input

// recoded in rust from https://www.youtube.com/watch?v=PGSba51aRYU
// credit Tsoding Daily

use rand::Rng;

const TEST_DATASET: [[f32; 2]; 7] =
[
    [0., 0.],
    [1., 2.],
    [2., 4.],
    [3., 6.],
    [4., 8.],
    [5., 10.],
    [6., 12.]
];

fn sigma(n: usize) -> f32 {
    let mut result: f32 = 0.;

    for i in 1..=n {
        result += i as f32;
    }

    result
}

fn cost(w: f32) -> f32 {
    // size of training data
    let size = TEST_DATASET.len();

    // total score
    let mut cost = 0.0;

    // i in amount of iterations
    for i in 0..size {
        let mut x = TEST_DATASET[i][0];
        let y = x*w;
        cost += score(TEST_DATASET[i][1], y);
    }

    cost /= size as f32;
    cost
}

// e is expected result
// r is result u got
fn score(e: f32, r: f32) -> f32{
    let mut score = 0.;

    let distance = r - e;
    score = distance.powf(2.);

    score
}

fn main() {
    let mut rng = rand::thread_rng();
    let random_number = rng.gen_range(0.0..=10.0);

    // weight
    let mut w = random_number;
    // bias
    let mut b = 0.0;

    let modi = 1e-3;
    let lrate = 1e-3;

    let iterations = 10000;

    for i in 0..iterations {
        let cost_0 = (cost((w + modi)) - cost(w)) / modi;
        w -= lrate * cost_0;

        println!("cost: {}", cost(w));
    }

    println!("total score: {}", cost(w));

    // Test run
    println!("{}", w * 5000.)
}
