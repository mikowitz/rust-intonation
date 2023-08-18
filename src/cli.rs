use crate::diamond::Diamond;
use crate::lattice::{Lattice, LatticeDimension, LatticeDimensionBounds::*};
use crate::ratio::Ratio;
use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[clap(author = "Michael Berkowitz", version)]
/// Tools for working with JI ratios, lattices, and tonality diamonds
///
/// Provides the ability to
///
/// * convert JI ratios to approximate ET intervals,
///
/// * construct and display n*n tonality diamonds,
///
/// * index into n-dimensional ratio lattices.
///
/// Run individual commands below for additional help.
struct Cli {
    #[command(subcommand)]
    cmd: SubCommand,
}

#[derive(Subcommand, Debug, Clone)]
enum SubCommand {
    /// Construct a tonality diamond from the given limits.
    ///
    /// Displays a tonality diamond (otonalities on top, utonalities
    /// on the bottom) built from the given integer limits.
    ///
    /// Ex. `rust-intonation diamond -l 1 3 5`
    ///
    /// will return a 3x3 tonality diamond of ratios that have only
    /// 1, 3 or 5 as their largest prime factor
    Diamond {
        #[clap(short = 'l', long = "limits", num_args = 1.., default_values = ["1", "5", "3"])]
        limits: Vec<u32>,
    },
    /// Create and query a JI lattice.
    ///
    /// Constructs an in-memory n-dimensional JI interval lattice from
    /// the given ratios, and returns the intervals at the given indices,
    /// provided in the form of comma-separated lists.
    ///
    /// Ex. `rust-intonation lattice -r 3/2 5/4 7/4 -i 0,0,1 1,1,1 -1,0,2`
    ///
    /// will return the intervals at index sets [0,0,1], [1,1,1], and [-1,0,2]
    /// for a 3-dimensional lattice constructed from the ratios 3/2, 5/4,
    /// and 7/4.
    ///
    /// If no indices are given, there will be no output.
    Lattice {
        #[clap(short = 'r', long = "ratios", num_args = 1.., default_values = ["3/2", "5/4"])]
        ratios: Vec<String>,
        #[clap(short = 'i', long = "indices", num_args = 0.., allow_hyphen_values = true)]
        indices: Vec<String>,
    },
    /// Find the ET approximation of JI ratios.
    ///
    /// Returns the ET approximation of the ratios passed in, defined as
    /// a pair of an ET interval name and the number of cents by which
    /// the JI ratio differs from it.
    ///
    /// Ex. `rust-intonation ratios -r 3/2`
    ///
    /// will return `(PerfectFifth, 1.954956)`, that is, the ratio 3/2 is greater
    /// than an ET perfect fifth by ~2 cents.
    Ratios {
        #[clap(short = 'r', long = "ratio", num_args = 0..)]
        ratios: Vec<String>,
    },
}

pub fn run() {
    let args = Cli::parse();
    match args.cmd {
        SubCommand::Diamond { limits } => println!("{}", Diamond::new(limits).display()),
        SubCommand::Lattice { ratios, indices } => {
            let ratios = parse_ratios(ratios);
            let indices = parse_indices(indices);

            let lattice_dimensions = ratios
                .iter()
                .map(|r| LatticeDimension::new(*r, Infinite))
                .collect::<Vec<LatticeDimension>>();

            let lattice = Lattice::new(lattice_dimensions);

            for i in indices {
                print_ratio(lattice.at(&i));
            }
        }
        SubCommand::Ratios { ratios } => {
            for ratio in parse_ratios(ratios) {
                print_ratio(ratio);
            }
        }
    }
}

fn print_ratio(ratio: Ratio) {
    println!(
        "{}\t{:?}",
        ratio,
        ratio.to_approximate_equal_tempered_interval()
    );
}

fn parse_indices(indices: Vec<String>) -> Vec<Vec<i32>> {
    indices.iter().map(|i| parse_index(i)).collect()
}

fn parse_index(s: &str) -> Vec<i32> {
    s.split(',').map(|n| n.parse().unwrap()).collect()
}

fn parse_ratios(ratios: Vec<String>) -> Vec<Ratio> {
    ratios.iter().map(|r| parse_ratio(r)).collect()
}

fn parse_ratio(s: &str) -> Ratio {
    let parts = s.split('/').collect::<Vec<&str>>();
    let numer: i32 = parts[0].parse().unwrap();
    let denom: i32 = parts[1].parse().unwrap();
    Ratio::new(numer, denom)
}
