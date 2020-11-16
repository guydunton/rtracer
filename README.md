# RTracer

[![Rust](https://github.com/guydunton/rtracer/workflows/Rust/badge.svg?branch=master)](https://github.com/guydunton/rtracer/actions)
[![GitHub](https://img.shields.io/github/license/guydunton/rtracer?color=blue)](https://opensource.org/licenses/MIT)

A ray tracer build in Rust from the excellent book "The Ray Tracer Challenge" by Jamis Buck ([Link](https://pragprog.com/titles/jbtracer/)).

The project renders a static scene like the one below. When rendering is complete the scene is saved as a `out.png` into the working directory:

<img src="images/generation.gif" alt="Results of chapter 9 in the rtracer window" width=400 />

See [below](#results-of-each-chapter) for the results at the end of each chapter.

## Build, Run & Test

The project just uses Cargo to build & run:

```bash
cargo build # Build the code in debug mode
cargo run   # Run the code
cargo test  # Run the unit tests
```

## Results of each chapter

| Chapter | Description                                    | Image                                                                   |
| ------- | ---------------------------------------------- | ----------------------------------------------------------------------- |
| 1 - 4   | Background maths for tuples, matrices & colors | N/A                                                                     |
| 5       | Flat color rendering                           | <img src="images/chapter_5.png" alt="Results of chapter 5" width=200 /> |
| 6       | Add lighting and shading                       | <img src="images/chapter_6.png" alt="Results of chapter 6" width=200 /> |
| 7       | Creating a complex scene from spheres          | <img src="images/chapter_7.png" alt="Results of chapter 7" width=200 /> |
| 8       | Adding shadows                                 | <img src="images/chapter_8.png" alt="Results of chapter 8" width=200 /> |
| 9       | Adding planes for the background               | <img src="images/chapter_9.png" alt="Results of chapter 9" width=200 /> |
