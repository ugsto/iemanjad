# Iemanjad

![https://img.shields.io/badge/Rust-000000.svg?style=flat&logo=Rust&logoColor=white](https://img.shields.io/badge/Rust-000000.svg?style=flat&logo=Rust&logoColor=white)

---

> [!CAUTION]
> This application is **not** production ready.

## Overview

iemanjad is a daemon server designed to manage general posts. This server acts as a CRUD hub.

## Getting Started

Before starting, ensure you have Rust installed on your system with `cargo`.

### Installation

1. Clone the . repository:

```sh
git clone https://github.com/ugsto/iemanjad.git
```

2. Install the application:

```sh
cargo install --path iemanjad
```

### Running

Use the following command to run iemanjad:

```sh
iemanjad
```

By default it expects a directory `/etc/iemanjad/` in which the current user have write permissions, but you can change it with the `--db-address` flag, specifying another path to persist data or use a separate instance of surrealdb. For example:

```sh
iemanjad --db-address speedb:///tmp/iemanjad  # Keep in mind that for persistent databases, /tmp is a terrible idea.
```

Another configurable trait is where to listen for incoming connections. By default, it listens on `127.0.0.1:7029`, but you can use unix sockets or another address with the `--api-bind` flag, like this:

```sh
iemanjad --api-bind /tmp/iemanjad.sock
```

### Tests

To execute tests, run (in the project directory):

```sh
cargo test
```
