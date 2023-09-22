# Nu Plugin From Bencode

A converter plugin from the [bencode][bep_0003] format for [Nushell][nushell].

The plugin was last tested on nushell version `0.85.0`.

The plugin is unstable as the interface `Nu` for plugins is unstable.

The plugin could be useful for inspecting a [BitTorrent][bittorrent] metainfo
file (*.torrent), but it is primarily used to explore writing a `Nu` plugin.

## Installation

```sh
cargo install nu_plugin_from_bencode
```

[Register][nushell_register] the plugin:

```sh
register <path to plugin>/nu_plugin_from_bencode
```

By default, cargo installs to `$HOME/.cargo/bin` on Unix systems.

## Usage

The `from bencode` command is provided with no parameter arguments.

To use:

```sh
> open ubuntu-20.04.4-live-server-amd64.iso.torrent | from bencode
╭───────────────┬─────────────────────────────────────╮
│ announce      │ https://torrent.ubuntu.com/announce │
│ announce-list │ [list 2 items]                      │
│ comment       │ Ubuntu CD releases.ubuntu.com       │
│ created by    │ mktorrent 1.1                       │
│ creation date │ 1645734525                          │
│ info          │ {record 4 fields}                   │
╰───────────────┴─────────────────────────────────────╯
> open ubuntu-20.04.4-live-server-amd64.iso.torrent | from bencode | select announce-list.0.0
╭───────────────────┬─────────────────────────────────────╮
│ announce-list.0.0 │ https://torrent.ubuntu.com/announce │
╰───────────────────┴─────────────────────────────────────╯
> open ubuntu-20.04.4-live-server-amd64.iso.torrent | from bencode | get announce-list.1.0
https://ipv6.torrent.ubuntu.com/announce
> open ubuntu-20.04.1-live-server-amd64.iso.torrent | from bencode | select info.name info.length
╭─────────────┬──────────────────────────────────────╮
│ info.name   │ ubuntu-20.04.4-live-server-amd64.iso │
│ info.length │ 1331691520                           │
╰─────────────┴──────────────────────────────────────╯
```

## License

Licensed under either of [Apache License, Version 2.0][LICENSE_APACHE] or [MIT
License][LICENSE_MIT] at your option.

### Contributions

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

[LICENSE_APACHE]: LICENSE-APACHE
[LICENSE_MIT]: LICENSE-MIT
[bep_0003]: http://bittorrent.org/beps/bep_0003.html
[nushell]: https://www.nushell.sh/
[bittorrent]: http://bittorrent.org/
[nushell_register]: https://www.nushell.sh/book/plugins.html#adding-a-plugin
