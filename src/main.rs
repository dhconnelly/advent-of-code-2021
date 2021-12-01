mod day1;

struct Day {
    name: &'static str,
    run: fn(&str),
}

static DAYS: &[Day] = &[Day { name: "day1", run: day1::run }];

fn unwrap<T, E: std::fmt::Display>(r: Result<T, E>) -> T {
    r.unwrap_or_else(|e| {
        println!("{}", e);
        std::process::exit(1);
    })
}

fn main() {
    let name = unwrap(std::env::args().nth(1).ok_or("missing day"));
    let day = unwrap(DAYS.iter().find(|day| day.name == name).ok_or("bad day"));
    let path = unwrap(std::env::args().nth(2).ok_or("missing input path"));
    let text = unwrap(std::fs::read_to_string(&path));
    (day.run)(&text);
}
