<h1 align="center">when3meet</h1>
<div align="center">
 <strong>
   🦀 The Rust when2meet CLI
 </strong>
</div>

<br />


<div align="center">
  <!-- Github Actions -->
  <a href="https://github.com/garrettladley/when3meet/actions/workflows/general.yml">
    <img src="https://github.com/garrettladley/when3meet/actions/workflows/general.yml/badge.svg"
      alt="actions status" />
  </a>
  <!-- Version -->
  <a href="https://crates.io/crates/when3meet">
    <img src="https://img.shields.io/crates/v/when3meet.svg?style=flat-square"
    alt="Crates.io version" />
  </a>
  <!-- Docs -->
  <a href="https://docs.rs/when3meet">
    <img src="https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square"
      alt="docs.rs docs" />
  </a>
  <!-- Downloads -->
  <a href="https://crates.io/crates/when3meet">
    <img src="https://img.shields.io/crates/d/when3meet.svg?style=flat-square"
      alt="Download" />
  </a>
</div>

<div align="center">
  <h4>
    <a href="#install">
      Install
    </a>
    <span> | </span>
    <a href="#usage">
      Usage
    </a>
    <span> | </span>
    <a href="#contributing--issues">
      Contributing & Issues
    </a>
    <span> | </span>
    <a href="https://docs.rs/when3meet">
      Docs
    </a>
  </h4>
</div>

<br />

<div align="center">
  <small>Built with ❤️ and 🦀 by Garrett Ladley</small>
</div>



## Install

```sh
cargo install when3meet
```

## Usage

```
when3meet -h
CLI tool to find the optimal time to meet given a when2meet URL

Usage: when3meet [OPTIONS] --when2meet-url <WHEN2MEET_URL>

Options:
  -r, --required-people <REQUIRED_PEOPLE>...
          The people required at the meeting. If not provided, assumed to be all people
  -f, --flexible-naming
          Perform case insensitive contains based matching on required people
  -w, --when2meet-url <WHEN2MEET_URL>
          The URL to the when2meet page
  -o, --output-file-path <OUTPUT_FILE_PATH>
          The output file path. If not provided, it will be printed to stdout
  -h, --help
          Print help
  -V, --version
          Print version
```

## Contributing & Issues

If you have would like to contribute or encounter any issues, feel free to open a PR or issue!
