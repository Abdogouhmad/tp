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
    ($fmt:expr $(, $args:tt)*) => {
        let formatted_str = format!($fmt $(, $args)*);
        let colored_str = $crate::macros::replace_color_tags(&formatted_str);
        eprintln!("{}", colored_str);
    };
}

pub fn replace_color_tags(input: &str) -> String {
    let mut output = input.to_string();
    let colors = vec![
        ("<b>", "\x1b[30m"),
        ("<r>", "\x1b[31m"),
        ("<g>", "\x1b[32m"),
        ("<y>", "\x1b[33m"),
        ("<b>", "\x1b[34m"),
        ("<m>", "\x1b[35m"),
        ("<c>", "\x1b[36m"),
        ("<w>", "\x1b[37m"),
        ("</b>", "\x1b[0m"),
        ("</r>", "\x1b[0m"),
        ("</g>", "\x1b[0m"),
        ("</y>", "\x1b[0m"),
        ("</b>", "\x1b[0m"),
        ("</m>", "\x1b[0m"),
        ("</c>", "\x1b[0m"),
        ("</w>", "\x1b[0m"),
    ];

    for (tag, code) in colors {
        output = output.replace(tag, code);
    }

    output
}
