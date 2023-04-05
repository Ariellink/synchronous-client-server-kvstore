
Struct KvServer <E>

impl KvSever

- `pub fn serve(&mut self, addr: String) -> Result<()>`  
    
- `fn handle_connection(&mut self, mut stream: TcpStream) -> Result<()>`