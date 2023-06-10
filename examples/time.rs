use chrono::FixedOffset;

fn main() {
    // drive::r#const::TIME_FORMAT
    let result = chrono::DateTime::parse_from_rfc3339("2022-06-22T03:15:02Z").unwrap();
    let format = result
        .with_timezone(&FixedOffset::east(8 * 3600))
        .format("%Y-%m-%d %H:%M:%S");
    println!("{}", format.to_string());
}
