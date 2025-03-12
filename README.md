# JTime - Jira Time Tracking CLI Tool
[![Build and Test](https://github.com/monter08/jtime/actions/workflows/ci.yml/badge.svg)](https://github.com/monter08/jtime/actions/workflows/ci.yml)

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
# Build and install
cargo install --git https://github.com/monter08/jtime.git
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
jtime l XX-1234 2 1h30m

# Log time for multiple days
jtime l XX-1234 2-5 1h30m

# Skip confirmation prompt
jtime l XX-1234 -y
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
jtime m -m 2

# Use cached data
jtime m --cache
jtim m -c
```

### View weekly logs

View time logs for the current week:

```bash
# View current week
jtime week

# Short version
jtime w

# Use cached data
jtime w --cache
jtime w -c

# View previous week
jtime w --prev
jtime w -p
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

The configuration file is stored at `~/.config/jtime/config.json`.

Example configuration:

```json
{
  "jira_url": "https://your-company.atlassian.net",
  "jira_token": "your-jira-api-token",
  "show_weekends": false
}
```

## License

MIT
