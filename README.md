# Tracker
A CLI activity tracker built in Rust.

## Functionality
This program tracks the number of minutes spent on activities each day and saves the data to a JSON file. Time can be added or subtracted from an activity via the command line. You can also print out a summary of time spent on various activities over a date range.

When running the program (either using `cargo run` in the project directory or by running the executable), you'll need to also provide a function to run (`add`, `sub`, or `sum`) and the necessary arguments for each function.

To see a list of valid functions and their required arguments, enter `help` after the command to run the program (e.g. enter `cargo run help` from within the project directory).

## Example
Example data for June 1, 2023 to June 5, 2023 has been provided in the project directory. This data will be used by the program if it is run from inside the project directory using `cargo run`.

To see a summary over this date range, enter:
```
cargo run sum 2023-6-1 2023-6-5
```

To see the summary for a particular date, such as June 2, enter:
```
cargo run sum 2023-6-2
```

To add time for an activity, such as 30 minutes of guitar on June 1, 2023, enter:
```
cargo run add 2023-6-1 guitar 30
```

To subtract time from an activity, such as 60 minutes of exercise on June 2, enter:
```
cargo run sub 2023-6-2 exercise 60
```

## File Structure
Besides the files provided by Cargo ("Cargo.toml" and "Cargo.lock"), this project consists of four code files ("main.rs", "lib.rs", "tracker_data.rs", and "date.rs") and one data file ("tracker_data.json"). This data file is a JSON formatted file that keeps track of dates, activities, and minutes as entered by the user via the command line. It is not intended to be edited directly (although of course it could be). The program can only understand it if it retains a particular structure. (If this file is missing in the directory from which the program is run, the program will create a new data file automatically.)

The binary crate "main.rs" contains the main function and a few helper functions. It is responsible for parsing arguments passed in via the command line and figuring out which function to call. The library crate "lib.rs" just provides the connection for the other two files. The first of these files, "tracker_data.rs", contains the TrackerData struct and its methods. Each time the program is called, one of these structs is created and its data is populated using the JSON data file. It is then used in some way (based on user input) and then saved back to the JSON data file. The other file, "date.rs", contains the Date struct and its methods. This struct is used throughout the program as a way to store date type values.
