use colored::{ColoredString, Colorize};

pub fn print_console(msg: ColoredString) {
    println!("{} {}", "> ".dimmed(), msg)
}

pub fn print_info(msg: String) {
    print_console(msg.dimmed().yellow().italic());
}

pub fn print_sent(msg: String) {
    print_console(msg.italic().blue().bold())
}

pub fn print_received(msg: String) {
    print_console(msg.red().bold())
}
