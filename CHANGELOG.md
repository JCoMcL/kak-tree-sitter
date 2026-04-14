# Changelog

<!--toc:start-->
- [Changelog](#changelog)
  - [kak-tree-sitter v3.2.1](#kak-tree-sitter-v321)
    - [Patch changes](#patch-changes)
  - [ktsctl v3.1.2](#ktsctl-v312)
    - [Patch changes](#patch-changes-1)
  - [kak-tree-sitter-config v.4.2.0](#kak-tree-sitter-config-v420)
    - [Minor changes](#minor-changes)
    - [Patch changes](#patch-changes-2)
  - [kak-tree-sitter v3.2.0](#kak-tree-sitter-v320)
    - [Minor changes](#minor-changes-1)
  - [kak-tree-sitter v3.1.3](#kak-tree-sitter-v313)
    - [Patch changes](#patch-changes-3)
  - [kak-tree-sitter-config v.4.1.2](#kak-tree-sitter-config-v412)
    - [Patch changes](#patch-changes-4)
  - [ktsctl v.3.1.2](#ktsctl-v312-1)
    - [Patch changes](#patch-changes-5)
  - [ktsctl v3.1.1](#ktsctl-v311)
    - [Patch changes](#patch-changes-6)
  - [kak-tree-sitter v3.1.2](#kak-tree-sitter-v312)
    - [Patch changes](#patch-changes-7)
  - [kak-tree-sitter-config v4.1.1](#kak-tree-sitter-config-v411)
    - [Patch changes](#patch-changes-8)
  - [kak-tree-sitter v3.1.1](#kak-tree-sitter-v311)
    - [Patch changes](#patch-changes-9)
  - [kak-tree-sitter v3.1.0](#kak-tree-sitter-v310)
    - [Patch changes](#patch-changes-10)
  - [ktsctl v3.1.0](#ktsctl-v310)
    - [Minor changes](#minor-changes-2)
    - [Patch changes](#patch-changes-11)
  - [kak-tree-sitter-config v4.1.0](#kak-tree-sitter-config-v410)
    - [Minor changes](#minor-changes-3)
  - [kak-tree-sitter v3.0.0](#kak-tree-sitter-v300)
    - [Major changes](#major-changes)
    - [Minor changes](#minor-changes-4)
    - [Patch changes](#patch-changes-12)
  - [ktsctl v3.0.0](#ktsctl-v300)
    - [Major changes](#major-changes-1)
    - [Minor changes](#minor-changes-5)
    - [Patch changes](#patch-changes-13)
  - [kak-tree-sitter-config v4.0.0](#kak-tree-sitter-config-v400)
    - [Major changes](#major-changes-2)
    - [Minor changes](#minor-changes-6)
    - [Patch changes](#patch-changes-14)
<!--toc:end-->

This is the changelog of the **kak-tree-sitter** project, which is composed of three sub-projects:

- `kak-tree-sitter`, the binary bridging **tree-sitter** with **Kakoune**.
- `ktsctl`, the CLI companion.
- `kak-tree-sitter-config`, the library used to parse and use the configuration in both binaries.

> For older versions of the changelogs, please refer to [CHANGELOG.old.md](./CHANGELOG.old.md)

## kak-tree-sitter v3.2.1

<details>
  <summary><b>Commit set</b></summary>
  <ul>
    <li><a href="https://git.sr.ht/~hadronized/kak-tree-sitter/commit/afed73a">afed73a</a> Update Cargo.lock.</li>
  <ul>
</details>

### Patch changes

- Update `Cargo.lock`.

## ktsctl v3.1.2

<details>
  <summary><b>Commit set</b></summary>
  <ul>
    <li><a href="https://git.sr.ht/~hadronized/kak-tree-sitter/commit/afed73a">afed73a</a> Update Cargo.lock.</li>
  <ul>
</details>

### Patch changes

- Update `Cargo.lock`.

## kak-tree-sitter-config v.4.2.0

<details>
  <summary><b>Commit set</b></summary>
  <ul>
    <li><a href="https://git.sr.ht/~hadronized/kak-tree-sitter/commit/07f0000">07f0000</a> Update bash grammar and queries.</li>
    <li><a href="https://git.sr.ht/~hadronized/kak-tree-sitter/commit/d680fc9">d680fc9</a> Add KDL support.</li>
  <ul>
</details>

### Minor changes

- Add suport for `KDL`.

### Patch changes

- Update `bash` grammar and queries pins.

## kak-tree-sitter v3.2.0

<details>
  <summary><b>Commit set</b></summary>
  <ul>
    <li><a href="https://git.sr.ht/~hadronized/kak-tree-sitter/commit/ce87ddb">96df26f</a> Add tree-sitter-version.</li>
  <ul>
</details>

### Minor changes

- Add `tree-sitter-version`.
 
## kak-tree-sitter v3.1.3

<details>
  <summary><b>Commit set</b></summary>
  <ul>
    <li><a href="https://git.sr.ht/~hadronized/kak-tree-sitter/commit/96df26f">96df26f</a> Fix Julia queries for new tree-sitter.</li>
  <ul>
</details>

### Patch changes

- Bump dependencies.

## kak-tree-sitter-config v.4.1.2

<details>
  <summary><b>Commit set</b></summary>
  <ul>
    <li><a href="https://git.sr.ht/~hadronized/kak-tree-sitter/commit/96df26f">96df26f</a> Fix Julia queries for new tree-sitter.</li>
  <ul>
</details>

### Patch changes

- Fix **julia** queries.

## ktsctl v.3.1.2

<details>
  <summary><b>Commit set</b></summary>
  <ul>
    <li><a href="https://git.sr.ht/~hadronized/kak-tree-sitter/commit/96df26f">96df26f</a> Fix Julia queries for new tree-sitter.</li>
  <ul>
</details>

### Patch changes

- Fix **julia** queries.

## ktsctl v3.1.1

### Patch changes

<details>
  <summary><b>Commit set</b></summary>
  <ul>
    <li><a href="https://git.sr.ht/~hadronized/kak-tree-sitter/commit/96df26f">96df26f</a> Fix Julia queries for new tree-sitter.</li>
  <ul>
</details>

- Update dependencies.

## kak-tree-sitter v3.1.2

<details>
  <summary><b>Commit set</b></summary>
  <ul>
    <li><a href="https://git.sr.ht/~hadronized/kak-tree-sitter/commit/fa66e6b">fa66e6b</a> Put common manifest keys and dependencies in the workspace.</li>
    <li><a href="https://git.sr.ht/~hadronized/kak-tree-sitter/commit/c7169db">c7169db</a> Bump dependencies.</li>
    <li><a href="https://git.sr.ht/~hadronized/kak-tree-sitter/commit/7b40207">7b40207</a> Fix end of line highlighting issue with tree-house.</li>
    <li><a href="https://git.sr.ht/~hadronized/kak-tree-sitter/commit/58143f7">58143f7</a> [kak-tree-sitter] Remove the trim when updating tree-house buffer.</li>
  <ul>
</details>

### Patch changes

- Rework the workspace layout.
- Bump dependencies.
- Fix end-of-line highlighting issue.

## kak-tree-sitter-config v4.1.1

<details>
  <summary><b>Commit set</b></summary>
  <ul>
    <li><a href="https://git.sr.ht/~hadronized/kak-tree-sitter/commit/fa66e6b">fa66e6b</a> Put common manifest keys and dependencies in the workspace.</li>
    <li><a href="https://git.sr.ht/~hadronized/kak-tree-sitter/commit/c7169db">c7169db</a> Bump dependencies.</li>
    <li><a href="https://git.sr.ht/~hadronized/kak-tree-sitter/commit/eeb2a2d">eeb2a2d</a> Update markdown / markdown.inline grammars.</li>
  <ul>
</details>

### Patch changes

- Rework the workspace layout.
- Update `markdown` and `markdown.inline` queries.
- Bump dependencies.

## kak-tree-sitter v3.1.1

<details>
  <summary><b>Commit set</b></summary>
  <ul>
    <li><a href="https://git.sr.ht/~hadronized/kak-tree-sitter/commit/753536b">753536b</a> 753536b Remove unused dependencies.</li>
    <li><a href="https://git.sr.ht/~hadronized/kak-tree-sitter/commit/82e3351">82e3351</a> 82e3351 Remove tree-sitter and tree-sitter-highlight dependency.</li>
  <ul>
</details>

### Patch changes

- Remove unused dependencies (`tree-sitter`; we now use `tree-house`).

## kak-tree-sitter v3.1.0

<details>
  <summary><b>Commit set</b></summary>
  <ul>
    <li><a href="https://git.sr.ht/~hadronized/kak-tree-sitter/commit/49d5192">49d5192</a> Fixed scratch buffers not being highlighted in bleeding edge Kakoune</li>
  <ul>
</details>

### Patch changes

- Fix scratch buffers not correctly highlighted in recent versions of Kakoune (> 2025.06).

## ktsctl v3.1.0

<details>
  <summary><b>Commit set</b></summary>
  <ul>
    <li><a href="https://git.sr.ht/~hadronized/kak-tree-sitter/commit/b235218">b235218</a> [ktsctl] Fix outdated --help.</li>
    <li><a href="https://git.sr.ht/~hadronized/kak-tree-sitter/commit/dd8c708">dd8c708</a> [ktsctl] Add trace logs for ktsctl git fetches.</li>
  <ul>
</details>

### Minor changes

- Add trace logs for ktsctl internals.

### Patch changes

- Fixes outdated `--help` output.

## kak-tree-sitter-config v4.1.0

<details>
  <summary><b>Commit set</b></summary>
  <ul>
    <li><a href="https://git.sr.ht/~hadronized/kak-tree-sitter/commit/8934f86">8934f86</a> Add pest grammar</li>
    <li><a href="https://git.sr.ht/~hadronized/kak-tree-sitter/commit/5b63a02">5b63a02</a> Added devicetree</li>
    <li><a href="https://git.sr.ht/~hadronized/kak-tree-sitter/commit/058e2d7">058e2d7</a> Add ocaml and ocaml-interface</li>
    <li><a href="https://git.sr.ht/~hadronized/kak-tree-sitter/commit/db60a23">db60a23</a> add grammar/queries config for lua</li>
    <li><a href="https://git.sr.ht/~hadronized/kak-tree-sitter/commit/498a22e">498a22e</a> Add just language configuration</li>
    <li><a href="https://git.sr.ht/~hadronized/kak-tree-sitter/commit/7b761cb">7b761cb</a> Add crystal language configuration</li>
  <ul>
</details>

### Minor changes

- Add `pest` support.
- Add `devicetree` support.
- Add `ocaml` and `ocamlinterface` support.
- Add `lua` support.
- Add `just` support.
- Add `crystal` support.

## kak-tree-sitter v3.0.0

<details>
  <summary><b>Commit set</b></summary>
  <ul>
    <li><a href="https://git.sr.ht/~hadronized/kak-tree-sitter/commit/cdcfb42">cdcfb42</a> Fix upper byte boundary in highlighting code.</li>
    <li><a href="https://git.sr.ht/~hadronized/kak-tree-sitter/commit/18c6e80">18c6e80</a> Properly load grammars.</li>
    <li><a href="https://git.sr.ht/~hadronized/kak-tree-sitter/commit/b5771db">b5771db</a> Do not return highlights if there’s none computed.</li>
    <li><a href="https://git.sr.ht/~hadronized/kak-tree-sitter/commit/abfeefe">abfeefe</a> Make tree-house grammar error more verbose.</li>
    <li><a href="https://git.sr.ht/~hadronized/kak-tree-sitter/commit/0cd2f51">0cd2f51</a> Fix symbol loading issue when the language name has a dot in it.</li>
    <li><a href="https://git.sr.ht/~hadronized/kak-tree-sitter/commit/14fce0a">14fce0a</a> Fix typo in debug log.</li>
    <li><a href="https://git.sr.ht/~hadronized/kak-tree-sitter/commit/8bee812">8bee812</a> Fix highlighting logic with tree-house.</li>
    <li><a href="https://git.sr.ht/~hadronized/kak-tree-sitter/commit/7ee3797">7ee3797</a> Fix tests regarding highlights with tree-house.</li>
    <li><a href="https://git.sr.ht/~hadronized/kak-tree-sitter/commit/a459d17">a459d17</a> Fix face definition.</li>
    <li><a href="https://git.sr.ht/~hadronized/kak-tree-sitter/commit/61c58d1">61c58d1</a> Convert to ropey/tree-house.</li>
    <li><a href="https://git.sr.ht/~hadronized/kak-tree-sitter/commit/cc55d3e">cc55d3e</a> Prepare switching to tree-house.</li>
    <li><a href="https://git.sr.ht/~hadronized/kak-tree-sitter/commit/f946149">f946149</a> ktsctl: operate on many languages at once.</li>
    <li><a href="https://git.sr.ht/~hadronized/kak-tree-sitter/commit/a5bda78">a5bda78</a> Abort early after updating a buffer if it didn’t change.</li>
    <li><a href="https://git.sr.ht/~hadronized/kak-tree-sitter/commit/eea7d5c">eea7d5c</a> Add tests for the triple buffer.</li>
    <li><a href="https://git.sr.ht/~hadronized/kak-tree-sitter/commit/ed00fe4">ed00fe4</a> Implement triple-buffering and remove back-buffers.</li>
    <li><a href="https://git.sr.ht/~hadronized/kak-tree-sitter/commit/3fcc99c">3fcc99c</a> Make fifo copying more explicit.</li>
    <li><a href="https://git.sr.ht/~hadronized/kak-tree-sitter/commit/6911185">6911185</a> Fix text-objects not correctly sent.</li>
    <li><a href="https://git.sr.ht/~hadronized/kak-tree-sitter/commit/ad15fdb">ad15fdb</a> Refactor IOHandler into its own module.</li>
    <li><a href="https://git.sr.ht/~hadronized/kak-tree-sitter/commit/5fb0060">5fb0060</a> Reimplement proper async IO decoupling.</li>
    <li><a href="https://git.sr.ht/~hadronized/kak-tree-sitter/commit/43b3a3f">43b3a3f</a> Remove trace logs and added some timer-based debug logs.</li>
    <li><a href="https://git.sr.ht/~hadronized/kak-tree-sitter/commit/5d9295e">5d9295e</a> Change configuration to make grammars dissociate from languages.</li>
    <li><a href="https://git.sr.ht/~hadronized/kak-tree-sitter/commit/fcab935">fcab935</a> Make grammars shared via Rc.</li>
    <li><a href="https://git.sr.ht/~hadronized/kak-tree-sitter/commit/ab3874f">ab3874f</a> Automatically generate face definition based on the config.</li>
    <li><a href="https://git.sr.ht/~hadronized/kak-tree-sitter/commit/53e4dde">53e4dde</a> Fix typos.</li>
    <li><a href="https://git.sr.ht/~hadronized/kak-tree-sitter/commit/be260eb">be260eb</a> [feature/22] Remove box type ascription when creating lazy languages.</li>
    <li><a href="https://git.sr.ht/~hadronized/kak-tree-sitter/commit/73a4eeb">73a4eeb</a> [feature/22] Massively simplify on-demand loading with LazyCell.</li>
    <li><a href="https://git.sr.ht/~hadronized/kak-tree-sitter/commit/846ab1a">846ab1a</a> [feature/22] (disgusting) version of on-demand loading of languages.</li>
    <li><a href="https://git.sr.ht/~hadronized/kak-tree-sitter/commit/4a6cce7">4a6cce7</a> Fix ts_constant_builtin face missing.</li>
  <ul>
</details>

### Major changes

- Many internal and structural changes (new configuration structure).
- Switch to [tree-house](https://github.com/helix-editor/tree-house). The switch boosts performance massively
  and is a building stone for upcoming features (partial updates).

### Minor changes

- Add more logs and timer-based logging.

### Patch changes

- Languages are now lazy-loaded, which means that opening Kakoune on a scratch buffer will not cause any
  grammars nor queries to load. Loading happens when a buffer is created with a recognized and accepted
  file type by KTS.
- Reimplementation of async IO decoupling, especially via triple-buffering, to prevent freezing Kakoune if
  a bug is present in a grammar, causing KTS to take an abnormal time processing the buffer, highlights,
  queries, etc.
- Text-objects are correctly sent.
- Optimizes memory usage and scheme to prevent allocating during highlighting, massively boosting
  performances.

## ktsctl v3.0.0

<details>
  <summary><b>Commit set</b></summary>
  <ul>
    <li><a href="https://git.sr.ht/~hadronized/kak-tree-sitter/commit/1f9f3c3">1f9f3c3</a> [patch/61761] Rename Default into DefaultConfig.</li>
    <li><a href="https://git.sr.ht/~hadronized/kak-tree-sitter/commit/4c565a9">4c565a9</a> ktsctl: Print default config</li>
    <li><a href="https://git.sr.ht/~hadronized/kak-tree-sitter/commit/673fd2f">673fd2f</a> ktsctl: Enhance reporting.</li>
    <li><a href="https://git.sr.ht/~hadronized/kak-tree-sitter/commit/7692f4d">7692f4d</a> Fix type (grammer -> queries).</li>
    <li><a href="https://git.sr.ht/~hadronized/kak-tree-sitter/commit/f946149">f946149</a> ktsctl: operate on many languages at once.</li>
    <li><a href="https://git.sr.ht/~hadronized/kak-tree-sitter/commit/fbedc85">fbedc85</a> [feature/27] Simplify the default config by removing optional fields.</li>
    <li><a href="https://git.sr.ht/~hadronized/kak-tree-sitter/commit/6cbd42c">6cbd42c</a> Fix grammars not being correctly sync + TSX.</li>
    <li><a href="https://git.sr.ht/~hadronized/kak-tree-sitter/commit/5d9295e">5d9295e</a> Change configuration to make grammars dissociate from languages.</li>
  </ul>
</details>

### Major changes

- Configuration has changed to make grammars dissociate from languages.

### Minor changes

- All commands acting on languages now support a list of languages instead of a single one. For instance,
  `ktsctl sync rust toml`.
- Enhance reporting and stop hiding intermediate logs.
- Add `default-config` to print to standard output the content of the default configuration.

### Patch changes

- Fix grammars not correctly synchronized.

## kak-tree-sitter-config v4.0.0

<details>
  <summary><b>Commit set</b></summary>
  <ul>
    <li><a href="https://git.sr.ht/~hadronized/kak-tree-sitter/commit/4c565a9">4c565a9</a> ktsctl: Print default config</li>
    <li><a href="https://git.sr.ht/~hadronized/kak-tree-sitter/commit/29ee234">29ee234</a> Update markdown and markdown.inline.</li>
    <li><a href="https://git.sr.ht/~hadronized/kak-tree-sitter/commit/24d80ca">24d80ca</a> Add qml language support</li>
    <li><a href="https://git.sr.ht/~hadronized/kak-tree-sitter/commit/307fc50">307fc50</a> add config for lean</li>
    <li><a href="https://git.sr.ht/~hadronized/kak-tree-sitter/commit/218762f">218762f</a> Fix configuration overrides / mandatory fields.</li>
    <li><a href="https://git.sr.ht/~hadronized/kak-tree-sitter/commit/fbedc85">fbedc85</a> [feature/27] Simplify the default config by removing optional fields.</li>
    <li><a href="https://git.sr.ht/~hadronized/kak-tree-sitter/commit/07477e7">07477e7</a> [feature/27] Make all GrammarConfig fields optional but source.</li>
    <li><a href="https://git.sr.ht/~hadronized/kak-tree-sitter/commit/38a241f">38a241f</a> Fix typst query configuration</li>
    <li><a href="https://git.sr.ht/~hadronized/kak-tree-sitter/commit/5fbc5df">5fbc5df</a> Bumped ini grammar</li>
    <li><a href="https://git.sr.ht/~hadronized/kak-tree-sitter/commit/c5c1951">c5c1951</a> Add mermaid grammar</li>
    <li><a href="https://git.sr.ht/~hadronized/kak-tree-sitter/commit/6cbd42c">6cbd42c</a> Fix grammars not being correctly sync + TSX.</li>
    <li><a href="https://git.sr.ht/~hadronized/kak-tree-sitter/commit/c8314df">c8314df</a> Pin jsonc’ queries.</li>
    <li><a href="https://git.sr.ht/~hadronized/kak-tree-sitter/commit/55a8f80">55a8f80</a> Add support for jsonc.</li>
    <li><a href="https://git.sr.ht/~hadronized/kak-tree-sitter/commit/5d9295e">5d9295e</a> Change configuration to make grammars dissociate from languages.</li>
    <li><a href="https://git.sr.ht/~hadronized/kak-tree-sitter/commit/395f229">395f229</a> Add missing capture groups for JSX.</li>
    <li><a href="https://git.sr.ht/~hadronized/kak-tree-sitter/commit/7f598af">7f598af</a> Fix Vue queries pin.</li>
    <li><a href="https://git.sr.ht/~hadronized/kak-tree-sitter/commit/84c46bf">84c46bf</a> Pin JSX and TSX to KTS ref.</li>
    <li><a href="https://git.sr.ht/~hadronized/kak-tree-sitter/commit/2e5b784">2e5b784</a> Fix JSX / TSX highlights.scm.</li>
    <li><a href="https://git.sr.ht/~hadronized/kak-tree-sitter/commit/ab3874f">ab3874f</a> Automatically generate face definition based on the config.</li>
    <li><a href="https://git.sr.ht/~hadronized/kak-tree-sitter/commit/b50e52b">b50e52b</a> changed typescript query source</li>
    <li><a href="https://git.sr.ht/~hadronized/kak-tree-sitter/commit/6b984af">6b984af</a> Add support for rust-format-args.</li>
    <li><a href="https://git.sr.ht/~hadronized/kak-tree-sitter/commit/0534e34">0534e34</a> Bump Rust queries (fix incorrect highlights).</li>
    <li><a href="https://git.sr.ht/~hadronized/kak-tree-sitter/commit/4a772dc">4a772dc</a> Update cpp grammar and point queries to commit featuring nullptr->null update.</li>
    <li><a href="https://git.sr.ht/~hadronized/kak-tree-sitter/commit/22d0353">22d0353</a> Add typst to default config</li>
  </ul>
</details>

### Major changes

- The whole configuration is now optional besides `source`, which allows users to contribute / have custom
  configuration that is much smaller.
- Kakoune face definitions are now generated automatically from the config, so we do not have to hardcode
  `set-face` declarations, which occasionally caused inconsistency.
- Change configuration to make grammars dissociate from languages, allowing reuse for languages requiring the
  same grammar.
- Small internal change to allow to access the default configuration (mostly used by `ktsctl default-config`).

### Minor changes

- **typst**` is now supported in the default config.
- Add support for `rust-format-args`, allowing to highlight inside macros such as `println!` or `format!`.
- Add support for **jsonc**.
- Add support for **mermaid**.
- Add support for **lean**.
- Add support for **qml**.

### Patch changes

- **C++** grammar has been updated as well as its queries (`nullptr -> null`).
- Fix **Rust** incorect highlights.
- Update **Typescript** query source.
- Fix **JSX** and **TSX** highlights, as well as capture groups for **JSX**.
- Fix **Vue** queries pin.
- Update **Ini**.
- Update **Markdown** configuration.
