# Phase 0 landscape: assumption-first reasoning graph (draft, 2026-09-18)

## Strongest competitor
Mindwrinkles (mindwrinkles.com), launched 2026-08-02, closed, free 3/day or $34 lifetime.
Paste one argument -> map of conclusion / premises / hidden assumptions / objections, Walton schemes,
rounds of counter-attacks, score card. Marketing copy mentions "how it changes if each one fails" but
no interactive assumption toggle, no persistent branches/diff, no formal validity, no API, short-argument scale.

## Matrix
| Name | Type / status | Verified capability | Price / license | Gap vs idea |
|---|---|---|---|---|
| Mindwrinkles | closed web app, shipped Aug 2026 | LLM map w/ hidden-assumption nodes, Walton schemes, attack rounds | $34 lifetime | no branching/diff, no formal check, single short argument, closed |
| LARP (github flypggl-cpu/LARP) | prompt collection + py scripts, 2 stars, active | hidden premises, alternatives, missing evidence, evidence-to-hypothesis tables; indented text tree | CC BY-NC-SA | not a product, no graph viz, non-commercial |
| doubt (github alsoleg89/doubt) | Node CLI + agent skill, 7 stars, Jul 2026 | hand-authored .doubt.json evidence maps (supports/contradicts/qualifies/missing) -> HTML | MIT | manual, no LLM extraction, no counterfactual, no validity |
| Pirozelli et al. 2026 (arXiv 2605.13793, github do-ald533/llm_argumentation) | research code, 0 stars | 12-step LLM pipeline incl. implicit-premise recovery, DAG w/ support/attack, transitive reduction; CSV out | unspecified | no viz, no counterfactual, no validity; good reference design |
| Argdown | OSS syntax + VS Code + CLI, 1k stars, v2 Dec 2025, active Sep 2026 | text -> argument maps | MIT | no AI, no assumption typing, no validity; use as export format |
| Argumentree | closed SaaS freemium | AI extraction from transcripts -> pro/con trees, consensus scores | freemium | team decisions, pro/con not dependency, no counterfactual |
| Kialo / Kialo Edu | closed freemium | manual pro/con debate trees | freemium | debate format, manual, no AI |
| Criticly | closed desktop overlay | 5 lenses on selected text (critique, questions, counter-args) | $8.90/mo, $49 lifetime | text only, no graph |
| socratic-council | OSS Tauri/Rust+React, 8 stars, Sep 2026 | 16-agent debate w/ live argument map, fork conversation | Apache-2.0 | simulates debate, not audit of user's reasoning |
| Discourse Graphs | OSS Roam/Obsidian, 47 stars, active Sep 2026, CZI-funded | manual question/claim/evidence graphs for science | Apache-2.0 | manual, no AI, no counterfactual |
| Arbor HTR (arXiv 2606.11926), HEP (arXiv 2607.09195) | research, Jun/Jul 2026 | hypothesis trees w/ belief probabilities for autonomous AI scientists | code not released | for agent-run experiments, not human reasoning audit |
| PARC ACH 2.0 | free, abandoned (2000s) | hypotheses x evidence matrix, disconfirmation | free | dead, matrix not graph, no AI |
| logikon | OSS Python, dormant since Sep 2024 | scores LLM reasoning traces | AGPL-3.0 | evaluates LLM output, not user reasoning |
| draw.io / Miro / FigJam templates | manual whiteboards (draw.io has AI generate) | boxes and arrows | free/freemium | no semantics, no propagation |
| Plain chat LLM (Claude/ChatGPT) | the real substitute | one-shot "what am I assuming?" | subscription | no persistence, no graph, no branch diff, no mechanical propagation, no formal check |

## Enabling tech, not competitors
- Autoformalization to Lean (ProofBridge, Kimina-Autoformalizer): research-grade, brittle for NL arguments. Do not build v1 on it.
- Pragmatic validity: propositional/first-order encoding + SAT/SMT (Z3), Assumption-Based Argumentation semantics, reductio via LLM-driven contradiction search.
- graphify: borrow EXTRACTED/INFERRED/AMBIGUOUS provenance tags, Leiden communities, CLI + graph.json + self-contained HTML, query/path/explain.

## Differentiation statement
Given that Mindwrinkles, Argumentree, Kialo and plain chat LLMs already surface hidden assumptions in a single argument, this project still earns its existence because (1) it treats the assumption as a first-class, toggleable node in a persistent directed graph, so negating one mechanically shows which conclusions lose support and lets the user branch, explore an alternative assumption with the agent, and diff branches side by side; (2) it is open source and local-first, works on any text at any scale (an idea dump, a research paper, a case file) and exports to open formats, where every current alternative is closed, single-argument, or manual; (3) it attaches a checkable layer (dependency propagation, contradiction search, SMT-backed validity where the graph is formalizable) that no shipping tool has.

## Kill criteria
None triggered. No first-party platform feature; closest OSS are 2-47 star single-maintainer projects; recent paid launches (Mindwrinkles, Criticly) signal emerging demand, not failure; adjacent products exist so the niche is not empty.

## Risks
1. Mindwrinkles is 6 weeks old and pointed the same way; could add branching. Mitigation: OSS + scale + formal layer.
2. Chat-box substitute captures ~70% of one-shot value. Product must win on what chat cannot do.
3. Formal validity is the hardest feature. Scope v1 to propagation + LLM reductio; SMT v2; Lean out of scope.

## KAN paper demo material (Career/Career/Research/rough_hook_documentation/article)
- Abstract conclusion "infrastructure limitations prevent direct KAN implementation in performance-critical chess engines" rests on unstated assumption that KAN inference needs runtime spline evaluation. Alternate branch = LUT-KAN int8 quantization (Bullet fork Phase 3).
- Bullet fork Phase 1: "+0.3% did not reproduce -22%; likely cause: training dynamics" = INFERRED, untested node.
- Line 283: "suggesting learnable activations benefit visual recognition tasks where subtle feature relationships are critical" = causal explanation inferred from one result.
