use rand::{self, seq::SliceRandom};

fn play_prison_game(prisoners: &[usize], boxes: &[usize]) -> bool {
    for prisoner in prisoners {
        let mut boxes_remaining = prisoners.len() / 2;
        let mut current_box = *prisoner;
        while boxes_remaining > 0 {
            let val = boxes[current_box];
            if val == *prisoner {
                break;
            } else {
                current_box = val;
                boxes_remaining -= 1;
            }
        }

        if boxes_remaining == 0 {
            return false;
        }
    }

    true
}

fn create_range(range: usize) -> Vec<u32> {
    let mut v: Vec<u32> = Vec::with_capacity(range);
    for i in 0..range {
        v.push(i as u32);
    }

    v
}

fn main() {
    const PRISONER_COUNT: usize = 100;
    let mut prisoners = [0; PRISONER_COUNT];
    for i in 0..PRISONER_COUNT {
        prisoners[i] = i;
    }

    println!("Calculating...");
    const TOTAL_ROUNDS: i32 = 10_000;
    let mut rounds_won = 0;
    for _ in 0..TOTAL_ROUNDS {
        let mut randomized_values = [0; PRISONER_COUNT];
        for i in 0..PRISONER_COUNT {
            randomized_values[i] = i;
        }
    
        let mut rng = rand::thread_rng();
        randomized_values.shuffle(&mut rng);

        let prisoners_win = play_prison_game(&prisoners, &randomized_values);
        if prisoners_win {
            rounds_won += 1;
        }
    }

    println!("Chance of winning: {:.2}%", (rounds_won as f32 / TOTAL_ROUNDS as f32) * 100.0);
}
