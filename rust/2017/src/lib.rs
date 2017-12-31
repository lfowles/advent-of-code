mod adventofcode {
    #[macro_export]
    macro_rules! get_puzzle_input {
        ( $day:tt ) => (include_str!(concat!("../../puzzle_inputs/", stringify!($day))).trim());
    }
}