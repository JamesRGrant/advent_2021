use crate::Solve;

pub struct Problem {
    s1: i64,
    s2: i64,
}
impl Solve for Problem {
    fn p1(&mut self) -> i64 {
        let mut pos = [self.s1, self.s2];
        let mut score = [0, 0];

        // Use the 3x the middle roll
        // 1,2,3 = 3*2; 4,5,6 = 3*5
        let mut inc: i64 = 2;

        // Players take turns via 0..=1 cycle
        for i in (0..=1).cycle() {
            algorithm1(&mut inc, &mut pos[i], &mut score[i]);
            if score[i] >= 1000 {
                // Answer: last roll * losing player score
                // inc already advanced, so go from middle unrolled to last rolled (e.g. 5 -> 3)
                return (inc - 2) * score[(i + 1) % 2];
            }
        }
        0
    }
    fn p2(&mut self) -> i64 {
        // The frequency of sum of the three die rolls
        let result_freq = [(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)];
        let mut wins = [0, 0];
        let mut games = vec![Game {
            next_turn: 0,
            pos: [self.s1, self.s2],
            score: [0, 0],
            universes: 1,
        }];

        while !games.is_empty() {
            // process the turn for any one of the games
            let g = games.pop().unwrap();

            for (inc, freq) in result_freq {
                let pos = (g.pos[g.next_turn] + inc) % 10;
                let mut score = g.score[g.next_turn] + pos;
                if pos == 0 {
                    score += 10;
                }
                if score >= 21 {
                    // If someone one, record it and stop those games
                    wins[g.next_turn] += g.universes * freq;
                } else {
                    // Game still on, store it with the new values post-this-turn
                    let mut new_games = Game {
                        next_turn: (g.next_turn + 1) % 2,
                        pos: g.pos,
                        score: g.score,
                        universes: g.universes * freq,
                    };
                    new_games.pos[g.next_turn] = pos;
                    new_games.score[g.next_turn] = score;
                    games.push(new_games);
                }
            }
        }

        std::cmp::max(wins[0], wins[1])
    }
}
impl Problem {
    pub fn new(input: &[String]) -> Self {
        Problem {
            s1: input[0][28..29].parse::<i64>().unwrap(),
            s2: input[1][28..29].parse::<i64>().unwrap(),
        }
    }
}

fn algorithm1(inc: &mut i64, pos: &mut i64, score: &mut i64) {
    // Technically, the die should reset after 100 (e.g. 102 is 2),
    // but because it is negates out on the 10 space board (via mod 10),
    // we can just keep counting up so we have the total rolls in the game.
    *pos = ((*inc * 3) + *pos) % 10;
    *score += *pos;
    if *pos == 0 {
        *score += 10;
    }
    *inc += 3;
}

struct Game {
    next_turn: usize,
    pos: [i64; 2],
    score: [i64; 2],
    universes: i64,
}
