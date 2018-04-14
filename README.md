# Minimal testing fixtures for Rust

The built in testing framework of Rust is lightweight, one could even say that
it is bare-bones. Oftentimes, some kind of fixtures ‒ *things* that are similar
through many tests ‒ come handy, to avoid code duplication.

There's a poor-man's implementation of a fixture:

```rust
struct Fixture {
    // Whatever, some values go in here...
    value: u32,
}

impl Default for Fixture {
    fn default() -> Self {
        // Any test setup (preparing a database, creating test files, etc) might
        // go in here.
        Self {
            value: 42,
        }
    }
}

impl Drop for Fixture {
    fn drop(&mut self) {
        // Any test tear down (shutting down the database) might go in here.
    }
}

#[test]
fn perform_test() {
    let fixture = Fixture::default();

    assert_eq!(42, fixture.value);
}
```

While this works, it's far from elegant. This crate allows using the fixture in
a bit more convenient way:

```rust
#[minimal_fixture] // Instead of #[test]
fn perform_another_test(fixture: &Fixture) {
    assert_eq!(42, fixture.value);
}
```

As additional feature, it is possible to create a fixture that supplies multiple
values (see the docs). The test is run with each such value.

It is possible to supply multiple fixtures, and the test is run with each
combination of the values they provide.

## Status

This is still in an early experimental state. It is likely it won't work in many
cases. Also, error handling is somewhat minimal and if it fails to compile, the
error messages aren't very helpful.

It exists mostly to answer two questions:
* Is it actually useful?
* If so, is a separate (proc-macro) crate the way to go, or would writing an RFC
  and trying to include it into the in-built testing framework. There are some
  features that don't seem possible with proc-macro only.

For these reasons (and because I'm not happy with the name), it is not yet
published on crates.io.

## Nightly features

For now, proc-macro is a nightly only feature, so to use it, you need to enable
it:

```rust
#![feature(proc_macro)]
extern crate minimal_fixtures;
```

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms
or conditions.
