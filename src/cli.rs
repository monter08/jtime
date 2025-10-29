use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(
    name = "jtime",
    author = "Dawid Stachow",
    version = env!("CARGO_PKG_VERSION"),
    about = "Jira time tracking CLI tool"
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// (Alias: l)
    /// Log time for a task
    /// Example: jtime log XX-1234 --day 2 --time 1h30m
    /// Default day is today
    /// Default time is 8h
    #[clap(alias = "l")]
    Log {
        /// Task identifier (eg. XX-1234)
        task: Option<String>,

        /// Day in this monh (eg. 2 or multiple days 2-5)
        /// Default is current day
        /// Example: 2
        day: Option<String>,

        /// Time spent (e.g. 1h30m)
        #[clap(default_value = "8h")]
        time: String,

        /// Comment for worklog (e.g. retro)
        comment: Option<String>,

        /// Skip confirmation
        #[clap(long, default_value_t = false)]
        yes: bool,

        /// Task but can be provided as option
        #[arg(short = 'd', long = "day", value_name = "DAY")]
        option_day: Option<String>,

        /// Task but can be provided as option
        #[arg(short = 't', long = "time", value_name = "TIME")]
        option_time: String,

        /// Comment but can be provided as option
        #[arg(short = 'c', long = "comment", value_name = "COMMENT")]
        option_comment: Option<String>,
    },

    /// (Alias: m)
    /// List monthly time logs
    /// Get for Febuary: jtime m --month 2
    /// Example: jtime m --cache
    #[clap(alias = "m")]
    Month {
        /// Use cached data
        #[clap(long, default_value_t = false)]
        cache: bool,

        /// Month number (1-12)
        /// Default is current month
        /// Example: 2
        #[clap(short, long)]
        month: Option<u32>,
    },

    /// (Alias: w)
    /// List weekly time logs
    /// Example: jtime week
    #[clap(alias = "w")]
    Week {
        /// Use cached data
        #[clap(long, short, default_value_t = false)]
        cache: bool,

        /// Previous week
        #[clap(long, short, default_value_t = false)]
        prev: bool,
    },

    /// (Alias: c)
    /// Show or set configuration
    /// Example: jtime config --url https://jira.com --token 123
    #[clap(alias = "c")]
    Config {
        /// Jira URL
        #[clap(long)]
        url: Option<String>,

        /// Jira token
        #[clap(long)]
        token: Option<String>,

        /// Nager url
        #[clap(long)]
        nager_url: Option<Option<String>>,

        /// Country code for nager
        #[clap(long)]
        nager_country_code: Option<Option<String>>,

        /// Show weekends
        #[clap(long)]
        show_weekends: Option<bool>,
    },

    /// (Alias: u)
    /// Update JTime to the latest version
    /// Example: jtime update
    #[clap(alias = "u")]
    Update,
}
