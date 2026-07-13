#![allow(non_snake_case)]
use clap::Parser;
use log::{info, warn};
use STRdust::{Cli, call};

fn main() {
    env_logger::init();
    let args = Cli::parse();
    if args.find_outliers && !args.unphased {
        warn!("--find-outliers is only effective with --unphased");
    }
    if args.haploid.is_some() {
        warn!(
            "As of v0.20.0, genotypes on --haploid chromosomes are reported as a single allele \
             value (e.g. GT '1', or '.' when missing) per the VCF specification, instead of the \
             previous diploid representation ('1/1', './.'). Per-allele FORMAT/INFO fields (RB, \
             FRB, MRL, SUP, SC, STDEV) likewise carry a single value at these loci. Downstream \
             tools that assumed diploid genotypes may need updating."
        );
    }
    info!("Collected arguments: {args:?}");
    call::genotype_repeats(args);
}
