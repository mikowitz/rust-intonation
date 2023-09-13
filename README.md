# rust-intonation

A Rustlang library for working with just intonation ratios, lattices, and tonality diamonds.

It also provides a command line tool, also named `rust-intonation` for working with these
same structs from the command line.

## Library

### Ratio

Ratios can be created from a pair of integers

```rust
use rust_intonation::ratio::Ratio;

let r1 = Ratio::new(3, 2);
let r2 = Ratio::new(5, 4);
```

Creating a new ratio will normalize it to the range `[1, 2)`

```rust
Ratio::new(1, 2); // Ratio::new(1, 1)
Ratio::new(25, 7); // Ratio::new(25, 14)
```

They can be multiplied and divided with the expected results

```rust
r1 * r2; // Ratio::new(15, 8);

r1 / r2; // Ratio::new(6, 5)
r2 / r1; // Ratio::new(5, 6)
```

They can be raised to an integral power (positive or negative)

```rust
Ratio::new(3, 2).pow(0); // Ratio::new(1, 1)
Ratio::new(3, 2).pow(2); // Ratio::new(9, 8)
Ratio::new(3, 2).pow(-2); // Ratio::new(16, 9)
```

Their complement can be calculated (given a ratio `r`, its complement
is the ratio `s` such that `r * s` forms a perfect octave)

```rust
Ratio::new(3, 2).complement(); // Ratio::new(4, 3)
Ratio::new(10, 9).complement(); // Ratio::new(9, 5)
```

An equal temperament approximation of a JI ratio can be calculated,
as well as the difference in cents between the JI ratio and the ET
interval.

```rust
Ratio::new(3, 2).to_approximate_equal_tempered_interval(); // (PerfectFifth, 1.954956)
```

The highest prime limit of the ratio can be calculated

```rust
Ratio::new(3, 2).limit(); // 3
Ratio::new(5, 4).limit(); // 5
Ratio::new(8, 5).limit(); // 5
```

## Tonality Diamond

A tonality diamond can be constructed from a vector of integer limits

```rust
use rust_intonation::diamond::Diamond;

let diamond = Diamond::new(vec![1, 5, 3]);
```

The diamond can then be printed out with otonalities on the top, and
utonalities on the bottom:

```rust
println!("{}", diamond.display());
```
```
                3/2

        5/4             6/5

1/1             1/1             1/1

        8/5             5/3

                4/3
```

## Lattices

You can construct an n-dimensional JI ratio lattice from a
vector of `LatticeDimension` structs.

### LatticeDimension

A `LatticeDimension` defines a ratio by which the dimension is calculated,
as well as a bounding rule for how the dimension is indexed into.

The possible bounding rules are:

* `Infinite` - the lattice extends infinitely in both directions. Indexing into the lattice will return the value at the index given.
* `LengthBounded(n)` - the lattice extends through the range `[0, n)`. Indexing into the lattice at `n` will return the value at 0. If `n` is negative, indexing at 1 will loop around and return the value at `n+1` (e.g. if `n` is -2, indexing at 1 will return the value in the lattice at index -1)
* `RangeBounded(a, b)` - the lattice extends through the range `[a, b]`. Indexing into the lattice at `b+1` will return the value at index `a`, and indexing into the lattice at `a-'` will return the value at `b`.

### Constructing a Lattice

```rust
use rust_intonation::{
    lattice::{Lattice, LatticeDimensions, LatticeDimensionBounds},
    ratio::Ratio
};

let lattice = Lattice::new(
    vec![
        LatticeDimension::new(
            Ratio::new(3, 2),
            LatticeDimensionBounds::Infinite,
        ),
        LatticeDimension::new(
            Ratio::new(5, 4),
            LatticeDimensionBounds::Infinite,
        ),
        LatticeDimension::new(
            Ratio::new(7, 4),
            LatticeDimensionBounds::Infinite,
        ),
    ]
);
```

You can then index into the lattice to return the ratio at the given coordinates:

```rust
lattice.at([0, 0, 0]); // Ratio::new(1, 1)
lattice.at([1, 0, 0]); // Ratio::new(3, 2)
lattice.at([1, 1, 0]); // Ratio::new(15, 8)
lattice.at([1, 1, 1]); // Ratio::new(105, 32)
lattice.at([-1, -1,- 1]); // Ratio::new(256, 105)
```

**NB** By default, `rust-intonation` uses 32-bit integers, so with a large enough lattice
and high enough indices, it *is* possible to encounter integer overflow.
However, since the largest possible 32-bit integer is `2,147,483,647`, this limit
should be sufficient for all but the most extreme cases.

#### Avoiding integer overflow

If you need to be able to work with larger ratio components, it is possible to construct
a `Lattice` using 64-bit integers by explicitly instantiating the Lattice with `<T = i64>`

```rust
use rust_intonation::{
    lattice::{Lattice, LatticeDimensions, LatticeDimensionBounds},
    ratio::Ratio
};

let lattice: Lattice<i64> = Lattice::new(
    vec![
        LatticeDimension::new(
            Ratio::new(3, 2),
            LatticeDimensionBounds::Infinite,
        ),
        LatticeDimension::new(
            Ratio::new(5, 4),
            LatticeDimensionBounds::Infinite,
        ),
        LatticeDimension::new(
            Ratio::new(7, 4),
            LatticeDimensionBounds::Infinite,
        ),
    ]
);
```

## CLI

The CLI tool provides a way to interact with the library in an environment
where `rustc` and `cargo` might not be available, or creating a Rust library for only some quick calculations may prove overly cumbersome.

The CLI tool provides three commands:

* `ratios`
* `diamond`
* `lattice`

### ratios

This command allows you to pass in any number of just intonation ratios
in `n/d` format, and it will print out the nearest equal temperament
approximations of those ratios, along with the difference in cents between
the JI ratio and the ET interval

```bash
$ rust-intonation ratios -r 3/2 5/4
3/2     (PerfectFifth, 1.954956)
5/4     (MajorThird, -13.68631)
```

### diamond

This command allows you to pass in any number of interval limits and
will print out a tonality diamond (otonalities on top, utonalities
on the bottom) constructed from those limits.

```bash
$ rust-intonation diamond -l 1 5 3
                3/2

        5/4             6/5

1/1             1/1             1/1

        8/5             5/3

                4/3
```

### lattice

This command allows to define the dimensions for an n-dimensional JI lattice,
as well as a set of indices into that lattice. It will then return the list
of JI ratios at those indices, along with their ET approximations.

```bash
$ rust-intonation lattice --ratios 3/2 5/4 7/4 --indices 1,1,1 2,2,2 -1,0,1
105/32  (MajorSixth, -42.905396)
11025/4096      (PerfectFourth, 14.189209)
7/3     (MinorThird, -33.12915)
```

**NB** that each n-dimensional index coordinate set is comma-separated, but
the different coordinates are separated by spaces.
