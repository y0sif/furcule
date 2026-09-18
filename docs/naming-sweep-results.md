# namesweep results

Product-name sweep for a local-first reasoning-graph tool. All availability cells below come from checks run on 2026-09-18 from this machine; 'unchecked' means the check failed or was not run.

## Pipeline and numbers

- Word list: `/usr/share/dict/words` absent; used dwyl `words_alpha.txt` (370,105 words). Lowercase ASCII alpha, 4-9 letters: 200,008.
- Shape filter (inflection suffixes, medical/chemical substrings, vowel ratio 0.3-0.6, no 4-consonant run): 99,830.
- Rarity (wordfreq zipf_frequency('en') in [1.3, 3.2]): 22,422.
- Seed sample: **seed 20260918, 1,200 words**. .com via dns.google NS lookup (8 threads, 10 s timeout): 3 NXDOMAIN (striatum, equimolar, oviposit); Verisign RDAP confirmed only **oviposit** as unregistered (striatum, equimolar are registered but undelegated).
- Because the seed sample yielded 1 usable word, the sweep was widened (same tooling, same seed for sampling):
  - the whole 1.3-3.2 band (22,422 words): 99 NXDOMAIN, **43 RDAP-confirmed free**, 32 with an English dictionary entry;
  - the 1.0-1.3 band (5,239 words): 117 NXDOMAIN, **95 RDAP-confirmed free**, 56 with an entry;
  - a noun-shaped subset of words wordfreq does not know (zipf 0; extra prefix/suffix cuts; 28,872 candidates, 12,000 sampled with seed 20260918): 3,659 NXDOMAIN, **3,536 RDAP-confirmed free**, **1,425 with an English Wiktionary entry**.
- Total .com-checked: 39,661 words; RDAP-confirmed free: 3,674; with an English entry: 1,513.
- DNS NXDOMAIN is not enough: 56 of 99 NXDOMAIN words in band 1 were registered per RDAP (57% false positives), so every .com claim here is RDAP-confirmed (HTTP 404 from rdap.verisign.com).
- .io/.dev/.app/.sh: DNS NS lookup on all 168 words that reached step 5 (138 from bands 1+2, 30 zero-band shortlist): 168/168 NXDOMAIN on all four. RDAP confirmation on the 40 shortlisted words: .dev/.app via pubapi.registry.google (validated: web.dev/cash.app=200, nonsense=404), .io/.sh via rdap.identitydigital.services (validated: github.io/nic.sh=200, nonsense=404): 40/40 free on all four.
- crates.io / npm / PyPI (HTTP 404 = free) on the same 168 words: 168/168 free on all three.
- Dictionary: api.dictionaryapi.dev was unreachable for the whole session (connection failures, probed repeatedly), so definitions come from Wiktionary (REST definition endpoint for bands 1+2; wikitext first sense for the zero band) and etymologies from the Wiktionary English 'Etymology' section. Words without an English Wiktionary entry were dropped.
- GitHub: 40 words (10 from bands 1+2, 30 from the zero band) via the search API (`WORD in:name`, per_page=3) and `/users/WORD`, authenticated with the local `gh` token after unauthenticated calls hit connection errors; all 80 calls completed.
- Network note: this machine's IPv6 is flaky; forcing IPv4 fixed intermittent urllib failures. dns.google, rdap.verisign.com and Wiktionary were reachable throughout.

## Scoring

Each of the 40 shortlisted words was scored 1-5 on: sounds like a product name; easy to say and spell; concrete logo image; faint metaphor for the product (bonus, must not be literal); no negative or silly connotation; no strong existing brand. Total out of 30. Judgement is mine; availability is measured.

## Top 12

| # | word | part of speech, definition | origin | say it | why it fits | logo image | .com/.io/.dev/.app/.sh | crates/npm/pypi | GitHub | username | score | caveat |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | **harmotome** | noun: A rare zeolite, a hydrated barium silicate that forms vitreous white monoclinic crystals | From grc:ἁρμός and τομή. | HAR-mo-tome | Greek harmos+tome, 'cut at the joints': carving an argument at its joints | a white crystal cluster | free/free/free/free/free | free/free/free | 0 repos, top 0 stars | free | 26/30 |  |
| 2 | **cromorne** | noun: A French woodwind instrument resembling the crumhorn | unchecked (no etymology section on Wiktionary) | kro-MORN | none needed; a curved reed instrument, name-like the way 'bun' is | a curved horn | free/free/free/free/free | free/free/free | 0 repos, top 0 stars | taken | 26/30 |  |
| 3 | **absconsa** | noun: A kind of dark lantern | unchecked (no etymology section on Wiktionary) | ab-SKON-sa | a detective's dark lantern: shutter open, it throws light on one thing | a bullseye lantern | free/free/free/free/free | free/free/free | 0 repos, top 0 stars | free | 25/30 | echo of 'abscond' |
| 4 | **ramuscule** | noun: A twig, a small branch | From la:ramusculus. | ra-MUS-kyool | a small branch: the product branches to explore alternatives | a forked twig | free/free/free/free/free | free/free/free | 0 repos, top 0 stars | free | 25/30 | echo of 'minuscule'/'muscle' |
| 5 | **boucharde** | noun: A bush hammer | unchecked (no etymology section on Wiktionary) | boo-SHARD | a bush hammer shapes stone by chipping, as a lathe shapes by removing | a textured stone hammer | free/free/free/free/free | free/free/free | 2 repos, top 0 stars | taken | 24/30 | French 'ch'; surname Bouchard |
| 6 | **urceole** | noun: A vessel for water or washing the hands in Roman Catholicism | From la:urceolus. | UR-see-ole | a small pitcher: it holds what you pour in | a small pitcher | free/free/free/free/free | free/free/free | 0 repos, top 0 stars | free | 24/30 | stress not obvious |
| 7 | **sticcado** | noun: A musical instrument resembling a xylophone. The bars were made of metal, wood, or glass | Compare it:steccato. | sti-KAH-do | bars laid in order; none needed | xylophone bars | free/free/free/free/free | free/free/free | 1 repos, top 0 stars | free | 23/30 | staccato confusion; 'sticcato' variant |
| 8 | **speronaro** | noun: A kind of Sicilian open boat | unchecked (no etymology section on Wiktionary) | spe-ro-NAH-ro | none; a light open boat you can row anywhere | a small open boat | free/free/free/free/free | free/free/free | 0 repos, top 0 stars | free | 23/30 | 9 letters |
| 9 | **euphroe** | noun: A long wooden slat, with holes for cords, that holds up an awning | unchecked (no etymology section on Wiktionary) | YOO-froh | a slat that holds many cords taut: the string on a corkboard | a wooden slat with cords | free/free/free/free/free | free/free/free | 0 repos, top 0 stars | free | 23/30 | variant spelling 'uphroe' |
| 10 | **siphuncle** | noun: A strand of tissue passing longitudinally through the shell of a cephalopod, used primarily in emptying water from new chambers as the shell grows | From la:sīphunculus. | SY-funk-ul | the tube linking a nautilus's chambers: links between nodes | a nautilus section | free/free/free/free/free | free/free/free | 0 repos, top 0 stars | free | 22/30 | echo of 'carbuncle' |
| 11 | **involucre** | noun: Conspicuous bract, bract pair or ring of bracts at the base of an inflorescence | Borrowed from fr:involucre, from la:involūcrum. . | IN-vuh-loo-ker | a ring of bracts that encloses a flower head: wraps the argument | a ring of leaves | free/free/free/free/free | free/free/free | 7 repos, top 0 stars | free | 22/30 | 'involucrum' variant; 7 unrelated repos |
| 12 | **craspedon** | noun: A border or fringe | From grc:κράσπεδον. | KRAS-pe-don | a border or fringe: the edge of what is known | a fringed border | free/free/free/free/free | free/free/free | 0 repos, top 0 stars | free | 22/30 |  |

Ties at 22/30 not in the top 12: sematrope (existing repo falk-hueffner/sematrope, 17 stars), emplecton (weak logo), jorram and toccatina (GitHub username taken), flagonet (reads as flag+net).

## All 40 shortlisted words, ranked by score

| word | pos | definition | origin | .com | .io | .dev | .app | .sh | crates | npm | pypi | GitHub repos (top stars) | username | score | note |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| cromorne | noun | A French woodwind instrument resembling the crumhorn | unchecked (no etymology section on Wiktionary) | free | free | free | free | free | free | free | free | 0 repos, top 0 stars | taken | 26 |  |
| harmotome | noun | A rare zeolite, a hydrated barium silicate that forms vitreous white monoclinic crystals | From grc:ἁρμός and τομή. | free | free | free | free | free | free | free | free | 0 repos, top 0 stars | free | 26 |  |
| absconsa | noun | A kind of dark lantern | unchecked (no etymology section on Wiktionary) | free | free | free | free | free | free | free | free | 0 repos, top 0 stars | free | 25 | echo of 'abscond' |
| ramuscule | noun | A twig, a small branch | From la:ramusculus. | free | free | free | free | free | free | free | free | 0 repos, top 0 stars | free | 25 | echo of 'minuscule'/'muscle' |
| boucharde | noun | A bush hammer | unchecked (no etymology section on Wiktionary) | free | free | free | free | free | free | free | free | 2 repos, top 0 stars | taken | 24 | French 'ch'; surname Bouchard |
| urceole | noun | A vessel for water or washing the hands in Roman Catholicism | From la:urceolus. | free | free | free | free | free | free | free | free | 0 repos, top 0 stars | free | 24 | stress not obvious |
| sticcado | noun | A musical instrument resembling a xylophone. The bars were made of metal, wood, or glass | Compare it:steccato. | free | free | free | free | free | free | free | free | 1 repos, top 0 stars | free | 23 | staccato confusion; 'sticcato' variant |
| euphroe | noun | A long wooden slat, with holes for cords, that holds up an awning | unchecked (no etymology section on Wiktionary) | free | free | free | free | free | free | free | free | 0 repos, top 0 stars | free | 23 | variant spelling 'uphroe' |
| speronaro | noun | A kind of Sicilian open boat | unchecked (no etymology section on Wiktionary) | free | free | free | free | free | free | free | free | 0 repos, top 0 stars | free | 23 | 9 letters |
| siphuncle | noun | A strand of tissue passing longitudinally through the shell of a cephalopod, used primarily in emptying water  | From la:sīphunculus. | free | free | free | free | free | free | free | free | 0 repos, top 0 stars | free | 22 | echo of 'carbuncle' |
| involucre | noun | Conspicuous bract, bract pair or ring of bracts at the base of an inflorescence | Borrowed from fr:involucre, from la:involūcrum. . | free | free | free | free | free | free | free | free | 7 repos, top 0 stars | free | 22 | 'involucrum' variant; 7 unrelated repos |
| craspedon | noun | A border or fringe | From grc:κράσπεδον. | free | free | free | free | free | free | free | free | 0 repos, top 0 stars | free | 22 |  |
| sematrope | noun | An instrument for signalling by reflecting the rays of the sun in different directions | From . | free | free | free | free | free | free | free | free | 1 repos, top 17 stars | free | 22 | existing repo falk-hueffner/sematrope (17 stars) |
| emplecton | noun | A kind of masonry in which the outer faces of the wall are ashlar, the space between being filled with broken  | fr:- or la:emplecton, from grc:ἐμπλέκω; ἐν + πλέκω. | free | free | free | free | free | free | free | free | 0 repos, top 0 stars | free | 22 |  |
| jorram | noun | A Gaelic boatmen's song to accompany the strokes of rowing | unchecked (no etymology section on Wiktionary) | free | free | free | free | free | free | free | free | 1 repos, top 0 stars | taken | 22 | GitHub user taken |
| toccatina | noun | A short or simple toccata | From it:toccatina, itself toccata + -ina. | free | free | free | free | free | free | free | free | 0 repos, top 0 stars | taken | 22 | GitHub user taken; common music term |
| flagonet | noun | A small flagon | From flagon + -et. | free | free | free | free | free | free | free | free | 0 repos, top 0 stars | free | 22 | reads like flag+net |
| protocone | noun | A cusp in the corner of an upper molar tooth in mammals | unchecked (no etymology section on Wiktionary) | free | free | free | free | free | free | free | free | 0 repos, top 0 stars | taken | 21 | proto+cone compound feel; GitHub user taken |
| dehiscent | adjective | Of or pertaining to dehiscence, i.e., a rupture, as with a surgical wound opening up, often with a flow of ser | la:dehīscēns, present participle of dehīscō, from hīscō. | free | free | free | free | free | free | free | free | 2 repos, top 2 stars | taken | 21 | adjective; surgical sense; GitHub user taken |
| chorizont | noun | One who challenges a widely held assumption of authorship for a major work, especially one who believes that t | From grc:χωρίζων, present participle of χωρίζω. | free | free | free | free | free | free | free | free | 0 repos, top 0 stars | free | 21 | echoes chorizo/horizon |
| griffaun | noun | A kind of spade, shovel or hoe | unchecked (no etymology section on Wiktionary) | free | free | free | free | free | free | free | free | 0 repos, top 0 stars | free | 21 | griffin echo |
| pennoncel | noun | A small pennon | From enm:penoncel, from fro:penuncel, diminutive of penon. See pennon, pennant.  | free | free | free | free | free | free | free | free | 0 repos, top 0 stars | free | 21 | spelling variants |
| kittereen | noun | A one-horse, two-wheel chaise or buggy, with or without a movable top | unchecked (no etymology section on Wiktionary) | free | free | free | free | free | free | free | free | 0 repos, top 0 stars | free | 21 | sounds silly |
| phacolith | noun | A lens-shaped mass that occurs in an anticlinal crest or synclinal trough | as phacolite, from . | free | free | free | free | free | free | free | free | 0 repos, top 0 stars | free | 21 | -lith reads geological/medical |
| penneech | noun | An old English card game for two players with hands of seven cards, and the trump suit changing with each tric | unchecked (no etymology section on Wiktionary) | free | free | free | free | free | free | free | free | 0 repos, top 0 stars | free | 20 | silly |
| budgerow | noun | A kind of large river barge | From hi:बजरा. | free | free | free | free | free | free | free | free | 1 repos, top 0 stars | free | 20 | budgie echo; Hindi origin |
| fasciole | noun | A band of minute tubercles, bearing modified spines, found on the shells of spatangoid sea urchins | Borrowed from la:fasciola. See fascia. | free | free | free | free | free | free | free | free | 0 repos, top 0 stars | free | 19 | fascia/fascist echo |
| mesosoma | noun | The middle part of the body, or tagma, of arthropods whose body is composed of three parts (the other two bein | From meso + soma. | free | free | free | free | free | free | free | free | 0 repos, top 0 stars | free | 19 | zoological |
| diapasm | noun | powdered aromatic herbs, sometimes made into little balls and strung together | From la:diapasma, grc:διάπασμα, ultimately from πάσσω; compare diapasme. | free | free | free | free | free | free | free | free | 1 repos, top 0 stars | free | 19 | spasm echo |
| faburden | noun | A kind of counterpoint with a drone bass | From fr:faux bourdon. See false, and burden. | free | free | free | free | free | free | free | free | 0 repos, top 0 stars | taken | 19 | 'burden'; GitHub user taken |
| halpace | noun | A haut-pas; a dais | unchecked (no etymology section on Wiktionary) | free | free | free | free | free | free | free | free | 0 repos, top 0 stars | taken | 19 | GitHub user taken |
| glomerule | noun | A head or dense cluster of flowers, formed by condensation of a cyme, as in the flowering dogwood | Diminutive of la:glomus ball. | free | free | free | free | free | free | free | free | 1 repos, top 4 stars | free | 19 | glomerulus (kidney) echo; a kidney-ML repo exists |
| camauro | noun | A cap, of crimson velvet, trimmed with ermine, worn by the pope | Ultimately related to the word camel. | free | free | free | free | free | free | free | free | 1 repos, top 0 stars | free | 19 | papal association |
| stratiote | noun | A small landowner who had an obligation of military service in time of war | Borrowed from grc:στρατιώτης. | free | free | free | free | free | free | free | free | 1 repos, top 0 stars | taken | 19 | GitHub user taken |
| choriamb | noun | A choriambus | unchecked (no etymology section on Wiktionary) | free | free | free | free | free | free | free | free | 0 repos, top 0 stars | free | 19 |  |
| hieromonk | noun | A monk of the Eastern Church who is also a priest | From gkm:ἱερομόναχος, from grc:ἱερός + μοναχός. | free | free | free | free | free | free | free | free | 1 repos, top 0 stars | free | 18 | religious |
| seignory | noun | Alternative form of seigniory | unchecked (no etymology section on Wiktionary) | free | free | free | free | free | free | free | free | 0 repos, top 0 stars | free | 18 | alternative spelling of seigniory |
| selvagee | noun | A skein or hank of rope yarns wound round with yarns or marline, used for stoppers, straps, etc | unchecked (no etymology section on Wiktionary) | free | free | free | free | free | free | free | free | 10 repos, top 0 stars | taken | 18 | 10 repos; user taken; selvage echo |
| enfeoff | verb | To transfer a fief to, to endow with a fief; to put (a person) in legal possession of a freehold interest | From Late enm:enfeffen,enfeoffen, from fro:enfeffer, enfieffer (compare Anglo-in | free | free | free | free | free | free | free | free | 0 repos, top 0 stars | free | 17 | verb; 'ff' spelling |
| hypostome | noun | Any of certain mouth appendages of some insects, arachnids, crustaceans, and hydrozoa; in particular:
The vent | unchecked (no etymology section on Wiktionary) | free | free | free | free | free | free | free | free | 2 repos, top 1 stars | free | 16 | medical/zoological |

## Rejected at judgement (examples)

Band 1 (43 free): mostly medical/botanical adjectives (arytenoid, catarrhal, petechial, squamosal, subapical...), sect/people names (jansenist, socinian, midianite, nabataean, ayyubid - the last also excluded as Arabic), negatives (disfavour, unfilial, foulbrood), plurals (tegulae, cercariae), a misspelling (gutteral) and an archaic adverb (forasmuch). Kept: siphuncle, involucre, dehiscent.
Band 2 (95 free): geologic stages and political -isms (toarcian, gaullism, titoism, hitlerite), taxa (anodonta, plethodon, tipulidae), medical (uraemic, leucotomy, mycetoma), comparatives (unkindest, whiniest). Kept: fasciole, hieromonk, hypostome, protocone, seignory, mesosoma, enfeoff.
Zero band (1,425 with entries): 1,186 passed a morphology/definition pre-filter and were read by hand; most are chemistry, anatomy, taxa, compounds (hawsehole, wasteweir), coined -tron/-scope/-type words, divination words (belomancy, halomancy), be-/a- verbs and adjectives, or foreign-script loans with unclear pronunciation (drageoir, clarseach, plemochoe). 30 kept for checks.

## Recommendation

1. **harmotome** - a real mineral (a zeolite), 9 letters, HAR-mo-tome, Greek harmos + tome, 'cut at the joints', which is exactly what the tool does to an argument without saying so; crystal logo; 0 GitHub repos, username free, every domain and registry free.
2. **absconsa** - a dark lantern (the detective's tool), ab-SKON-sa, concrete bullseye-lantern logo; only risk is the 'abscond' echo; 0 repos, username free, everything free.
3. **cromorne** - a French reed instrument, kro-MORN, the most 'name-like' word in the set with no product meaning at all (the bun/origami category); 0 repos and all domains/registries free, but the GitHub username is taken.
Runners-up: ramuscule (twig, branching metaphor, but 'minuscule' echo), boucharde (bush hammer, lathe-like metaphor, French 'ch' and GitHub user taken), urceole (small pitcher, stress not obvious).

## Files

survivors_step2.tsv (22,422 rare words), sample_step3.tsv (seed sample), com_results*.tsv (DNS), rdap_results*.tsv (RDAP .com), com_free_*.txt, step5_results*.json (TLD DNS + registries), step5b_rdap_tlds.json (TLD RDAP), step6_results*.json and exists_zero.json (dictionary), etym_all.json, step7_results_*.json (GitHub), final_rows.json, and the scripts step1_filter.py ... step7_github.py, assemble.py.
