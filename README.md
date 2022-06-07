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
expected browser output :

[{"order_id":1,"order_date":"21 Jan 2020","order_status":"Delivered"},{"order_id":2,"order_date":"2 Feb 2020","order_status":"Pending"}]

you can also view the tcp server in the cargo  work space 
- start the tcp server from one terminal window  by running this command
```
cargo run -p  tcpserver
```
- start the tcp client in another termial  window 
```
cargo run -p tcpclient
```

expected output :      
[Got response from server:"Hello"]
