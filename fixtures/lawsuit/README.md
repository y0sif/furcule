# Fixture: Lawsuit

A civil case summary with a timeline, testimony and exhibits. Tests entity handling and contradiction detection between testimonies.

Files, added in the testing phase:
- `input.md`: the source text the extractor runs on.
- `expected.json`: the assumption and inference nodes a correct extraction must contain, and the conclusions that must fall when the key assumption is negated.
