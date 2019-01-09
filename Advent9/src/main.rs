use std::collections::VecDeque;
use std::error::Error;
use std::usize;
struct Players {
    scores: Vec<u32>,
    player: usize,
}
impl Players {
    fn new(n: usize) -> Players {
        Players {
            player: 0,
            scores: vec![0; n],
        }
    }
    fn add_score(&mut self, s: u32) {
        self.scores[self.player] += s;
        self.player = (self.player + 1) % self.scores.len();
    }
    fn skip(&mut self, n: usize) {
        self.player = (self.player + n) % self.scores.len();
    }
}

struct Ring {
    marbles: VecDeque<u32>,
    cursor: usize,
}
impl Ring {
    pub fn new() -> Ring {
        Ring {
            marbles: VecDeque::new(),
            cursor: 0,
        }
    }
    fn cycle(&mut self, i: i32) {
        if i > 0 {
            let tmp = self.marbles.pop_back().unwrap();
            self.marbles.push_front(tmp);
            self.cycle(i - 1);
        }
        if i < 0 {
            let tmp = self.marbles.pop_front().unwrap();
            self.marbles.push_back(tmp);
            self.cycle(i + 1);
        }
    }

    fn insert(&mut self, marble: u32, players: &mut Players) {
        let mut score = 0;
        if self.marbles.len() < 2 {
            self.marbles.push_back(marble);
            players.add_score(0);
            return;
        } else if marble % 23 != 0 {
            self.cycle(2);
            self.marbles.push_back(marble);
        } else {
            //add marble to score, then remove one marble from ring
            score += marble;
            // 0 1 2 3 4 5 6
            self.cycle(-7);
            let remove_score = self.marbles.pop_back().unwrap();
            score += remove_score;
        }
        players.add_score(score);
    }
}

fn main() {
    //446 players; last marble is worth 71522 points => 390592
    let end_score = 71522 * 100;
    let players = 446;
    let mut score = Players::new(players);
    let mut ring = Ring::new();
    for i in 0..=end_score {
        ring.insert(i, &mut score);
    }
    println!("ans2: {:?}", score.scores.iter().max());
}
