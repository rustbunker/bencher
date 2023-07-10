use bencher_json::{NonEmpty, ResourceId};
use clap::{Parser, Subcommand, ValueEnum};
use uuid::Uuid;

use crate::cli::{CliBackend, CliPagination};

#[derive(Subcommand, Debug)]
pub enum CliBenchmark {
    /// List benchmarks
    #[clap(alias = "ls")]
    List(CliBenchmarkList),
    /// View a benchmark
    View(CliBenchmarkView),
}

#[derive(Parser, Debug)]
pub struct CliBenchmarkList {
    /// Project slug or UUID
    #[clap(long)]
    pub project: ResourceId,

    /// Benchmark name
    #[clap(long)]
    pub name: Option<NonEmpty>,

    #[clap(flatten)]
    pub pagination: CliPagination<CliBenchmarksSort>,

    #[clap(flatten)]
    pub backend: CliBackend,
}

#[derive(ValueEnum, Debug, Clone)]
#[clap(rename_all = "snake_case")]
pub enum CliBenchmarksSort {
    /// Name of the benchmark
    Name,
}

#[derive(Parser, Debug)]
pub struct CliBenchmarkView {
    /// Project slug or UUID
    #[clap(long)]
    pub project: ResourceId,

    /// Benchmark UUID
    pub benchmark: Uuid,

    #[clap(flatten)]
    pub backend: CliBackend,
}
