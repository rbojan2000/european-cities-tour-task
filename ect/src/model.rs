use clap::{Parser, ValueEnum};


/// Represents a single edge record read from the dataset file
pub struct DatasetEdge {
    pub from_city: String,
    pub from_country: String,
    pub to_city: String,
    pub to_country: String,
    pub distance: u32,
}

#[derive(ValueEnum, Clone, Debug)]
pub enum Task {
    BuildGraph,
    BuildMst,
    MeasureTime,
}

#[derive(ValueEnum, Clone, Debug)]
pub enum Algorithm {
    Serial,
    Parallel,
}

#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct Args {
    #[arg(long, value_enum)]
    pub task: Task,

    #[arg(long, value_enum)]
    pub algorithm: Algorithm,

    #[arg(long, value_delimiter = ',')] // comma separated list, optional
    pub cities: Option<Vec<String>>,
}
