// `cargo build` will build the project and output the binary to target/debug/fintest
// `cargo run` will build and run the code!

use clap::value_t_or_exit;
use clap::App;
use clap::Arg;
use clap::SubCommand;

fn main() {
    let matches = App::new("Financial methods")
        .version("0.0")
        .author("David Andrade <david.f.andrade@gmail.com>")
        .about("A variety of numerical financial utilities.")
        .subcommand(
            SubCommand::with_name("bp")
                .about("convert a decimal rate to basis points")
                .arg(Arg::with_name("value").help("decimal value (eg, a rate)")),
        )
        .get_matches();

    //     println!("Hello World!");
    //     println!("subcommand = {:#?}", matches.subcommand);
    //     println!("subcommand = {:#?}", matches.subcommand());
    match matches.subcommand() {
        ("bp", Some(args)) => {
            println!("{}", bp(value_t_or_exit!(args.value_of("value"), f64)));
        }
        _ => println!("poop"),
    }
}

fn bp(x: f64) -> f64 {
    // foobar this is a comment
    x * 10000.0
}
