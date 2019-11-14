use runner::arguments::Arguments;
use runner::Runner;

fn main() {
    let args = std::env::args();
    let arguments = Arguments::new(args);

    if let Err(e) = Runner::new(arguments).run() {
        eprintln!("{}", e);
    }
}
