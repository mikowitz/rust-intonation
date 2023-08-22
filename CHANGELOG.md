# rust_intonation

## Unreleased

* Add `Play` trait and implement using `rodio` for `Ratio` and `EqualTemperedInterval`
* Add `play` and `compare` CLI subcommands

## v0.2.0 (August 22, 2023)

* Make structs generic against `num::PrimInt` types
* Default to using `i32` and `f64` numeric types

## v0.1.0 (August 21, 2023)

* Initial release, with support for ratios, tonality diamonds, and n-dimensional lattices.
