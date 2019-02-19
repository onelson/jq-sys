Generated bindings for [jq] 1.6.

The bindings were generated with `bindgen`, which is available via
`cargo install`.

Example:

```
$ git submodule update --init
$ bindgen modules/jq/src/jq.h -o src/bindings.rs
```

## Building

### Using the bundled feature

When the `bundled` feature is enabled it will add a dependency on [jq-src], which
will attempt to build and link to the library. This may cover simple cases, but for
anything exotic (such as cross-building), you will want to install or build `libjq` 
yourself.

> Note: when using the `bundled` feature libjq is provided by
> the [jq-src] crate, which requires `gcc`, `autoreconf`, `make`, etc in
> your `PATH` to build.

### Without the bundled feature

When *not using* the `bundled` feature, you'd either have to compile `libjq` yourself, or use libs furnished by your
system package manager.

For example on debian systems, you might install `libjq1 libjq-dev libonig4 libonig-dev`. 

The following env vars can be used to provide hints to the build script.

| Name | Purpose | Notes |
| --- | --- | --- |
| `JQ_LIB_DIR` | Path to the location of the library. ||
| `JQ_LIB_STATIC` | Use static linking instead of shared. ||
| `JQ_NO_ONIG` | Disable linking to `oniguruma` for regex support. ||
| `ONIG_LIB_DIR` | Path to the location of the library. | Defaults to `JQ_LIB_DIR`, ignored if `JQ_NO_ONIG` is set. |
| `ONIG_LIB_STATIC` | Use static linking instead of shared. | Ignored if `JQ_NO_ONIG` is set. |

# Changelog

## 0.2.0 (2019-02-18)

Additions:

- New env vars added to allow linkage to `libjq` and `libonig` to be customized.
- Adds `pkg-config` feature (on by default) to help configure linkage, as a fallback when env vars are not set.

Breaking changes:

 - `bundled` feature is no longer enabled by default.

## 0.1.1 (2019-01-13)

Added `bundled` feature (on by default) to allow dependents to opt in or out
from using the [jq-src] crate for linkage.

## 0.1.0 (2019-01-13)

Initial release.


[jq-src]: https://crates.io/crate/jq-src
[jq]: https://github.com/stedolan/jq
