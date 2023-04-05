# Module std::net

`TcpListener` and `TcpStream` provide functionality for communication over TCP.  [std::net](https://doc.rust-lang.org/std/net/index.html)

`Struct std::net::TcpListener`: A TCP socket server, listening for connections.

*server side*: Creates a TCP listener bound to 127.0.0.1:80
```rust
let listener = TcpListener::bind("127.0.0.1:80").unwrap();
//pub fn bind<A: ToSocketAddrs>(addr: A) -> std::io::Result<TcpListener>
```

`Struct std::net::TcpStream`: A TCP stream between a local and a remote socket.  

*client side:*
```rust
let mut stream = TcpStream::connect("127.0.0.1:34254")?;
//pub fn connect<A: ToSocketAddrs>(addr: A) -> std::io::Result<TcpStream>
```

### ERRORs
`-> std::io::Result<()>`

### server 处理请求

```rust
use std::net::{TcpListener, TcpStream};

fn handle_client(stream: TcpStream) {
    // ...
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:80")?;

    // accept connections and process them serially
    for stream in listener.incoming() {
        handle_client(stream?);
    }
    Ok(())
```
