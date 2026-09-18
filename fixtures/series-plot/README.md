# Fixture: Series plot

Plot summary of a TV series up to an episode, plus a viewer's predicted ending. Tests branch and diff: the prediction is a branch on the known facts, and loose ends are gap nodes.

Files, added in the testing phase:
- `input.md`: the source text the extractor runs on.
- `expected.json`: the assumption and inference nodes a correct extraction must contain, and the conclusions that must fall when the key assumption is negated.
