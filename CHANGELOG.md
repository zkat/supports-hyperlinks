# `supports-hyperlinks` Release Changelog

<a name="2.0.0"></a>
## 2.0.0 (2023-03-13)

This semver-major release is just for switching away from `atty` due to
soundness and maintenance concerns. See [this issue in `supports-color` for
more details](https://github.com/zkat/supports-color/issues/9)

### Features

* **tty:** Switch from `atty` to `is-terminal` (#3) ([a321f355](https://github.com/zkat/supports-hyperlinks/commit/a321f35558f9dcb47d225c25e74d8c0d911bbaa8))
    * **BREAKING CHANGE**: the exported `Stream` enum is no longer an `atty` type, and this crate no longer accepts `atty` types as input.

<a name="1.2.0"></a>
## 1.2.0 (2021-09-16)

### Bug Fixes

* **conditions:** allow fallthrough when nested checks are false ([3aa5ffbb](https://github.com/zkat/supports-hyperlinks/commit/3aa5ffbba5bd1c902864f4fa4f3f9bbd0c0bcb0b))

### Features

* **force:** add support for forcing hyperlinks ([96d75a7c](https://github.com/zkat/supports-hyperlinks/commit/96d75a7ce7bac6a6fd3f7630eb0579750d4ebb82))

<a name="1.1.0"></a>
## 1.1.0 (2021-09-11)

### Features

* **on:** add atty and on() function for ease of use ([bf17421f](https://github.com/zkat/supports-hyperlinks/commit/bf17421f14791ab6308d209c5c0eda72081bd664))

<a name="1.0.0"></a>
## 1.0.0 (2021-09-11)

### Features

* **api:** initial commit ([4eb64a5b](https://github.com/zkat/supports-hyperlinks/commit/4eb64a5b67ff913ce269077e01f430d45a5aa40d))
