# TCP echo server

## run the server.

```sh
$ cargo run tcp server 127.0.0.1:8080
```

## access the server.

### via telnet

```sh
$ telnet 127.0.0.1 8080
```

### via rust

```sh
$ cargo run tcp client 127.0.0.1:8080
```
