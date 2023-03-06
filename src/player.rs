use std::{cmp::Ordering, fs};

const MCC_COUNT: usize = 31;

pub struct Player {
    pub skill: f64,
    pub username: String,
    coin_history: Vec<i32>,
}

impl Player {
    pub fn get_coins_at(&self, mcc_index: usize) -> i32 {
        self.coin_history[mcc_index]
    }
    pub fn get_score_against(&self, opponent: &Player) -> (f64, f64) {
        let mut score = (1f64, 1f64);
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
    let path = "coins.txt";
    let contents = fs::read_to_string(path).unwrap();
    let contents: Vec<&str> = contents.split('\n').collect();
    for line in contents {
        let info: Vec<&str> = line.split(',').collect();
        let username = String::from(info[0]);
        let mut coin_history = Vec::new();
        let mut has_played = false;
        for str in info.iter().skip(1).take(MCC_COUNT) {
            let coins = str.parse().unwrap();
            coin_history.push(coins);
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
