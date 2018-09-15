## Font design decisions

First off, it should be said that I started with the Pico-8 font (used under cc0).

Since I did not want to have to find or create Unicode fonts, drawing a line in the sand and supporting only 8 bit characters was the next best thing.

I knew I needed to have representations of the suits, and I knew I would be referring to the cards in text, so adding suit characters was a pretty obvious idea. There's no good looking way I am aware of to make recognizable suits in less then 5x5 pixels. More on this later.

Once the suits were characters, I then wanted to render rotated-upside-down characters for the card graphics. While flipping them in memory was certainly possible, I had the entire second half of the byte to work with, so putting the flipped versions on the other side where we can OR in a bit to flip them seemed attractive.

The 10 character was done mostly as a convenience so the card rending code was simpler. I could have gone with "T" or some other single character but that just looks weird, and is hard to parse visually for those used to the standard deck.

Back to the 5x5 fonts. We want to have suit-rank character pairs in order to tersely represent cards within text. The other characters are 4x5, (including the horizontal spacing column) so while vertically there are no problems, there are some horizontally. If numbers as suits are laid out next to each other using the same layout the characters collide and it does not look good. Ideally, we want the font layout to be as simple as possible, so some visual degradation would tolerable, but the characters touching as unattractively as they do is not acceptable.
