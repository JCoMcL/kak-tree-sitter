# kak-tree-sitter

<!--toc:start-->
- [kak-tree-sitter](#kak-tree-sitter)
  - [Features](#features)
  - [Wiki](#wiki)
  - [Contributing](#contributing)
  - [Chat](#chat)
  - [Credits](#credits)
<!--toc:end-->

This is a binary server that interfaces [tree-sitter](https://tree-sitter.github.io/) with
[kakoune](https://kakoune.org/).

> Important note: by default, no colorscheme supporting tree-sitter is set for you. You have to pick one or write your
> own. See [this section from the man](./docs/man/highlighting.md#Highlighting) for further information.

[![asciicast](https://asciinema.org/a/606062.svg)](https://asciinema.org/a/606062)

## Features

- [x] Semantic highlighting.
  - Automatically detects whether a buffer language type can be highlighted.
  - Removes any default highlighter and replaces them with a tree-sitter based.
- [x] Semantic selections (types, functions, parameters, comments, tests, etc.)
  - Similar features to `f`, `?`, `<a-/>`, etc.
  - Full _object_ mode support (i.e. `<a-i>`, `{`, `<a-]>`, etc.)
- [ ] Indents
- [ ] Incremental parsing
- [x] Fetch, compile and install grammars / queries with ease (via the use of the `ktsctl` controller companion)
- [x] Ships with no mappings, defined options, but allows to use well-crafted values, user-modes, mappings and
  commands by picking them by hand.
- [x] Transformation-oriented; actual data (i.e. grammars, queries, etc.) can be used from any sources.
- [x] Shell completions.

## Wiki

See the [Wiki](https://man.sr.ht/~hadronized/kak-tree-sitter) to know how to install, use, configure and get
runtime resources.

## Contributing

### Submit an issue / feature request

If you have a SourceHut account, feel free to open an issue on one of the [trackers](https://sr.ht/~hadronized/kak-tree-sitter/trackers).

If you do not own a SourceHut account, feel free to send an email in on the [discuss mailing list](https://lists.sr.ht/~hadronized/kak-tree-sitter-discuss).
A contributor will create a ticket and send you a link you can subscribe your email to to follow progress.

[Submit an issue through email](~hadronized/kak-tree-sitter-discuss@lists.sr.ht)

### Contributing patches

Have a look at [CONTRIBUTING.md](./CONTRIBUTING.md).

## Chat

Feel free to join `#kakoune` on the [libera.chat](https://libera.chat/) IRC network, or the
[Kakoune Community discord serve](https://discord.gg/yjv9CC5X).

## Credits

This program was inspired by:

- [Helix](https://helix-editor.com)
- [kak-tree](https://github.com/ul/kak-tree)
