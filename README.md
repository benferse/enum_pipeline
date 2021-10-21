# enum_pipline

Provides a way to use enums to describe and execute ordered data pipelines. 🦀🐾

[![CI](https://github.com/bengreenier/enum_pipeline/actions/workflows/rust.yml/badge.svg)](https://github.com/bengreenier/enum_pipeline/actions/workflows/rust.yml)
[![Crates.io](https://img.shields.io/crates/d/enum_pipeline)](https://crates.io/crates/enum_pipeline)
[![docs.rs](https://img.shields.io/docsrs/enum_pipeline)](https://docs.rs/enum_pipeline)

I needed a succinct way to describe 2d pixel map operations for a game I'm working on. I wanted callers to be able to easily determine all possible operations (hence `enum`), with per-operation data (hence variants), and their operation-specific logic (proc-macro coming soon). This is what I came up with!

```rs
use enum_pipeline::{
    Execute, IntoPipelineVec
};

enum Operations {
    Allocate(f32, f32),
    Init,
    Run(f32)
}

impl Execute for Operations {
    fn execute(self) {
        match self {
            Operations::Allocate(x, y) => println!("allocate something"),
            Operations::Init => println!("init"),
            Operations::Run(delta) => println!("do work")
        }
    }
}

fn do_work() {
    let my_op_pipeline = vec![
        Operations::Init,
        Operations::Allocate(1.0, 1.0),
        Operations::Init,
        Operations::Run(1.0),
    ]
    .into_pipeline();

    my_op_pipeline.execute();
    // prints:
    // init
    // allocate something
    // init
    // do work
}
```

There are variants for pipelines with global data as well (passed as an argument to `execute`), and I'm working on a proc-macro that can generate the boilerplate `match` logic, shelling out to different user provided functions for each operation.

## TODO

- [ ] finish the proc-macro stuff
- [ ] document the proc-macro
- [ ] add example directory

## License

MIT
