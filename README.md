# quiet-stroll

## Decription

This is repository is intend to create a POC of using rust to deliver client/server FS tools to:
- **walk**, crawl the file system from an entrypoint in the file tree
- **listdir**, simply list the files in a directory
- **glob**, use glob 

## Under the hood
- walk is perform by [jwalk](https://github.com/Byron/jwalk)
- REST server is perform by [dropshot](https://github.com/oxidecomputer/dropshot)
- glob is perform by [glob](https://docs.rs/glob/latest/glob/)
