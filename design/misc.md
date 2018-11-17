Before I had the following comment in the code:
```
/// There should be no operations that can cause cards not to be conserved between all Hand
/// instances.
```

But then I decided that maintaining that makes things much more complicated. Animations mean I'd have to have two separate types, A Non-Copy Card type, and a Copy "CardInfo" type that would not be conserved. Since it's nearly trivial to simply make random moves and check that cards are conserved, I don't think preventing that with this particular type system is a good idea. If it was as easy as say making a Deck type with a constructor and annotating it, then sure, but Rust 2015 edition can't do that.

___

```rust
//A demonstration that the following code produces no runtime overhead.
//https://godbolt.org/z/7tLBUx
pub struct Rect {
    x: u8,
    y: u8,
    w: u8,
    h: u8,
}

impl Rect {
    #[inline]
    pub fn point(&self) -> (u8,u8) {
        (self.x, self.y)
    }
}

impl From<((u8,u8), (u8,u8))> for Rect {
    #[inline]
    fn from(((x,y), (w,h)): ((u8,u8), (u8,u8))) -> Self {
        Rect {x,y,w,h}
    }
}

pub fn sum(rect: Rect) -> u8 {
    let (x, y) = rect.point();
    x + y
}

pub fn sum2(Rect{x, y, ..}: Rect) -> u8 {
    x + y
}

pub fn convert(r1: ((u8,u8), (u8,u8)), r2: ((u8,u8), (u8,u8))) -> u8 {
    sum(From::from(r1)) + sum2(From::from(r2))
}

pub fn same(r1: Rect, r2: Rect) -> u8 {
    sum(r1) + sum2(r2)
}
//implicit
pub fn im_sum<R: Into<Rect>>(r: R) -> u8 {
    let rect = r.into();
    let (x, y) = rect.point();
    x + y
}

pub fn im_sum2<R: Into<Rect>>(r: R) -> u8 {
    let Rect{x, y, ..} = r.into();
    x + y
}

pub fn im_convert(r1: ((u8,u8), (u8,u8)), r2: ((u8,u8), (u8,u8))) -> u8 {
    im_sum(r1) + im_sum2(r2)
}

pub fn im_same(r1: Rect, r2: Rect) -> u8 {
    im_sum(r1) + im_sum2(r2)
}
```

___

We're changing the card changes to be stored per card set instead of per card. We need the resulting data structure to be able to do the following:
* Produce an iterator of changes for a given card.
* Present a way to edit the current changes for a card set. Current code is receiving a mutable vector reference which it replaces with another vector instead of mutating.

Given we need to work per card *set* this implies a sparse data structure, probably a `HashMap`. But then that makes iterating over the changes for a card more complicated. And checking whether there are any changes for a card is by far the more common operation!


In order to make repeatedly retrieving card changes faster and, in this case, more straight forward, we'll maintain a list of which card sets are associated with each card, in order. The order will be determined by generation indexes stored with each list of changes in the `HashMap`