Detects whether the current terminal supports [hyperlinks in terminal
emulators](https://gist.github.com/egmontkob/eb114294efbcd5adb1944c9f3cb5feda).

It tries to detect and support all known terminals and terminal families that
support this. If a declaration is wrong, missing, or could be improved, please
send a PR!

NB: This crate does NOT detect whether the current environment is a TTY and
does all its work based on environment variables. You will have to use your
own TTY-detection code (or use something like
[atty](https://crates.io/crates/atty)) to make this detection more robust.

## Example

The API is super simple!

```rust
if supports_hyperlinks::supports_hyperlinks() {
    println!("This terminal supports hyperlinks");
} else {
    println!("This terminal does not support hyperlinks");
}
```

And that's it!
