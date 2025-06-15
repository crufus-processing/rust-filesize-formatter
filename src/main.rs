use std::env;
use std::fmt;

/// Constants for file size conversions.
const KB: f64 = 1000.0;
const MB: f64 = 1_000_000.0;
const GB: f64 = 1_000_000_000.0;

/// A struct to hold a single file size represented in different units.
struct Sizes {
    bytes: String,
    kilobytes: String,
    megabytes: String,
    gigabytes: String,
}

/// Implement methods for the Sizes struct to convert from bytes to other units.
impl Sizes {
    // Create a new Sizes instance from a given size in bytes.
    fn from_bytes(size_in_bytes: u64) -> Self {
        Self {
            bytes: format!("{} bytes", size_in_bytes),
            kilobytes: format!("{:.2} kb", size_in_bytes as f64 / KB),
            megabytes: format!("{:.2} mb", size_in_bytes as f64 / MB),
            gigabytes: format!("{:.2} gb", size_in_bytes as f64 / GB),
        }
    }
}

/// Implement the Display trait for Sizes to format the output.
impl fmt::Display for Sizes {
    // Format the Sizes struct for display.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "   bytes: {}", self.bytes)?;
        writeln!(f, "   kilobytes: {}", self.kilobytes)?;
        writeln!(f, "   megabytes: {}", self.megabytes)?;
        writeln!(f, "   gigabytes: {}", self.gigabytes)?;
        Ok(())
    }
}

/// Enum to represent file sizes in different units.
enum FileSize {
    Bytes(u64),
    Kilobytes(f64),
    Megabytes(f64),
    Gigabytes(f64),
}

/// Implement methods for the FileSize enum to create instances and normalize sizes.
impl FileSize {
    // Create a new FileSize instance based on the size and unit provided.
    fn new(size: f64, unit: &str) -> Result<Self, String> {
        if size < 0.0 {
            return Err("Invalid file size. Size cannot be a negative number.".to_string()); // Return an error if the file size is negative
        }

        match unit {
            "bytes" => Ok(FileSize::Bytes(size as u64)),
            "kb" => Ok(FileSize::Kilobytes(size)),
            "mb" => Ok(FileSize::Megabytes(size)),
            "gb" => Ok(FileSize::Gigabytes(size)),
            _ => Err("Error: Invalid unit. Supported units: 'bytes', 'kb', 'mb', or 'gb'.".to_string()), // Return an error if the unit is invalid
        }
    }

    // Normalize the file size to bytes.
    fn normalize_to_bytes(&self) -> u64 {
        match self {
            FileSize::Bytes(size) => *size,
            FileSize::Kilobytes(size) => (*size * KB) as u64,
            FileSize::Megabytes(size) => (*size * MB) as u64,
            FileSize::Gigabytes(size) => (*size * GB) as u64,
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect(); // Collect command line arguments

    // Check if the correct number of arguments is provided
    if args.len() != 3 {
        eprintln!("Usage: {} <file_size> <unit (bytes/kb/mb/gb)>", args[0]);
        std::process::exit(2); // Exit with code 2 for incorrect command usage
    }

    let input = format!("{} {}", args[1], args[2]); // Combine the file size and unit input for display

    // Validate and parse the file size argument
    let file_size: f64 = match args[1].parse::<f64>() {
        Ok(size_input) if size_input >= 0.0 => size_input,
        Ok(_) => {
            eprintln!("Invalid file size. Size cannot be a negative number.");
            std::process::exit(4); // Exit with code 4 for negative file size input
        }
        Err(error) => {
            eprintln!("Invalid file size: {}. Size cannot be a non-numeric value.", error);
            std::process::exit(4); // Exit with code 4 for non-numeric file size input
        }
    };

    // Validate the unit argument
    let unit = match args[2].to_lowercase().as_str() {
        "bytes" | "kb" | "mb" | "gb" => args[2].to_lowercase(),
        _ => {
            eprintln!(
                "Invalid unit: '{}'. Supported units: 'bytes', 'kb', 'mb', or 'gb'.",
                args[2]
            );
            std::process::exit(2); // Exit with code 2 for invalid unit input
        }
    };

    // Create a FileSize instance and normalize it to bytes. Then, convert it to Sizes and display the results.
    match FileSize::new(file_size, &unit) {
        Ok(file_size) => {
            let size_in_bytes = file_size.normalize_to_bytes(); // Normalize the file size to bytes
            let sizes = Sizes::from_bytes(size_in_bytes); // Create a Sizes instance to hold the file size conversions
            println!("file size ({}):", input); // Display the file size and unit input
            println!("{}", sizes); // Display the file size in different units
            std::process::exit(0); // Exit with code 0 for successful execution
        }
        Err(error) => {
            eprintln!("{}", error); // Display appropriate error message if the file size or unit are invalid

            // Exit with appropriate code based on the error
            if error.contains("Invalid file size. Size cannot be a negative number.") {
                std::process::exit(4); // Exit with code 4 for invalid file size
            } else {
                std::process::exit(2); // Exit with code 2 for invalid unit
            }
        }
    }
}
