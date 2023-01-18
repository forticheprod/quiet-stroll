# quiet-stroll

## Decription

This repository is intend to create a POC of using rust to deliver client/server FS tools to:

- **walk**, crawl the file system from an entrypoint in the file tree
- **listdir**, simply list the files in a directory
- **glob**, use glob

## Under the hood

- walk is perform by [jwalk](https://github.com/Byron/jwalk)
- REST server is perform by [rocket](https://rocket.rs)
- glob is perform by [glob](https://docs.rs/glob/latest/glob/)

## Installation

## Before running

## License

Quiet Stroll is licensed under MIT License (LICENSE-MIT or <http://opensource.org/licenses/MIT>)
