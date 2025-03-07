# JTime - Jira Time Tracking CLI Tool

A simple command-line tool to track and manage your time in Jira.

## Features

- Log time to Jira tasks
- View monthly time logs
- View weekly time logs

## Installation

### Easy Installation

```bash
curl -sSL https://raw.githubusercontent.com/monter08/jtime/main/install.sh | bash
```

### From source

```bash
# Clone the repository
git clone https://github.com/monter08/jtime.git
cd rjira

# Build and install
cargo install --path .
```

## Usage

### Log time

Log time spent on a specific Jira task:

```bash
# Log 8 hours (default) for today
jtime log XX-1234

# Short version
jtime l XX-1234

# Log 1 hour and 30 minutes for the 2nd day of current month
jtime l XX-1234 --day 2 --time 1h30m

# Log time for multiple days
jtime l XX-1234 --day 2-5 --time 1h30m

# Skip confirmation prompt
jtime l XX-1234 --yes
```

### View monthly logs

View time logs for the current or specified month:

```bash
# View current month
jtime month

# Short version
jtime m

# View February
jtime m --month 2

# Use cached data
jtime m --cache
```

### View weekly logs

View time logs for the current week:

```bash
# View current week
jtime week

# Short version
jtime w

# Use cached data
jtime week --cache
```

### Configuration

Set up or view your Jira configuration:

```bash
# View current configuration
jtime config

# Set Jira URL
jtime config --url https://your-company.atlassian.net

# Set Jira API token
jtime config --token your-api-token

# Configure weekend display
jtime config --show-weekends true
```

## Configuration

The configuration file is stored at `~/.config/jtime/config.toml`.

Example configuration:

```toml
url = "https://your-company.atlassian.net"
token = "your-jira-api-token"
show_weekends = false
```

## License

MIT
