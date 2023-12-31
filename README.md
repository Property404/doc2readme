# cargo-doc2readme

Convert crate documention into a README

Alternative to [cargo-readme](https://docs.rs/cargo-readme). Unlike `cargo-readme`,
`cargo-doc2readme` parses the output of rustdoc instead of extracting the doc comments directly
from the rust source. The main advantage here is that `cargo-doc2readme` can handle relative
links in crate documentation.

## Basic Usage

Install:

```shell
cargo install cargo-doc2readme --git https://github.com/Property404/doc2readme
```

Usage:

```shell
$ cargo doc2readme -o README.md
```

## Templating

`cargo-doc2readme` uses [minjinja](https://docs.rs/minijinja) as its
templating engine, which happens to be a superset of `cargo-readme`’s templating engine. Like
`cargo-readme`, `cargo-doc2readme` uses `README.tpl` as the template by default if it exists,
but this can be overridden with the `--template` command line option.

The default template is:

```jinja
# {{crate}}

{{readme}}
{% if license != none %}
## License

{{license}}
{% endif %}
```

### Template variables

* `crate` - the crate name, alias for `package.name`
* `license` - the crate license, alias for `package.license`
* `readme` - the generated readme text
* `version` - the crate version, alias for `package.version`
* `package` - All package keys

## Todo

* Get dependencies published
* `cargo-readme` feature parity

## License

GPL-3.0
