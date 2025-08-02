use crate::model::DatasetEdge;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

/// Reads dataset file and returns Vec of DatasetEdge
pub fn read_dataset_file<P: AsRef<Path>>(path: P) -> io::Result<Vec<DatasetEdge>> {
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);

    let mut edges = Vec::new();

    for line_result in reader.lines() {
        let line = line_result?;
        let parts: Vec<&str> = line.split(',').map(|s| s.trim()).collect();

        if parts.len() != 5 {
            eprintln!("Skipping malformed line: {}", line);
            continue;
        }

        let distance = match parts[4].parse::<u32>() {
            Ok(d) => d,
            Err(_) => {
                eprintln!("Invalid distance on line: {}", line);
                continue;
            }
        };

        edges.push(DatasetEdge {
            from_city: parts[0].to_string(),
            from_country: parts[1].to_string(),
            to_city: parts[2].to_string(),
            to_country: parts[3].to_string(),
            distance,
        });
    }

    Ok(edges)
}
