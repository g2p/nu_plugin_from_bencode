# Nu Plugin From Bencode

A converter plugin from the bencode format for [Nushell][nushell].

The plugin was last tested on nushell version `0.20.0`.

## Installation

Clone the repository. Change the working directory to the cloned directory.

```
cargo install --path .
```

## Usage

The `from_bencode` command is provided with no parameter arguments.

To use:

```
> open ubuntu-20.04.1-live-server-amd64.iso.torrent | from_bencode
───┬─────────────────────────────────────┬────────────────┬───────────────────────────────┬───────────────┬───────────────┬───────────────────────────────────────
 # │              announce               │ announce-list  │            comment            │  created by   │ creation date │                 info                  
───┼─────────────────────────────────────┼────────────────┼───────────────────────────────┼───────────────┼───────────────┼───────────────────────────────────────
 0 │ https://torrent.ubuntu.com/announce │ [table 2 rows] │ Ubuntu CD releases.ubuntu.com │ mktorrent 1.1 │    1596727698 │ [row length name piece length pieces] 
───┴─────────────────────────────────────┴────────────────┴───────────────────────────────┴───────────────┴───────────────┴───────────────────────────────────────
> open ubuntu-20.04.1-live-server-amd64.iso.torrent | from_bencode | select announce-list
───┬────────────────
 # │ announce-list  
───┼────────────────
 0 │ [table 1 rows] 
 1 │ [table 1 rows] 
───┴────────────────
> open ubuntu-20.04.1-live-server-amd64.iso.torrent | from_bencode | select announce-list.0
───┬─────────────────────────────────────
 # │           announce-list.0           
───┼─────────────────────────────────────
 0 │ https://torrent.ubuntu.com/announce 
───┴─────────────────────────────────────
> open ubuntu-20.04.1-live-server-amd64.iso.torrent | from_bencode | select announce-list.1
───┬──────────────────────────────────────────
 # │             announce-list.1              
───┼──────────────────────────────────────────
 0 │ https://ipv6.torrent.ubuntu.com/announce 
───┴──────────────────────────────────────────
> open ubuntu-20.04.1-live-server-amd64.iso.torrent | from_bencode | get announce-list.1
https://ipv6.torrent.ubuntu.com/announce
> open ubuntu-20.04.1-live-server-amd64.iso.torrent | from_bencode | get info
───┬───────────┬──────────────────────────────────────┬──────────────┬───────────────────────
 # │  length   │                 name                 │ piece length │        pieces         
───┼───────────┼──────────────────────────────────────┼──────────────┼───────────────────────
 0 │ 958398464 │ ubuntu-20.04.1-live-server-amd64.iso │       262144 │ <binary: 73120 bytes> 
───┴───────────┴──────────────────────────────────────┴──────────────┴───────────────────────
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
[nushell]: https://www.nushell.sh/
