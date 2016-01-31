![Carcasian](https://raw.githubusercontent.com/albertofem/carcasian/master/logo.png)

[![Build Status](https://travis-ci.org/albertofem/carcasian.svg?branch=master)](https://travis-ci.org/albertofem/carcasian)

In memory key-value database written in Rust using Redis protocol. This project was made to illustrate the contents of my lightning talk at Social Point HQ (Barcelona). This code should not be used in production, it's just made for learning purposes.

# Supported commands

This database uses a small subset of commands based on the Redis protocol: http://redis.io/topics/protocol

Currently, the following commands are supported:

* SET <key> <value>
* GET <key>
* DEL <key>
* EXISTS <key>
* SADD <key> <member>
* SISMEMBER <key> <member>
* SREM <key> <member>
* SMEMBERS <key>

# Compiling and running

You first need to install Rust (nightly). You can use **[multirust](https://github.com/brson/multirust)** to do this in an easy way. When all is run, you can just run:

```
make server
```

To fire up the server (by default, listening on port 8991). You can also use the cargo command directly:

```
cargo run --bin carcasian-server -- --host 127.0.0.1 --port 8991
```

# Documentation

If you want to read the inline documentation, you can run:

```
make doc
```

This will open an HTML representation in your browser.

# Comments

There is a great ton of comments in the code. Some of them could not be entirely accurate of precise, so you should take them with a grain of salt.

# Talk

The slides of this talk are available at speaker deck:

[https://speakerdeck.com/albertofem/and-thou-shalt-have-rigour-a-gentle-introduction-to-rust](https://speakerdeck.com/albertofem/and-thou-shalt-have-rigour-a-gentle-introduction-to-rust)

# TODO

* Implement expirations
* Implement INFO command, operations per second and memory usage
* Implement hashes (HSET, etc.)
* Implement MONITOR
