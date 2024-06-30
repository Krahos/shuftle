use std::{fmt::Display, ops::Deref};

use crate::common::{
    cards::{Card, ItalianCard, ItalianRank, Suit},
    hands::{PlayerId, TrickTakingGame},
};
use num_rational::Rational32;
use std::cmp::Ordering;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TressetteGame {}

impl TrickTakingGame for TressetteGame {
    type CardType = TressetteCard;

    const PLAYERS: usize = 4;

    const TRICKS: usize = 10;

    /// Contains the logic to determine who won the trick in a standard
    /// tressette game: The winner of the trick is always the player who played
    /// the highest card with the same `Suit` of the first `TressetteCard`
    /// played that trick. See the implementation of `Ord` and `PartialOrd` for
    /// `TressetteCard` for more info. The implementation of this trait is meant
    /// to only be used internally by `OngoingTrick`, however it's possible to
    /// call it elsewhere if needed.
    ///
    /// # Panics
    ///
    /// It can only panic in case of a bug in this crate.
    ///
    /// # Examples
    ///
    /// ```
    /// use shuftlib::common::{hands::{TrickTakingGame, PlayerId}, cards::{ItalianRank, Suit}};
    /// use shuftlib::tressette::{TressetteGame, TressetteCard};
    ///
    /// let cards = [
    ///   TressetteCard::new(ItalianRank::Ace, Suit::Hearts),
    ///   TressetteCard::new(ItalianRank::Two, Suit::Hearts),
    ///   TressetteCard::new(ItalianRank::Three, Suit::Hearts),
    ///   TressetteCard::new(ItalianRank::Four, Suit::Hearts),
    /// ];
    ///
    /// let taker = TressetteGame::determine_taker(&cards, PlayerId::new(2).unwrap());
    /// assert_eq!(taker, PlayerId::new(2).unwrap());
    /// ```
    fn determine_taker(
        cards: &[TressetteCard; Self::PLAYERS],
        first_to_play: PlayerId<{ Self::PLAYERS }>,
    ) -> PlayerId<{ Self::PLAYERS }> {
        let leading_suit = cards[*first_to_play].suit();
        let (taker, _) = cards
            .iter()
            .enumerate()
            .filter(|(_, &c)| c.suit() == leading_suit)
            .max_by_key(|(_, &c)| c)
            .expect("Max by key returned None. This shouldn't have happened, since it's being called on a non empty slice.");

        PlayerId::new(taker).expect("Initialization of a new PlayerId failed. This shouldn't have happened, since the input usize was computed starting from a fixed length slice.")
    }
}

impl TressetteGame {
    pub fn new() -> Self {
        TressetteGame {}
    }
}

#[derive(PartialEq, Eq, Debug, Clone, Copy, Default, Hash)]
/// Representation of a card used in variations of the Tressette game. It's just
/// a new type over `ItalianCard`.
pub struct TressetteCard {
    card: ItalianCard,
}

impl PartialOrd for TressetteCard {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for TressetteCard {
    #[allow(clippy::expect_used)]
    fn cmp(&self, other: &Self) -> Ordering {
        let rank_order = [
            ItalianRank::Four,
            ItalianRank::Five,
            ItalianRank::Six,
            ItalianRank::Seven,
            ItalianRank::Jack,
            ItalianRank::Knight,
            ItalianRank::King,
            ItalianRank::Ace,
            ItalianRank::Two,
            ItalianRank::Three,
        ];

        let self_rank_index = rank_order.iter().position(|&r| self.card.rank() == r).expect("The rank of self wasn't found inside the Ord implementation for TressetteCard. This shouldn't have happened, please file a bug report.");
        let other_rank_index = rank_order.iter().position(|&r| other.card.rank() == r).expect("The rank of other wasn't found inside the Ord implementation for TressetteCard. This shouldn't have happened, please file a bug report.");

        self_rank_index.cmp(&other_rank_index)
    }
}

impl Display for TressetteCard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.card)
    }
}

impl Card for TressetteCard {}

impl From<ItalianCard> for TressetteCard {
    fn from(value: ItalianCard) -> Self {
        TressetteCard { card: value }
    }
}

impl Deref for TressetteCard {
    type Target = ItalianCard;

    fn deref(&self) -> &Self::Target {
        &self.card
    }
}

impl TressetteCard {
    /// Gets the value of the card by the rules of the Tressette game:
    /// - Ace = 1
    /// - 2, 3 and figures = 1/3
    /// - the rest = 0/3
    ///
    /// # Examples
    /// ```
    /// use shuftlib::{tressette::TressetteCard, common::cards::{Suit, ItalianRank}};
    /// use num_rational::Rational32;
    ///
    /// let ace = TressetteCard::new(ItalianRank::Ace, Suit::Hearts);
    /// let two = TressetteCard::new(ItalianRank::Two, Suit::Spades);
    /// let four = TressetteCard::new(ItalianRank::Four, Suit::Clubs);
    /// assert_eq!(ace.value(), Rational32::new(3,3));
    /// assert_eq!(two.value(), Rational32::new(1,3));
    /// assert_eq!(four.value(), Rational32::new(0,3));
    /// ```
    pub fn value(&self) -> Rational32 {
        match self.rank() {
            ItalianRank::Ace => Rational32::new(3, 3),
            ItalianRank::Two
            | ItalianRank::Three
            | ItalianRank::King
            | ItalianRank::Knight
            | ItalianRank::Jack => Rational32::new(1, 3),
            ItalianRank::Four | ItalianRank::Five | ItalianRank::Six | ItalianRank::Seven => {
                Rational32::new(0, 3)
            }
        }
    }

    /// Generates a new `TressetteCard` starting from an `ItalianRank` and
    /// a `Suit`.
    pub fn new(rank: ItalianRank, suit: Suit) -> Self {
        let card = ItalianCard::new(rank, suit);

        TressetteCard { card }
    }
}
