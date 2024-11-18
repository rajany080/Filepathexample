use std::error;
use std::fs;
use std::io;
use std::path::Path;
use chrono::{DateTime, Utc};


// function {Iso format to unix timestamp}
fn iso_to_unix_timestamp(iso_string: &str) -> Result<u64, &'static str> {
    match iso_string.parse::<DateTime<Utc>>() {
        Ok(parsed_date) => Ok(parsed_date.timestamp_millis() as u64),
        Err(_) => Err("Invalid Iso timestamp format.")
    }
}

// Function to get the directory path
// also verifies if the path provided is valid or not.
fn get_directory_path() -> io::Result<String> {
    println!("Enter the path to the directory containing the files:");
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let trimmed = input.trim();

    if Path::new(trimmed).is_dir() {
        Ok(trimmed.to_string())
    } else {
        Err(io::Error::new(io::ErrorKind::NotFound, "Invalid directory path"))
    }
}

// Function to ask the user which type of file they want.
fn get_file_type() -> String {
    println!("Enter the file type to process (e.g., 'iot_rewards_share'):");
    println!("1: iot_rewards_share");
    println!("2: another_file_type"); 

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    match input.trim() {
        "1" => "iot_reward_share".to_string(),
        "2" => "another_file_type".to_string(),
        _ => {
            println!("Invalid option. Defaulting to 'iot_rewards_share'.");
            "iot_rewards_share".to_string()
        }
    }
}


fn get_timestamps() -> io::Result<(u64, u64)> {
    println!("Enter the start timestamp in ISO format (e.g., 2024-11-17T15:45:00Z):");
    let mut start_input = String::new();
    io::stdin().read_line(&mut start_input)?;
    let start = iso_to_unix_timestamp(start_input.trim())
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e))?;

    println!("Enter the end timestamp in ISO format (e.g., 2024-11-17T16:45:00Z):");
    let mut end_input = String::new();
    io::stdin().read_line(&mut end_input)?;
    let end = iso_to_unix_timestamp(end_input.trim())
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e))?;

    Ok((start, end))
}

// when the provided path is valid and the file type exists, it lists all the files.
fn list_files_in_directory(
    path: &str, 
    file_type: &str, 
    start_timestamp: u64, 
    end_timestamp: u64
) -> io::Result<()> {
    let entries = fs::read_dir(path)?; // Read directory contents

    println!("Files matching the criteria:");

    for entry in entries {
        let entry = entry?;
        let file_name = entry.file_name();
        let file_name_str = file_name.to_string_lossy();

        // Check if the file name contains the specific file type
        if file_name_str.contains(file_type) {
            // Extract the timestamp from the file name
            if let Some(timestamp_str) = file_name_str
                .split('.')
                .nth(1) // Assuming the timestamp is the second part after splitting by `.`
            {
                // Convert the timestamp to a u64 and compare it with the range
                if let Ok(timestamp) = timestamp_str.parse::<u64>() {
                    if timestamp >= start_timestamp && timestamp <= end_timestamp {
                        println!("{}", file_name_str); // Print matching file names
                    }
                }
            }
        }
    }

    Ok(())
}


fn main() {
    match get_directory_path() {
        Ok(path) => {
            println!("Processing files in directory: {}", path);

            let file_type = get_file_type();

            match get_timestamps() {
                Ok((start, end)) => {
                    println!("Processing files between timestamps: {} and {}", start, end);

                    // List files of the selected type in the directory
                    if let Err(e) = list_files_in_directory(&path, &file_type, start,end) {
                        eprintln!("Error listing files: {}", e);
                    }
                }
                Err(e) => eprintln!("Error parsing timestamps: {}", e),
            }
        }
        Err(e) => eprintln!("Error: {}", e),
    }
}
