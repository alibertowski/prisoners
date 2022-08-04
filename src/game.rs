use rand::{prelude::*};

pub struct Game {
    prisoner_count: usize,
    prisoners: Vec<usize>,
    rng: ThreadRng
}

impl Game {
    pub fn new(count: usize) -> Self {
        Self {
            prisoner_count: count,
            prisoners: create_range(count),
            rng: rand::thread_rng()
        }
    }

    pub fn play(&mut self) -> bool {
        let boxes = self.randomize_boxes(self.prisoner_count);

        for prisoner in &self.prisoners {
            let mut boxes_remaining = self.prisoner_count / 2;
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

    fn randomize_boxes(&mut self, count: usize) -> Vec<usize> {
        let mut boxes: Vec<usize> = create_range(count);
        boxes.shuffle(&mut self.rng);

        boxes
    }
}

fn create_range(range: usize) -> Vec<usize> {
    let mut v: Vec<usize> = Vec::with_capacity(range);
    for i in 0..range {
        v.push(i);
    }

    v
}