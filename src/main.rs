use prisoners::game::Game;

fn main() {
    const PRISONER_COUNT: usize = 100;
    let mut g = Game::new(PRISONER_COUNT);

    println!("Calculating...");
    const TOTAL_ROUNDS: i32 = 1000;
    let mut rounds_won = 0;
    for _ in 0..TOTAL_ROUNDS {
        if g.play() {
            rounds_won += 1;
        }
    }

    println!("Chance of winning: {:.2}%", (rounds_won as f32 / TOTAL_ROUNDS as f32) * 100.0);
}
