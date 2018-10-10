/// A trait meant for creating an instance of a tyoe to be swapped with std::mem::replace.
/// This is sometimes necessary in order to take data from a mutable reference, even if
/// the reference is about to be overwritten. One place this comes up is switching the
/// varaint of an enum given a mutable reference to it. For example, given this type:

/// ```
/// enum Example {
///     A(u32)
///     B(u32)
/// }
///```

/// To go from a `&mut Example` currently in the `Example::A` variant to one with the same `u32`
/// value but in the `Example::B` variant, we need to use `std::mem::replace` and so we need a
/// dummy value to put into the `Example::A`. For a `u32` we can just use `0` But for complex
/// types it's not clear what the best value would be without exampling the type more closely,
/// and implementing it requires setting any private fields, so the empty value specificaton
/// should be close to the type. That is what this trait is for.

/// Implementations of this trait are allowed to assume that the instnace will never be read,
/// so it may break invarants of that type. Implementations should avoid needless allocation.

pub trait Empty {
    fn empty() -> Self;

    fn take(&mut self) -> Self
    where
        Self: Sized,
    {
        use std::mem::replace;

        replace(self, Empty::empty())
    }
}

/// A trait that is used to avoid having multiple copies of the card selection menu procedure.
use inner_common::Card;
use std::borrow::BorrowMut;
pub trait CardSubChoice: BorrowMut<Card> + Default {
    fn should_show_done_button(&self) -> bool;
    fn mark_done(&mut self);
    fn next_layer(&mut self);
    fn get_status_lines(&self) -> StatusLines;
}

pub type StatusLine = [u8; 8];
pub type StatusLines = [StatusLine; 2];

pub fn bytes_to_status_line(bytes: &[u8]) -> StatusLine {
    use std::cmp::min;
    let mut output = [0; 8];
    for i in 0..min(bytes.len(), 8) {
        output[i] = bytes[i];
    }
    output
}
