<div align="center">
    <h1>MyGCPay CLI</h1>
    <img src="logo.png" width="230">
    <br/>

[Voir la version française](./README.fr_ca.md)


</div>


## Description

MyGCPay CLI is an UNOFFICIAL Rust command-line client for working with data from the Government of Canada's MyGCPay portal.

It is designed for local, scriptable access to:

- your MyGCPay session cookie
- paycheque lists and paycheque details
- the application's local cache and data directory
- a built-in calendar of pay dates and related annotated dates

The tool prints JSON for data-oriented commands, which makes it useful both for direct terminal use and for automation in PowerShell or other shell scripts.

## Features

- Store and validate the MyGCPay session cookie used for authenticated requests
- List paycheques from MyGCPay
- Fetch the details for a specific paycheque
- Optionally fetch full details for every paycheque in one command
- Show the response shape for paycheque commands without making requests
- Show the app home directory and cache directory
- Clear cached data
- Show a local calendar of pay dates, holidays, and related annotations
- Emit structured NDJSON logs when needed

## Build and Install

### Prerequisites

- Rust 1.94 or newer
- Access to MyGCPay in a web browser

### Build locally

```powershell
cargo build --release
```

The compiled executable will be available at:

```text
target/release/mgcp.exe
```

### Run from source

```powershell
cargo run -- --help
```

### Install into Cargo's bin directory

```powershell
cargo install --path .
```

After installation, you can use:

```powershell
mgcp --help
```

## Command Overview

```text
mgcp cookie ...
mgcp paycheque ...
mgcp cache ...
mgcp home ...
mgcp calendar ...
```

Common global options:

- `--debug` enables debug logging and panic backtraces
- `--log-filter <DIRECTIVE>` sets the tracing filter
- `--log-file <FILE|DIR>` writes structured NDJSON logs

## Authentication

MyGCPay CLI uses your MyGCPay session cookie to make authenticated requests.

To capture and store the cookie:

1. Visit <https://mapayegc-mygcpay.tpsgc-pwgsc.gc.ca/en/mygcpay/>.
2. Open the browser developer tools and go to the Network tab.
3. Refresh the page.
4. Find a request to `paycheque-data/`.
5. Right-click the request and choose `Copy > Copy request headers`.
6. Pipe the copied headers into the CLI.

PowerShell example:

```powershell
Get-Clipboard | mgcp cookie set
```

You can also pass the cookie value directly:

```powershell
mgcp cookie set "Cookie: <your-cookie-value>"
```

Check whether a cookie is currently stored:

```powershell
mgcp cookie check
```

Clear the stored cookie:

```powershell
mgcp cookie clear
```

## Usage Examples

### List paycheques

List the currently visible paycheques:

```powershell
mgcp paycheque list
```

Include archived paycheques and fetch full details for each one:

```powershell
mgcp paycheque list --all
```

Adjust the delay between detailed requests when using `--all`:

```powershell
mgcp paycheque list --all --sleep 1s
```

Inspect the output schema without making requests:

```powershell
mgcp paycheque list --shape
mgcp paycheque list --all --shape
```

### Show one paycheque

Fetch one paycheque by ID:

```powershell
mgcp paycheque show <ID>
```

Inspect the schema for the detailed response:

```powershell
mgcp paycheque show <ID> --shape
```

### Show calendar data

Show annotated dates for the current year:

```powershell
mgcp calendar show
```

Show a specific year:

```powershell
mgcp calendar show 2026
```

The calendar output includes attributes such as `PayDate`, `Holiday`, `CivicHoliday`, and `NationalQuebecHoliday`.

### Inspect local storage paths

Show the application home directory:

```powershell
mgcp home path show
```

Show the cache directory:

```powershell
mgcp cache path show
```

Clear the cache directory:

```powershell
mgcp cache clean
```

## Output and Logging

Data commands print pretty-formatted JSON to standard output. That makes the tool easy to combine with other commands.

Example:

```powershell
mgcp paycheque list | Out-File paycheques.json
```

To write structured logs:

```powershell
mgcp --log-file .\logs\ paycheque list
```

To enable debug logging:

```powershell
mgcp --debug paycheque list
```

## Notes

- `mgcp paycheque list --all` performs one request per paycheque detail after retrieving the list.
- `mgcp cookie set` accepts either a raw cookie value or copied request headers via standard input.
- If the stored cookie expires, repeat the cookie capture process.

## Development

Useful commands while working on the project:

```powershell
cargo fmt
cargo clippy --all-targets --all-features
cargo test
```

## Copyright

Copyright belongs to © His Majesty the King in Right of Canada, as represented by the Minister of Agriculture and Agri-Food, 2026.
