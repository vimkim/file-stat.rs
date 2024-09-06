# File Stat Server in Rust

A demo server to learn rust concurrency and async operations with tokio. 

You should prepare a large random file (5 - 10 MB) to test this.

The client queries the word count and line count of the file to the server.


![image](https://github.com/user-attachments/assets/db33d5c6-a2b7-4079-bb0a-ceb374125db6)

![image](https://github.com/user-attachments/assets/f357bf77-57e5-418e-b32f-f1a77310a48c)



### prepare

```bash
seq 10000 > large.txt
```

### basic usage

```
# server
cargo run
```

```
# client
cargo run <filename> <req_num>
```

### Have fun

```bash
seq 1 20 | xargs -I {} -P 20 target/debug/file-stat-client "/large-file.txt" {}

# or

parallel -j 20 target/debug/file-stat-client "/large-file.txt" ::: {1..20}
```

The results will let you understand the basics of rust concurrency.

---
