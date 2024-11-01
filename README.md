`xpath` is a command line utility to evaluate [XPath] expressions on XML or HTML documents. It is a small wrapper around the [libxml2] library.

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

This tool is written in Rust, so you'll need to install the Rust compiler to build it.

Once you have, clone this repository, `cd` into it, and run `cargo install --path .` to compile and install.


[XPath]: https://en.wikipedia.org/wiki/XPath
[libxml2]: https://gitlab.gnome.org/GNOME/libxml2/-/wikis/home
