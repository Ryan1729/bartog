use game_state::in_game::{CardMovement, RelativePlayerSet, RelativeHand};
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
    let selection = CardSelection::all_values()[0];
    let cms = all_values_with_selection(selection);
    for cm in cms.iter() {
        println!("{}", cm);
    }
    println!("{}", cms.len());
}
