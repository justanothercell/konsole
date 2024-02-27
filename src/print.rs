use crate::input::refresh;

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {
        $crate::__redirect_print!(std::format!($($arg)*); std::print!($($arg)*))
    };
}

#[macro_export]
macro_rules! println {
    () => {
        $crate::__redirect_print!("\n"; std::println!())
    };
    ($($arg:tt)*) => {
        $crate::__redirect_print!(std::format!("{}\n", std::format!($($arg)*)); std::println!($($arg)*))
    };
}

#[macro_export]
macro_rules! __redirect_print {
    ($formatted: expr; $std_print: expr) => {
        if $crate::is_active() {
            $crate::print::printout($formatted)
        } else {
            $std_print
        }
    };
}

pub fn printout(line: impl AsRef<str>) {
    let line = line.as_ref();
    let (w, _) = crate::size();
    std::print!("\r{}", " ".repeat(w));
    if line.ends_with('\n') {
        std::print!("\r{:1$}", line, w);
    } else {
        std::println!("\r{:1$}", line, w);
    }
    refresh();
}