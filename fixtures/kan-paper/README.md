# Fixture: KAN paper

A real research paper (the author's own IEEE article on Kolmogorov-Arnold Networks for chess). Tests extraction on scientific prose. Known unstated assumption: KAN inference needs runtime spline evaluation, from which the paper concludes engines cannot use KAN; the alternate branch is table-based quantisation.

Files, added in the testing phase:
- `input.md`: the source text the extractor runs on.
- `expected.json`: the assumption and inference nodes a correct extraction must contain, and the conclusions that must fall when the key assumption is negated.
