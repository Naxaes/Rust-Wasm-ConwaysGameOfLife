#![feature(test)]

extern crate test;
extern crate conways_game_of_life;

// 'black_box' is an “opaque “black_box” to the optimizer”, so the compiler can’t optimize away
// our computation in the benchmark.
use test::black_box;

#[bench]
fn universe_ticks(b: &mut test::Bencher) {
    let mut universe = conways_game_of_life::Universe::new();

    b.iter(|| {
        for _ in 0..10 {
            black_box(universe.tick());
        }
    });
}
