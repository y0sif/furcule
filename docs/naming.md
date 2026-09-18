# Naming

**Status:** decided 2026-09-18: **Furcule** (a small fork; FUR-kyool). All domains, registries and the GitHub username were free at decision time; register furcule.com/.dev/.io before publishing.
Rename history: placeholder `nameless` -> `furcule` on 2026-09-18.

## Constraints (decided 2026-09-18)

- One real dictionary word. Not a compound, not coined, not two words glued.
- Must NOT describe the product. A good name in general, like bun, ant, origami, colibri, bond. A faint metaphor you can discover later is a plus, never literal.
- No birds or animals as a crutch, no food, no Egyptian or Arabic words. Latin, Greek, Italian, French, Japanese, Norse-derived English words are fine.
- Pronounceable on sight, one obvious spelling, 4 to 9 letters, lowercase ASCII.
- .com must be free, plus .io .dev .app .sh, crates.io, npm, PyPI, GitHub username, and no notable GitHub repo of that name.

## Finding

Every English word with measurable usage already has its .com registered (speculators register pronounceable 5 to 7 letter strings continuously; sindrel.com was registered on 2026-09-02). A free .com therefore means an obscure word. The pick is the obscure word that still sounds like a brand.

Sweep of 39,661 words: 3,674 with a free .com (registry RDAP confirmed), 1,513 of those in an English dictionary, 168 checked on every registry (all free). Full report: `naming-sweep-results.md`. Raw .com-free list from the widest band, 3,536 words of which about 1,400 have a dictionary entry: `naming-candidates-raw.txt`.

## Shortlist (all free on .com .io .dev .app .sh, crates.io, npm, PyPI, GitHub username free, zero repos unless noted)

| Word | What it is | Say it | Logo | Hidden link (optional) | Notes |
|---|---|---|---|---|---|
| harmotome | white zeolite mineral | HAR-mo-tome | crystal cluster | Greek harmos + tome, "cut at the joints" | fully clear; agent's #1 |
| cromorne | French woodwind | kro-MORN | curved horn | none, pure name | GitHub username taken |
| rhabdome | light-sensing rod in an insect eye | RAB-dome | faceted eye | the part that sees | fully clear |
| absconsa | a dark lantern | ab-SKON-sa | bullseye lantern | detective's tool | echo of "abscond" |
| quardeel | a cask | kwar-DEEL | barrel | none | fully clear |
| turricle | a small tower | TUR-i-kl | tower | none | fully clear |
| furcule | a small fork | FUR-kyool | fork / wishbone | forking, branching | 1 unrelated repo |
| craspedon | a border or fringe | KRAS-pe-don | fringed edge | edge of the known | fully clear |
| staffete | a relay courier (It. staffetta) | sta-FET | baton | passes node to node | fully clear |
| ramuscule | a twig, small branch | ra-MUS-kyool | forked twig | branching | echo of minuscule |
| urceole | a small water vessel | UR-see-ole | pitcher | none | stress not obvious |
| emplecton | a masonry style | em-PLEK-ton | stone wall | none | fully clear |
| sticcado | xylophone-like instrument | sti-KAH-do | bars | none | staccato confusion |
| euphroe | slat with holes that holds awning cords | YOO-froh | slat with cords | corkboard string | variant spelling uphroe |
| siphuncle | tube linking a nautilus shell's chambers | SY-funk-ul | nautilus section | links between chambers | echo of carbuncle |

## Rejected (do not re-propose)

- Descriptive or Arabic: zann, redstring, enthymeme, unassume, tacit.
- Regional, food, or animal crutch: koshari, karkade, hudhud, nawras, kunafa, zaatar, pumice.
- Compounds or coined: moonlathe, tidewren, kilnmoth, kitewren, inklathe, tandril, sindrel, veldren and similar.
- Taken somewhere that matters: oryx, fennec, jerboa, bramble, thicket, ferret, tamarin, ochre, lupin, orrery, tessel, vexil, umbriel, quaoar.

## How to verify a new candidate

```bash
n=WORD
curl -s -o /dev/null -w "%{http_code}\n" https://rdap.verisign.com/com/v1/domain/$n.com            # .com
curl -s -o /dev/null -w "%{http_code}\n" https://rdap.identitydigital.services/rdap/domain/$n.io   # .io (also .sh)
curl -s -o /dev/null -w "%{http_code}\n" https://pubapi.registry.google/rdap/domain/$n.dev          # .dev (also .app)
curl -s -o /dev/null -w "%{http_code}\n" -A "name-check" https://crates.io/api/v1/crates/$n
curl -s -o /dev/null -w "%{http_code}\n" https://registry.npmjs.org/$n
curl -s -o /dev/null -w "%{http_code}\n" https://pypi.org/pypi/$n/json
curl -s -o /dev/null -w "%{http_code}\n" -A "name-check" https://api.github.com/users/$n
# 404 = free, 200 = taken. Do not trust plain DNS: registered-but-undelegated domains look free to DNS.
```
