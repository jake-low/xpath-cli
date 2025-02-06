# xpath-cli

This is a command line tool called `xpath` which evaluates [XPath] expressions on XML or HTML documents. It is a small wrapper around the excellent [libxml2] library.

## Example

Use it for HTML

```
$ curl -L https://en.wikipedia.org/wiki/Special:Random | xpath '//h1/span/text()'
Vampire bat
```

Or XML

```
$ curl -L https://osm.org/api/0.6/changeset/157745397 | xpath '//changeset/tag[@k = "comment"]/@v'
This Starbucks was torn down, seemingly to have extra space to put Halloween candy on display.
```

## Installation

This tool is written in Rust, so you'll need to install the Rust compiler and toolchain to build it.

Once you have, you can install this tool by running `cargo install xpath-cli`

Alternately, you can clone this repository, `cd` into it, and run `cargo install --path .`

## License

The code in this repository is offered under the ISC License. See the [LICENSE](./LICENSE) file for details.

[XPath]: https://en.wikipedia.org/wiki/XPath
[libxml2]: https://gitlab.gnome.org/GNOME/libxml2/-/wikis/home
