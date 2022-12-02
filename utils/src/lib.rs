
/// Read the input for this day's puzzle
///
/// The input is assumed to be in a file called "input.txt" within the
/// crate's root directory.
///
/// Returns a [`Result`] containing the input file's contents as a string
/// 
/// [`Result`]: std::io::Result
#[macro_export]
macro_rules! read_puzzle_input {
    () => {
        {
            let path = "input.txt";

            if let Ok(input) = std::fs::read_to_string(path) {
                Ok(input)
            } else {
                let path = std::path::Path::new(env!("CARGO_BIN_NAME")).join(path);
                std::fs::read_to_string(path)
            }
        }
    };
}
