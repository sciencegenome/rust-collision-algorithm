use clap::Parser;

#[derive(Debug, Parser)]
#[clap(version)]

pub struct GraphOffsetArgs {
    /// please provide the path to the fastq file
    pub fastqfile: String,
    /// please provide the kmer lookup table construction
    pub offsetsize: usize,
}
