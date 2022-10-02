#[macro_export]
macro_rules! error {
    () => {
        println!();
    };
    ($($arg:tt)*) => {{
	use colored::Colorize;
	if CONFIG.print_error_messages {
            println!("{}", format_args!($($arg)*).to_string().red().bold());
	}
    }};
}
#[macro_export]
macro_rules! warning {
    () => {
        println!();
    };
    ($($arg:tt)*) => {{
	use colored::Colorize;
	if CONFIG.print_warning_messages {
            println!("{}", format_args!($($arg)*).to_string().yellow().bold());
	}
    }};
}
#[macro_export]
macro_rules! success {
    () => {
        println!();
    };
    ($($arg:tt)*) => {{
	use colored::Colorize;
	if CONFIG.print_success_messages {
            println!("{}", format_args!($($arg)*).to_string().green().bold());
	}
    }};
}
