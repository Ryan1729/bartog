use game_state::in_game::{CardMovement, RelativePlayerSet, RelativePlayer, RelativeHand};
use common::{AllValues, CardSelection};

fn all_values_with_selection(selection: CardSelection) -> Vec<CardMovement> {
    let sets = RelativePlayerSet::all_non_empty_values();
    let hands = RelativeHand::all_values();

    let mut output =
        Vec::with_capacity(sets.len() * hands.len() * hands.len());

    for &affected in sets.iter() {
        for &source in hands.iter() {
            for &target in hands.iter() {
                if source == target {
                    continue;
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

    output
}

fn main() {
    let mut output = String::new();

    output.push_str(
        "0 same\n1 next\n2 across\n3 previous\n4 deck\n5 discard\n#\n"
    );

    let selection = CardSelection::all_values()[0];
    let cms = all_values_with_selection(selection);
    use std::fmt::Write;
    fn to_index(hand: RelativeHand) -> u8 {
        match hand {
            RelativeHand::Player(p) => match p{
                RelativePlayer::Same => 0,
                RelativePlayer::Next => 1,
                RelativePlayer::Across => 2,
                RelativePlayer::Previous => 3,
         },
           RelativeHand::Deck => 4,
           RelativeHand::Discard => 5,
        }
    }

    for cm in cms.iter() {
        write!(
            output,
            "{} {} {:04b}\n",
            to_index(cm.source),
            to_index(cm.target),
            unsafe{std::mem::transmute::<RelativePlayerSet, u8>(cm.affected)}
        );
    }

    println!("{}", output);
}
