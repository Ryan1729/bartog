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