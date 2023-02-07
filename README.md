[![Tests](https://github.com/noahgift/rdedupe/actions/workflows/tests.yml/badge.svg)](https://github.com/noahgift/rdedupe/actions/workflows/tests.yml)
[![Build binary release](https://github.com/noahgift/rdedupe/actions/workflows/release.yml/badge.svg)](https://github.com/noahgift/rdedupe/actions/workflows/release.yml)
[![Clippy](https://github.com/noahgift/rdedupe/actions/workflows/lint.yml/badge.svg)](https://github.com/noahgift/rdedupe/actions/workflows/lint.yml)
[![Rustfmt](https://github.com/noahgift/rdedupe/actions/workflows/rustfmt.yml/badge.svg)](https://github.com/noahgift/rdedupe/actions/workflows/rustfmt.yml)
（To Modify）

## Project 1: Rust CLI: Data Gathering - Finding the CDS pdfs and download them

A Rust based tool to download common data set PDFs through layers of links

### Goals

* Build a Rust Command-Line tool that downloads pdfs containing Common Data Set (CDS) for top US higher education institutions.
* Later, the tool could be improved to extract certain information from the downloaded pdfs, and calculate certain statistics based on the extracted info.

![Architectural diagram](png link)（To Modify）


#### Current Status (To Modify)

* Added 
* Added 




#### Future Improvements

* Extract 'first year enrollment' for all top 50 schools from each CDS pdfs
* Calculate the percentage of first-year students attending US higher education who attends a top 50 school


### Building and Running (to modify)

* Build:  cd into rdedupe and run `make all`
* Run:  `cargo run -- dedupe --path tests --pattern .txt`
* Run tests:  `make test`

### OS X Install (to modify)

* Install rust via [rustup](https://rustup.rs/)
* Add to `~/.cargo/config`

```bash
[target.x86_64-apple-darwin]
rustflags = [
  "-C", "link-arg=-undefined",
  "-C", "link-arg=dynamic_lookup",
]

[target.aarch64-apple-darwin]
rustflags = [
  "-C", "link-arg=-undefined",
  "-C", "link-arg=dynamic_lookup",
]
```
* run `make all` in rdedupe directory


#### References

* [example project repo: rdedupe](https://github.com/noahgift/rdedupe)
* [rust-mlops-template](https://github.com/nogibjj/rust-mlops-template)
* [US-Universities-CDS-link-summary(in Chinese)](https://aadps.net/2022/14616.html)
* ['lopdf' crate docs](https://docs.rs/lopdf/0.29.0/lopdf/)
* ['reqwest' crate docs](https://docs.rs/reqwest/latest/reqwest/)

