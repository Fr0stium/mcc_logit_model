mod math_utils;
mod player;
use player::Player;

const MAX_ITERATIONS: i32 = 10;
const THRESHOLD: f64 = 1e-12;

fn generate_skill_levels() -> Vec<Player> {
    let mut players = player::get_players();
    // First, iteratively remove all players that have never won or never lost.
    // Ratings cannot be generated for these players using the current method of maximum likelihood estimation.
    loop {
        let mut indices = Vec::<usize>::new(); // This will be ordered from smallest to largest.
        for (index, player) in players.iter().enumerate() {
            let (mut total_wins, mut total_losses) = (0.0, 0.0);
            for opponent in players.iter() {
                let (player_wins, opponent_wins) = player.get_score_against(opponent);
                total_wins += player_wins;
                total_losses += opponent_wins;
            }
            if total_wins == 0.0 || total_losses == 0.0 {
                indices.push(index);
            }
        }
        // Exit the loop when there are no more players to remove.
        if indices.is_empty() {
            break;
        }
        // Since indices is sorted, we can just remove players in reverse.
        for index in indices.iter().rev() {
            players.remove(*index);
        }
    }
    // Now find the maximum likelihood estimate for the skill level of each player.
    let n = players.len();
    for _ in 0..MAX_ITERATIONS {
        let mut gradient = vec![0.0; n + 1];
        let mut augmented_hessian = vec![vec![1.0; n + 2]; n + 1]; // The hessian matrix augmented with the negative gradient matrix.
        for (i, player) in players.iter().enumerate() {
            let mut gradient_sum = 0.0;
            let mut hessian_sum = 0.0;
            for (j, opponent) in players.iter().enumerate() {
                let (player_wins, opponent_wins) = player.get_score_against(opponent);
                let skill_difference = opponent.skill - player.skill;

                gradient_sum += player_wins * math_utils::hazard(skill_difference)
                    - opponent_wins * math_utils::hazard(-skill_difference);
                augmented_hessian[i][j] = player_wins * math_utils::hazard_prime(skill_difference)
                    + opponent_wins * math_utils::hazard_prime(-skill_difference);
                hessian_sum -= augmented_hessian[i][j];
            }
            gradient[i] = gradient_sum;
            augmented_hessian[i][i] = hessian_sum;
            augmented_hessian[i][n + 1] = -gradient[i]; // Augment the negative gradient to the hessian.
        }
        augmented_hessian[n][n + 1] = 0.0;
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
