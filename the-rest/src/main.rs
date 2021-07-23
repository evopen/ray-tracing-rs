use rand::{Rng, SeedableRng};

fn main() {
    let mut rng = rand::thread_rng();
    let n: u64 = 1000;
    let mut inside_circle: u64 = 0;
    let mut inside_circle_stratified = 0;

    for i in 0..n {
        for j in 0..n {
            let x = rng.gen_range(-1.0..=1.0);
            let y = rng.gen_range(-1.0..=1.0);
            if x * x + y * y < 1.0 {
                inside_circle += 1;
            }
            let x = i as f64 + rng.gen_range(0.0..1.0);
            let y = j as f64 + rng.gen_range(0.0..1.0);
            if x * x + y * y < (n * n) as f64 {
                inside_circle_stratified += 1;
            }
        }
    }

    println!(
        "Regular Estimate of Pi = {}",
        4.0 * inside_circle as f64 / (n * n) as f64
    );
    println!(
        "Stratified Estimate of Pi = {}",
        4.0 * inside_circle_stratified as f64 / (n * n) as f64
    );
}
