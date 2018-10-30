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

* Assigning custom rules of which cards are allowed to be played. Also includes changing those rules during the a single game.
    * ✔ Lots of interactions with other rules, and (hopefully) easy enough to implement with a bytecode approach.

* Changing what in-game information is publicly visible, for example making certain cards in someone's hand visible.
    * ✔ This may require extra UI, but essentially only that. Plus computer players taking it into account.
    
* Rule hiding.
    * ❓ Technically easy to do but is it interesting?


## Development plan

* Write simple single player version of crazy eights. Might even skip making the 8 wild.
* Add support for the kind of rule that gives the most possibilities on it's own. That kind appears to be "Assigning custom rules of which cards are allowed to be played."
* Continue adding new kinds of rules until satisfied with the results.

## Designing a bytecode

We need to change which rules are active at run-time, so we need a representation of them as data. We would also like to be able to easily save that data to disk, in order to have long running games. This points to a bytecode as a good solution, assuming we can design one which can represent all the rules we want to represent, and is sufficiently easy to uniformly generate valid instances of it.

The simplest way to represent rules, from the perspective of writing them, would be to assign a consecutive number to every possible permutation of each rule. This makes generating a random rule trivial: just pick number between 0 and the maximum rule number, inclusive. This has a few disadvantages though. Decoding which rule a given number refers to would require a massive lookup table which as the number of possible rules balloons, will likely exceed memory constraints. There's also the question of what the lookup table contains exactly. We will not be able to have a separate implementation of each rule because even if we generated them, they would be unlikely to fit in memory if we add as many rules as we want to.

So we need something that we can examine and produce, at minimum a predicate which takes the card to be played and the top card of the discard pile. And it seems like we'd want it to be able to represent every pure function like that. Because there are 52 possibilities for each card, each function would need to respond to 52 ^ 2 = 2704 different possible inputs and if we're representing every possible predicate of that form that's equivalent to every 2704 bit number. That means there are 2^2704 which is approximately 9.66 * 10 ^ 813. Also it means that our instruction, for this severely reduced subset of the functionality we eventually would need to be at least 2704 bits long. 

While we could probably work with that if that was all we needed to use, if other kinds of rules require similar amounts of storage then we'd probably run into speed problems. Luckily however, that proposed data format would represent a whole bunch of functions that we don't really want to allow anyway! For instance, we don't want to be able to represent the predicate that doesn't allow us to play any of the cards whatsoever. While not explicit in the rules of Bartog, when playing with actual humans, generally no one will choose a rule which will cause the game to potentially never end, given everyone understands the rule to produce that result. While restricting ourselves to only those predicates that allow every card to be played on at least one *other* card does technically disallow a some rules that peopel might reasonably make, (making a single card unplayable in a hand/card swapping heavy environment, for example) as a starting point, it seems reasonable. We can always add those possibilities in later if we want.

So the question becomes how do we represent that subset of the possible functions? The functions we want are those that, (if we assume currying) for every first card parameter returns a function that returns at least one true for an input which is not the card itself. Assuming that we don't add a second deck, then that set of functions can be identified with the set of functions that return true in at least two cases. If we did decide to add more decks then <del>we can just restrict ourselves to the functions that return at least one true</del> ... it gets more complicated.

Hold on, it still might be possible to have all players have an unwinnable hand with each card only playable on two cards. If every card can only be played on itself and another card and that card con only be played on itself and the first card, then only a single card can be played at all! I think in tis project we are going to have to decide between allowing unfinishable games and disallowing finishable games that we can't, or can't easily prove that they are finishable with every deal. (I wonder how hard it would be to prove whether or not a given bytecode is incomplete, given it doesn't allow unrestricted arithmetic so AFAIK Gödel's incompleteness theorems wouldn't apply.) If I wanted to just allow anything I'd use a language that had an `eval` function. So since I'm planning to compile Rust into WASM, I guess we're going with restricting the space of valid games. Other option include declaring the game a draw whenever all players pass in a row, or allowing a javascript escape hatch given we are running in the browser, if you really want to allow unwinnable games. Unfortunately I suspect that unwinnable games will creep in in the interactions between rule kinds. However, at least some of the time having fewer possibilities means less work, so trying to limit them might be worthwhile.

Which predicates can we prove only result in finishable games? If the playable cards form two disconnected graphs, (where cards are nodes and the directed edges represent "can play on",) then if a player gets a single card of both graphs, assuming no card swapping rules, then they cannot win. (Maybe it would make sense to just add some card swapping rules?) So if there is a second graph which has a number of nodes greater than or equal to the number of players, then it's possible to have an unwinnable deal. (each player gets one of the smaller graph's cards.) It seems like it would be easier to just force there to be only one connected graph and only very few games would be disallowed that except for possibly some interesting hand/card swapping heavy things, wouldn't be that interesting anyway.

Is having a single "can play on" graph *sufficient* to prevent unwinnable games or merely necessary? For that matter, are there unwinnable crazy-eights-without-wild-8s games? Regarding whether a single connected graph is sufficient, there also needs to be a path of at least length two from each node to itself. I suspect that there also need to be a path from every node through every other card back to itself. The simplest graph, (as in least edges,) that satisfies this property is a ring of directed edges that passes through every card. 

Is an all-cards ring graph sufficient to prevent unwinnable games? Without loss of generality, let's assume that the ring goes Ace to king in each suit, and the suits are connected in alphabetical order: clubs, diamonds, hearts, spades, then the Ace of clubs can be played on the King of spades. We can also just think of this as cards numbered from 1 to 52 where 1 is playable on 52. We can assume, in absence of rules to the contrary, that every card but the initial card will make it into some player's hand. We will also explicitly state that the discard pile (except the top card,) should be shuffled into the draw pile when the draw pile runs out, and that the game continues when no one can draw, (it might make sense to just make the game stop without a winner, or with n winners at this point. Or maybe allow every player to choose a rule to remove/change back to the base ruleset). So at least one card can be played, and since the next card can be played eventually too, whether the previous card in in the discard pile or not. So we can inductively demonstrate that the game is winnable.

How many directed graphs with 52 nodes with at least one path that passes through each node, and returns to the start, are there? Some quick searching reminds me that the graph-theory phrase describing the property I want the graphs to have is being "strongly connected". So the quest then is, how many strongly connected graphs with 52 nodes are there? After asking [this question](https://cs.stackexchange.com/a/93779/75201) I learned this sequence is [the OEIS's A003030](https://oeis.org/A003030). From there I got the answer for 52, which is 

> 2145598717320326468976833182567815277735689660511863845922886136710706562075159194822418709013272507121408901364191158917767874587468475627798962025962697880518917928020117712145262983496291035985909694872808617886657781476453930755882666280090019686504861538429118343458645533778393119461126319797728207959939031964742570821145151927863948812260395221921316460641179316601097003271280243363077531777715456983542632613369589602678234787253646820450918702666995854410413397300624031275363765919200142772811525624652366125299242303990521998487049607679125768106495516359235929259766966065002034504696498233647671660883839731523566982966657250456794090644290683973625383987092987369510576411140624292795269749585677019693052531520525065168782013772660631386976515105614386260652019895428364129938702336

which is approximately 2.1 * 10 ^ 798. Recall that the total including unwinnable games was around 9.66 * 10 ^ 813. So if we had represented all possible arrangements, the vast majority of them would be unwinnable! Storing an arbitrary one of the winnable possibilities requires 331 and a half bytes or 2652 bits, (rounding up to the nearest power of two). 

The question now is, how to represent it in something approaching that level of compactness!

The easiest thing to do seems to be allocating 52 bits for each card that indicate whether that card can be played on each of the cards, (or vice versa I suppose.) That would require the full 52 * 52 = 2704 bits, (which would probably be faster to access if we gave each card its own 64 bits, requiring 64 * 52 = 3328 bits). It seems most practical to just check whether a given graph has one strongly connected component, since apparently there's [at least](https://en.wikipedia.org/wiki/Tarjan's_strongly_connected_components_algorithm) [two distinct](https://en.wikipedia.org/wiki/Path-based_strong_component_algorithm) algorithms to find all strongly connected components, which easily allows to check how many there are, in linear time, in the number of edges and nodes. I think we might be able to optimize in our particular case by checking if there are above a minimum number of bits, if the edges factor in the running happens to be large. This complicates generating random connected graph instances only slightly. We can just generate the required number of bits, and then check whether there is a single strongly connected component, and if not, (which our previous math indicates will happen almost all the time,) then set some zero bits. This breaks uniform sampling, but a bias towards permissive graphs sounds like a feature not a bug.

Another important aspect of a representation is how easy it is to create a UI for it. creating a UI that did not structurally allow the specification of non-strongly-connected sounds difficult and unlikely to yield usable results, (52 layers of menus is far too many and yet not enough!) But instead, prompting for which card or common group of cards the player would like to alter the connectivity for, then presenting them with 52 toggles, then alerting them if they try to submit a non-strongly connected graph, seems like a livable UI.

___

The time has come to actually implement the  UI for this.

I'm imagining something like this:

```
+---------------+
|               |
| Description   |
|               |
+---------+     |
|   Q♣    +-----+
+---------+Reset|
|   K♣    +-----+
+---------+     |
|   A♦    +-----+
+---------+Cance|
|   2♦    +-----+
+---------+     |
|   3♦    +-----+
+---------+ Done|
+---------------+
```

The card button column would be scrollable. When the bottom button is selected and the player presses down then the buttons move up revealing more if there are any, and vice versa for pressing up. (Should the scrolling wrap?)

"Reset" would change the connectivity to the default one, "Cancel" would take you back to the, (as yet unimplemented,) rule select screen, and leave the connectivity graph the way it was before the player changed it, and "Done" would confirm the changes and start the next game. All three of these should probably have confirmation dialogs.

After the player selects a card button they are presented with 52 checkboxes, presumably also scrollable, which allow selecting all the possible outgoing connections. (one might think that we should leave the card's own checkbox out, but eventually we are likely to add the possibility of multiple copies of a card in play at once)

We want to have good logs that describe the changes to the previous state, since logging the entire graph is impractical, (you'd have to scroll past hundreds of screens and/or decipher compact symbols.) Another argument for logging the changes is that, in the common case, only a few connections will be changed, since the player's UI makes it hard to make sweeping changes, and to be fair we shouldn't have the computer players do that either. 

So therefore we need to store the changes to the state, even though it would be simpler to store a copy of the entire state. A basic change consists of a card that is being changed, (52 possibilities,) and the outgoing connections for it (2^52 possibilities.) Since 52 possibilities can be expressed in 6 bits (2^6 = 64 > 52 > 32 = 2^5), we can easily fit a change into 64 bits.

A notable exception to the small changes rule is resetting to the default. We should log a reset to default as something like "PLAYER_NAME has reset the can play on rules back to the default". To store this we can make the all ones bit pattern mean resetting, and check for that one in particular when logging.

That leaves us with some possible bit patterns left over. If we want to later, we could add patterns that perform more complicated changes and buttons that perform them. For instance, allowing all cards to be playable on each other, or allowing all cards of a particular rank to be played on all cards, or disallowing all cards of a particular suit to be played on cards of a particular rank, or similar changes with suit and rank switched in and out. We may want to wait and see what "shortcuts" are desired by actual players in practice.

It may or may not be desirable to combine together changes before putting them in the log. For example, if all the cards of a particular rank, (say twos) can now be played on a suit (say spades), then should we figure this out and log "PLAYER_NAME has changed the rules to allow playing all 2s on spades"? This seems rather difficult to cover all the cases for. It seems like there would be ambiguous cases where we could end up saying something true that obscures the more meaningful effects of the actual change. If we did add buttons for more complicated changes, then recognizing exactly those kinds of changes might make sense. Waiting and seeing how readable the logs are in practice seems wise.

We will also need to allow the player to check what the current playability graph is. It seems impossible to present the whole graph "at a glance" so instead, they will have to indicate the card they are interested in looking at, then be shown a list of cards that card is playable on. I'm not currently sure of the best way to present that in our limited screen space. Would it be possible to create a readable string for each of the 2^52 possible outgoing connections? Otherwise, we can display the checkboxes from the outgoing connection selection screen.

____

We have now implemented the changing the can-play graph and a set of flags which determines which cards are wild.

One thing we could do now is allow toggling what wild means, including fiddly details that don't come up under the usual rules. For example, whether a non-wild card of a given rank can be played on a wild card of the same rank.

The thing is it would be nice to use a representation of the rules that didn't require observing these edge/corner cases directly. Bytecode that is run when deciding whether a card is wild, and what that means would fit that description. More generally, so would byte code that is run whenever a card is played at all.

However, the average bytecode that can represent all the possibilities we might want, can also represent would also be able to represent things we do not want. We need someway of restricting, (or restricting our interpretation of) our bytecode so things like infinite loops are not representable.

One simple way to avoid infinite loops is to require a loop count for each loop. A time when we would need to loop for more than `DECK_SIZE + 2` times does not immediately jump to mind, (and if we did, technically we should be able to repeat the loop body and do two loops one after the other. So since currently ` DECK_SIZE == 52` that leaves us 8 - 6 = 2 bit we could use for other purposes in any loop variables/byte codes. For example, assuming 8 bit instructions, we could define any instruction that matches with 01xx_xxxx to mean loop for xx_xxxx times once we see a special `END_LOOP` instruction. 

It appears that language wide issues like loops can be dealt with fairly easily. They are also fairly easy to test for, given enough time: randomly generate bytecode of a given length and make sure it terminates within some number of cycles. What is harder to deal with is ensuring game play specific invariants are maintained. For example, the can play graph currently allows the game to be unwinnable, though we have a plan to count the number of strongly connected components, and ensure that there is at most one. There are known algorithms to check this in interactive time-frames, for the fixed size graph we are looking at. This may not be true of a given bytecode. In particular when verifying card-triggered bytecode, in some sense we need to consider a given card-trigger against every other possible card-trigger, since they could interact in such a way that causes un-ending games.

In previous experiments involving program generation, early termination has been an issue. That is, the generated programs tend not to run/do as much as I wanted. Most possible programs don’t really do much. Assuming we go with a special chunk of bytecode for each card, for each game action, (eg. when the card is played, when it is played on, etc.) then technically not doing much is not a game breaker. But, it would still be unsatisfying if the CPU players chose almost only rules that didn’t really do anything. Things that combine with other rules to create unexpected consequences are cool, but rules that affect almost nothing generally don’t do that. We want a bytecode that is dense with meaning. What is the best way to do that?

One approach to making a dense bytecode, with some issues, is making each instruction do a lot. The main issue with this is that then the instructions likely don’t compose as well. We may be able to avoid this problem with care though. 

What makes bytecode composable? If any instruction can be ran after any other instruction then that would be maximally composable. In other experiments involving program generation, a difficult part that needed to be repeatedly fixed up was generating instructions that relied on data being on the stack. So it would appear that the best way forward would be to make every, or most every, bytecode not read the stack. Given that constraint it doesn’t really seem useful to write to the stack, and therefore a stack seems unnecessary. At that point all that’s left of the bytecode is the representation of modifications to the state, which might be enough. 

Even this restricted form of “bytecode” might allow too many ways to not do anything. For every state change the inverse change exists. This means for a series of instructions of length L there are n*L/2 ways to represent “do nothing” where n is the number of possible state changes. 

Say L = 2. Then out of the possible values for the instruction pair, there are n ways of representing “do nothing”, where the total possibilities for the pair is n^2. The fraction of “do nothing” pairs = n/(n^2). The limit of that function approaches 0, so maybe this is fine?

Assuming that the amount of “do nothing” actions is acceptable, the question becomes how to represent them. One simple way to represent a change to a state is to store the previous state and the new state, but we don’t want our action representation to depend on the current value of the state. However we also want to represent every possible kind of change, that is every possible function from state to state. If the state was a single byte we would need to represent 256^256 functions. While that is a massive number, we can represent that with log2(256^256) = 2048 bits. or 256 bytes. We can use that amount of space by simply listing the byte each possible input byte corresponds to. We can use this same scheme to represent every function for a state of any size S, using log2(S^S) bits.

Given we want to be able to represent every possible transformation to the relevant game state, the method described in the previous paragraph is the most compact way to do that. It might worth examining whether we actually want to represent every possible transformation to the state. The vast majority of all possible transformations to the state will almost certainly never be used, and if they were used they would be incomprehensible to human players. The best examples of this are functions which change nearly every, but not every part of the state to a different value. Even though the function always acts the same, if there is more than one card that triggers a similar change, human players will be unable to meaningfully decide between playing one or the other without an excessive amount of effort, which must be repeated for each game state the player has the opportunity to play the cards in. I predict that most players would play the cads without doing the work to know what they do in their entirety. They may quickly check for any obviously bad effects, which has a good likely-hood of backfiring but the more likely scenario is players either playing them immediately to get rid of them, or avoiding playing them until something random/unknown seems better than the alternative. Is this player experience something we want to work to achieve? While in some ways it is simpler to just represent everything, we could make certain things arguably simpler by either disallowing more than a certain amount of state changes or by only allowing state changes to one section of the state at a time. These restrictions could be wisely chosen such that humans would not run into them without specifically trying to test the limits of the system and/or in such a way that the restriction seems reasonable. The original rules include provisions against single rules trying to do too much through how much the players consider to be "one rule". a similar matter of taste may exist around how much the game state can change due to a single card being played. The thing is, because this is a matter of taste/opinon it' hard to define where the line should be drawn ahead of time. The major thing we would save by restricting the representation, assuming we put enough work into restricting the cpu rule generation, is some memory. Since changing the representation to make it wider would be annoying, it may not be worth risking that by restricting the changes without knowing either the perf impact or the edges of the change-space that humans will not want to use.

Something that even representing every possible function from state to state doesn't capture is allowing players to choose things. These are essentially either a function with the player as an extra parameter implemented by asking the player, or a function of the player id, and the game state, implemented by the AI code. The most straightforward way to represent this seems to be to use the state-state function representation mentioned above for those kinds of functions and having a separate representation for the choices part. This way we can add the choice part after the non-choice part is implemented.

____

We're now faced with the question of which current player's turn to next player's turn functions we want to keep in the game. The reason we don't want to keep them all is that they can be unfair, making them the obvious optimum rule to add. This makes the decision of which rule to choose much less interesting. So let's remove some of them. 

The question is, which functions are fair? 

Let `u` be the player setting the rule, and `0`, `1`, and `2` be the other players.

```
u -> 0
0 -> 1
1 -> 2
2 -> u
```
This represents the regular turn order, that is it sets the next player's turn to the same it would have been otherwise.
It is more or less the identity function. It is fair.

```
u -> u
0 -> 0
1 -> 1
2 -> 2
```
This sets the next turn to the player who played the card. This may not look fair at first, but there is no particular reason to think that the player making the rule will draw the card. So it is in fact fair.

```
u -> u
0 -> u
1 -> u
2 -> u
```
This is a classic example of an unfair function. This is in fact the most unfair function of this type.

```
u -> 0
0 -> 1
1 -> u
2 -> u
```
This is also unfair but to a smaller degree. It would still warp the game if allowed.

```
u -> 2
0 -> 2
1 -> 2
2 -> 2
```
This is also unfair, since while it also gives `2` more turns, it essentially makes the game be between `u` and `2`, if the function happens often enough.

```
u -> 0
0 -> 0
1 -> 0
2 -> 0
```
Is this unfair? It only benefits players other than the player who is making the rule so it would not warp the game if it were allowed. But if we don't consider this not fair then it implies that fairness depends on which player you are. If we accept this definition of fair then we would need to allow players to make certain rules that other players cannot make. That seems unfair in a different way. It also implies a lot of additional complexity.

One reasonable definition of fairness is that there is exactly one input player that results in each possible output player. This implies that a function is fair iff it a permutation of the identity function. This implies that there are exactly 4! = 24 fair functions of this type. That is less than 10% of all possible functions, but if we want to we can add more later. And previous entries in this file suggest that we will not be starved for possible rules.

Another question arises: Are all compositions of fair functions fair? Put another way, are the fair functions closed under composition?

Here's what we called the identity function again.
```
u -> 0
0 -> 1
1 -> 2
2 -> u
```
Oddly for something that is called the identity function, it is not the result of composing it with itself.

```
u -> u
0 -> 0
1 -> 1
2 -> 2
```
This function *is* its own self-composition. We can call this the compositional identity, and the other one the turn-order identity.

If we successively compose the turn-order identity with itself we do get back to it eventually, and without resulting in any unfair functions.

```
u -> 0  0 -> 1  1 -> 2  2 -> u  u -> 0
0 -> 1  1 -> 2  2 -> u  u -> 0  0 -> 1
1 -> 2  2 -> u  u -> 0  0 -> 1  1 -> 2
2 -> u  u -> 0  0 -> 1  1 -> 2  2 -> u
```
In the above diagram we can look at each column immediately after a `->` to see the effect of the composed function up to that point.

```
u -> 1
0 -> 2
1 -> u
2 -> 0
```
This function sets the turn to the player across the table from the player who played it.

```
u -> 1  1 -> u
0 -> 2  2 -> 0
1 -> u  u -> 1
2 -> 0  0 -> 2
```
Noticing that we get the final column of `u 0 1 2` we can stop the diagram since we know that the next composition would result in the original function.

Let's consider another fair function that is not a vertical rotation of the compositional identity. (Note that the turn-order identity is a rotation of it.) Lets also pick one that is not a vertical rotation of the last function we looked at.
```
u -> 2  2 -> u
0 -> 1  1 -> 0
1 -> 0  0 -> 1
2 -> u  u -> 2
```

Since we have not found any counter examples yet, let's try to prove that they are in fact closed under composition, by contradiction. In order for a composition of a function to not result in a fair function it must change the total number of at least one of the players in the output. But the function being composed, (the one called last) if called with every possible argument, will be passed one of each player, and since it is fair it will return exactly one back. So they are closed under composition!

____

For expressing rules we will need the concept of a "relative subset of players". That is a specification of a subset of the players which changes relative to a particular player. This allows us to express that all players other than the player that played a card should draw a card, for example. We would also be able to talk about the player across from the player that played it, the next player in the regular turn order and so forth.

We just finished making turn changing functions that worked as described above. But those are referring to specific players, and that meant we had to filter out most of the possibilities for being unfair. Can we use a representation of turn to turn functions that uses relative player subsets, which wouldn't have that problem? Would it capture all the functions that we have allowed in the previous scheme? Would it allow fair ones we have not been able to represent otherwise? Since we need relative player subsets anyway, it seems worth investigating.

We need some notation to talk about relative player subsets. For these turn functions we will only need 1 element subsets, which we can identify with single elements without loss of information. If we have a notation for single elements we can also use standard set notation for sets.

Given exactly four players arranged in a circle there are exactly four relative players we can talk about: "same" that player themselves, "next" the player who comes next in the turn order, "previous" the player who comes prior to this player in the term order, and "across" the player that is across the table or alternately two players ahead or two players back. We can abbreviate these to "s", "n", "p", and "a" respectively. And we can spell "snap" by placing the letters in order, "pans" by placing them in reverse order, and even if we start such that we end with "same" then we still spell "naps" and "span". on a more prectical note, we can use this abbreviated notation with u, 0, 1, and 2 without notational overlap.

There are at least two different ways we can talk about functions from specific players to relative players. We can always talk about players relative to "u" or we can talk about players relative to the fixed player the function is being applied to.

```
u -> n
0 -> a
1 -> p
2 -> s
```
This is the turn-order identity in always relative to "u" form.

```
u -> s
0 -> n
1 -> a
2 -> p
```
This is the compositional identity in the same form.



```
u -> n
0 -> n
1 -> n
2 -> n
```
This is the turn-order identity in the argument-relative form.

```
u -> s
0 -> s
1 -> s
2 -> s
```
This is the compositional identity in the same form.


The always relative to "u" form is easily translatable to and from the fixed player form, if desired, by simply replacing characters as follows: `u -> s`, `0 -> n`, `1 -> a`, and `2 -> p`. Note that this is essentially the same as applying the relative to "u" compositional identity. The argument relative form takes a bit more effort. However, at least for the simple examples examined so far, argument-relative form seems to be easier to read, and to more closely resemble how such rules are usually specified in English.

As an exercise, lets convert a randomly chosen fair fixed player function to argument relative form.

```
u -> 2
0 -> 1
1 -> u
2 -> 0
```
Here's such a function.

Looking at the rows one by one we can note the result and the argument and fill in the relative player that would produce the desired result. So if `u` should map to `2` then we should fill in `p`. Similarly if `0` should map to `1` we should fill in `n`.

```
u -> p
0 -> n
1 -> a
2 -> a
```
Here's the completed conversion.
In this form, the function no longer seems as fair. `u` gets to go two turns after they play the card, `0` gets no such advantage, and while when `2` plays it `u` gets skipped, when `1` plays it `u` gets to go immediately.

If fairness is defined as not caring which player in particular played the card, then it appears that really the only fair functions are those that are all the same relative player for every fixed player, which we can, again, identify with the relative players themselves with no loss of information.

Oh...

```
u -> s
0 -> s
1 -> s
2 -> s  ->  s
```

```
u -> n
0 -> n
1 -> n
2 -> n  ->  n
```

```
u -> a
0 -> a
1 -> a
2 -> a  ->  a
```

```
u -> p
0 -> p
1 -> p
2 -> p  ->  p
```
