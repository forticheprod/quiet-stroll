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

`cargo install quiet-stroll`

## Usage

Quiet Stroll is based under [rocket](https://rocket.rs) framework

### Testing

#### Hello World

By default you will see `Hello, world!` by visiting <http://localhost:8000>.

#### Coffee: HTCPCP

You can test error code with the [Hyper Text Coffee Pot Control Protocol](https://en.wikipedia.org/wiki/Hyper_Text_Coffee_Pot_Control_Protocol)
by visiting <http://localhost:8000/coffee> and not `coffe`

## Documentation

You can see the swagger documentation by visiting <http://localhost:8000/docs>

## License

Quiet Stroll is licensed under MIT License (LICENSE-MIT or <http://opensource.org/licenses/MIT>)
