#![allow(non_snake_case)]

use clap::{Parser, ValueEnum};
use std::path::PathBuf;

pub mod bam_pool;
pub mod batching;
pub mod call;
pub mod consensus;
pub mod dbscan;
pub mod features;
pub mod genotype;
pub mod motif;
pub mod parse_bam;
pub mod phase_insertions;
pub mod repeats;
pub mod utils;
pub mod vcf;

/// Strategy for splitting unphased reads into haplotypes.
#[derive(ValueEnum, Clone, Copy, Debug, PartialEq, Eq)]
pub enum PhasingStrategy {
	/// Length-weighted Levenshtein distance + Ward hierarchical clustering.
	Ward,
	/// k-mer composition feature vectors + DBSCAN.
	Dbscan,
	/// QC mode: report the Ward call but additionally run DBSCAN and flag substantial
	/// length discordance (DISCORDANT_LENGTH / DBSCAN_RB) for review.
	Both,
}

/// The arguments end up in the Cli struct.
#[derive(Parser, Debug)]
#[command(author, version, about = "Tool to genotype STRs from long reads", long_about = None)]
pub struct Cli {
	/// reference genome
	#[arg(value_parser = is_file)]
	pub fasta: String,

	/// BAM or CRAM file to call STRs in
	#[arg(value_parser = is_file)]
	pub bam: String,

	/// Region string to genotype expansion in (format: chr:start-end)
	#[arg(short, long)]
	pub region: Option<String>,

	/// Bed file with region(s) to genotype expansion(s) in
	#[arg(short = 'R', long, value_parser = is_file)]
	pub region_file: Option<String>,

	/// Genotype the pathogenic STRs from STRchive
	#[arg(long, default_value_t = false)]
	pub pathogenic: bool,

	/// minimal length of insertion/deletion operation
	#[arg(short, long, default_value_t = 1)]
	pub minlen: usize,

	/// minimal number of supporting reads per haplotype
	#[arg(short, long, default_value_t = 3)]
	pub support: usize,

	/// Number of parallel threads to use
	#[arg(short, long, default_value_t = 1)]
	pub threads: usize,

	/// Sample name to use in VCF header, if not provided, the bam file name is used
	#[arg(long)]
	pub sample: Option<String>,

	/// Print information on somatic variability
	#[arg(long, default_value_t = false)]
	pub somatic: bool,

	/// Reads are not phased
	#[arg(long, default_value_t = false)]
	pub unphased: bool,

	/// Identify poorly supported outlier expansions (only with --unphased)
	#[arg(long, default_value_t = false)]
	pub find_outliers: bool,

	/// Minimum fraction of reads required for a cluster to be considered a haplotype (only with --unphased)
	#[arg(long, default_value_t = 0.1)]
	pub min_haplotype_fraction: f32,

	/// Strategy for splitting unphased reads into haplotypes (only with --unphased).
	/// 'ward': length-weighted Levenshtein + hierarchical clustering (default).
	/// 'dbscan': k-mer composition feature vectors + DBSCAN (experimental, robust to length-variable expansions).
	/// 'both': QC mode that reports the Ward call but also runs DBSCAN and flags substantial
	/// length discordance (DISCORDANT_LENGTH / DBSCAN_RB) for review.
	#[arg(long = "phasing", value_name = "STRATEGY", value_enum, default_value_t = PhasingStrategy::Ward)]
	pub phasing_strategy: PhasingStrategy,

	/// comma-separated list of haploid (sex) chromosomes
	#[arg(long)]
	pub haploid: Option<String>,

	/// Debug mode
	#[arg(long, default_value_t = false)]
	pub debug: bool,

	/// Sort output by chrom, start and end
	#[arg(long, default_value_t = false)]
	pub sorted: bool,

	/// Max number of reads to use to generate consensus alt sequence
	#[arg(long, default_value_t = 20)]
	pub consensus_reads: usize,

	/// Max number of reads to extract per locus from the bam file for genotyping (use -1 for all reads)
	#[arg(long, default_value_t = 60, allow_hyphen_values = true)]
	pub max_number_reads: isize,

	/// Maximum locus size to consider (intervals larger than this will be filtered out)
	#[arg(long)]
	pub max_locus: Option<u32>,

	/// Always use full alignment (disable fast reference check via CIGAR)
	#[arg(long, default_value_t = false)]
	pub alignment_all: bool,
}

fn is_file(pathname: &str) -> Result<String, String> {
	if pathname.starts_with("http")
		|| pathname.starts_with("https://")
		|| pathname.starts_with("s3")
	{
		return Ok(pathname.to_string());
	}

	let path = PathBuf::from(pathname);
	if path.is_file() {
		Ok(pathname.to_string())
	} else {
		Err(format!("Input file {} is invalid", path.display()))
	}
}

#[cfg(test)]
#[ctor::ctor(unsafe)]
fn init() {
	let _ = env_logger::builder().is_test(true).try_init();
}

#[test]
fn verify_app() {
	use clap::CommandFactory;
	Cli::command().debug_assert()
}
