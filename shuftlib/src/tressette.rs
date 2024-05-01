use std::{fmt::Display, ops::Deref};

use crate::cards::{Card, ItalianCard, ItalianRank, Suit};
use anyhow::bail;
use num_rational::Rational32;
use std::cmp::Ordering;

#[derive(PartialEq, Eq, Debug, Clone, Copy, Default, Hash)]
/// Representation of a card used in variations of the Tressette game.
/// It's just a new type over `ItalianCard`.
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

        let self_rank_index = rank_order.iter().position(|&r| self.card.rank() == r).expect("The rank of self wasn't found in the rank_order array. This shouldn't have happened, please file a bug report.");
        let other_rank_index = rank_order.iter().position(|&r| other.card.rank() == r).expect("The rank of other wasn't found in the rank_order array. This shouldn't have happened, please file a bug report.");

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
    /// ```rust
    /// use shuftlib::{tressette::TressetteCard, cards::{Suit, ItalianRank}};
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

    /// Generates a new `TressetteCard` starting from an `ItalianRank` and a
    /// `Suit`.
    pub fn new(rank: ItalianRank, suit: Suit) -> Self {
        let card = ItalianCard::new(rank, suit);

        TressetteCard { card }
    }
}

/// A player id can only be in the range 0..4.
#[derive(Default, Debug, Clone, Copy, Eq, PartialEq, PartialOrd, Ord)]
pub struct PlayerId(usize);

impl PlayerId {
    /// This method simply increments `self` by 1. Note that `PlayerId` can
    /// only be in the range 0..4, so incrementing `self` when the value is 3,
    /// will reset its value to 0.
    /// # Examples
    /// ``` rust
    /// use shuftlib::tressette::PlayerId;
    /// let mut player_id = PlayerId::new(0).unwrap();
    /// player_id.inc();
    /// assert_eq!(player_id, PlayerId::new(1).unwrap());
    /// player_id.inc();
    /// player_id.inc();
    /// player_id.inc();
    /// assert_eq!(player_id, PlayerId::new(0).unwrap());
    /// ```
    pub fn inc(&mut self) {
        if self.0 < 3 {
            self.0 += 1;
        } else {
            self.0 = 0;
        }
    }

    /// Creates a value of type `PlayerId`. Returns None if value is >= 4,
    /// otherwise returns Some(PlayerId(value)).
    pub fn new(value: usize) -> Option<Self> {
        if value < 4 {
            Some(PlayerId(value))
        } else {
            None
        }
    }
}

impl TryFrom<usize> for PlayerId {
    type Error = anyhow::Error;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        if (0..4).contains(&value) {
            Ok(PlayerId(value))
        } else {
            bail!(
                "Tried to convert {} into a PlayerId, but acceptable values are in range 0..4",
                value
            )
        }
    }
}

impl Display for PlayerId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// The number of players in a standard tressette game.
const PLAYERS: usize = 4;

/// A trick is a set of 4 `Play`s and a taker, the player who won the trick,
/// represented as `PlayerId`.
#[derive(Debug, PartialEq, Eq)]
pub struct Trick {
    cards: [TressetteCard; PLAYERS],
    taker: PlayerId,
}

impl Display for Trick {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {} {} {} {}",
            self.cards[0], self.cards[1], self.cards[2], self.cards[3], self.taker
        )
    }
}

impl Trick {
    /// Returns the `TressetteCard` this trick has been won with.
    pub fn taken_with(&self) -> TressetteCard {
        self.cards[self.taker.0]
    }

    /// Getter for the `PlayerId` of the player who won the trick.
    pub fn taker(&self) -> PlayerId {
        self.taker
    }
}

/// A temporary state of a trick that's still not over: not all the players
/// made their move or a taker hasn't been determined yet.
/// The implementation of this type must guarantee that the internal cards
/// array never contains duplicates.
#[derive(Clone, Copy, Debug, Default)]
pub struct OngoingTrick {
    cards: [Option<TressetteCard>; PLAYERS],
    first_to_play: PlayerId,
    next_to_play: PlayerId,
}

impl Deref for OngoingTrick {
    type Target = [Option<TressetteCard>; PLAYERS];

    fn deref(&self) -> &Self::Target {
        &self.cards
    }
}

impl OngoingTrick {
    /// Adds the `TressetteCard` passed as parameter to the `OngoingTrick`.
    /// It does not check if the card has been played in previous tricks.
    ///
    /// # Errors
    /// This method can fail when trying to play a card that's already been played
    /// and thus contained in this `OngoingTrick`.
    ///
    /// # Examples
    /// ```rust
    /// use shuftlib::{tressette::{OngoingTrick, PlayerId, TressetteCard}, cards::{ItalianRank, Suit}};
    ///
    /// let card = TressetteCard::new(ItalianRank::Ace, Suit::Hearts);
    /// let mut trick = OngoingTrick::default();
    /// // First card added to a trick will always be `Ok`.
    /// trick.play(card).unwrap();
    ///
    /// assert_eq!(trick[0], Some(card));
    /// // In a default `OngoingTrick`, the first player to move is player 0.
    /// // After each card is played, next player index is incremented.
    /// assert_eq!(trick.next_to_play(), PlayerId::new(1).unwrap());
    ///
    /// // Trying to play the same card again results in an error
    /// assert!(trick.play(card).is_err());
    /// ```
    pub fn play(&mut self, card: TressetteCard) -> anyhow::Result<()> {
        if self.cards.contains(&Some(card)) {
            bail!("")
        }

        self.cards[self.next_to_play.0] = Some(card);
        self.next_to_play.inc();
        Ok(())
    }

    /// Gets the `PlayerId` of the player who goes first in a trick.
    pub fn first_to_play(self) -> PlayerId {
        self.first_to_play
    }

    /// Gets the `PlayerId` of the player who has to play next.
    pub fn next_to_play(self) -> PlayerId {
        self.next_to_play
    }

    /// Tries to transform the current `OngoingTrick` into a `Trick` by
    /// determining the taker of the trick. It doesn't make any assumption on
    /// the state of the game. It also does not check if it contains duplicates:
    /// the implemenation of `OngoingTrick` guarantees this property.
    /// # Errors
    /// Fails if any of the moves of the `OngoingTrick` this is called on is
    /// None. It means that not all players made their move yet, so a taker
    /// can't be determined.
    /// # Examples
    /// ```rust
    /// use shuftlib::{tressette::{OngoingTrick, PlayerId, TressetteCard}, cards::{ItalianRank, Suit}};
    ///
    /// let cards = [
    ///   TressetteCard::new(ItalianRank::Ace, Suit::Hearts),
    ///   TressetteCard::new(ItalianRank::Two, Suit::Hearts),
    ///   TressetteCard::new(ItalianRank::Three, Suit::Hearts),
    ///   TressetteCard::new(ItalianRank::Four, Suit::Hearts),
    /// ];
    /// let mut ongoing_trick = OngoingTrick::default();
    /// ongoing_trick.play(cards[0]).unwrap();
    ///
    /// assert_eq!(None, ongoing_trick.finish());
    ///
    /// cards.iter().skip(1).for_each(|&p| {ongoing_trick.play(p).unwrap();});
    ///
    /// println!("{cards:?}");
    /// let trick = ongoing_trick.finish().unwrap();
    /// assert_eq!(Some(trick.taker()), PlayerId::new(2));
    /// ```
    pub fn finish(self) -> Option<Trick> {
        let mut cards: [TressetteCard; PLAYERS] = [TressetteCard::default(); PLAYERS];
        if self
            .iter()
            .enumerate()
            .map(|(i, &x)| {
                if let Some(c) = x {
                    cards[i] = c;
                    true
                } else {
                    false
                }
            })
            .any(|is_some| !is_some)
        {
            return None;
        }

        let leading_suit = cards[self.first_to_play.0].suit();
        if let Some((taker, _)) = cards
            .iter()
            .enumerate()
            .filter(|(_, &c)| c.suit() == leading_suit)
            .max_by_key(|(_, &c)| c)
        {
            return Some(Trick {
                cards,
                taker: PlayerId(taker),
            });
        }
        None
    }

    /// Creates a new `OngoingTrick`, by defining the player who has to play first.
    pub fn new(first_to_play: PlayerId) -> Self {
        Self {
            cards: [None; PLAYERS],
            first_to_play,
            next_to_play: first_to_play,
        }
    }
}

/// There are always 10 tricks in a standard tressette game.
const _TRICKS: usize = 10;

#[cfg(test)]
mod tests {
    use proptest::collection::hash_set;
    use proptest::{array, prelude::*};
    use std::panic::{self, AssertUnwindSafe};

    use crate::{
        cards::{ItalianRank, Suit},
        tressette::{OngoingTrick, PlayerId, TressetteCard},
    };

    /// Strategy to create a random `ItalianRank`.
    fn rank_strategy() -> impl Strategy<Value = ItalianRank> {
        prop_oneof![
            Just(ItalianRank::Ace),
            Just(ItalianRank::Two),
            Just(ItalianRank::Three),
            Just(ItalianRank::Four),
            Just(ItalianRank::Five),
            Just(ItalianRank::Six),
            Just(ItalianRank::Seven),
            Just(ItalianRank::Jack),
            Just(ItalianRank::Knight),
            Just(ItalianRank::King),
        ]
    }

    /// Strategy to create a random `Suit`.
    fn suit_strategy() -> impl Strategy<Value = Suit> {
        prop_oneof![
            Just(Suit::Hearts),
            Just(Suit::Clubs),
            Just(Suit::Spades),
            Just(Suit::Diamonds),
        ]
    }

    /// Strategy to create a random `TressetteCard`.
    fn tressette_card_strategy() -> impl Strategy<Value = TressetteCard> {
        (rank_strategy(), suit_strategy()).prop_map(|(rank, suit)| TressetteCard::new(rank, suit))
    }

    /// Strategy to create an `OngoingTrick` filled with random cards.
    /// Since the `OngoingTrick` already contains the cards, `first_to_play`
    /// and `next_to_play` are irrelevant. Change this function accordingly if
    /// You need those to have the correct value.
    fn ongoing_trick_strategy() -> impl Strategy<Value = OngoingTrick> {
        hash_set(tressette_card_strategy(), 4).prop_map(|hash_set| {
            let mut cards = [None; 4];
            hash_set
                .iter()
                .enumerate()
                .for_each(|(i, &c)| cards[i] = Some(c));

            OngoingTrick {
                cards,
                first_to_play: PlayerId(0),
                next_to_play: PlayerId(0),
            }
        })
    }

    proptest! {
        #[test]
        fn play_method_works(cards in array::uniform4(tressette_card_strategy())) {
            let mut trick = OngoingTrick::default();

            // `catch_unwind()` will return `Ok` if the closure does not panic,
            // it will return `Err` if it panics.
            let result = panic::catch_unwind(AssertUnwindSafe(|| {
                for (index, &card) in cards.iter().enumerate() {
                    // Panicking if there are duplicates in the cards array.
                    trick.play(card).unwrap();
                    // If the card was successfully played, it will be contained
                    // inside the `OngoingTrick` struct as `Some`.
                    assert_eq!(trick[index], Some(card));
            }}));


            // If the input array contains duplicates, the above code should
            // result in an `Err`.
            if contains_duplicates(&cards) {
                prop_assert!(result.is_err());
            } else {
                prop_assert!(result.is_ok());
                // All the elements of the `OngoingTrick` should also be `Some`
                // at this point.
                prop_assert!(!trick.iter().any(|&x| x.is_none()));
                // And the next_to_play should be the player who went first.
                prop_assert_eq!(trick.next_to_play(), trick.first_to_play());
            }
        }
    }

    proptest! {
        #[test]
        fn finish_method_works(ongoing_trick in ongoing_trick_strategy()) {
            let maybe_trick = ongoing_trick.finish();

            let cards: Option<Vec<TressetteCard>> = ongoing_trick.into_iter().collect();
            let cards = cards.unwrap();

            // Property #1: if the ongoing trick contains duplicates, something
            // went very wrong somewhere.
            if contains_duplicates(&cards) {
                prop_assert!(maybe_trick.is_none());
            } else {
                prop_assert!(maybe_trick.is_some());
                let trick = maybe_trick.unwrap();
                // Property #2: the suit of the card this trick has been taken
                // with, has to be exactly the same as the first card played.
                prop_assert_eq!(trick.taken_with().suit(), ongoing_trick[ongoing_trick.first_to_play.0].unwrap().suit(), "The suit of the card this trick has been taken with wasn't the same suit of the first card played");
                // Property #3: the rank of the card this trick has been taken
                // with has to be the highest, among the cards with the same suit.
                prop_assert!(!cards.iter().filter(|c| c.suit() == trick.taken_with().suit()).any(|&c| c > trick.taken_with()));
            }
        }
    }

    fn contains_duplicates(cards: &[TressetteCard]) -> bool {
        for i in 0..cards.len() - 1 {
            for j in (i + 1)..cards.len() {
                if cards[i] == cards[j] {
                    return true;
                }
            }
        }
        false
    }
}
