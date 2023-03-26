# 设计应用层协议

命令如何在client和server之间传输？
由于这里直接使用了tcp协议的网络接口来传输，我们需要处理粘包的问题。

这里直接使用serde现成的reader/writer接口去实现。构建了一个Client结构体对tcpstream进行了封装。

## Client

client --send_request--> server (1)
                         server handles the request (2)
client <--send_response  server (3)
client handles the response (4)

### (1)
client needs to know 'addr:port' of the server
client sends the **message**
1. Messagen constructs
 (1) message is `TcpStream`
 (2) what is request?
     我们需要将`ArgMatches`转变成request，然后在把request发到server端。转换类似于模式匹配。我们需要把request的几种形式枚举出来。
     和之前用于写日志的command.rs中的枚举也是类似的, `enum Command`不需要有get, 因为get需要记录到strcuted logs中。
     request还是有三种 GET, SET, RM。也必须要支持序列化和反序列化。
（3）使用一个BufWriter在每次写完数据后flush来降低系统的开销。
（4）response则是使用deserializer接口构建reader,并指定对应的反序列类型，以达到读长度，再读数据。

    ```rust
    use serde::Deserialize;
    use serde::Serialize;

    #[derive(Serialize,Deserialize,Debug)]
    pub enum Request {
        SET(String,String),
        RM(String),
        GET(String),
    }   

    ```
    Client::request(&request) 
    serde_json::to_writer(writer: W, value: &T): serialize the given data struct as JSON into IO stream. 
        -W: Write,
        -T: ?Sized + Serialize,

    
# 有关于TCP流式协议和粘包的问题
1. 由于tcp是面向流stream的协议，即协议的内容是像流水一样的字节流，内容与内容之间没有明确的**分界标志**，需要我们人为地去给这些协议划分边界。tcp不会按照应用开发者的期望保持send输入数据的边界，导致接收侧有可能一下子收到多个应用层报文，需要应用开发者自己分开
    > stream -> datagram : 流到数据包转换
    靠设计一个带包头的应用层报文结构就能解决。包头定长，以特定标志开头，里带着负载长度，这样接收侧只要以定长尝试读取包头，再按照包头里的负载长度读取负载就行了，多出来的数据都留在缓冲区里即可。

2. 用户数据被tcp发出去的时候，存在多个小尺寸数据被封装在一个tcp报文中发出去的可能性。这种“粘”不是接收侧的效果，而是由于Nagle算法（或者TCP_CORK）的存在，在发送的时候，就把应用开发者多次send的数据，“粘”在一个tcp报文里面发出去了，于是，先被send的数据可能需要等待一段时间，才能跟后面被send的数据一起组成报文发出去。

    > 其实在99%的情况下，Nagle算法根本就不会导致可感知的传输延迟，只是在某些场景下，Nagle算法和延迟ACK机制碰到一起，才会导致可感知的延迟。TCP_NODELAY

## Takeways
> client多次send的数据被server一下子收到，需要在应用层协议中将tcp stream 转变成datagram. 

> TCP_NODELAY


# Rust std::net::TcpStream

`TcpStream::connect()` 打开了client到server的tcp连接，并创建了一个tcpstream。之后数据可以通过被reading或writing传输。  

#### `TcpStream`实现了Read和Write的trait。   
裸用的话，read就是`TcpStream::read(&mut self, buf:&mut [u8]) -> Result<usize>`。从source pull字节到指定的buffer中，然后返回读了多少字节。  
write就是将一个buffer写入指定的writer,`fn write(&mut self, buf: &[u8]) -> Result<usize>`


client <----tcpstream----> server

```rust
use std::net::TcpStream;
fn main() -> std::io::Result<()> {

    let mut stream:: TcpStream = TcpStream::connect("ipaddress:port")?;
    stream.write()
}
```

# Client 
https://docs.rs/serde_json/latest/serde_json/de/struct.IoRead.html 

```rust
struct Client {
    //for response
    reader: Deserializer<serde_json::de::IoRead<BufReader<TcpStream>>>,
    //for request
    writer: BufWriter<TcpStream>,
}
```
用TcpConnection得到的tcpstream来建立构建Client的reader和writer。

`Struct serde_json::Deserializer`: A structure that deserializes JSON into Rust values.

*doc:*
https://docs.rs/serde_json/latest/serde_json/struct.Deserializer.html 

```rust
impl<R> Deserializer<IoRead<R>>
where
    R: Read,

pub fn from_reader(reader: R) -> Self

//Creates a JSON deserializer from an io::Read.

//Reader-based deserializers do not support deserializing borrowed types like &str, since the std::io::Read trait has no non-copying methods – everything it does involves copying bytes out of the data source.

```
### 构造Client
```rust
impl Client {
    fn new(addr: &str) -> Result<Client> {
        let stream = TcpStream::connect(addr)?; 
        Ok(Client {
            reader: Deserializer::from_reader(BufReader::new(stream.try_clone())),
            writer: BufWriter::new(stream),
        })
```
BufWiter<W>: 尖括号中的东西就是实现了write trait的对象。

### Client::request
1. 前面的构造函数`Client::new()`相当于只是将TcpSteam包进了buffer, 再封装进Client类。  
2. 因为客户端已经输入了命令（1/3），接下来需要把命令解析出来的东西，构造request类。  
3. request()将类进行序列化

request函数
fn request(&mut self, request: &Request) -> Result<Option<String>> {...}