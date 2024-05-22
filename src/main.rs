use fnr::app::Options;
use clap::Parser;

fn main() {
    let opts: Options = Options::parse();
    
    println!("{:?}", opts);
}
