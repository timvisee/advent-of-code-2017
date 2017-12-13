use std::process::exit;

use clap::{App, Arg, ArgMatches};

use app::*;



/// Argument handler.
pub struct ArgHandler<'a> {
    matches: ArgMatches<'a>
}

impl<'a> ArgHandler<'a> {
    /// Handle arguments, return an arg handler containing results.
    pub fn handle() -> ArgHandler<'a> {
        ArgHandler {
            matches: ArgHandler::do_match(),
        }
    }

    /// Match all arguments
    fn do_match() -> ArgMatches<'a> {
        App::new(APP_NAME)
            .version(APP_VERSION)
            .author(APP_AUTHOR)
            .about(APP_ABOUT)
            .arg(Arg::with_name("save")
                .short("s")
                .long("save")
                .help("save challenge output to output file"))
            .arg(Arg::with_name("manual")
                .short("m")
                .long("manual")
                .help("force manual input entry, even if input file exists"))
            .arg(Arg::with_name("day")
                .short("d")
                .long("day")
                .help(&format!("select day to run (1-{})", DAY_MAX))
                .takes_value(true))
            .arg(Arg::with_name("part")
                .short("p")
                .long("part")
                .help(&format!("select part to run (1-{})", PART_MAX))
                .takes_value(true))
            .get_matches()
    }

    /// Check whether manual input is enabled.
    pub fn is_manual(&self) -> bool {
        self.matches.occurrences_of("manual") > 0
    }

    /// Check whether output saving is enabled.
    pub fn is_save(&self) -> bool {
        self.matches.occurrences_of("save") > 0
    }

    /// Get the selected day.
    ///
    /// If no day was selected, None is returned.
    pub fn day(&self) -> Option<u8> {
        // Get the day value, return if not selected
        let raw = self.matches.value_of("day");
        if raw.is_none() {
            return None;
        }

        // Trim
        let raw = raw.unwrap().trim();
        if raw.is_empty() {
            return None;
        }

        // Try to parse to a number
        let day = raw.parse();
        if day.is_err() {
            println!("Invalid day entered, must be a number.");
            exit(1);
        }

        // Return the day
        day.ok()
    }

    /// Get the selected part.
    ///
    /// If no part was selected, None is returned.
    pub fn part(&self) -> Option<u8> {
        // Get the part value, return if not selected
        let raw = self.matches.value_of("part");
        if raw.is_none() {
            return None;
        }

        // Trim
        let raw = raw.unwrap().trim();
        if raw.is_empty() {
            return None;
        }

        // Try to parse to a number
        let part = raw.parse();
        if part.is_err() {
            println!("Invalid part entered, must be a number.");
            exit(1);
        }

        // Return the part
        part.ok()
    }
}
