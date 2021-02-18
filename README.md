# How to run it

- [Install rust](https://rustup.rs/) if you haven't already

**Run locally:**

Server: `HOST=localhost PORT=80 cargo run --bin server`

Client: `HOST=localhost PORT=80 cargo run --bin client`

**Run on remote server + local client**

Server: `HOST=localhost PORT=80 cargo run --bin server`

Client: `HOST=134.122.15.165 PORT=80 cargo run --bin client` (where the sever IP is 134.122.15.165)
