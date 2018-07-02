# Rule Space

A full implementation of all the possibilities of Bartog would, depending on how you look at it, require AI capable of perfectly understanding human language, simulating humans, including physical movement, speech and cognition, or simulation of the entire universe. None of those are (yet) on the table for this project. So we are forced to leave out some parts of the possibility space, due to the difficulty in implementing them in either an accurate or even an entertaining manner.

Since we can't implement it all we need to make decisions about what to implement and what not to. A primary drive in this project is making an interesting rule space with lots of variability and interactions.

For some kinds of rules, like physical races, etc. It's easy to decide that the effort to implement them outweigh the amount of interesting interactions they provide. Do X while also doing Y physical thing, (or in this case, playing this action minigame,) does not produce really interesting interactions with the rest of the rule space. Doing two mingames one after the other or simultaneously is somewhat interesting but if we wanted to do that well it would probably be better to drop the card choosing part entirely.

But, other kinds of rules require more examination to decide whether they are worth including. So it makes sense to organize the different suggested rules, and possibly some I add by extension of the listed ones, or make up from whole cloth, into kinds which we can then examine to determine how much time and effort they would take to implement and how many interesting interactions, (internally or with other kinds,) they create.

##### Aside about parameterization

We definitely want to parameterize rules because it allows many more possibilities for interesting interactions.

Of the Bartog rules documents I have collected so far, the one from [ftp.cse.unsw.edu.au](ftp://ftp.cse.unsw.edu.au/pub/users/malcolmr/nomic/other_games/bartog.txt), (which appears to be a dead link now, but the data is saved in this repo since another copy was found [here](https://www.pagat.com/docs/bartog.txt)) has the most parameterized set of rule suggestions. That document also introduces a notation for concisely talking about how many cards from the deck a rule applies to, which I will reproduce verbatim here:

>"card(n)", where n is a number from 2 to 13
>        - Any of a set of about n cards, by value, regardless of suit. 
>          Good choices of sets of cards are: 
>          primes (not including 1), squares, cubes, odds, evens, 
>          multiples of x, court cards, or particular suits or colors.
>          In all such cases Jack, Queen, King are usually considered
>          to have values 11, 12 and 13 respectively. Ace has value 1.
>"card(1)"
>        - Any single card, by value, regardless of suit. (Generally a card
>          with does not already have a rule attached, altho overlap is 
>          allowed, even encouraged, on occasion).
>"card(1c)" (or "card(1s)")
>        - Any single card by value and by colour (or by suit). This is
>          generally recommended for rules which are considered to have
.          "dangerous" effect, and for which it is desirable to have apply
>          only rarely.

I bring this up because it prompts some thought about how to have the computer players choose which set of cards to apply a rule to. The "easy" answer is to just uniformly randomly choose one of the possibilities. But this has a high likelihood of producing either rules that almost never come up, (since most of the possibilities are single cards.) Another possibility is to evaluate each rule myself and assign weightings to it manually, but that seems labour intensive. I want adding new kinds of rules to be at most an O(1) amount of work, preferably less, (one instance of work adding many more rules because of combinatoric explosion etc.,) and adding a constant factor is not desired there. Therefore having one generic weighting that is automatically applied to every rule with one or more card-application parameters, seems like the best option. Presumably one which increases the probability of small groups of cards (but larger than single cards,) being selected would work well in most cases. If it does become problematic for certain rules, and changing the default would have negative consequences, then we can certainly special case those rules, but it seems reasonable to expect that a good default can be found.

## Organizing into Kinds of Rules

There can be different levels of kinds of rules. That is, given a particular kind of rules it may be possible to break it down further into sub-kinds, each still part of the original super-kind but also distinct enough that one sub-kind may be deemed worthwhile to implement and the other may not. It's also possible that multiple different kinds can be combined into a super-kind and the descision about whether to implement them or not can be made at the super-kind level. For example, it is possible to split "physical minigames" into several sub-kinds, say races and strength tests among others, but as mentioned before we don't think it's worth it to put in the effort to simulate physical stuff so we don't need to further categorize things. Essentially, we might end up categorizing things at the wrong level for our current purpose of deciding what to implement. But we need to start somewhere, so sorting things into well-defined kinds seems worth doing.

Here are some kinds, which may or may not overlap, along with comments regarding whether it seems like it should be implemented. In order to make searching for rule kinds with a particular opinion noted about them easier we use the symbol "✔" to indicate approval, "✘" to indicate disapproval, and "❓" to indicate unsuredness or ways that an approach could be made to work if we want more rules at some point later.

* Affecting whose turn it is and/or will be in the future.
    * ✔ This directly affects who gets to play a card next and therefore who will win the game, as well as what order effects can come into play. 

* Directly moving cards, for example, to another player's hand, or to the discard pile.
    * ✔ This directly affects what cards get played and therefore who will win the game, as well as what order effects can come into play. 

* Affecting whether a card can be played, (on a standard turn.) This includes allowing extra cards to be played.
    * ✔ This directly affects who will win the game, as well as what order effects can come into play. 

* Real-time elements, for example: allowing a card to be played out of turn.
    * ✘ These would require more work to implement, and tuning of the amount of time to allow for human response, the probability to have computer players miss their chance to do so. And in return we don't seem to get a whole lot of interesting gameplay, just reflex tests. ❓ We could I suppose, maintain a turn-based nature by prompting for additional plays in-between turns and just allowing every player unlimited (or effectively-unlimited) time for that, since this is a single-player game so one person taking a long time to decide won't leave another person waiting.

* Restricting speech or other non-directly game-affecting actions.
    * ✘ While this would require more work to implement, in a single player game, there's no reason for the player to speak in the first place! ❓ An artificial meta reason to speak could be added I guess? Not sure how that would work though. You get thirsty so you need to ask for a drink sometimes?

* Requiring speech or other non-directly game-affecting actions.
    * ❓ This has the problem of deciding how often the computers forget to do these, and the interface and timing for players catching them at it. Also, it's hard, (although not impossible) to allow more than a small fixed set of actions. Would it be fun for the player to have to pick the required thing from a list of actions that they will perform each turn? What about making make computers say silly things, or even saying silly things even when you don't have to? Those kind of feel like they would get old fast. But they would take longer to get old if there are sound effects! Could we make the interface require "talking" to computer players somehow?
    * All that said, since saying the name of the game when you have one card left is part (nearly?) every version of the base rules, we are under pressure to include this one. ... ✔ 

* Triggered changes to other effects. For instance if a "Next player draws X cards" trigger happened, another one of those triggers can double that effect, redirect it, or zero out the previous one, (the player can discard those cards).
    * ✔ More of a good thing is probably a good thing! However this is kind of a "parasitic mechanic" since it requires other effects to be meaningful.

* Meta-game changes. That is changing how games affect future games with the ruleset, including preventing them or removing rules based on triggers.
    * ✔ Definitely some interesting possibilities here. We might want a mode where these are disabled though.

* Changing how winning is determined.
    * ❓ This seems hard to do other than just adding in some presets.

* Additional hands/games/players/discard piles/etc.
    * ❓ This seems somewhat hard to do in a way that allows rich interactions with other things, but not impossible.Interesting possibilities if we do pull if off though! What should adding a Two-hands rule do if there is already two hands?

* Assigning states to players which confer extra restrictions and/or abilities.
    * ✔ Having a fixed number total of states, (say 8, including the empty state,) seems reasonable enough and it should be easy to plug in the other rule generation into this.

* Assigning custom rules of which cards allowed to be played. Also includes changing those rules during the a single game.
    * ✔ Lots of interactions with other rules, and (hopefully) easy enough to implement with a bytecode approach.

* Changing what in-game information is publicly visible, for example making certain cards in someone's hand visible.
    * ✔ This may require extra UI, but essentially only that. Plus computer players taking it into account.
    
* Rule hiding.
    * ❓ Technically easy to do but is it interesting?
