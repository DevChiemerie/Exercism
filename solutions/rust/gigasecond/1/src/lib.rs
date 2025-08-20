//use time::PrimitiveDateTime as DateTime;

// Returns a DateTime one billion seconds after start.
/*pub fn after(start: DateTime) -> DateTime {
    todo!("What time is a gigasecond later than {start}");
}*/

use time::macros::datetime;
use time::Duration;

fn main() {
    let datetime = datetime!(2015-01-01 00:00:00);
    // let datetime = time::macros::datetime! (2015-01-01 00:00:00);
    let gigasecond = Duration::seconds(1_000_000_000);

    let future_datetime = datetime + gigasecond;

    println!("Input date and time: {}", datetime);

    println!(
        "Date and time after adding a gigasecond: {}",
        future_datetime
    );
}
