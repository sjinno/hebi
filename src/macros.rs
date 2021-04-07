#[macro_export]
macro_rules! clear {
    (all) => {{
        print!("\x1B[2J\x1B[1;1H");
    }};
}

#[macro_export]
macro_rules! color {
    (bait) => {{
        format!("{}▲{}", color::Fg(Yellow), color::Fg(Reset))
    }};

    (snake) => {{
        format!("{}●{}", color::Fg(LightGreen), color::Fg(Reset))
    }};
}
