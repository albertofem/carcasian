carcasian
====

In memory key-value database written in Rust using Redis protocol. This project was made to illustrate the contents of my lightning talk at Social Point HQ (Barcelona). This code should not be used in production, it's just made for learning purposes.

# Supported commands

This database uses a small subset of commands based on the Redis protocol: http://redis.io/topics/protocol

Currently, the following commands are supported:

SET <key> <value>
GET <key>
INFO

The INFO will only account for operations per seconds and peak memory usage.

# Comments

There is a great ton of comments in the code. Some of them could not be entirely accurate of precise, so you should take them with a grain of salt.

# Talk

The slides of this talk are available at speaker deck:


There is also a video available in SocialPoint engineering YouTube channel:
