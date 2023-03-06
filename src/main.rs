mod math_utils;
mod player;
use player::Player;

fn generate_skill_levels() -> Vec<Player> {
    const MAX_ITERATIONS: u8 = 10;
    const THRESHOLD: f64 = 1e-12;
    let mut players = player::get_players();
    let n = players.len();
    for _ in 0..MAX_ITERATIONS {
        let mut gradient = vec![0f64; n + 1];
        // The hessian matrix augmented with the negative of the gradient matrix:
        let mut augmented_hessian = vec![vec![1f64; n + 2]; n + 1];
        for (i, opponent) in players.iter().enumerate() {
            let mut gradient_sum = 0f64;
            let mut hessian_sum = 0f64;
            for (j, player) in players.iter().enumerate() {
                let (player_wins, opponent_wins) = player.get_score_against(opponent);
                let skill_difference = player.skill - opponent.skill;
                gradient_sum += opponent_wins * math_utils::hazard(skill_difference)
                    - player_wins * math_utils::hazard(-skill_difference);
                augmented_hessian[i][j] = opponent_wins
                    * math_utils::hazard_prime(skill_difference)
                    + player_wins * math_utils::hazard_prime(-skill_difference);
                hessian_sum -= augmented_hessian[i][j];
            }
            gradient[i] = gradient_sum;
            augmented_hessian[i][i] = hessian_sum;
            augmented_hessian[i][n + 1] = -gradient[i]; // Augment the negative gradient to the hessian.
        }
        augmented_hessian[n][n + 1] = 0f64;
        let add_to_skill_levels = math_utils::gaussian_elimination(&mut augmented_hessian);
        for i in 0..n {
            players[i].skill += add_to_skill_levels[i];
        }
        let magnitude = add_to_skill_levels
            .iter()
            .map(|x| x * x)
            .sum::<f64>()
            .sqrt();
        if magnitude < THRESHOLD {
            break;
        }
    }
    players
}

fn main() {
    let mut players = generate_skill_levels();
    players.sort_by(|a, b| b.skill.partial_cmp(&a.skill).unwrap());
    for player in &players {
        println!("{}, {}", player.username, player.skill);
    }
}
