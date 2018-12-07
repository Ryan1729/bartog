## TODO

Making additional menus is too annoying to hook things up for. Make some usage code that has the interface  I'd actually want, then make it work
    I want something like `choose!(CardFlags, CardFlags, Vec<in_game::change>)`
        We might require extra information like how the text should be generated etc.

Choose card set to affect rather than single card when choosing card play ability.
  remember to change how CPU generates these

Either allow multiple changes from every rules type or from none of them!

Add "when played on" rules in a similar manner to "when played"

Add card revealing
  First one is a simple "cards are revealed" checkbox in the rules menu.
  Then we'll want to have the cpu players take advantage of the knowledge.
  Then we'll need to have a way to represent the cards being revealed or not
    Would a set of card flags for each hand where each flag indicates whether
    the nth card in the hand is revealed work? we'd need to invalidate it fairly
    often.

Since usually they amount to "random card", consider replacing selections with "random card"?
  Maybe random for hands only?

allow making a set of cards act as a particular card.
    we'd like to be able to say things like 8s count as 4s and have the 8 of spades count as a 4 of spades and the 8 of hearts count as a 4 of hearts and so on, but I'm not sure of a good interface for that.

in in_game::Change selection screen if there are no changes made to the rules make the done button a cancel button.

When b button is pressed on the menus, jump to "cancel".
