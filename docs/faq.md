# FAQ

**What is Furcule?**
An open source tool that turns any text into a graph of facts, assumptions and conclusions, lets you negate an assumption to see which conclusions fall, and lets you branch and diff alternative lines of reasoning. It runs locally and is driven by the AI agent or chat app you already use.

**Why the name?**
A furcule is a small fork. The logo is a fork with a node at the split: the node is the assumption, the prongs are the branches. The word itself means nothing about the product on purpose.

**How does Furcule compare to Mindwrinkles?**
Mindwrinkles maps one pasted argument and scores it. Furcule keeps a persistent graph of any text, makes assumptions toggleable, propagates what falls, branches, diffs and checks. Mindwrinkles is closed; Furcule is MIT and local. See `comparison.md`.

**How does it compare to Argdown or Kialo?**
Those are authoring tools: you write the map. Furcule extracts the map with your agent, then reasons over it. It will export to Argdown.

**Can't I just ask Claude or ChatGPT what I am assuming?**
Yes, and you should. That gives you a list once. Furcule keeps the graph, shows which conclusions rest on which assumptions, animates what falls when you negate one, and holds two branches side by side. It is designed to be driven by that chat through MCP.

**Does it need an API key?**
No. The engine never calls a language model. Your coding agent or chat app connects over MCP and does the reading; Furcule keeps score. An optional bring-your-own-key mode may come later for people who want the binary to run its own agent.

**What is MCP and why does it matter here?**
The Model Context Protocol lets an AI app call external tools. Claude Code, OpenCode, Claude.ai and ChatGPT all speak it. Furcule exposes its graph operations as MCP tools, so any of those can build and interrogate a graph without Furcule paying for inference.

**What kinds of text does it work on?**
Anything with claims in it: a project idea you dictated, a research paper, a legal case summary, a detective case with suspects, a TV series plot with your predicted ending. The `fixtures/` directory has one of each.

**Is it a mind map?**
No. Nodes have kinds and status, edges have direction and meaning, and an inference stands or falls based on its supports. A mind map has none of that.

**How is "what falls" computed?**
Facts and assumptions stand unless negated. An inference stands if any of its support groups has every source standing. Negating a node recomputes standing over the graph. See `docs/schema.md`.

**Can it prove a conclusion is valid?**
Not in v1. v1 does mechanical propagation and gives your agent a reductio prompt to try to derive a contradiction. v0.3 adds SMT-backed validity for graphs that formalise cleanly. Full autoformalisation into a proof assistant is out of scope.

**Where is my data?**
In a directory you choose: `graph.json`, `branches/`, `sources/`. Plain files, git-friendly, nothing leaves your machine unless you connect a remote agent.

**Is there a hosted version?**
Planned for v0.3: cloud-saved graphs, shareable board links and a hosted agent for people without one. The local tool stays free and complete.

**What platforms?**
Linux, macOS and Windows, single binary. Install from source today; crates.io, npx, AUR and Homebrew with v0.1.
