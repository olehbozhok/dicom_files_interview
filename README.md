# Dicom Files Utility

This project is a Rust-based utility designed to extract and display information from DICOM files. The utility can output the extracted data in different formats (text or CSV) and redirect the output to either `stdout` or a specified file.

## Features

- **Format Selection**: Display DICOM file data as either plain text or CSV.
- **Output Redirection**: Choose to redirect output to `stdout` or save it to a file.
- **Customizable**: Easily extendable to support additional formats and output options.

## Build Instructions

To build this utility, ensure that you have Rust installed on your machine. If you don't have Rust installed, you can get it from [here](https://www.rust-lang.org/).

### Clone the Repository

```bash
git clone https://github.com/olehbozhok/dicom_files_interview
cd dicom_files_interview
```

### Build the Project

```bash
cargo build --release
```

This will generate an executable in the `target/release` directory.

## Usage

Once the project is built, you can run the utility with the following command:

```bash
./target/release/dicom_parser --format <FORMAT> --output <OUTPUT> <DICOM_FILE/FOLDER>
```

### Arguments

```
Command-line utility to catalog the patients in a drive

Usage: dicom_parser [OPTIONS] <PATH>

Arguments:
  <PATH>  The path to the folder or file to read

Options:
  -n, --num-workers <NUM_WORKERS>
          Count of reader workers, may be usefull to increase value, if you use ssd hard drive [default: 1]
  -o, --output-format <OUTPUT_FORMAT>
          Set the output format to TEXT or CSV [default: text] [possible values: text, csv]
  -r, --result-filepath <RESULT_FILEPATH>
          Specify a value to save the result to a file. By default, the result is redirected to stdout
  -h, --help
          Print help
```
### Example

```bash
./target/release/dicom_parser /path/to/folder --format csv --output stdout 
```

This command will output the DICOM file data in CSV format to the console.

```bash
./target/release/dicom_parser /path/to/folder --format csv --output result.csv 
```
This command will output the DICOM file data in CSV format to the file result.csv.

Errors will be redirected to the stderr stream, allowing you to handle data and errors separately as needed.

## Considerations and Assumptions

- **DICOM Files**: This utility processes valid DICOM files. Invalid files will be skipped, and an error message will be displayed on stderr.
- **Dependencies**: The project relies on the `dicom` crate for handling DICOM file operations.
- **Extensibility**: The current implementation follows the Strategy Pattern to allow easy extension for additional output formats and destinations.

## Contribution

Contributions to this project are welcome! If you find any issues or have suggestions for improvements, feel free to open an issue or submit a pull request.

## License

This project is licensed under the MIT License. See the `LICENSE` file for more details.

---

Feel free to customize this template further based on the specific details of your project!


___

The program has been tested using files containing generated test data, which are available at  [link](https://github.com/robyoung/dicom-test-files)
