# Gentle practical introduction to Rust

Simple Rust project, incrementally growing.

Check out intermediate `git` commits to see the program grow:

 1. Hello world
 1. Introduction to common Rust language patterns
 1. A regular expression parser (see https://github.com/stefan-kaestle/code4fun-rust for a simple performance comparison with Java and Python)
 1. A REST API server that exposes this in the browser via json
 1. A database backend, fetching data from a mysql server

Finally, a web-server (with `actix`) using a mysql database as backend (with `diesel`).

For participants of the Code4Fun, there is still a mysql server up and running (and will be for a while longer):

`echo DATABASE_URL="mysql://rustuser:\%23Code4Fun@130.61.92.85/rustdemo" > .env`

Otherwise, you can also setup your own. The working set can be downloaded from:

https://dev.mysql.com/doc/employee/en/
