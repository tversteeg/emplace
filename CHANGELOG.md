# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [1.6.0](https://github.com/tversteeg/emplace/compare/v1.5.3...v1.6.0) - 2024-05-25

### Added
- *(paru)* support paru ([#387](https://github.com/tversteeg/emplace/pull/387))

### Fixed
- *(deps)* update rust crate anyhow to v1.0.86
- *(deps)* update rust crate anyhow to v1.0.85
- *(deps)* update rust crate itertools to 0.13.0
- *(deps)* update rust crate toml to v0.8.13
- *(deps)* update rust crate serde to v1.0.202
- *(deps)* update rust crate camino to v1.1.7
- *(deps)* update rust crate serde to v1.0.201
- *(deps)* update rust crate anyhow to v1.0.83
- *(deps)* update rust crate serde to 1.0.200
- *(deps)* update rust crate serde to 1.0.199
- *(deps)* update rust crate serde to 1.0.198
- *(deps)* update rust crate anyhow to 1.0.82
- *(deps)* update rust crate clap_complete to 4.5.2
- *(deps)* update rust crate enum_dispatch to 0.3.13
- *(deps)* update rust crate clap to 4.5.4
- *(deps)* update rust crate toml to 0.8.12
- *(deps)* update rust crate clap to 4.5.3
- *(deps)* update rust crate anyhow to 1.0.81
- *(deps)* update rust crate toml to 0.8.11
- *(deps)* update rust crate strum_macros to 0.26.2
- *(deps)* update rust crate strum to 0.26.2
- *(deps)* update rust crate clap to 4.5.2
- *(deps)* update rust crate simplelog to 0.12.2
- *(deps)* update rust crate log to 0.4.21
- *(deps)* update rust crate serde to 1.0.197
- *(deps)* update rust crate anyhow to 1.0.80
- *(deps)* update rust crate clap_complete to 4.5.1
- *(deps)* update rust crate clap to 4.5.1
- *(deps)* update rust crate clap_complete to 4.5.0
- *(deps)* update rust crate clap to 4.5.0

### Other
- *(deps)* update rust crate indoc to 2.0.5

## [1.5.3](https://github.com/tversteeg/emplace/compare/v1.5.2...v1.5.3) - 2024-02-07

### Fixed
- *(bugreport)* remove sys-info from bugreport so it compiles on Android Termux

## [1.5.2](https://github.com/tversteeg/emplace/compare/v1.5.1...v1.5.2) - 2024-02-06

### Fixed
- *(config)* set config path in init even if it can't be canonicalized
- *(deps)* update rust crate toml to 0.8.10
- *(deps)* update rust crate clap_complete to 4.4.10

### Other
- *(ci)* set renovate to automerge

## [1.5.1](https://github.com/tversteeg/emplace/compare/v1.5.0...v1.5.1) - 2024-02-01

### Fixed
- *(cli)* use proper clap downcasting types for paths

### Other
- *(readme)* document `cargo binstall`

## [1.5.0](https://github.com/tversteeg/emplace/compare/v1.4.3...v1.5.0) - 2024-02-01

### Added
- *(cargo-binstall)* implement cargo-binstall detection

## [1.4.3](https://github.com/tversteeg/emplace/compare/v1.4.2...v1.4.3) - 2024-01-31

### Other
- *(windows)* fix compilation error on Windows
- *(ci)* simplify a lot, with release-plz
- *(deps)* update dependencies & apply clippy lints
- Delete dependabot.yml
- Bump toolmantim/release-drafter from 5.20.0 to 5.21.0
- Bump actions/checkout from 2 to 3
- Add renovate.json
- Bump anyhow from 1.0.60 to 1.0.61
- Bump clap_complete from 3.2.3 to 3.2.4
- Bump clap from 3.2.16 to 3.2.17
- Remove myself from issue assignees
- Update dependencies
- Bump clap from 3.1.17 to 3.1.18
- Bump toolmantim/release-drafter from 5.19.0 to 5.20.0
- Bump clap from 3.1.15 to 3.1.17
- Bump indoc from 1.0.4 to 1.0.6
- Bump dialoguer from 0.10.0 to 0.10.1
- Bump log from 0.4.16 to 0.4.17
- Bump clap from 3.1.12 to 3.1.15
- Bump serde from 1.0.136 to 1.0.137
- Bump clap from 3.1.10 to 3.1.12
- Bump anyhow from 1.0.56 to 1.0.57
- Bump simplelog from 0.11.2 to 0.12.0
- Bump clap from 3.1.9 to 3.1.10
- Bump clap from 3.1.8 to 3.1.9
- Bump strum from 0.23.0 to 0.24.0
- Bump toml from 0.5.8 to 0.5.9
- Bump clap from 3.1.7 to 3.1.8
- Bump bugreport from 0.4.1 to 0.5.0
- Bump clap from 3.1.6 to 3.1.7
- Bump log from 0.4.14 to 0.4.16
- Bump strum_macros from 0.23.1 to 0.24.0
- Bump toolmantim/release-drafter from 5.18.1 to 5.19.0
- Bump clap from 3.1.5 to 3.1.6
- Bump anyhow from 1.0.55 to 1.0.56
- Bump actions/upload-artifact from 2.3.1 to 3
- Bump anyhow from 1.0.53 to 1.0.55
- Bump actions/download-artifact from 2.1.0 to 3
- Bump clap from 3.1.0 to 3.1.5
- Bump indoc from 1.0.3 to 1.0.4
- Bump clap from 3.0.14 to 3.1.0
- Bump enum_dispatch from 0.3.7 to 0.3.8
- Bump dialoguer from 0.9.0 to 0.10.0
- Bump toolmantim/release-drafter from 5.17.6 to 5.18.1
- Bump clap_generate from 3.0.2 to 3.0.3
- Bump clap from 3.0.13 to 3.0.14
- Bump clap from 3.0.12 to 3.0.13
- Bump serde from 1.0.135 to 1.0.136
- Bump clap from 3.0.11 to 3.0.12
- Bump clap_generate from 3.0.1 to 3.0.2
- Bump anyhow from 1.0.52 to 1.0.53
- Bump serde from 1.0.134 to 1.0.135
- Bump clap from 3.0.10 to 3.0.11
- Bump serde from 1.0.133 to 1.0.134
- Bump toolmantim/release-drafter from 5.17.5 to 5.17.6
- Bump clap from 3.0.9 to 3.0.10
- Bump toolmantim/release-drafter from 5.16.1 to 5.17.5
- Bump clap from 3.0.7 to 3.0.9
- Bump toolmantim/release-drafter from 5.15.0 to 5.16.1
- Bump clap from 3.0.6 to 3.0.7
- Bump clap from 3.0.5 to 3.0.6
- Bump simplelog from 0.11.1 to 0.11.2
- Bump clap from 3.0.4 to 3.0.5
- Bump clap_generate from 3.0.0 to 3.0.1
- Bump clap from 3.0.1 to 3.0.4
- Bump serde from 1.0.132 to 1.0.133
- Bump clap_generate from 3.0.0-rc.5 to 3.0.0
- Bump anyhow from 1.0.51 to 1.0.52
- Bump serde from 1.0.131 to 1.0.132
- Update clap
- Bump actions/upload-artifact from 2.3.0 to 2.3.1
- Bump serde from 1.0.130 to 1.0.131
- Bump simplelog from 0.11.0 to 0.11.1
- Bump actions/download-artifact from 2.0.10 to 2.1.0
- Bump itertools from 0.10.1 to 0.10.3
- Bump actions/upload-artifact from 2.2.4 to 2.3.0
- Bump anyhow from 1.0.48 to 1.0.51
- Bump strum_macros from 0.23.0 to 0.23.1
- Bump anyhow from 1.0.47 to 1.0.48
- Bump anyhow from 1.0.45 to 1.0.47
- Bump strum from 0.22.0 to 0.23.0
- Bump strum_macros from 0.22.0 to 0.23.0
