use clap::{App, Arg, SubCommand};
use dot::options::{Action, Criteria, Filter, Options};

fn main() {
    let matches = App::new("dot")
        .version("0.1.0")
        .author("Daniel Wolbach")
        .about("Manage your configuration files with ease")
        .subcommand(
            SubCommand::with_name("save")
                .about("Save your configuration files")
                .arg(
                    Arg::with_name("all")
                        .long("all")
                        .short("a")
                        .conflicts_with_all(&["group", "name"])
                        .multiple(false)
                        .takes_value(false)
                        .help("Save all sets"),
                )
                .arg(
                    Arg::with_name("name")
                        .long("name")
                        .short("n")
                        .conflicts_with("all")
                        .multiple(true)
                        .takes_value(true)
                        .help("Save sets that match a name"),
                )
                .arg(
                    Arg::with_name("group")
                        .long("group")
                        .short("g")
                        .conflicts_with("all")
                        .multiple(true)
                        .takes_value(true)
                        .help("Save sets that match a group"),
                ),
        )
        .subcommand(
            SubCommand::with_name("apply")
                .about("Apply your configuration files")
                .arg(
                    Arg::with_name("all")
                        .long("all")
                        .short("a")
                        .conflicts_with_all(&["group", "name"])
                        .multiple(false)
                        .takes_value(false)
                        .help("Apply all sets"),
                )
                .arg(
                    Arg::with_name("name")
                        .long("name")
                        .short("n")
                        .conflicts_with("all")
                        .multiple(true)
                        .takes_value(true)
                        .help("Apply sets that match a name"),
                )
                .arg(
                    Arg::with_name("group")
                        .long("group")
                        .short("g")
                        .conflicts_with("all")
                        .multiple(true)
                        .takes_value(true)
                        .help("Apply sets that match a group"),
                ),
        )
        .get_matches();

    let mut options = Options {
        action: None,
        filter: None,
    };

    if let Some(matches) = matches.subcommand_matches("save") {
        if matches.is_present("all") {
            options.action = Some(Action::Save);
            options.filter = Some(Filter::All);
        } else {
            let criteria = Criteria {
                names: matches
                    .values_of("name")
                    .unwrap_or(clap::Values::default())
                    .map(|val| return String::from(val))
                    .collect(),
                groups: matches
                    .values_of("group")
                    .unwrap_or(clap::Values::default())
                    .map(|val| return String::from(val))
                    .collect(),
            };

            options.action = Some(Action::Save);
            options.filter = Some(Filter::Match(criteria));
        }
    }

    if let Some(matches) = matches.subcommand_matches("apply") {
        if matches.is_present("all") {
            options.action = Some(Action::Apply);
            options.filter = Some(Filter::All);
        } else {
            let criteria = Criteria {
                names: matches
                    .values_of("name")
                    .unwrap_or(clap::Values::default())
                    .map(|val| return String::from(val))
                    .collect(),
                groups: matches
                    .values_of("group")
                    .unwrap_or(clap::Values::default())
                    .map(|val| return String::from(val))
                    .collect(),
            };

            options.action = Some(Action::Apply);
            options.filter = Some(Filter::Match(criteria));
        }
    }

    if let Err(err) = dot::run(&options) {
        eprintln!("{}", err);
    };
}
