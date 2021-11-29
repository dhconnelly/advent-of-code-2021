mod day;
mod day0;

const DAYS: &'static [day::Solution] = &[day0::SOLUTION];
const USAGE: &'static str = "usage: advent-of-code-2021 <day> <input_path>";

fn main() {
    let mut args = std::env::args().skip(1);
    let day = args.next().unwrap_or_else(|| {
        println!("{}\n", USAGE);
        println!("available days:");
        for soln in DAYS {
            println!("   {}", soln.name);
        }
        std::process::exit(1);
    });
    let soln = DAYS
        .iter()
        .find(|soln| soln.name == day)
        .unwrap_or_else(|| {
            println!("invalid day: {}", day);
            std::process::exit(1);
        });
    let path = args.next().unwrap_or_else(|| {
        println!("missing input path");
        std::process::exit(1);
    });
    let text = std::fs::read_to_string(&path).unwrap_or_else(|e| {
        println!("can't read input: {}", e);
        std::process::exit(1);
    });
    (soln.run)(&text).unwrap_or_else(|e| {
        println!("failed to execute {}: {}", day, e);
        std::process::exit(1);
    });
}
