Detects whether the current terminal supports [hyperlinks in terminal
emulators](https://gist.github.com/egmontkob/eb114294efbcd5adb1944c9f3cb5feda).

It tries to detect and support all known terminals and terminal families that
support this. If a declaration is wrong, missing, or could be improved, please
send a PR!

## Example

The API is super simple!

```rust
use supports_hyperlinks::Stream;

if supports_hyperlinks::on(Stream::Stdout) {
    println!("This terminal supports hyperlinks on stdout");
} else {
    println!("No hyperlinks, please");
}
```

And that's it!
