use temporal_core::{PlainDate, Calendar};

fn main() {
    let x = PlainDate::from_ymd(2001, 13, 1, Calendar::Iso8601).constrain();
    let x = x.iso_date().year();
    println!("{x}");
    println!("Hello, world!");
}
