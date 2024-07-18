#[macro_export]
macro_rules! colprintln {
    ($fmt:expr $(, $args:tt)*) => {{
        let formatted_str = format!($fmt $(, $args)*);
        let colored_str = $crate::macros::replace_color_tags(&formatted_str);
        println!("{}", colored_str);
    }};
}

#[macro_export]
macro_rules! eclprintln {
    ($fmt:expr $(, $args:tt)*) => {{
        let formatted_str = format!($fmt $(, $args)*);
        let colored_str = $crate::macros::replace_color_tags(&formatted_str);
        eprintln!("{}", colored_str);
    }};
}

pub fn replace_color_tags(input: &str) -> String {
    let mut output = input.to_string();
    let colors = vec![
        ("<b>", "\x1b[1;30m"),  // Bold black
        ("<r>", "\x1b[1;31m"),  // Bold red
        ("<g>", "\x1b[1;32m"),  // Bold green
        ("<y>", "\x1b[1;33m"),  // Bold yellow
        ("<bl>", "\x1b[1;34m"), // Bold blue
        ("<m>", "\x1b[1;35m"),  // Bold magenta
        ("<c>", "\x1b[1;36m"),  // Bold cyan
        ("<w>", "\x1b[1;37m"),  // Bold white
        ("</b>", "\x1b[0m"),    // Reset
        ("</r>", "\x1b[0m"),    // Reset
        ("</g>", "\x1b[0m"),    // Reset
        ("</y>", "\x1b[0m"),    // Reset
        ("</bl>", "\x1b[0m"),   // Reset
        ("</m>", "\x1b[0m"),    // Reset
        ("</c>", "\x1b[0m"),    // Reset
        ("</w>", "\x1b[0m"),    // Reset
    ];

    for (tag, code) in colors {
        output = output.replace(tag, code);
    }

    output
}
