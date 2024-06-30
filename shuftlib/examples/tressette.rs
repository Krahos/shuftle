// use shuftlib::{
//     cards::Deck,
//     tressette::{self, Game},
// };

fn main() {}

// fn main() {
//     let mut game = Game::new();

//     // A new game is typically played untill the accumulated of a team reaches 31.
//     while let Game::Ongoing(state) = game {
//         // Cards are distributed among players at the beginning of each game.
//         state.distribute();
//         // The game is played until every player is out of cards: Each player
//         // has 10 cards at the beginning, so there will be 10 tricks.
//         0..tressette::TRICKS.iter().for_each(|_| {
//             // Each player has to make their play, players are 4.
//             0..tressette::PLAYERS.iter().for_each(|_| {
//                 // The game decides who's turn it is to play.
//                 let player = state.next_to_play();
//                 // In this example we are selecting a random card among the
//                 // legally playable cards from the player hand. Here you can
//                 // put the logic to accept input from users or evaluate the card
//                 // to play with a more complicated and more correct algorithm.
//                 let playable = player.playable();
//                 let card = playable[rng.next(playable.len())];
//                 game.play(player.id(), card).unwrap();
//             })
//         })
//     }

//     for hand in game {
//         for trick in hand {
//             (0..tressette::PLAYERS).iter().for_each(|_| {
//                 // The game decides who's turn it is to play.
//                 let player = state.next_to_play();
//                 // In this example we are selecting a random card among the
//                 // legally playable cards from the player hand. Here you can
//                 // put the logic to accept input from users or evaluate the card
//                 // to play with a more complicated and more correct algorithm.
//                 let playable = player.playable();
//                 let card = playable[rng.next(playable.len())];
//                 game.play(player.id(), card).unwrap();
//             })
//         }
//     }

//     if let Game::Completed {
//         team1_score,
//         team2_score,
//         hands,
//     } = game
//     {
//         println!(
//             "team 1 score is: {}\nteam 2 score is: {}\ntotal hands played: {}",
//             team1_score,
//             team2_score,
//             hands.len()
//         );
//     }
// }
