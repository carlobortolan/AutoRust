# AutoRust

<!--[![Crate](https://img.shields.io/crates/v/autorust.svg)](https://crates.io/crates/autorust)-->
[![Rust CI](https://github.com/carlobortolan/autorust/actions/workflows/rust_ci.yml/badge.svg?branch=master)](https://github.com/carlobortolan/autorust/actions/workflows/rust_ci.yml)
[![Rust CD](https://github.com/carlobortolan/autorust/actions/workflows/rust_cd.yml/badge.svg)](https://github.com/carlobortolan/autorust/actions/workflows/rust_cd.yml)

A simple Autograd engine written in Rust that implements backpropagation (reverse-mode autodiff) over a dynamically
built DAG and a small neural networks library with a PyTorch-like API. The DAG only operates over scalar values.
However, this is enough to build up entire deep neural nets doing binary classification, as the demo notebook shows.

> [!IMPORTANT]
> As of February 18, 2024, this project has been put on hold and will probably not be worked on or finished in the near future.

<!-- ## Installation

```bash
cargo install autorust
```
-->

## Example usage

> [!NOTE]
Check out [/demo](./demo) for more detailed examples.

## Training a neural net

// TODO

The file `demo/demo.rs` provides a full demo of training a 2-layer neural network (MLP) binary classifier. This is
achieved by initializing a neural net from `autorust::mlp` module, implementing a simple svm "max-margin" binary
classification loss and using SGD for optimization. 

| <a href="demo/demo.rs"><img src="moon_in.png" width="400px;" alt=""/> | <a href="demo/demo.rs"><img src="moon_mlp.png" width="500px;" alt=""/> |
|-----------------------------------------------------------------------|------------------------------------------------------------------------|

## Tracing / visualization

// TODO
  
![2d neuron](gout.svg)

## Running tests

Simply run:

```bash
cargo test
```

> __Additional info__: _This is inspired
by [pytorch's autograd engine](https://pytorch.org/blog/overview-of-pytorch-autograd-engine) as well
as [micrograd](https://github.com/karpathy/micrograd) and used for experimenting with Rust and the concept of autograd
engines and neural nets._

## LICENSE

This project is licensed under the MIT license. See the [LICENSE](LICENSE) file for details.

---

© Carlo Bortolan

> Carlo Bortolan &nbsp;&middot;&nbsp;
> GitHub [carlobortolan](https://github.com/carlobortolan) &nbsp;&middot;&nbsp;
> contact via [carlobortolan@gmail.com](carlobortolan@gmail.com)
