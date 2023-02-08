## Mini Project week3: Rust CLI: Downloading PDFs from a URL

A Rust CLI tool to download PDFs providing a URL and a folder name

### Goals

* Build a Rust Command-Line tool that downloads pdfs linked in a certain webpage



#### Current Status (To Modify)

* Lint correctly, still has issue with the version fit of one of the crates, need to figure out later




#### Future Improvements
* Increase a step before download - go through a page filled with links, and figure out which among them has the data we want (and the pdfs we want). Excluding those with 404 not found
* Extract 'first year enrollment' for all top 50 schools from each CDS pdfs
* Calculate the percentage of first-year students attending US higher education who attends a top 50 school


### Building and Running (to modify)

* Build:  cd into rdedupe and run `make all`
* Run:  `cargo run -- dedupe --path tests --pattern .txt`
* Run tests:  `make test`



#### References

* [example project repo: rdedupe](https://github.com/noahgift/rdedupe)
* [rust-mlops-template](https://github.com/nogibjj/rust-mlops-template)
* [US-Universities-CDS-link-summary(in Chinese)](https://aadps.net/2022/14616.html)
* ['lopdf' crate docs](https://docs.rs/lopdf/0.29.0/lopdf/)
* ['reqwest' crate docs](https://docs.rs/reqwest/latest/reqwest/)

