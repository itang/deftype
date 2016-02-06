#[macro_export]
macro_rules! info {
    ($fmt: expr) => ({
        extern crate ansi_term;
        println!("{}: {}", ansi_term::Colour::Green.paint("INFO"), $fmt);
    });
    ($fmt: expr, $($arg: tt)*) => ({
        extern crate ansi_term;
        //let args = format!($fmt, $($arg)*);
        println!(concat!("{}: ", $fmt), ansi_term::Colour::Green.paint("INFO"), $($arg)*);
    });
}

#[macro_export]
macro_rules! error {
    ($fmt: expr) => ({
        extern crate ansi_term;
        println!("{}: {}", ansi_term::Colour::Red.paint("ERROR"), $fmt);
    });
    ($fmt: expr, $($arg: tt)*) => ({
        extern crate ansi_term;
        let args = format!($fmt, $($arg)*);
        println!("{}: {}", ansi_term::Colour::Red.paint("ERROR"), args);
    });
}
