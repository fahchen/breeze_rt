# BreezeRt

**Project Goal**: 
breeze_rt aims to explore the potential of combining Elixir and Rust to build
an efficient and reliable web server. This project leverages the distributed nature
of Elixir and the high performance of Rust to create a server with high scalability
and fast response times.

**Project Features**:
- Uses Elixir's ThousandIsland as a sockets handler to manage incoming connections.
- When ThousandIsland receives a connection, it forwards the message to an HTTP
server built with Rust's hyper.
- hyper is responsible for handling and responding to the requests.

**Technical Advantages**:
- Utilizes Elixir's distributed features to orchestrate and schedule the
execution of Rust code for request handling.
- Rust's safety and performance ensure fast and reliable request processin
