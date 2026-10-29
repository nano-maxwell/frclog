# frclog

frclog is an open-source, multi-log analysis tool built in Rust. It helps FRC teams understand how their robots' mechanisms perform across multiple matches, competitions, or seasons so they can build more reliable robots.

> [!NOTE]
> frclog is in early development. It supports CSV parsing, signal inspection, and statistics. Full WPILOG parsing is under active development and is not currently prepared for binary signal extraction. Multi-log comparison is also not complete.

## Features
- Extract signal timestamps, values, and names from CSV files
- List detected signals, their data type, and sample count
- Calculate type-specific statistics (e.g., mean, standard deviation, transitions) for numeric, boolean, and string signals

## Quick Start
1. Install Rust.
2. Clone the repository.
3. Choose a CSV file in `timestamp,signal,value` format to analyze, or use the provided [example.csv](example.csv) file.
4. Run the following command in the terminal from the repository's root directory:
   
   ```bash
   cargo run -- info example.csv
   ```

   The output will show the number of unique signals detected and the number of valid rows. `example.csv` can be replaced with a path to another CSV file.

## CLI Commands
- `info <file>` - Lists the number of unique detected signals and the valid rows in the provided file.
- `signal <file> <signal>` - Lists the name, data type, and all values/timestamps of the specified signal.
- `signals <file>` - Lists the unique detected signals in the provided file, their data type, sample count, and time range.
- `stats <file> <signal>` - Lists the sample count and type-specific statistics (e.g., mean, standard deviation, transitions) for the specified signal.

## Roadmap
Major upcoming development goals:
- Full WPILOG support for signal extraction and analysis
- Multi-log signal data comparison
- Desktop application built with Tauri 2.0

## License
frclog is licensed under the GNU General Public License v3.0. See [LICENSE](LICENSE) for details.
