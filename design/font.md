## Font design decisions

First off, it should be said that I started with the Pico-8 font (used under cc0).

Since I did not want to have to find or create Unicode fonts, drawing a line in the sand and supporting only 8 bit characters was the next best thing.

I knew I needed to have representations of the suits, and I knew I would be referring to the cards in text, so adding suit characters was a pretty obvious idea. There's no good looking way I am aware of to make recognizable suits in less then 5x5 pixels. More on this later.

Once the suits were characters, I then wanted to render rotated-upside-down characters for the card graphics. While flipping them in memory was certainly possible, I had the entire second half of the byte to work with, so putting the flipped versions on the other side where we can OR in a bit to flip them seemed attractive.

The 10 character was done mostly as a convenience so the card rending code was simpler. I could have gone with "T" or some other single character but that just looks weird, and is hard to parse visually for those used to the standard deck.

Back to the 5x5 fonts. We want to have rank-suit character pairs in order to tersely represent cards within text. The other characters are 4x5, (including the horizontal spacing column) so while vertically there are no problems, there are some horizontally. If numbers as suits are laid out next to each other using the same layout the characters collide and it does not look good. Ideally, we want the font layout to be as simple as possible, so some visual degradation would tolerable, but the characters touching as unattractively as they do is not acceptable.

So we need a way to indicate that the next characters need to be laid out in a special way. Since we are requiring more space, using a special character that will not be displayed to indicate the layout change means that he length of a string in bytes still directly indicates how wide it will be. Again, for simplicity's sake, we would like to not have to care which characters are in the byte stream when rendering them, as much as we can, so even though this only makes sense for number suit pairs, we can just go ahead and treat any characters we see after the special one the same as we would rank-suit pairs. 

Another complication is the special 10 character is double wide, so since 4 + 4 + 5 > 3 * 4, if we want the pairs to have a consistent width, (which for simplicity's sake we do,) we need each pair to take up 4 characters. So since we need an extra character, the simplest thing to do is just consume the 4th character and do nothing with it. We can assign meaning to it later if we want to, (maybe rendering the preceding characters in a certain colour?)

So if  we denote the special control character C, the rank-suit pair as RS and the ignored character as X, then to display a pair nicely spaced within 16 pixels, we simply insert "CRSX" into the string.

Since we do not want to forget which characters we are re-purposing, we should label them in the font. That way we know they are taken, the next time we look at the font to decide which byte value to use. While we're at it we should label the newline, since we are using that one as well. 

For all the different control purposes we might put byte values into in the future, we only need one glyph for the purpose of marking a value as used. Even though I'm not sure when we would ever really need to do so, it seems like a reasonable idea to reserve a byte value for displaying that glyph without any special handling. Byte 0x1 seems like a good one for tis purpose. Similarly, we might as well reserve the flipped version of that glyph, and the flipped versions of the control characters as well. We might even consider treating the flipped control characters like their non flipped counterparts if we ever do anything extensive with flipped text, just so we don't have to think about it.