use anyhow::bail;
use num_rational::Rational32;

use crate::cards::{Card, Deck, ItalianCard, ItalianRank, Suit};

pub struct TressetteCard {
    card: ItalianCard,
    card_value: CardValue,
}

impl Card for TressetteCard {}

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
    number: PlayerNumber,
    hand: Vec<TressetteCard>,
    tricks_taken: Vec<Trick>,
}

struct PlayerNumber(u8);

impl TryFrom<u8> for PlayerNumber {
    type Error = anyhow::Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if value > 3 {
            bail!("Provided value wasn't in the range 0..4");
        } else {
            Ok(PlayerNumber(value))
        }
    }
}
impl From<PlayerNumber> for u8 {
    fn from(value: PlayerNumber) -> Self {
        value.0
    }
}

const PLAYERS: usize = 4;

pub struct Trick {
    cards: [TressetteCard; PLAYERS],
}

pub struct TrickState {
    first_mov: usize,
    current_mov: usize,
}

const TRICKS: usize = 10;

pub struct GameState {
    players: [Player; PLAYERS],
    current_trick: usize,
}
