use std::net::TcpListener;
mod adm_commands;
mod server_handling;
mod db_operations;
use threadpool::ThreadPool;

// functionality todos:
    // rate limiting 
    // shutdown command
    // reboot command? will this require shell escaping? 

// RFC: 865
// TCP:
    // A server listens for TCP connections on TCP port
    // 17.  Once a connection is established a short message is sent out the
    //connection (and any data received is thrown away).  The service
    // closes the connection after sending the quote.
// UDP:
    // UDP Based Character Generator Service
    // Another quote of the day service is defined as a datagram based
    // application on UDP.  A server listens for UDP datagrams on UDP port
    // 17.  When a datagram is received, an answering datagram is sent
    // containing a quote (the data in the received datagram is ignored).

fn main() -> std::io::Result<()>{

    // ? propogates errors. returns err if fails, but unwraps if Ok()
    // TODO: on startup, take an IP
    let pool_size = dotenv::var("POOL")
        // Use a thread pool of 4 as a fallback if fails. 
        .unwrap_or_else(|_| "4".to_string())
        .parse::<usize>()
        .unwrap_or(4);

    let pool = ThreadPool::new(pool_size); 
    let tcp_listener: TcpListener = server_handling::start_server()?;
    server_handling::conn_handler(&tcp_listener, &pool).unwrap();

    Ok(())
}
