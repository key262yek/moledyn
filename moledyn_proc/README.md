# MoleDyn_proc

[![Crate](https://img.shields.io/crates/v/moledyn_proc.svg)](https://crates.io/crates/moledyn_proc)
[![API](https://docs.rs/moledyn_proc/badge.svg)](https://docs.rs/moledyn_proc)

A Procedural Macro library for [moledyn](https://docs.rs/moledyn).
It provides a macro unifying three other macros in the moledyn crate and auto-declaring variables used in a simulation.
The inner structure of macro is as following.
~~~ rust
construct_dataset!(SimulationData, 
        ContCircSystem, sys_arg, ContCircSystemArguments,
        [sys_size, f64, dim, usize ];
        ContBulkTarget, target_arg, ContBulkTargetArguments,
        [target_size, f64];
        ContPassiveLJSearcher, searcher_arg, ContPassiveLJSearcherArguments,
        [num_searcher, usize, ptl_size, f64, strength, f64];
        ExponentialStep, time_arg, ExponentialStepArguments,
        [dt_min, f64, dt_max, f64, length, usize];
        {VariableSimulation, sim_arg, VariableSimulationArguments,
        [idx_set, usize]});

setup_simulation!(args, 15, 1, TimeAnalysis, 
        "RTS_N_PTL_EXP_SEARCHER", 
        dataset, SimulationData, sys_arg, ContCircSystem, 
        target_arg, ContBulkTarget, searcher_arg, ContPassiveLJSearcher, 
        time_arg, ExponentialStep, sim_arg, VariableSimulation);

let sys_size    = sys_arg.sys_size;
let dim         = sys_arg.dim;

let _target_pos  = target_arg.target_pos.clone();
let target_size = target_arg.target_size;

let _mtype       = searcher_arg.mtype;
let _itype       = searcher_arg.itype.clone();
let ptl_size     = searcher_arg.ptl_size;
let strength     = searcher_arg.strength;
let num_searcher = searcher_arg.num_searcher;

let _dt_min          = time_arg.dt_min;
let _dt_max          = time_arg.dt_max;
let _length          = time_arg.length;

let num_ensemble= sim_arg.num_ensemble;
let idx_set     = sim_arg.idx_set;
let seed        = sim_arg.seed;
let output_dir  = sim_arg.output_dir.clone();

export_simulation_info!(dataset, output_dir, writer, WIDTH, 
        "RTS_N_PTL_EXP_SEARCHER",
        ContCircSystem, sys, sys_arg,
        ContBulkTarget, target, target_arg,
        ContPassiveLJSearcher, vec_searchers, searcher_arg,
        ExponentialStep, timeiter, time_arg,
        VariableSimulation, simulation, sim_arg);
~~~

The macro `construct_dataset` construct criteria to distinguish simulations with different configuration. 
For example, according to above source code, simulations with different system size, dimension, target size, target position, etc are considered as different configuration.
`setup_simulation` provides three features. First, it gives a guideline of input arguments for both a simulation mode and a data analysis mode.
It checks validity of input arguments, and parses arguments to corresponding variables.
If arguments is given in data analysis mode, it directly run an analysis code.




Documentation:

-   [The Rust Rand Book](https://rust-random.github.io/book)
-   [API reference (master branch)](https://rust-random.github.io/rand)
-   [API reference (docs.rs)](https://docs.rs/rand)


## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
rts_proc = "0.1.0"
```

To get started using Rand, see [The Book](https://rust-random.github.io/book).


## Versions

Rand is *mature* (suitable for general usage, with infrequent breaking releases
which minimise breakage) but not yet at 1.0. We maintain compatibility with
pinned versions of the Rust compiler (see below).

Current Rand versions are:

-   Version 0.7 was released in June 2019, moving most non-uniform distributions
    to an external crate, moving `from_entropy` to `SeedableRng`, and many small
    changes and fixes.
-   Version 0.8 was released in December 2020 with many small changes.

A detailed [changelog](CHANGELOG.md) is available for releases.

When upgrading to the next minor series (especially 0.4 → 0.5), we recommend
reading the [Upgrade Guide](https://rust-random.github.io/book/update.html).

Rand has not yet reached 1.0 implying some breaking changes may arrive in the
future ([SemVer](https://semver.org/) allows each 0.x.0 release to include
breaking changes), but is considered *mature*: breaking changes are minimised
and breaking releases are infrequent.

Rand libs have inter-dependencies and make use of the
[semver trick](https://github.com/dtolnay/semver-trick/) in order to make traits
compatible across crate versions. (This is especially important for `RngCore`
and `SeedableRng`.) A few crate releases are thus compatibility shims,
depending on the *next* lib version (e.g. `rand_core` versions `0.2.2` and
`0.3.1`). This means, for example, that `rand_core_0_4_0::SeedableRng` and
`rand_core_0_3_0::SeedableRng` are distinct, incompatible traits, which can
cause build errors. Usually, running `cargo update` is enough to fix any issues.

### Yanked versions

Some versions of Rand crates have been yanked ("unreleased"). Where this occurs,
the crate's CHANGELOG *should* be updated with a rationale, and a search on the
issue tracker with the keyword `yank` *should* uncover the motivation.

### Rust version requirements

Since version 0.8, Rand requires **Rustc version 1.36 or greater**.
Rand 0.7 requires Rustc 1.32 or greater while versions 0.5 require Rustc 1.22 or
greater, and 0.4 and 0.3 (since approx. June 2017) require Rustc version 1.15 or
greater. Subsets of the Rand code may work with older Rust versions, but this is
not supported.

Continuous Integration (CI) will always test the minimum supported Rustc version
(the MSRV). The current policy is that this can be updated in any
Rand release if required, but the change must be noted in the changelog.

## Crate Features

Rand is built with these features enabled by default:

-   `std` enables functionality dependent on the `std` lib
-   `alloc` (implied by `std`) enables functionality requiring an allocator
-   `getrandom` (implied by `std`) is an optional dependency providing the code
    behind `rngs::OsRng`
-   `std_rng` enables inclusion of `StdRng`, `thread_rng` and `random`
    (the latter two *also* require that `std` be enabled)

Optionally, the following dependencies can be enabled:

-   `log` enables logging via the `log` crate` crate

Additionally, these features configure Rand:

-   `small_rng` enables inclusion of the `SmallRng` PRNG
-   `nightly` enables some optimizations requiring nightly Rust
-   `simd_support` (experimental) enables sampling of SIMD values
    (uniformly random SIMD integers and floats), requiring nightly Rust

Note that nightly features are not stable and therefore not all library and
compiler versions will be compatible. This is especially true of Rand's
experimental `simd_support` feature.

Rand supports limited functionality in `no_std` mode (enabled via
`default-features = false`). In this case, `OsRng` and `from_entropy` are
unavailable (unless `getrandom` is enabled), large parts of `seq` are
unavailable (unless `alloc` is enabled), and `thread_rng` and `random` are
unavailable.

# License

Rand is distributed under the terms of both the MIT license and the
Apache License (Version 2.0).

See [LICENSE-APACHE](LICENSE-APACHE) and [LICENSE-MIT](LICENSE-MIT), and
[COPYRIGHT](COPYRIGHT) for details.