# Furcule compared

Researched live on 2026-09-18. Every claim about a tool was checked on its own page or repository, not from memory. See `landscape.md` for the full notes.

## The short version

Several tools now surface hidden assumptions in a single argument. None makes the assumption a toggleable node in a persistent graph, branches from it, diffs the branches, or checks the result. The honest substitute is a chat box: paste your idea into Claude and ask "what am I assuming?" That gets you a one-shot list. Furcule is for what happens next.

## Head to head

| | Furcule | Mindwrinkles | Argdown | Argumentree | Kialo | LARP | doubt | A chat LLM |
|---|---|---|---|---|---|---|---|---|
| Open source | MIT | no | MIT | no | no | CC BY-NC-SA | MIT | no |
| Local-first | yes | no | yes | no | no | yes | yes | no |
| Extracts from text with an LLM | your agent | yes | no | yes, transcripts | no | prompt | no | yes |
| Assumption as a node type | yes | yes | no | no | no | text tree | no | no |
| Negate an assumption, see what falls | yes | text only | no | no | no | no | no | no |
| Branches and diff | yes | no | no | no | no | no | no | no |
| Provenance on every node | yes | no | no | no | no | no | sources required | no |
| Contradiction and gap checks | yes | partial | no | no | no | partial | contradicts edges | no |
| Formal validity | v0.3 | no | no | no | no | no | no | no |
| Input scale | any text | one argument | manual | meetings | manual | one argument | manual | context window |
| Price | free | 34 USD lifetime | free | freemium | freemium | free | free | subscription |

## By tool

**Mindwrinkles** (mindwrinkles.com, launched 2026-08-02). Paste one argument, get a map of conclusion, premises, hidden assumptions and objections, scored with Walton's argumentation schemes and a few rounds of counter-attack. The closest product. Closed, single short argument, no interactive toggle, no branches, no formal layer, no API. If you want a score card for a debate claim, use it.

**Argdown** (argdown.org, MIT). A Markdown-like syntax for argument maps with a VS Code extension and CLI, at v2 since December 2025. Manual authoring, no AI, no assumption typing, no validity. Furcule will export to it.

**Argumentree**. Closed SaaS that extracts pro and con trees from meeting transcripts with consensus scoring, built for team decisions. Pro and con, not dependency; no counterfactuals.

**Kialo**. Manual pro and con debate trees for public and classroom debate. No AI extraction.

**LARP** (github.com/flypggl-cpu/LARP). A prompt collection that lays out hidden premises, alternatives and missing evidence as an indented text tree. Good thinking, non-commercial licence, not a product.

**doubt** (github.com/alsoleg89/doubt, MIT). A CLI and agent skill that renders hand-written evidence maps with supports, contradicts, qualifies and missing edges, refusing unsourced evidence. Closest in spirit on provenance, no extraction, no negation.

**Research to watch**: Pirozelli et al. 2026 describe a twelve-step LLM pipeline that recovers implicit premises into an argument graph, which Furcule's extract prompt follows. Arbor and the Hypothesis Evolution Protocol (2026) grow hypothesis trees for autonomous AI scientists; same shape, different user.

**A chat LLM**. Ask it what you are assuming and you get a good list once. It cannot keep the graph, propagate a negation, hold two branches at the same time, or diff them. Furcule is built to be driven by that same chat.
