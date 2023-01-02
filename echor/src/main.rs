//Arg is imported from clap crate
use clap::{App, Arg};

fn main() {
    let matches = App::new("ezio echor")
        .version("0.1.0")
        .author("Ezio <e@z.io>")
        .about("Rust echo")
        .arg(
            Arg::with_name("ezio_text")
                .value_name("TEXT")
                .help("Input text")
                .required(true)
                .min_values(1),
        )
        .arg(
            Arg::with_name("omit_newline")
                .short("n")
                .help("Do not print newline")
                .takes_value(false),
        )
        .get_matches();
    // looks like this is written declaratively?
    let text = matches.values_of_lossy("ezio_text").unwrap();
    let omit_newline = matches.is_present("omit_newline");
    print!("{}{}", text.join(" "), if omit_newline {""} else {"\n"});
}
