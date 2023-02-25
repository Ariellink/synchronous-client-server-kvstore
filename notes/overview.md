#bitcask - start by maintaining a log (write-ahead log 'WAL')
offsets into the on-disk logs


#Points
1. command: A request made to the database. 
	
	| 1. In-memory representation 
	| 2. a textual representation 
	| 3. a machine-readable serialized representaion 

	
	pub enum Command {
    	Set(String, String),
    	Rm(String),
	}

2. log: an on-disk squence of commands (received and executed order)

3. log pointer: a file offset into the log. 文件偏移量

4. log compaction: wen writes are issued to the database, they sometimes invalidate old log entries.
	| writing a = 0 then a = 1 makes the first log entry for "a" useless. Compaction is to reduce the stale commands from the log.

5. in-memory index: (key dir) -> (key, log pointer)

	|procedure: looking for the key in in-memory index -> use the log pointer to find the vaule in  on-disk logs
	|bitcask: the index for the entire db is stored in memory

6. index file: the on-disk represenation of the in-memory index. 


APIs: 
1. serde: serialize the set and rm command to a string
2. standard file I/O APIs: to write it to the disk


#KvStore methods:

fn KvStore::set(&mut self, key: String, value: String) -> Result<()> {}
写操作不需要 search in index
1. write the set command to disk in a sequential log
2. store the file offset of that command in the in-memory index from key to pointer?
3. when removing the key, kvs writes the rm command in the log, then removes the key from the in-memory index  ni'shi
*用户使用命令 kvs set mykey myvalue, 创建一个 set command包含 key和 value。序列化这条 command为 string, 将序列化后的 command append 到 log file中。成功返回 0，失败返回 non-zero.

fn KvStore::remove()
将 rm command写入日志落盘
更新in-memory index,将 in-memory index 中的 key 删除
*kvs rm mykey
*search in the in-memory index
* check if the key exists
**fails -> "Key not found" and exit with code non-zero
**succeeds -> 从 index 中删除掉 key，并获得 index.value
		   -> compact size + 之前set(key,v)的长度
		   -> 创建 rm command包含 key， 并将其序列化append到 log 中。
		   -> compact size + rm cmd的长度
		   -> exits with 0


fn KvStore::get()
1. search the index 找 key 和 offset 和 file_id
2. 在 log 中 找到 value
*read操作不需要记日志用户调用 kvs get mykey, 找到 index中的 key 对应的 log pointer。
*check the map for the log pointer? 
**fails -> "Key not found" and exit with code 0
**succeeds -> 反序列化 command 来得到 key 对应的最新的 value
		   -> print tha value to stdout and exits with 0

fn KvStore::recover()
* The log is a record of the transactions committed to the database. By "replaying" the records in the log on startup we reconstruct the previous state of the database.
1.application startup, 遍历 disk logs 从旧到新，重建 in-memory index, 来将所有 k/v pair 存储到内存中。
2. 

fn KvStore::compact()
