# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## What this is

A thin **shell** (wrapper / modding layer) written in Rust that sits *between* your
terminal and an **external** ICFP2006 UMIX VM interpreter. This binary does **not**
contain the VM — it spawns the interpreter as a child process and relays bytes both
ways, intercepting the stream to reframe and (eventually) reshape the VM's output.

The interpreter and VM image live as a *sibling* project, not in this repo (see the
`just run` path below).

## Commands

Run inside the Nix dev shell (`nix develop`, or automatically via direnv).

| Task | Command |
|---|---|
| Test (all) | `cargo test` |
| Test (one) | `cargo test <fn_name>` e.g. `cargo test switches_xml_mode` |
| Watch (test + docs on save) | `just watch` → `bacon --job test-doc` |
| Build | `cargo build` / `nix build` |
| Run against the VM | `just run` → `cargo run -- -i <interpreter> <vm>` |
| Format / lint | `just pre-commit-all` (rustfmt, nixpkgs-fmt, …) |

- `bacon` is typically already running (`just watch`). Assume the build is GREEN unless
  told otherwise; don't re-run `cargo test` just to confirm a known-green state.
- `just run` hardcodes a sibling path (`../icfp2006-rust-interpreter/...`). The binary's
  CLI is `-i <interpreter-path> <vm-path>` (see `src/main.rs`).

## Architecture

Data flows in one direction through a small pipeline; understanding it requires reading
`session.rs`, `scanner.rs`, and `command.rs` together.

```
your stdin ──[thread: relay()]──▶ child stdin
child stdout ─[main loop, 4 KiB chunks]─▶ Scanner.feed() ─▶ [Segment] ─▶ Renderer.render() ─▶ your screen
```

- **`session.rs` — the orchestrator.** `Session::new` spawns the interpreter with piped
  stdin/stdout. `start()` writes the `boot_script()` to the child, spawns one thread
  running `relay()` (your stdin → child stdin, verbatim), then loops reading child stdout
  → `Scanner` → `Renderer`. Child EOF ends the loop. Two threads (not one) because
  blocking `read()` on one side would otherwise freeze the other; this is the minimum for
  bidirectional blocking I/O.
- **`command.rs` — the startup sequence.** `Command` is `Raw` (fire-and-forget bytes) or
  `Switch(Mode)`; `encode()` turns it into bytes. `boot_script()` is a lazy iterator that
  drives auto-login → launch → switch the engine into XML output mode.
- **`scanner.rs` — the stateful streaming framer.** Holds `mode` (`English`/`Xml`) and a
  carry-over `buf`, so it tolerates data arriving split across arbitrary chunk boundaries.
  - *English mode*: pass bytes through as `Segment::Plain` until the `<success>\n` MARKER
    is seen (a MARKER split across chunks is handled by keeping its prefix in `buf`), then
    flip to Xml mode.
  - *Xml mode*: drive `quick-xml` events with a **depth counter** to carve out each
    complete well-formed document as `Segment::Xml(bytes)`. **Two-context model**: bytes
    *outside* a document (depth 0 — reward codes, blank lines) are lenient passthrough →
    `Segment::Plain`; bytes *inside* (depth ≥ 1) are strict well-formed XML.
- **`renderer.rs` — the sink.** Currently echoes both `Plain` and `Xml` segments verbatim.
  Typed parsing of `Segment::Xml` into Rust trees is a later stage.
- **`lib.rs`** wires the modules and defines `Mode`.

### Where the design lives

- `docs/superpowers/specs/*.md` — staged design specs (thin-shell v1, session refactor,
  XML parse). Read the relevant spec before changing `scanner`/`renderer`; they state the
  contract (e.g. "the only allowed assumption is that documents are well-formed XML").
- `docs/TODO.md` — loose roadmap. `data/xml/` — captured XML samples used as references.

## Conventions

- **Tests: English name + Japanese comment, always paired.** Every `#[test]` has an
  English fn name *and* a Japanese `//` comment stating intent (正常系/異常系). Match this.
- Annotation comments (`TODO`, `FIXME`, `HACK`, `NOTE`) are used throughout — keep them.

## Working in this repo (important)

This is a **learning project** — the user is learning Rust through ICFP2006. The operating
rules that change *how* you work here live in `CLAUDE.local.md` (private) and the memory
index, and they take precedence. In short:

- **No spoilers** about the ICFP2006 story or system — including "you'll need X later"
  style hints. Be a fellow explorer, not a walkthrough.
- **The user writes the `src` code.** Coach with failing tests and chat-side snippets;
  don't edit `src/` for them. (Docs/config like this file are fine to write.)
- **VCS is `jj` (jujutsu), never `git`.** Prefer `fd` over `find`, `rg` over `grep`.
