# Fixture: Detective case

A whodunit with suspects, places, alibis and evidence. Tests entity nodes, who-to-question-next from gap nodes, and whether the facts are sufficient for the accused conclusion.

Files, added in the testing phase:
- `input.md`: the source text the extractor runs on.
- `expected.json`: the assumption and inference nodes a correct extraction must contain, and the conclusions that must fall when the key assumption is negated.
