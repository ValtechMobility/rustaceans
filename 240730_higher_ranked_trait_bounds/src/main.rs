use std::fmt::Display;

trait Formatter {
    fn format<T: Display>(&self, value: T) -> String;
}

struct SimpleFormatter;

impl Formatter for SimpleFormatter {
    fn format<T: Display>(&self, value: T) -> String {
        format!("Value: {}", value)
    }
}

fn apply_format<F>(formatter: F) -> impl Fn(String) -> String
where
    F: Formatter,
{
    move |s| formatter.format(s)
}

fn main() {
    let formatter = SimpleFormatter;

    let format_fn = apply_format(formatter);

    let s1 = "Hello".to_string();

    let s2 = String::from("World");

    /* {
        let s3 = String::from("Hello");
        println!("{}", format_fn(&s3));
    } */

    println!("{}", format_fn(s1));
    println!("{}", format_fn(s2));
}
