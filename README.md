# rust-server
##description
- this is cargo workspace for building a http server in rust 

```
git clone https://github.com/onchezz/rust-server.git
```
cd into file the run 

```
cargo run -p httpserver
```
after running go to the your web browser and paste 
```
localhost:3000/
```
you can also view this items
```
http://localhost:3000/api/shipping/orders
```
you can also view the tcp server in the cargo  work space 
- start the tcp server from one terminal window  by running this command
```
cargo run -p  tcpserver
```
- start the tcp client in another termial  window 
```
cargo run -p tcpclient
```

expected output 
[Got response from server:"Hello"]
