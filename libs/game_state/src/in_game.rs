use common::{ByteStrRowDisplay, RowDisplay, *};
use game_state::Rules;

use rand::distributions::{Distribution, Standard};
use rand::Rng;

use std::fmt;

pub struct State {
    pub cpu_hands: [Hand; 3],
    pub hand: Hand,
    pub deck: Hand,
    pub discard: Hand,
    pub current_player: PlayerID,
    pub winners: Vec<PlayerID>,
    pub top_wild_declared_as: Option<Suit>,
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
    pub fn new<R: Rng>(rng: &mut R) -> Self {
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

        let current_player = cpu_hands.len() as u8; //rng.gen_range(0, cpu_hands.len() as u8 + 1);

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
}

pub const MAX_PLAYER_ID: PlayerID = 3;
pub const PLAYER_ID_COUNT: usize = (MAX_PLAYER_ID + 1) as _;
pub const PLAYER_ID: PlayerID = MAX_PLAYER_ID;

pub fn all_player_ids() -> [PlayerID; PLAYER_ID_COUNT] {
    let mut output = [0; PLAYER_ID_COUNT];
    for i in 0..=MAX_PLAYER_ID {
        output[i as usize] = i;
    }
    output
}

pub fn player_name(playerId: PlayerID) -> String {
    invariant_assert_eq!(MAX_PLAYER_ID, 3);
    if playerId < MAX_PLAYER_ID {
        format!("cpu {}", playerId)
    } else if playerId == MAX_PLAYER_ID {
        "you".to_owned()
    } else {
        "???".to_owned()
    }
}

pub fn player_1_char_name(playerId: PlayerID) -> String {
    invariant_assert_eq!(MAX_PLAYER_ID, 3);
    if playerId < MAX_PLAYER_ID {
        format!("{}", playerId)
    } else if playerId == MAX_PLAYER_ID {
        "u".to_owned()
    } else {
        "?".to_owned()
    }
}

pub fn get_pronoun(playerId: PlayerID) -> String {
    if playerId == MAX_PLAYER_ID {
        "you".to_string()
    } else {
        "they".to_string()
    }
}

pub trait ApplyToState {
    fn apply_to_state(&self, &mut State);
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Change {
    CurrentPlayer(RelativePlayer),
    CardLocation(CardMovement),
    //TopWild(TopWild),
}

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
        RelativePlayer::all_values()
            .into_iter()
            .map(Change::CurrentPlayer)
            .chain(
                CardMovement::all_values()
                    .into_iter()
                    .map(Change::CardLocation),
            ).collect()
    }
}

impl fmt::Debug for Change {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl fmt::Display for Change {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        change_match!{*self, {
            v => write!(f, "{}", v.to_string())
        }}
    }
}

impl RowDisplay for Change {
    fn row_label(&self) -> RowLabel {
        change_match!{*self, {
            v => v.row_label()
        }}
    }
}

impl Distribution<Change> for Standard {
    #[inline]
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Change {
        match rng.gen_range(0, 1) {
            _ => Change::CurrentPlayer(rng.gen()),
        }
    }
}

impl ApplyToState for Change {
    fn apply_to_state(&self, state: &mut State) {
        change_match!{*self, {
            v => v.apply_to_state(state)
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

impl ApplyToState for RelativePlayer {
    fn apply_to_state(&self, state: &mut State) {
        state.current_player =
                    //apply Previous to undo the autonatic incrementation that will happen later
                        RelativePlayer::Previous.apply(self.apply(state.current_player));
    }
}

impl fmt::Debug for RelativePlayer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            RelativePlayer::Same => write!(f, "my turn again"),
            RelativePlayer::Next => write!(f, "next after me"),
            RelativePlayer::Across => write!(f, "across from me"),
            RelativePlayer::Previous => write!(f, "previous to me"),
        }
    }
}

impl fmt::Display for RelativePlayer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
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
        Distribution<RelativePlayer> for Standard
        by picking from RelativePlayer::all_values()
    );

//This relies on MAX_PLAYER_ID being 3, and will require structural changes if it changes!
#[derive(Copy, Clone, PartialEq, Eq)]
pub struct RelativePlayerSet(u8);

impl AllValues for RelativePlayerSet {
    fn all_values() -> Vec<Self> {
        u8::all_values()
            .into_iter()
            .map(RelativePlayerSet)
            .collect()
    }
}

const SAME_FLAG: u8 = 1;
const NEXT_FLAG: u8 = 2;
const ACROSS_FLAG: u8 = 4;
const PREVIOUS_FLAG: u8 = 8;

impl RelativePlayerSet {
    #[inline]
    pub fn contains(&self, player: RelativePlayer) -> bool {
        match player {
            RelativePlayer::Same => (*self).0 & SAME_FLAG != 0,
            RelativePlayer::Next => (*self).0 & NEXT_FLAG != 0,
            RelativePlayer::Across => (*self).0 & ACROSS_FLAG != 0,
            RelativePlayer::Previous => (*self).0 & PREVIOUS_FLAG != 0,
        }
    }
}

impl fmt::Debug for RelativePlayerSet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
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
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
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

fn set_display(player: RelativePlayer, f: &mut fmt::Formatter) -> fmt::Result {
    match player {
        RelativePlayer::Previous => write!(f, "the previous player"),
        RelativePlayer::Across => write!(f, "the across player"),
        RelativePlayer::Next => write!(f, "the next player"),
        RelativePlayer::Same => write!(f, "the current player"),
    }
}

implement!(
        Distribution<RelativePlayerSet> for Standard
        by picking from RelativePlayerSet::all_values()
    );

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct CardMovement {
    affected: RelativePlayerSet,
    source: RelativeHand,
    target: RelativeHand,
    selection: CardSelection,
}

impl AllValues for CardMovement {
    fn all_values() -> Vec<CardMovement> {
        //TODO We will probably want to cache this. Possibly by putting it into a static constant.
        let sets = RelativePlayerSet::all_values();
        let hands = RelativeHand::all_values();
        let selections = CardSelection::all_values();

        let mut output =
            Vec::with_capacity(sets.len() * hands.len() * hands.len() * selections.len());

        for affected in sets {
            for &source in hands.iter() {
                for &target in hands.iter() {
                    for &selection in selections.iter() {
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
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{} move {} from {} to {}",
            self.affected, self.selection, self.source, self.target
        )
    }
}

impl Distribution<CardMovement> for Standard {
    #[inline]
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> CardMovement {
        CardMovement {
            affected: rng.gen(),
            source: rng.gen(),
            target: rng.gen(),
            selection: rng.gen(),
        }
    }
}

impl ApplyToState for CardMovement {
    fn apply_to_state(&self, state: &mut State) {
        unimplemented!()
    }
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum RelativeHand {
    Player(RelativePlayer),
    Deck,
    Discard,
}

impl fmt::Display for RelativeHand {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            RelativeHand::Player(p) => write!(f, "{}", p),
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
        Distribution<RelativeHand> for Standard
        by picking from RelativeHand::all_values()
    );

use std::num::NonZeroU8;
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum CardSelection {
    NthModuloCount(NonZeroU8),
    // NthIfPresent(u8),
    // ChosenBy(PlayerID),
}

impl fmt::Display for CardSelection {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CardSelection::NthModuloCount(n) => {
                ordinal_display(n.get(), f)?;
                write!(f, " card, looping if needed")
            }
        }
    }
}

impl AllValues for CardSelection {
    fn all_values() -> Vec<CardSelection> {
        NonZeroU8::all_values()
            .into_iter()
            .map(CardSelection::NthModuloCount)
            .collect()
    }
}

implement!(
        Distribution<CardSelection> for Standard
        by picking from CardSelection::all_values()
    );

fn ordinal_display(n: u8, f: &mut fmt::Formatter) -> fmt::Result {
    let s = n.to_string();

    let suffix = if s.ends_with("1") && !s.ends_with("11") {
        "st"
    } else if s.ends_with("2") && !s.ends_with("12") {
        "nd"
    } else if s.ends_with("3") && !s.ends_with("13") {
        "rd"
    } else {
        "th"
    };

    write!(f, "{}{}", s, suffix)
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
    pub changes: Vec<Change>,
    pub left_scroll: usize,
    pub right_scroll: usize,
    pub marker_y: u8,
    pub card: Card,
    pub layer: Layer,
    pub description: Vec<u8>,
}

pub struct ChoiceStateAndRules<'a> {
    pub choice_state: &'a mut ChoiceState,
    pub rules: &'a Rules,
}

implement!(<'a> BorrowMut<Card> for ChoiceStateAndRules<'a>: s, s.choice_state.card);

impl<'a> Reset for ChoiceStateAndRules<'a> {
    fn reset(&mut self) {
        self.choice_state.reset();
    }
}

impl<'a> CardSubChoice for ChoiceStateAndRules<'a> {
    fn should_show_done_button(&self) -> bool {
        true //TODO check if there has been any change to the changes
    }
    fn mark_done(&mut self) {
        self.choice_state.layer = Layer::Done;
    }
    fn next_layer(&mut self) {
        self.choice_state.layer = Layer::Changes;
    }
    fn get_status_lines(&self, card: Card) -> StatusLines {
        let len = self.rules.when_played.0[card as usize].len();
        [
            bytes_to_status_line(format!("{}", len).as_bytes()),
            bytes_to_status_line(if len == 1 { b"change. " } else { b"changes." }),
        ]
    }
}
