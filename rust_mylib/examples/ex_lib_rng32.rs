//! mathライブラリの使用例

use rust_mylib::lib_rng32;

fn main() {
    let mut random = lib_rng32::RNG32::new(0);
    print!("seed: {}, ", 0);
    println!(
        "random: {}, {}, {}",
        random.next(),
        random.next(),
        random.next()
    );
    for i in 0..5 {
        print!("seed: {}, ", i);
        random.reset(i);
        println!(
            "random: {}, {}, {}",
            random.next(),
            random.next(),
            random.next()
        );
    }
    let mut random_another =
        lib_rng32::RNG32::new_with_type(0, lib_rng32::GeneratorType::XOSHIRO32x4ss);
    print!("seed: {}, ", 0);
    println!(
        "random: {}, {}, {}",
        random_another.next(),
        random_another.next(),
        random_another.next()
    );
    return;
}
