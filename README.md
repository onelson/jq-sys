Generated bindings for [jq](https://github.com/stedolan/jq) 1.6.

The bindings were generated with `bindgen`, which is available via
`cargo install`.

```
$ git submodule update --init
$ bindgen modules/jq/src/jq.h -o src/bindings.rs
```

The `bundled` feature (on by default) will add a dependency on [jq-src], which
will build and link to the library.

Whe disabled, it'll be up to your crate to provide and link to the library.
This may change in the future, but for now this is how it is.

> Note: when using the `bundled` feature libjq is provided by
> the [jq-src] crate, which requires `gcc`, `autoreconf`, `make`, etc in
> your `PATH` to build.

# Changelog

## 0.1.1 (2019-01-13)

Added `bundled` feature (on by default) to allow dependents to opt in or out
from using the [jq-src] crate for linkage.

## 0.1.0 (2019-01-13)

Initial release.


[jq-src]: https://crates.io/crate/jq-src
