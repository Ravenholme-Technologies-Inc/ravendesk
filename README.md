# RavenDesk

RavenDesk is a command-line tool for keeping lightweight notes alongside a
project. Notes are stored locally in the directory where you run the command,
so they stay close to the project they belong to.

> **Development status: actively under development and incomplete.** RavenDesk
> currently provides basic note creation, listing, and deletion. It is an early
> version, and its commands, storage format, and behavior may change. It is not
> yet a complete project-management or task-tracking application.

## Features

- Add a note with text supplied on the command line.
- List notes saved for the current working directory.
- Delete a note by its ID.
- Store notes as readable JSON in a local hidden file.

## Requirements

- [Rust](https://www.rust-lang.org/tools/install) and Cargo.

## Build and run

Clone the repository, enter its directory, and build the executable:

```sh
git clone https://github.com/Ravenholme-Technologies-Inc/ravendesk.git
cd ravendesk
cargo build
```

Run the development build with Cargo:

```sh
cargo run -- <command>
```

Or build an optimized executable:

```sh
cargo build --release
```

The resulting executable is located at `target/release/ravendesk` (on Windows,
`target\release\ravendesk.exe`).

## Usage

Run commands from the project directory whose notes you want to manage. For
example:

```sh
cargo run -- add Remember to update the project documentation
cargo run -- list
cargo run -- delete 1730832000123
```

The `add` command joins all text arguments with spaces. The `list` command
prints each note's ID and content. Use the ID printed or stored for a note with
`delete`.

Once built, you can invoke the executable directly instead:

```sh
ravendesk add Remember to update the project documentation
ravendesk list
ravendesk delete 1730832000123
```

The CLI also supports standard help and version flags:

```sh
ravendesk --help
ravendesk --version
ravendesk add --help
```

## Local data

RavenDesk reads and writes `.ravendesk.json` in the **current working
directory**. Running it from different directories uses different note files.
The file is created when a note is first added; listing notes before that
reports that no notes were found.

The file contains a JSON array. Each note currently has this shape:

```json
{
  "id": 1730832000123,
  "content": "Remember to update the project documentation",
  "created_at": 1730832000
}
```

`id` is generated from the current Unix time in milliseconds, while
`created_at` is the Unix time in seconds. The data file is local to the
directory and is not intended as a synchronized or shared database. Back it up
before editing or removing it manually. If the JSON is malformed, RavenDesk
reports an error rather than replacing it with an empty list.

## Development

Build and check the project with:

```sh
cargo check
cargo test
```

The current codebase does not yet include automated tests. Changes to the
storage format or command behavior should be treated as potentially
incompatible while the project remains under active development.

## Current limitations and planned direction

The implementation is intentionally small and incomplete:

- Only adding, listing, and deleting notes are implemented.
- There are no task-management commands, despite the broader project-oriented
  goal.
- A GUI & CLI application that gathers all the notes from the system, and
  summarizes them is planned. Including LLM support for summary and diagnosis.
- There is no edit, search, export, synchronization, or backup command yet.
- The JSON file is rewritten when notes are added or deleted; concurrent
  writes are not coordinated.

These are areas for possible future development, not features currently
available in RavenDesk.

## License

No license is currently specified in this repository.
