//! A simple Raytracer

use crate::app::RunOpts;
use clap::{
    crate_authors, crate_description, crate_name, crate_version, App, Arg, ArgMatches, SubCommand,
};
use simplelog::*;

mod app;
mod math;
mod rendering;
mod scene;

fn main() {
    configure_logger();

    let matches = App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .subcommand(
            SubCommand::with_name("run")
                .about("runs the raytracer")
                .arg(
                    Arg::with_name("output")
                        .short("o")
                        .help("the output image file to write"),
                )
                .arg(
                    Arg::with_name("verbose")
                        .short("v")
                        .help("print detailed messages"),
                ),
        )
        .get_matches();

    match matches.subcommand() {
        ("run", Some(subcommand)) => prepare_run(subcommand),
        _ => println!("{}", matches.usage()),
    }
}

#[doc(hidden)]
fn prepare_run(p0: &ArgMatches) {
    let output_file = p0.value_of("output").unwrap_or("run.png");
    let verbose = p0.value_of("verbose");

    let run_opts = RunOpts::new(output_file, verbose.is_some());

    app::run(run_opts);
}

#[doc(hidden)]
fn configure_logger() {
    CombinedLogger::init(vec![TermLogger::new(
        LevelFilter::Trace,
        Config::default(),
        TerminalMode::Mixed,
        ColorChoice::Auto,
    )])
    .unwrap();
}
