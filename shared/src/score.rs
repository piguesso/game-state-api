pub fn calculate_score(
    topic: String,
    first_topic: String,
    second_topic: String,
    third_topic: String,
    has_stopped_game: bool,
) -> i32 {
    let mut score = 0;
    if topic == first_topic {
        score += 500;
    } else if topic == second_topic {
        score += 400;
    } else if topic == third_topic {
        score += 300;
    }
    if has_stopped_game {
        score = score * 2;
    }
    score
}
