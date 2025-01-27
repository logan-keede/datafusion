// Licensed to the Apache Software Foundation (ASF) under one
// or more contributor license agreements.  See the NOTICE file
// distributed with this work for additional information
// regarding copyright ownership.  The ASF licenses this file
// to you under the Apache License, Version 2.0 (the
// "License"); you may not use this file except in compliance
// with the License.  You may obtain a copy of the License at
//
//   http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing,
// software distributed under the License is distributed on an
// "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied.  See the License for the
// specific language governing permissions and limitations
// under the License.

use sqllogictest::parser::parse_file;
use sqllogictest::DefaultColumnType;
use sqllogictest::Record;
use sqllogictest::Record::{Query, Statement};
use std::error::Error;
use std::fs;
use std::path::Path;

// run inside sqllogictest or run with -p datafusion-sqllogictest
// cargo run --bin diff -- path1.slt path2(.slt or folder)
pub fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = std::env::args().collect();

    // Check if we have exactly two file paths as arguments
    if args.len() < 3 {
        return Err("Please provide two arguments: a file path or folder, and a second file or folder.".into());
    }

    let path1 = Path::new(&args[1]);
    let path2 = Path::new(&args[2]);

    let result = diff(path1, path2);
    assert!(result.is_ok(), "Expected no error, but got {:?}", result);
    Ok(())
}

pub fn diff(path1: &Path, path2: &Path) -> Result<(), Box<dyn Error>> {
    println!("Needles: {:?}", path1);
    println!("Haystack: {:?}", path2);

    // Parse the records from the first file
    let records1 = parse_file(path1)?;

    // let records2;

    let records2 = if path2.is_dir() {
        // If the second path is a directory, parse all files in that directory
        parse_files_in_directory(path2)?
    } else {
        // If the second path is a file, just parse that file
        parse_file(path2)?
    };
    let mut errors = Vec::new();

    // Check if each record in file 1 is contained in any record in file 2
    for record1 in &records1 {
        // let found;
        let found = if !check_type(record1) {
            false
        } else {
            records2
                .iter()
                .any(|record2| check_equality(record1, record2))
        };
        if check_equality(record1, record1) && !found {
            errors.push(format!(
                "Record from Needles not found in Haystack: {:?}",
                get_sql(record1)
            ));
        }
    }
    // If we have collected any errors, return them all at once
    if !errors.is_empty() {
        return Err(errors.join("\n").into());
    }
    println!("All records from Needles are present in Haystack.");
    Ok(())
}

fn get_sql(record: &Record<DefaultColumnType>) -> String {
    match record {
        Query { sql, .. } => sql.clone(),
        Statement { sql, .. } => sql.clone(),
        _ => String::new(),
    }
}
pub fn check_type(record1: &Record<DefaultColumnType>) -> bool {
    // the type which is acceptable by check_equality(Query and Statement)
    check_equality(record1, record1)
}

pub fn check_equality(
    record1: &Record<DefaultColumnType>,
    record2: &Record<DefaultColumnType>,
) -> bool {
    match (record1, record2) {
        (
            Query {
                loc: _,
                conditions: _,
                connection: _,
                sql: sql1,
                expected: expected1,
                retry: _,
            },
            Query {
                loc: _,
                conditions: _,
                connection: _,
                sql: sql2,
                expected: expected2,
                retry: _,
            },
        ) => sql1 == sql2 && expected1 == expected2,
        (
            Statement {
                loc: _,
                conditions: _,
                connection: _,
                sql: sql1,
                expected: expected1,
                retry: _,
            },
            Statement {
                loc: _,
                conditions: _,
                connection: _,
                sql: sql2,
                expected: expected2,
                retry: _,
            },
        ) => sql1 == sql2 && expected1 == expected2,
        _ => false,
    }
}

// Warning: This is not recursive, can be made recursive in future if needed.
fn parse_files_in_directory(
    directory: &Path,
) -> Result<Vec<Record<DefaultColumnType>>, Box<dyn Error>> {
    let mut all_records = Vec::new();

    // Read all files in the directory
    for entry in fs::read_dir(directory)? {
        let entry = entry?;
        let path = entry.path();

        // Only process files (not directories)
        if path.is_file() && path.extension().map(|ext| ext == "slt").unwrap_or(false) {
            let records = parse_file(&path)?;
            all_records.extend(records); // Add the records from this file
        }
    }

    Ok(all_records)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_files_aggregate_diff_archived() {
        // Create two test files
        let path1 = Path::new("./archive/complete_aggregate.slt");
        let path2 = Path::new("./test_files/aggregate");
        let result = diff(path1, path2);
        if result.is_err() {
            panic!("Expected no error, but got {:?}", result.err());
        }
    }
    #[test]
    fn test_files_aggregate_diff_base() {
        // Create two test files
        let path1 = Path::new("./test_files/aggregate/base_aggregate.slt");
        let path2 = Path::new("./archive/complete_aggregate.slt");
        let result = diff(path1, path2);
        if result.is_err() {
            panic!("Expected no error, but got {:?}", result.err());
        }
    }
}
