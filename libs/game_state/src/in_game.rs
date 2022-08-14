use crate::game_state::Rules;
use common::{ByteStrRowDisplay, RowDisplay, *, xs::Xs};

use lazy_static::lazy_static;
use std::fmt;

#[derive(Clone, Default)]
pub struct State {
    pub cpu_hands: [Hand; 3],
    pub hand: Hand,
    pub deck: Hand,
    pub discard: Hand,
    pub current_player: PlayerID,
    pub top_wild_declared_as: Option<Suit>,
    pub winners: Vec<PlayerID>,
    pub card_animations: Vec<CardAnimation>,
    // control state
    pub hand_index: u8,
}

macro_rules! dealt_hand {
    ($deck:expr, $spread:expr) => {{
        let mut hand = Hand::new($spread);

        hand.draw_from($deck);
        hand.draw_from($deck);
        hand.draw_from($deck);
        hand.draw_from($deck);
        hand.draw_from($deck);

        hand
    }};
}

impl State {
    pub fn new(rng: &mut Xs) -> Self {
        let mut deck = Hand::new_shuffled_deck(rng);

        let discard = Hand::new(Spread::stack(DISCARD_X, DISCARD_Y));

        let hand = dealt_hand!(
            &mut deck,
            Spread::LTR(TOP_AND_BOTTOM_HAND_EDGES, PLAYER_HAND_HEIGHT)
        );

        let cpu_hands = [
            dealt_hand!(
                &mut deck,
                Spread::TTB(LEFT_AND_RIGHT_HAND_EDGES, LEFT_CPU_HAND_X)
            ),
            dealt_hand!(
                &mut deck,
                Spread::LTR(TOP_AND_BOTTOM_HAND_EDGES, MIDDLE_CPU_HAND_HEIGHT,)
            ),
            dealt_hand!(
                &mut deck,
                Spread::TTB(LEFT_AND_RIGHT_HAND_EDGES, RIGHT_CPU_HAND_X)
            ),
        ];

        //The player whose turn comes after this player will go first.
        let current_player = xs::range(rng, 0..cpu_hands.len() as u32 + 1) as u8;

        invariant_assert!(current_player <= cpu_hands.len() as u8);

        let card_animations = Vec::with_capacity(DECK_SIZE as _);

        //We expect this to be replaced with a new vector when ever it it changed.
        let winners = Vec::with_capacity(0);

        State {
            cpu_hands,
            hand,
            deck,
            discard,
            current_player,
            winners,
            top_wild_declared_as: None,
            card_animations,
            hand_index: 0,
        }
    }

    pub fn reshuffle_discard(&mut self, rng: &mut Xs) -> Option<()> {
        let top_card = self.discard.draw()?;

        self.deck.fill(self.discard.drain());
        self.deck.shuffle(rng);

        self.discard.push(top_card);

        Some(())
    }

    pub fn remove_positioned_card(
        &mut self,
        playerId: PlayerID,
        card_index: u8,
    ) -> Option<PositionedCard> {
        let hand = self.get_hand_mut(playerId);
        hand.remove_if_present(card_index)
    }

    pub fn get_hand(&self, playerId: PlayerID) -> &Hand {
        let index = playerId as usize;
        let len = self.cpu_hands.len();
        if index < len {
            &self.cpu_hands[index]
        } else if index == len {
            &self.hand
        } else {
            invariant_violation!({ &self.discard }, "Could not find hand for {:?}", playerId)
        }
    }

    pub fn get_hand_mut(&mut self, playerId: PlayerID) -> &mut Hand {
        let index = playerId as usize;
        let len = self.cpu_hands.len();
        if index < len {
            &mut self.cpu_hands[index]
        } else if index == len {
            &mut self.hand
        } else {
            invariant_violation!(
                { &mut self.discard },
                "Could not find hand for {:?}",
                playerId
            )
        }
    }

    pub fn get_winner_text(&self) -> String {
        let winner_names: Vec<_> = self
            .winners
            .iter()
            .map(|&player| player_name(player))
            .collect();

        let mut winner_text = get_sentence_list(&winner_names);

        let suffix = if self.winners.len() == 1 && winner_text != "you" {
            " wins."
        } else if self.winners.len() > 3 {
            " all win."
        } else {
            " win."
        };

        winner_text.push_str(suffix);

        winner_text
    }

    pub fn missing_cards(&self) -> Vec<Card> {
        use std::collections::BTreeSet;

        let example_deck: BTreeSet<Card> = fresh_deck().into_iter().collect();

        let mut observed_deck = BTreeSet::new();

        let card_iter = self
            .deck
            .iter()
            .chain(self.discard.iter())
            .chain(self.cpu_hands.iter().flat_map(|h| h.iter()))
            .chain(self.hand.iter())
            .chain(self.card_animations.iter().map(|a| &a.card.card));

        for c in card_iter {
            observed_deck.insert(c.clone());
        }

        example_deck.difference(&observed_deck).cloned().collect()
    }

    pub fn animations_settled(&self) -> bool {
        self.card_animations.len() == 0
    }

    pub fn no_winners_yet(&self) -> bool {
        self.winners.len() == 0
    }

    pub fn round_is_over(&self) -> bool {
        !self.no_winners_yet() && self.animations_settled()
    }

    pub fn get_new_card_position(&self, hand: RelativeHand, player: PlayerID) -> (u8, u8) {
        match hand {
            RelativeHand::Deck => (DECK_X, DECK_Y),
            RelativeHand::Discard => (DISCARD_X, DISCARD_Y),
            RelativeHand::Player(p) => self.get_player_new_card_position(p.apply(player)),
        }
    }

    pub fn get_player_new_card_position(&self, player: PlayerID) -> (u8, u8) {
        let hand = self.get_hand(player);
        let len = hand.len();

        get_card_position(hand.spread, len + 1, len)
    }

    pub fn get_relative_hand_mut<'a>(
        &'a mut self,
        hand: RelativeHand,
        player: PlayerID,
    ) -> &'a mut Hand {
        match hand {
            RelativeHand::Deck => &mut self.deck,
            RelativeHand::Discard => &mut self.discard,
            RelativeHand::Player(p) => match p.apply(player) {
                id if is_player(id) => &mut self.hand,
                id => &mut self.cpu_hands[id as usize],
            },
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Change {
    CurrentPlayer(RelativePlayer),
    CardLocation(CardMovement),
    //TopWild(TopWild),
}

#[macro_export]
macro_rules! change_match {
        {$change:expr, {$name:ident => $code:expr}} => {
            match $change {
                Change::CurrentPlayer($name) => $code,
                Change::CardLocation($name) => $code,
            }
        }
    }

impl AllValues for Change {
    //TODO write a procedural macro or something to make mainatining this easier.
    fn all_values() -> Vec<Self> {
        let intial = if loops_allowed!() {
            RelativePlayer::all_values()
        } else {
            vec![]
        }
        .into_iter()
        .map(Change::CurrentPlayer);

        intial
            .chain(
                CardMovement::all_values()
                    .into_iter()
                    .map(Change::CardLocation),
            )
            .collect()
    }
}

lazy_static! {
    pub static ref ALL_CHANGES: Vec<Change> = Change::all_values();
}

implement!(
    from_rng for Change,
    by picking from &ALL_CHANGES
);

impl fmt::Debug for Change {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl fmt::Display for Change {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if f.alternate() {
            change_match! {*self, {
                v => write!(f, "{:#}", v)
            }}
        } else {
            change_match! {*self, {
                v => write!(f, "{}", v)
            }}
        }
    }
}

impl RowDisplay for Change {
    fn row_label(&self) -> RowLabel {
        change_match! {*self, {
            v => v.row_label()
        }}
    }
}

//This relies on MAX_PLAYER_ID being 3, and will require structural changes if it changes!
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum RelativePlayer {
    Same,
    Next,
    Across,
    Previous,
}

impl RelativePlayer {
    pub fn get_game_player(current_player: PlayerID) -> RelativePlayer {
        match current_player {
            0 => RelativePlayer::Previous,
            1 => RelativePlayer::Across,
            2 => RelativePlayer::Next,
            _ => RelativePlayer::Same,
        }
    }
}

impl AllValues for RelativePlayer {
    fn all_values() -> Vec<Self> {
        vec![
            RelativePlayer::Same,
            RelativePlayer::Next,
            RelativePlayer::Across,
            RelativePlayer::Previous,
        ]
    }
}

impl RelativePlayer {
    pub fn apply(&self, playerId: PlayerID) -> PlayerID {
        match *self {
            RelativePlayer::Same => playerId,
            RelativePlayer::Next => (playerId + 1) % (MAX_PLAYER_ID + 1),
            RelativePlayer::Across => (playerId + 2) % (MAX_PLAYER_ID + 1),
            RelativePlayer::Previous => (playerId + 3) % (MAX_PLAYER_ID + 1),
        }
    }
}

impl fmt::Debug for RelativePlayer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RelativePlayer::Same => write!(f, "my turn again"),
            RelativePlayer::Next => write!(f, "next after me"),
            RelativePlayer::Across => write!(f, "across from me"),
            RelativePlayer::Previous => write!(f, "previous to me"),
        }
    }
}

impl fmt::Display for RelativePlayer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if f.alternate() {
            return match self {
                RelativePlayer::Same => write!(f, "s"),
                RelativePlayer::Next => write!(f, "n"),
                RelativePlayer::Across => write!(f, "a"),
                RelativePlayer::Previous => write!(f, "p"),
            };
        }
        write!(f, "{:?}\n", self)?;

        for &id in all_player_ids().into_iter() {
            write!(
                f,
                "{}->{}",
                player_1_char_name(id),
                player_1_char_name(self.apply(id))
            )?;

            if id != MAX_PLAYER_ID {
                write!(f, ", ")?;
            }
        }
        Ok(())
    }
}

impl<'a> ByteStrRowDisplay<'a> for RelativePlayer {
    fn byte_str_row_label(&self) -> &'a [u8] {
        b"turn -> turn: "
    }
}

implement!(
    from_rng for RelativePlayer,
    by picking from RelativePlayer::all_values()
);

//This relies on MAX_PLAYER_ID being 3, and will require structural changes if it changes!
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct RelativePlayerSet(u8);

const SAME_FLAG: u8 = 1;
const NEXT_FLAG: u8 = 2;
const ACROSS_FLAG: u8 = 4;
const PREVIOUS_FLAG: u8 = 8;

impl RelativePlayerSet {
    #[inline]
    pub fn contains(&self, player: RelativePlayer) -> bool {
        match player {
            RelativePlayer::Same => self.0 & SAME_FLAG != 0,
            RelativePlayer::Next => self.0 & NEXT_FLAG != 0,
            RelativePlayer::Across => self.0 & ACROSS_FLAG != 0,
            RelativePlayer::Previous => self.0 & PREVIOUS_FLAG != 0,
        }
    }

    #[inline]
    pub fn insert(self, player: RelativePlayer) -> Self {
        RelativePlayerSet(match player {
            RelativePlayer::Same => self.0 | SAME_FLAG,
            RelativePlayer::Next => self.0 | NEXT_FLAG,
            RelativePlayer::Across => self.0 | ACROSS_FLAG,
            RelativePlayer::Previous => self.0 | PREVIOUS_FLAG,
        })
    }

    #[inline]
    pub fn remove(self, player: RelativePlayer) -> Self {
        let bits = match player {
            RelativePlayer::Same => self.0 & !SAME_FLAG,
            RelativePlayer::Next => self.0 & !NEXT_FLAG,
            RelativePlayer::Across => self.0 & !ACROSS_FLAG,
            RelativePlayer::Previous => self.0 & !PREVIOUS_FLAG,
        };
        RelativePlayerSet(bits)
    }
}

impl AllValues for RelativePlayerSet {
    fn all_values() -> Vec<Self> {
        RelativePlayerSet::sets_from_range(0..1 << 4)
    }
}

use std::ops::Range;

impl RelativePlayerSet {
    pub fn all_non_empty_values() -> Vec<Self> {
        RelativePlayerSet::sets_from_range(1..1 << 4)
    }

    fn sets_from_range(range: Range<u8>) -> Vec<Self> {
        range.into_iter().map(RelativePlayerSet).collect()
    }
}

impl Iterator for RelativePlayerSet {
    type Item = RelativePlayer;

    fn next(&mut self) -> Option<Self::Item> {
        for player in RelativePlayer::all_values() {
            if self.contains(player) {
                *self = self.remove(player);
                return Some(player);
            }
        }
        None
    }
}

impl RelativePlayerSet {
    pub fn absolute_players(&self, player: PlayerID) -> Vec<PlayerID> {
        self.clone().map(|p| p.apply(player)).collect()
    }
}

impl fmt::Debug for RelativePlayerSet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{")?;
        for player in RelativePlayer::all_values() {
            if self.contains(player) {
                write!(f, "{:?}", player)?;
            }
            match player {
                RelativePlayer::Same | RelativePlayer::Next | RelativePlayer::Across => {
                    write!(f, ", ")?;
                }
                RelativePlayer::Previous => {}
            }
        }
        write!(f, "}}")
    }
}

impl fmt::Display for RelativePlayerSet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if f.alternate() {
            write!(f, "{{")?;
            for player in RelativePlayer::all_values() {
                if self.contains(player) {
                    write!(f, "{:#}", player)?;
                }
            }
            write!(f, "}}")?;

            return Ok(());
        }
        match (
            self.contains(RelativePlayer::Previous),
            self.contains(RelativePlayer::Across),
            self.contains(RelativePlayer::Next),
            self.contains(RelativePlayer::Same),
        ) {
            (true, true, true, true) => write!(f, "everyone"),
            (true, true, true, false) => {
                write!(f, "everyone but ")?;
                set_display(RelativePlayer::Same, f)
            }
            (true, true, false, true) => {
                write!(f, "everyone but ")?;
                set_display(RelativePlayer::Next, f)
            }
            (true, false, true, true) => {
                write!(f, "everyone but ")?;
                set_display(RelativePlayer::Across, f)
            }
            (false, true, true, true) => {
                write!(f, "everyone but ")?;
                set_display(RelativePlayer::Previous, f)
            }

            (true, true, false, false) => write!(f, "the last two players"),
            (true, false, true, false) => write!(f, "the adjacent players"),
            (true, false, false, true) => write!(f, "the current and previous players"),
            (false, true, true, false) => write!(f, "the next two players"),
            (false, true, false, true) => write!(f, "the current and across players"),
            (false, false, true, true) => write!(f, "the current and next players"),

            (true, false, false, false) => set_display(RelativePlayer::Previous, f),
            (false, true, false, false) => set_display(RelativePlayer::Across, f),
            (false, false, true, false) => set_display(RelativePlayer::Next, f),
            (false, false, false, true) => set_display(RelativePlayer::Same, f),
            (false, false, false, false) => write!(f, "no one"),
        }
    }
}

fn set_display(player: RelativePlayer, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match player {
        RelativePlayer::Previous => write!(f, "the previous player"),
        RelativePlayer::Across => write!(f, "the across player"),
        RelativePlayer::Next => write!(f, "the next player"),
        RelativePlayer::Same => write!(f, "the current player"),
    }
}

implement!(
    from_rng for RelativePlayerSet,
    by picking from RelativePlayerSet::all_values()
);

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct CardMovement {
    pub affected: RelativePlayerSet,
    pub source: RelativeHand,
    pub target: RelativeHand,
    pub selection: CardSelection,
}

impl AllValues for CardMovement {
    fn all_values() -> Vec<CardMovement> {
        let sets = RelativePlayerSet::all_non_empty_values();
        let hands = RelativeHand::all_values();
        let selections = CardSelection::all_values();

        let mut output =
            Vec::with_capacity(sets.len() * hands.len() * hands.len() * selections.len());

        for selection in selections {
            for &affected in sets.iter() {
                for &source in hands.iter() {
                    for &target in hands.iter() {
                        if source == target {
                            continue;
                        }

                        if loops_allowed!() {
                            //allow all combinations
                        } else {
                            if target == RelativeHand::Player(RelativePlayer::Same) {
                                continue;
                            }

                            if source == RelativeHand::Discard && target != RelativeHand::Deck {
                                continue;
                            }

                            if source == RelativeHand::Deck && target != RelativeHand::Discard {
                                continue;
                            }
                        }

                        output.push(CardMovement {
                            affected,
                            source,
                            target,
                            selection,
                        });
                    }
                }
            }
        }

        output
    }
}

impl<'a> ByteStrRowDisplay<'a> for CardMovement {
    fn byte_str_row_label(&self) -> &'a [u8] {
        b"hand -> hand: "
    }
}

impl fmt::Display for CardMovement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:#} {:#} {:#} {:#}",
            self.affected, self.source, self.target, self.selection
        )?;

        write!(
            f,
            "{} moves {} from {} to {}",
            self.affected, self.selection, self.source, self.target
        )
    }
}

#[allow(dead_code)]
enum RefsMut<'a, T> {
    Pair(&'a mut T, &'a mut T),
    Same(&'a mut T),
}

#[allow(dead_code)]
fn get_refs_mut<'a>(
    state: &'a mut State,
    h1: RelativeHand,
    h2: RelativeHand,
    player: PlayerID,
) -> RefsMut<'a, Hand> {
    if h1 == h2 {
        RefsMut::Same(state.get_relative_hand_mut(h1, player))
    } else {
        let source: &mut Hand =
            unsafe { &mut *(state.get_relative_hand_mut(h1, player) as *mut Hand) };

        let target = state.get_relative_hand_mut(h2, player);

        RefsMut::Pair(source, target)
    }
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum RelativeHand {
    Player(RelativePlayer),
    Deck,
    Discard,
}

impl RelativeHand {
    pub fn get_game_player_hand(current_player: PlayerID) -> RelativeHand {
        RelativeHand::Player(RelativePlayer::get_game_player(current_player))
    }
}

impl fmt::Display for RelativeHand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if f.alternate() {
            return match *self {
                RelativeHand::Player(p) => write!(f, "{:#}", p),
                RelativeHand::Deck => write!(f, "deck"),
                RelativeHand::Discard => write!(f, "discard"),
            };
        }
        match *self {
            RelativeHand::Player(p) => match p {
                RelativePlayer::Same => write!(f, "their hand"),
                RelativePlayer::Next => write!(f, "the next player's hand"),
                RelativePlayer::Across => write!(f, "the hand of the player across from them"),
                RelativePlayer::Previous => write!(f, "the previous player's hand"),
            },
            RelativeHand::Deck => write!(f, "the deck"),
            RelativeHand::Discard => write!(f, "the discard pile"),
        }
    }
}

impl AllValues for RelativeHand {
    fn all_values() -> Vec<RelativeHand> {
        RelativePlayer::all_values()
            .into_iter()
            .map(RelativeHand::Player)
            .chain(vec![RelativeHand::Deck, RelativeHand::Discard])
            .collect()
    }
}

implement!(
    from_rng for RelativeHand,
    by picking from RelativeHand::all_values()
);

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum AbsoluteHand {
    Player(PlayerID),
    Deck,
    Discard,
}

impl RelativeHand {
    pub fn apply(self, player: PlayerID) -> AbsoluteHand {
        match self {
            RelativeHand::Player(p) => AbsoluteHand::Player(p.apply(player)),
            RelativeHand::Deck => AbsoluteHand::Deck,
            RelativeHand::Discard => AbsoluteHand::Discard,
        }
    }
}

impl fmt::Display for AbsoluteHand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if f.alternate() {
            return match *self {
                AbsoluteHand::Player(p) => write!(f, "{}", player_1_char_name(p)),
                AbsoluteHand::Deck => write!(f, "deck"),
                AbsoluteHand::Discard => write!(f, "discard"),
            };
        }
        match *self {
            AbsoluteHand::Player(p) => write!(f, "{}", player_name(p)),
            AbsoluteHand::Deck => write!(f, "the deck"),
            AbsoluteHand::Discard => write!(f, "the discard pile"),
        }
    }
}

#[derive(Clone, Debug)]
pub enum Layer {
    Card,
    Changes,
    Done,
}

impl Default for Layer {
    fn default() -> Self {
        Layer::Card
    }
}

#[derive(Clone, Debug, Default)]
pub struct ChoiceState {
    reset_changes: Vec<Change>,
    pub changes: Vec<Change>,
    pub left_scroll: usize,
    pub right_scroll: usize,
    pub marker_y: u8,
    pub scroll_card: Card,
    pub card_set: CardFlags,
    pub layer: Layer,
    pub description: Vec<u8>,
}

pub struct ChoiceStateAndRules<'a> {
    pub choice_state: &'a mut ChoiceState,
    pub rules: &'a Rules,
}

implement!(
    <'a> BorrowPairMut<Card, CardFlags> for ChoiceStateAndRules<'a>:
     s, (s.choice_state.scroll_card, s.choice_state.card_set)
 );

impl<'a> CardFlagsSubChoice for ChoiceStateAndRules<'a> {
    fn mark_done(&mut self) {
        self.choice_state.layer = Layer::Changes;
    }
    fn reset(&mut self) {
        self.choice_state.changes.clear();
        for c in self.choice_state.reset_changes.iter() {
            self.choice_state.changes.push(c.clone())
        }
    }
    fn get_status_lines(&self) -> StatusLines {
        let len = self
            .rules
            .when_played
            .get_card_flags_changes(self.choice_state.card_set)
            .count();
        [
            bytes_to_status_line(format!("{}", len).as_bytes()),
            bytes_to_status_line(if len == 1 { b"change. " } else { b"changes." }),
        ]
    }
}
