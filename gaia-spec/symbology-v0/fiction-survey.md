# Fiction survey: symbol systems (inspiration only, not evidence of security)

Every row is fiction. Source quality is noted because several are fan wikis or forum threads, which are weak evidence.

| System | What it is | Design idea worth borrowing | Source (quality) |
|---|---|---|---|
| Wakandan (Black Panther, 2018) | Alphabet designed by Hannah Beachler's team, based on Nsibidi and other real African writing; mapped one-to-one to Latin | Shapes built from proto-glyphs common across real writing systems | https://www.omniglot.com/conscripts/wakandan.htm ; https://www.printmag.com/comics-animation-design/the-typography-of-black-panther/ (reference sites) |
| Kryptonian, Man of Steel | Designed by linguist Christine Schreyer. Abugida: symbol = consonant, orientation = vowel; every symbol has at least one closed cell | The closed-cell orientation rule used in symbology v0 | https://dailyplanetdc.com/2020/09/21/understanding-schreyers-kryptonian-language/ (fan site with a named designer) |
| Kryptonian, comics history | Long history of decorative script without a fixed specification | Script without a spec cannot carry data | https://en.wikipedia.org/wiki/Kryptonian (encyclopedia) |
| Kree (MCU) | Has its own language, written with "Kree Glyphs"; no design documentation found | None usable yet: no spec or designer found | https://marvelcinematicuniverse.fandom.com/wiki/Kree (fan wiki, weak) |
| Asgardian (MCU) | Writing system similar to Nordic runes; spoken language similar to Norwegian | Reuse of a real rune family (Elder Futhark) gives readers an anchor; but no design source found | https://marvelcinematicuniverse.fandom.com/wiki/Asgardians (fan wiki, weak); https://www.reddit.com/r/marvelstudios/comments/o6w5do/can_anyone_decipher_this/ (forum) |
| Marvel comics runes | Runes in comics are often used nonsensically in magic dialogue, while real runes have specific meanings; Thor's Thurisaz symbol is a deliberate exception | Lesson: a symbol only carries information if its meaning is fixed in a spec | https://www.cbr.com/thor-costume-rune-meaning/ (entertainment journalism) |
| DC Atlantean (films) | Film accounts disagree: one version has a click-and-squeal underwater language, another has no special language. No designed script found | None usable: inconsistent canon, no spec | https://www.reddit.com/r/DC_Cinematic/comments/tn7bgy/so_whats_up_with_the_atlantean_language/ (forum, weak) |
| Atlantean from Disney's *Atlantis* (NOT DC or Marvel) | Constructed language by Marc Okrand with a script by John Emerson; the script has extra characters so it can serve as a simple cipher code in promotion | Shows a designed script can include a deliberate cipher layer, and also why such a cipher is weak | https://en.wikipedia.org/wiki/Atlantean_language (encyclopedia) |

## Findings

1. The best-documented systems (Kryptonian by Schreyer, Wakandan by Beachler) were built with real linguistic or typographic research. The weakest have no specification at all.
2. Several famous alphabets are one-to-one substitutions of English; the symbology tests show a simple frequency attack fully recovers such a mapping.
3. Borrowed idea in use: a guaranteed closed cell in every glyph so orientation can be read.
4. Not found: a primary design source for Kree, Asgardian or DC Atlantean. These stay open until one is found.
