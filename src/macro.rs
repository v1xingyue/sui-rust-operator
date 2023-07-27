#[macro_export]
macro_rules! print_beauty {
    () => {
        println!();
    };
    ($s:expr) => {
        utils::mark_with_style($s.to_string(), &utils::random_style());
    };
    ($format:expr$(,$arg:expr)*) => {
        utils::mark_with_style(format!($format $(,$arg)*), &utils::random_style());
    };
}
