use std::{
    cmp::Ordering,
    fs::File,
    io::{BufRead, BufReader},
};

const MCC_COUNT: usize = 31;

pub struct Player {
    pub skill: f64,
    pub username: String,
    coin_history: [i32; MCC_COUNT],
}

impl Player {
    pub fn get_coins_at(&self, mcc_index: usize) -> i32 {
        self.coin_history[mcc_index]
    }

    pub fn get_score_against(&self, opponent: &Player) -> (f64, f64) {
        let mut score = (0f64, 0f64);

        if self.username == opponent.username {
            return score;
        }

        for i in 0..MCC_COUNT {
            let player_coins = self.get_coins_at(i);
            let opponent_coins = opponent.get_coins_at(i);

            if player_coins == -1 || opponent_coins == -1 {
                continue;
            }

            match player_coins.cmp(&opponent_coins) {
                Ordering::Greater => score.0 += 1f64,
                Ordering::Less => score.1 += 1f64,
                Ordering::Equal => {
                    score.0 += 0.5;
                    score.1 += 0.5;
                }
            }
        }

        score
    }
}

pub fn get_players() -> Vec<Player> {
    let mut players = Vec::new();
    let file = File::open("coins.txt").unwrap();
    let reader = BufReader::new(file);

    for line in reader.lines().flatten() {
        let info: Vec<&str> = line.split(',').collect();
        let username = String::from(info[0]);
        let mut coin_history = [-1i32; MCC_COUNT];
        let mut has_played = false;

        for i in 1..MCC_COUNT + 1 {
            let coins = info[i].parse().unwrap();
            coin_history[i - 1] = coins;
            if !has_played && coins > 0 {
                has_played = true;
            }
        }

        if !has_played {
            continue;
        }

        let player = Player {
            coin_history,
            skill: 0f64,
            username,
        };

        players.push(player);
    }

    players
}
