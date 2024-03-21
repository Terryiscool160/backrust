# backrust

[![build status](https://github.com/Terryiscool160/backrust/actions/workflows/rust.yml/badge.svg?branch=main)](https://github.com/Terryiscool160/backrust/actions/workflows/rust.yml)
[![dependency status](https://deps.rs/repo/github/terryiscool160/backrust/status.svg)](https://deps.rs/repo/github/terryiscool160/backrust)
[![License: GPL v3](https://img.shields.io/badge/License-GPLv3-blue.svg)](https://www.gnu.org/licenses/gpl-3.0)

###

an automated backup tool intended to be ran with crontab - it will connect to a variety of different mariadb hosts, dump the databases (using mariadb-dump), and upload them to a specified backblaze s3 bucket

**WARNING**: this tool is not intended to be used in a large scale production environment, it is intended to be used for personal projects or small scale projects

**NOTICE**: dumping databases with "\*" is currently not working yet

## Running Locally

1. you can install rust by following the [official guide](https://www.rust-lang.org/tools/install)
2. copy `Config.example.toml` to `Config.toml` and fill in the required fields
3. execute `cargo run`
4. the backup process should begin, notifying you of any errors

# contributing

feel free to make a pull request with any changes you feel are fit
