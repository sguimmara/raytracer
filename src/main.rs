use simplelog::*;
use clap::{App, Arg, SubCommand, crate_version, crate_authors, crate_name, crate_description, ArgMatches};
use crate::app::RunOpts;

mod app;
mod scene;
mod math;
mod rendering;

fn main() {
    configure_logger();

    let matches = App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        // .arg(Arg::with_name("config")
        //     .short("c")
        //     .long("config")
        //     .value_name("FILE")
        //     .help("Sets a custom config file")
        //     .takes_value(true))
        // .arg(Arg::with_name("INPUT")
        //     .help("Sets the input file to use")
        //     .required(true)
        //     .index(1))
        // .arg(Arg::with_name("v")
        //     .short("v")
        //     .multiple(true)
        //     .help("Sets the level of verbosity"))
        .subcommand(SubCommand::with_name("run")
            .about("runs the raytracer")
            .arg(Arg::with_name("output")
                .short("o")
                .help("the output image file to write"))
            .arg(Arg::with_name("verbose")
                .short("v")
                .help("print detailed messages")))
        .get_matches();

    match matches.subcommand() {
        ("run", Some(subcommand)) => prepare_run(subcommand),
        _ => println!("{}", matches.usage())
    }
    // let mut fb = FrameBuffer::new(108, 25);
    // log::info!("created {}", fb);
    //
    // let camera = Camera::new(Rgb::red());
    // log::info!("created {}", camera);
    //
    // log::info!("start rendering...");
    //
    // camera.render(&mut fb);
    //
    // let backend = FileBackend::new("/home/sguimmara/Downloads/foo.png");
    //
    // log::info!("saving render to {}", backend);
    // backend.write(&fb);
    //
    // log::info!("finished.");
}

fn prepare_run(p0: &ArgMatches) {
    let output_file = p0.value_of("output").unwrap_or("run.png");
    let verbose = p0.value_of("verbose");

    let run_opts = RunOpts::new(output_file, verbose.is_some());

    app::run(run_opts);
}

fn configure_logger() {
    CombinedLogger::init(vec![TermLogger::new(
        LevelFilter::Trace,
        Config::default(),
        TerminalMode::Mixed,
        ColorChoice::Auto,
    )])
    .unwrap();
}
