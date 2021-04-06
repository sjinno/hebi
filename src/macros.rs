#[macro_export]
macro_rules! clear {
    (all) => {{
        println!("\x1B[2J\x1B[1;1H");
    }};
}
