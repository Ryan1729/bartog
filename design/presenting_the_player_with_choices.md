# Presenting the player with choices

During the course of the game we will be presenting the player with choices. Eventually the choices we present will be determined by runtime only data, so we need a general purpose way to do so. When resenting choices we need to wait an unknown number of frames for the player to make their choice. The player may of course, decide to shut off the game instead of choosing any choice, but I don't think that will need any special handling beyond waiting an unknown number of frames. 

When waiting an unknown number of frames, we will need to figure out that a choice needs to be made, then check to see if the player has made it this frame and wait otherwise, like in this example pseudo-code:

```rust
if do_yes_button() {
    //they said yes
    process_yes()
} else if do_no_button() {
    //they said no
    process_no()
} else {
    //they haven't said yet.
    //wait until next frame and check again
}
```

Code like the above is very special purpose, with the particular possible choices all hardcoded.

The kind of UI shown to the player is driven mainly by the type of the result of their choice.
We would like to be able to ask a choice to be made simply by providing its type. 
Say something like the following:

```rust
if let Chose(number) = get_u8_choice(/*...*/) {
    //...
} else {
    //wait until next frame and check again
}
```

I suppose it would be nice to be able to have the type be inferred like so:

```rust
if let Chose(number) = get_choice(/*...*/) {
    //use the number in a way that makes it clear it's a number
} else {
    //wait until next frame and check again
}

//elsewhere

if let Chose(some_enum) = get_choice(/*...*/) {
    //use the some_enum in a way that makes it clear it's a some_enum
} else {
    //wait until next frame and check again
}

```

... but I still have to write the implementation for each type somewhere so simply not having to specify which I'm using at a given point is not a big deal.

What I find more pressing is how combining multiple choices works. That is, how ergonomic is it to define a tree of choices, where the path taken is based on what choice was made previously.

Because we've got a requirements that suggest a type parameter, and we want to do one thing `and_then` another, it seems reasonable to consider a monadic interface. That is, something like this:

```rust
let result = get_bool_choice(/*...*/).and_then(|b| {
    if b {
        get_u8_choice(/*...*/)
    } else {
        get_bool_choice(/*...*/)
    }
});
```

This looks reasonably pleasant to use, but upon looking at this the question becomes, how do I use `result` and what is it exactly? If I assume the implied monad is called `Choice<T>` then `result` should be `Choice<u8>` some of the time and `Choice<bool>` other times. This implies that the return type should be a discriminated union of u8 and bool, and that the code should look more like this:

```rust
enum ByteOrBool {
    Byte(u8),
    Bool(bool)
}
use ByteOrBool::*;

let result = get_bool_choice(/*...*/).and_then(|b| {
    if b {
        get_u8_choice(/*...*/).map(Byte)
    } else {
        get_bool_choice(/*...*/).map(Bool)
    }
});
```

This would work, but it implies that I have to define enums for each type *combination* and I cannot have trees where the leaves are the same type without adding newtypes. 

While I have not used it before this, I think this might be a job for [std::Any](https://doc.rust-lang.org/std/any/index.html).

____

Another issue to do with presenting choices is drawing order, and getting a reference to the framebuffer. While it would be most convenient in some ways to simply make the choice at the point we want to, that requires us to drag the framebuffer into the update call, and we would need to ensure that the updates happen after the draw or something. Instead, if we store the currently wanted choice and then draw it after the rest of the graphics, that solves both problems.

With the current choice stored on the state, for multi state choices we could, instead of switching to `None`, we could store what was chosen and what needs to be chosen still. The question is, can we also use a version of the monadic chaining above to create the choice? We want an API like the following, if we can get it:

```rust
let result = /*...*/;

if let Chose(choice) = result {
    //Do the thing
} else {
    //Come around next frame and do this again
}
```

This seems straight forward enough for a single choice. Simply make the result expression a function of the state, (Or a method, whatever.):

```
let result = choose(state, /*...*/);
```

Then since we store the same thing to the state every frame, we'll keep drawing the same image/presenting the same choice.

But if we have a multi-part choice, how does that work?

My first guess would be that we need some way to detect that a previous choice was made which matches the current one. If we do, then we assume the current data is correct and present it. But ... would it ever break if we just assumed that when we had a previous choice that it was the one we should use? Assuming that we clear out the choice whenever we return the choice, or cancel a choice then I think so! I haven't thought deeply about how canceling a choice would work, but I think just clearing the current choice, if any, might work.

Assuming that all works out as I suspect, there's still the issue of storing what will be a functionally infinite amount of possible choices in a finite space. I think we would need to use std::any, unless we figure that there's actually only a small number of types of choices, in which case we can use an enum.

____

One issue with this API as proposed so far is that code like the following will not work:

```rust
if let Chose(choiceA) = /*Choice A ...*/ {
    //Do the thing
    if let Chose(choiceB) = /*Choice B ...*/ {
        //Do the thing with both choices
        //except...
    } else {
        //Come around next frame and do this again
        //except...
    }
} else {
    //Come around next frame and do this again
}
```

If the code is written this way, given the assumption that there is only one choice, then after the player makes the first choice, they *will* be prompted with the second one, but then the first `if let Chose(/**/)` will have to handle it!
One way around this is to require that is to disallow nested choice like this, and instead making an expressive enough choice construction API that anything you could do with an API like the above that actually worked, you could do "in one go" by making a single choice expression. So the nested choice would need to be written something like this:

```rust
let choice = make_A_choice().map(|choiceA| {
    //could choose the choice type as a result of previous choice
    make_B_choice()
});

if let Chose(chose) = present_choice(state, choice) {
    if let Some((choiceA, choiceB)) = /* chose ...*/ {
        //Do the thing
        
        //Do the thing with both choices
    } else {
        //panic?
    }
} else {
    //Come around next frame and do this again
}
```
Note: `/* chose ...*/` would need to involve std::any (or "just" unsafely accessing unions,) since we want to be able to store any possible choice in the same place in the state.

However, I'm not certain this is sufficiently expressive. What would be more expressive is if the user could return arbitrary things from the callback to examine later and if we could examine and even mutate the state as a result of those changes. That is, something like this:
```rust
let choice = make_A_choice().and_then(|chose, state| {
    //Do the thing
    //could choose the choice type as a result of previous choice
    make_B_choice()
});

if let Chose((choiceA, choiceB)) = present_choice(state, choice) {
    //Do the thing with both choices
} else {
    //Come around next frame and do this again
}
```
I hope I won't need to use std::any in the `and_then` callback, but I'm not sure. I should at least be able to pass in a typeId as a second parameter to `and_then` or something.
This code example makes me wonder whether the final `if let Chose((choiceA, choiceB))` part could be done inside a callback and to stop then we would just need to return a unit "choice", (since there's only one option it's not really a choice!) to finish it. Then the code could be something like:
```rust
let choice = /*...*/;

present_choice(state, choice);
```
...with the extra complications inside the `/*...*/` expression.

Another way would be to restructure the way the choices are stored such that the nested if version of the API actually works! I think that implies that the leaf nodes of the `if tree` would all need to reset the stored choice back to `None` which doesn't sound great.

Of all of these I think the deciding factor will be, "Which is easiest to generate at run-time?" 

___

While beginning to implement a fixed number of choice types version of choices, for the plain crazy eights version, I've noticed that there the issue of where to store the UI state of the choice. For example, in the case of a yes/no choice which is implemented by presenting two buttons to the user, where should which button is currently highlighted be stored? If we were using the mouse, we could simply "re-discover" which button is highlighted every frame. Also, if we were uniquely identifying each button then we could store the id. But, at least so far we have not been uniquely identifying each button, since we figure that we will only be presenting one window to the player at a time, and we have not implemented card selection as a special case of button pressing. However, we might want to break one or both of those two assumptions at some point. Of the two of them, it seems more likely, if we develop a good button system, that we might want to re-implement card selecting as buttons, (implying we'd want some way of uniquely identifying buttons,) instead of displaying multiple windows. 

So it seems that there are essentially two paths:
    * Store the state for a given choice only when there is a choice. So we might as well put it on the choice. Then each type of choice can have input update logic defined for it.
    * Uniquely, (at least for each frame,) identify each UI element. Have a global UI context which will keep track of which UI element is selected. Then each choice type will need to handle states that are outside their expected range. That's not too hard though, just map to some valid id. The input logic would need to know  where the UI elements are positioned relative to each other, (at least in some relative sense,) so when the player presses in a direction, the correct element is selected.

