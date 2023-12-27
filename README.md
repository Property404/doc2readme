# cargo-doc2readme

Convert crate documention into a README

Alternative to [cargo-readme](https://docs.rs/cargo-readme). Unlike `cargo-readme`,
`cargo-doc2readme` parses the output of rustdoc instead of extracting the doc comments directly
from the rust source. The main advantage here is that `cargo-doc2readme` can handle relative
links in crate documentation.

## [Basic Usage](#basic-usage)

Install:

```
cargo install --git https://github.com/Property404/doc2readme
```

Usage:

```
$ cargo doc2readme -o README.md
```

## [Templating](#templating)

`cargo-doc2readme` usages [minjinja](https://docs.rs/minijinja) as its
templating engine, which happens to be a superset of `cargo-readme`’s templating engine. Like
`cargo-readme`, `cargo-doc2readme` uses `README.tpl` as the template by default if it exists.

The default template is:

```
# {{crate}}

{{readme}}
{% if license != none %}
## License

{{license}}
{% endif %}
```

### [Template variables](#template-variables)

* crate - the crate name
* license - the crate license
* readme - the generated readme text
* version - the crate version

## [Todo](#todo)

* Add back language info to codeblocks? (Not sure if possible)
* Better license text? With contributing section?
* Refactor to make unit testable


## License

GPL-3.0
