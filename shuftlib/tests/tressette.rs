use rand::Rng;
use shuftlib::{
    common::hands::{OngoingHand, OngoingTrick, Player, PlayerId, TrickTakingGame},
    tressette::{self, TressetteRules},
};

fn main() {
    let mut first = true;
    let mut leading_suit = None;
    let mut rng = rand::thread_rng();
    let first_to_play = PlayerId::new(0).unwrap();
    let mut score = (0, 0);
    let players = [
        Player::new(PlayerId::new(0).unwrap()),
        Player::new(PlayerId::new(1).unwrap()),
        Player::new(PlayerId::new(2).unwrap()),
        Player::new(PlayerId::new(3).unwrap()),
    ];

    while !TressetteRules::is_completed(score) {
        let ongoing_hand = OngoingHand::<TressetteRules>::new();
        for trick_id in 0..TressetteRules::TRICKS {
            let ongoing_trick = OngoingTrick::<TressetteRules>::new(first_to_play);
            for player_count in 0..TressetteRules::PLAYERS {
                let next_to_play = ongoing_trick.next_to_play();
                let playable = TressetteRules::playable(&players[*next_to_play], leading_suit);
            }
        }
    }

    assert_ne!(score.0, score.1);
    assert!(score.0 >= tressette::SCORE_TO_WIN || score.1 >= tressette::SCORE_TO_WIN);
}
