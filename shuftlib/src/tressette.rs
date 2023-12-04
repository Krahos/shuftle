use crate::cards::{Card, Deck, ItalianCard, ItalianRank, Suit};
use num_rational::Rational32;

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub struct TressetteCard {
    card: ItalianCard,
    card_value: CardValue,
}

impl Card for TressetteCard {}

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum CardValue {
    One,
    OneThird,
    Zero,
}

impl From<CardValue> for Rational32 {
    fn from(value: CardValue) -> Self {
        match value {
            CardValue::One => Rational32::new(3, 3),
            CardValue::OneThird => Rational32::new(1, 3),
            CardValue::Zero => Rational32::new(0, 3),
        }
    }
}

impl From<ItalianRank> for CardValue {
    fn from(value: ItalianRank) -> Self {
        match value {
            ItalianRank::Ace => CardValue::One,
            ItalianRank::Two
            | ItalianRank::Three
            | ItalianRank::King
            | ItalianRank::Knight
            | ItalianRank::Jack => CardValue::OneThird,
            ItalianRank::Four | ItalianRank::Five | ItalianRank::Six | ItalianRank::Seven => {
                CardValue::Zero
            }
        }
    }
}

impl From<ItalianCard> for TressetteCard {
    fn from(value: ItalianCard) -> Self {
        TressetteCard {
            card: value,
            card_value: value.rank().into(),
        }
    }
}

impl TressetteCard {
    pub fn rank(&self) -> ItalianRank {
        self.card.rank()
    }

    pub fn suit(&self) -> Suit {
        self.card.suit()
    }

    pub fn value(&self) -> &CardValue {
        &self.card_value
    }

    pub fn new(rank: ItalianRank, suit: Suit) -> Self {
        let card = ItalianCard::new(rank, suit);

        TressetteCard {
            card,
            card_value: rank.into(),
        }
    }
}

pub struct Player {
    hand: Vec<TressetteCard>,
    tricks_taken: Vec<TakenTrick>,
}

impl Player {
    fn new() -> Self {
        Player {
            hand: Vec::with_capacity(TRICKS),
            tricks_taken: Vec::new(),
        }
    }

    pub fn has_suit(&self, suit: Suit) -> bool {
        self.hand.iter().any(|c| c.suit() == suit)
    }

    fn remove_card(&mut self, card: TressetteCard) -> Result<TressetteCard, RemovalError> {
        for i in 0..self.hand.len() {
            if card == self.hand[i] {
                return Ok(self.hand.remove(i));
            }
        }
        Err(RemovalError)
    }
}

#[derive(Debug)]
struct RemovalError;

impl Default for Player {
    fn default() -> Self {
        Self::new()
    }
}

const PLAYERS: usize = 4;

pub struct TakenTrick {
    cards: [TressetteCard; PLAYERS],
}

struct Trick {
    cards: [Option<TressetteCard>; PLAYERS],
}

const TRICKS: usize = 10;

pub struct GameState {
    players: [Player; PLAYERS],
    trick_counter: usize,
    dealer: usize,
    player_to_move: usize,
    trick_suit: Option<Suit>,
    first_to_move: usize,
    trick: Option<Trick>,
}

impl GameState {
    pub fn new() -> Self {
        // Create a deck
        let mut deck = Deck::italian();
        // Shuffle it
        deck.shuffle();

        // Create the players
        let mut players: [Player; PLAYERS] = Default::default();

        // Cards in tressette are distributed twice for each player.
        // And for each time distributing, each one is given 5 cards.
        const TIMES_DISTRIBUTING: usize = 2;
        for _ in 0..TIMES_DISTRIBUTING {
            for player in &mut players {
                const DISTRIBUTED_CARDS: usize = 5;
                for _ in 0..DISTRIBUTED_CARDS {
                    player.hand.push(deck.draw().unwrap_or_default().into());
                }
            }
        }

        GameState {
            players,
            trick_counter: 0,
            dealer: 0,
            player_to_move: 0,
            trick_suit: None,
            first_to_move: 0,
            trick: None,
        }
    }

    pub fn play_card(&mut self, player: usize, card: TressetteCard) -> Result<(), PlayError> {
        // If it's not the input player's turn.
        if player != self.player_to_move {
            return Err(PlayError::NotTheirTurn);
        }

        // If they are trying to play a card they don't have.
        if !self.players[player].hand.contains(&card) {
            return Err(PlayError::CardNotOwned);
        }

        if player == self.first_to_move {
            // The suit to play becomes the one just played.
            self.trick_suit = Some(card.suit());

            // Actually put the card on the board.
            let mut cards = [None, None, None, None];
            cards[player] = Some(card);
            let trick = Some(Trick { cards });

            self.trick = trick;

            self.players[player].remove_card(card).unwrap();

            // Next player to play is the one to the right of who just played.
            self.inc_player_to_move();
        } else {
            // If they have the trick suit, but they are trying to play another one.
            if self.players[player].has_suit(self.trick_suit.unwrap())
                && card.suit() != self.trick_suit.unwrap()
            {
                return Err(PlayError::DidntFollowSuit);
            }

            // if let Some(&mut trick) = &self.trick {}
        }

        Ok(())
    }

    fn inc_player_to_move(&mut self) {
        if self.player_to_move < PLAYERS - 1 {
            self.player_to_move += 1;
        } else {
            self.player_to_move = 0;
        }
    }
}

pub enum PlayError {
    NotTheirTurn,
    CardNotOwned,
    DidntFollowSuit,
    Other,
}

impl Default for GameState {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::GameState;

    #[test]
    fn inc_player_to_move_works() {
        let mut state = GameState::new();

        assert_eq!(0, state.player_to_move);

        state.inc_player_to_move();
        assert_eq!(1, state.player_to_move);

        state.inc_player_to_move();
        state.inc_player_to_move();
        state.inc_player_to_move();
        assert_eq!(0, state.player_to_move);
    }
}
