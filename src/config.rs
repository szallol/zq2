use clap::{Arg, Command};

#[derive(Debug)]
pub struct Config {
    pub inputs: Vec<String>,
    pub outputs: Vec<String>,
    pub query: String,
}

impl Config {
    pub fn from_args() -> Self {
        let matches = Command::new("zq")
            .version("0.1.0")
            .about("query tool(SQL) for different data sources and destinations")
            .author("Szallo L. <szallol@gmail.com>")
            .arg(
                Arg::with_name("input")
                    .short('i')
                    .long("input")
                    .value_name("INPUT")
                    .help("Input source")
                    .multiple(true)
                    .required(true)
                    .takes_value(true),
            )
            .arg(
                Arg::with_name("output")
                    .short('o')
                    .long("output")
                    .value_name("OUTPUT")
                    .help("Output destination")
                    .multiple(true)
                    .required(false)
                    .takes_value(true),
            )
            .arg(
                Arg::with_name("query")
                    .short('q')
                    .long("query")
                    .value_name("QUERY")
                    .help("Query to execute on imported data")
                    .multiple(false)
                    .required(true)
                    .takes_value(true),
            )
            .get_matches();

        let input_strs = matches
            .values_of("input")
            .unwrap()
            .map(|s| s.to_string())
            .collect::<Vec<_>>();
        let out_strs = matches
            .values_of("output")
            .unwrap()
            .map(|s| s.to_string())
            .collect::<Vec<_>>();
        let query = matches.value_of("query").unwrap();

        Self {
            inputs: input_strs,
            outputs: out_strs,
            query: query.to_string(),
        }
    }
}
