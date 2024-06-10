# Oxide

Experimental POC to use `iroh-net` as a p2p library to make a Quic/TCP Tunnel for infosec.

### Usage

Server (Alice) starts listening on all interfaces and listens for incoming traffic. Forwarding it to port 8080 on localhost.

```
oxide forward --host localhost:8080
```


Client (Bob) connects to Alice and is able to view Alices webpage `localhost:8080` on `localhost:3000`

```
oxide connect-tcp --addr localhost:3000 nodeaticket
```

The devices will establish a connection. No configuration required. 
