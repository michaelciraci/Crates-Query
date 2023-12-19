## crates-query

Command line tool to query [crates.io](https://crates.io) index

#### Installation
```
cargo install crates-query
```

On NetBSD a pre-complied binary is available from the official repositories. To install it, simply run:
```
pkgin install crates-query
```

### Usage

There are several query subcommands:
 * [Dependencies](#list-a-given-crates-dependencies)
 * [Rust Version](#get-the-minimum-rust-version)
 * [Features](#query-features-available)
 * [Versions](#get-versions-published)

By default `crates-query` will always use the latest available, but a specific version can be specified using the `--ver` flag

#### List a given crates dependencies
```
$ crates-query hyper dependencies
hyper 1.0.1 dependencies:

bytes ^1
form_urlencoded ^1
futures-channel ^0.3
futures-util ^0.3
h2 ^0.4
http ^1
http-body ^1
http-body-util ^0.1
http-body-util ^0.1
httparse ^1.8
httpdate ^1.0
itoa ^1
libc ^0.2
pin-project-lite ^0.2.4
pretty_env_logger ^0.5
serde ^1.0
serde_json ^1.0
spmc ^0.3
tokio ^1
tokio ^1
tokio-test ^0.4
tracing ^0.1
want ^0.3
```

#### Get the minimum rust version
```
$ crates-query hyper rust-version
hyper 1.0.1 rust version:

Minimum Rust Version: 1.63
```

#### Query features available
```
$ crates-query hyper features
hyper 1.0.1 features:

server
nightly
default
full
http2
ffi
http1
tracing
client
```

#### Get versions published
```
$ crates-query rand_core versions
rand_core versions:

0.0.1
0.1.0-pre.0
0.1.0
0.2.0-pre.0
0.2.0
0.2.1
0.3.0
0.2.2
0.4.0
0.3.1
0.5.0
0.4.1
0.4.2
0.5.1
0.6.0
0.6.1
0.6.2
0.6.3
0.6.4
```


