#![feature(test)]

extern crate test;
extern crate conways_game_of_life;


#[bench]
fn universe_ticks(b: &mut test::Bencher) {
    let mut universe = conways_game_of_life::Universe::new();

    b.iter(|| {
        universe.tick();
    });
}
