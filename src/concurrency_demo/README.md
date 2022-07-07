# Concurrency Demo

A demo project to present a concurrency features of the Rust programming
language.

The HTTP server is based on the 'The Rust Programming Language' book and has
been adapted for demonstration needs.

## Detail Description

Project consists of two independent executable i.e. server and client
application, that communicate with each other using HTTP protocol.

### Server

The HTTP server awaits for HTTP request and sends a simple HTML response which
can be rendered by the browser.

If client's request contains integer values, the server, in the background, will
sum these values, and the result is included in the response.

Empty or non-integer request will be treated as value of `1`.

### Client

Sends HTTP requests with some abstract integer data, which can represent e.g.
sensor measurement. The value is currently hardcoded but any other value can be
handled by the server as well.

## Concurrency Issues

The following concurrency issues are demonstrated in this project.

### Handling Several Clients in Parallel

As explained in 'The Rust Programming Language' book, the server is a
multi-threaded one, with a thread pool capable of handling 1024 client request
in parallel.

### Non-blocking handling of the request data

Server is processing request data in the background in a non-blocking way,
thanks to Multi-producer, single-consumer FIFO message queue (MPSC).

Threads handling HTTP request are streaming data to the worker which is
processing them, and worker is sending processed data back to the main thread.

```
[Request 1] --\
[Request 2] ---> [MPSC] -> [Worker] -> [Main Thread]
[Request 3] --/
```

## Building and Running

### Server

Issue the following commands to start the server:

```bash
cd server
cargo run
```

### Client

Issue the following commands to send one request:

```bash
cd client
cargo run
```

To loop client, issue the following command:

```bash
while true; do client/target/debug/client; done
```

#### Browser

Browser can also be used to send request, by loading the following page
<http://127.0.0.1:7878>.

#### Curl

Issue the following command to send a request with a value of 13:

```
curl http://127.0.0.1:7878/13
```
