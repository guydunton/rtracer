# RTracer

[![Rust](https://github.com/guydunton/rtracer/workflows/Rust/badge.svg?branch=master)](https://github.com/guydunton/rtracer/actions)
[![GitHub](https://img.shields.io/github/license/guydunton/rtracer?color=blue)](https://opensource.org/licenses/MIT)

A ray tracer build in Rust from the excellent book "The Ray Tracer Challenge" by Jamis Buck ([Link](https://pragprog.com/titles/jbtracer/)).

Currently the project will output a static image (out.png) when ran like the one below.

![The result of chapter 6](images/chapter_6.png)

## Build, Run & Test

The project just uses Cargo to build & run:

```bash
cargo build
cargo run
cargo test
```

## Tests

The project has built in tests using my testing framework [rust-catch](https://github.com/guydunton/rust-catch).
