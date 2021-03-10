// `cargo build` will build the project and output the binary to target/debug/fintest
// `cargo run` will build and run the code!

use clap::value_t_or_exit;
use clap::values_t_or_exit;
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
                .arg(Arg::with_name("value").help("decimal value (eg, a rate)")))
        .subcommand(
            SubCommand::with_name("fvccn")
                .about("Calculate the future value (FV) of single amount `a`, invested for `n` periods at interest rate `r` per period but with continuous compounding (cc) per period")
                .arg(Arg::with_name("amount"))
                .arg(Arg::with_name("rate"))
                .arg(Arg::with_name("periods")))
        .subcommand(
          SubCommand::with_name("fvccnr")
              .about("Calculate the future value (FV) of single amount `a`, invested for `n` periods at varying cc interest rates ``r₁, r₂,...,rₙ`` per period but with continuous compounding (cc) per period")
              .arg(Arg::with_name("amount"))
              .arg(Arg::with_name("rates").require_delimiter(true)))
        .subcommand(
          SubCommand::with_name("pvacc")
              .about("Calculate the present value (PV) of an annuity payment `a` received each period for `n` periods using an cc discount (interest) rate of `r` per period.")
              .arg(Arg::with_name("periods").help("number of periods"))
              .arg(Arg::with_name("rate").help("discount (interest) rate"))
              .arg(Arg::with_name("amount").help("amount received per period")))
        .subcommand(
          SubCommand::with_name("pvagcc")
              .about("Calculate the present value (PV) of annuity payment `a`, growing at cc rate `g`, received each period for `n` periods using a cc discount (interest) rate of `r` per period, where ``0 ⩽ g < r ⩽ 1``.")
              .arg(Arg::with_name("periods").help("number of periods"))
              .arg(Arg::with_name("rate").help("discount (interest) rate"))
              .arg(Arg::with_name("amount").help("initial amount received per period"))
              .arg(Arg::with_name("growth").help("continuous growth rate of payments over time")))
        .subcommand(
          SubCommand::with_name("wsum")
              .about("Return a simple weighted-sum of array `x` where `w` is the array of weights with same dimension as `x`.  This will return equivalent of scalar-product or dot-product. This is commonly used in finance for summing cash flows in the DCF model where each are weighted by time and risk.")
              .arg(Arg::with_name("weights").help("array of weights").require_delimiter(true))
              .arg(Arg::with_name("values").help("array of values").require_delimiter(true)))
        .get_matches();

    match matches.subcommand() {
        ("bp", Some(args)) => {
            let value = value_t_or_exit!(args.value_of("value"), f64);
            println!("{}", value * 10000.0);
        }
        ("fvccn", Some(args)) => {
            let amount = value_t_or_exit!(args.value_of("amount"), f64);
            let rate = value_t_or_exit!(args.value_of("rate"), f64);
            let periods = value_t_or_exit!(args.value_of("periods"), f64);
            println!("{}", amount * exp(rate * periods));
        }
        ("fvccnr", Some(args)) => {
            let amount = value_t_or_exit!(args.value_of("amount"), f64);
            let rates = values_t_or_exit!(args.values_of("rates"), f64);
            println!("{}", amount * exp(rates.iter().sum::<f64>()));
        }
        ("pvacc", Some(args)) => {
            let n = value_t_or_exit!(args.value_of("periods"), f64);
            let r = value_t_or_exit!(args.value_of("rate"), f64);
            let a = value_t_or_exit!(args.value_of("amount"), f64);
            println!("{}", a / (exp(r) - 1.0) * (1.0 - exp(-r * n)));
        }
        ("pvagcc", Some(args)) => {
            let n = value_t_or_exit!(args.value_of("periods"), f64);
            let r = value_t_or_exit!(args.value_of("rate"), f64);
            let a = value_t_or_exit!(args.value_of("amount"), f64);
            let g = value_t_or_exit!(args.value_of("growth"), f64);
            println!("{}", a / (exp(r) - exp(g)) * (1.0 - exp(n * (g - r))));
        }
        ("wsum", Some(args)) => {
            let w = values_t_or_exit!(args.values_of("weights"), f64);
            let x = values_t_or_exit!(args.values_of("values"), f64);
            println!(
                "{}",
                w.iter().zip(x.iter()).map(|(a, b)| a * b).sum::<f64>()
            );
        }
        _ => println!("TODO show help text"),
    }
}

fn exp(x: f64) -> f64 {
    x.exp()
}
