
use std::str;
use std::net::{TcpListener};
mod admCommands;
mod serverHandling;
mod dbOperations;


// implementation notes: 
// need to open port for listening for connections
// need to write a start_server() function that gets the QOTD or makes the DB and returns err if no quotes avaliable.
// need a function to check QOTD status - how many quotes avaliable. 
// need a failsafe - send a default quote if no quotes today. 
// need a option to send update to default quote (and hence a general way of interpreting commands to the server.)
// tcp: on connection - send data and close
// udp: client sends datagram to port. On datagram receive - send quote.
// need a function to insert QOTDs into the sqllite file 
// need to rate limit IPs. Memoizable? 

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



fn update_qotd(quote: &str) {
    // update sqllite file with new qotd.
}

fn main() -> std::io::Result<()>{

    // ? propogates errors. returns err if fails, but unwraps if Ok()
    // TODO: on startup, take an IP
    let tcp_listener: TcpListener = serverHandling::start_server()?;
    serverHandling::conn_handler(&tcp_listener);

    // implmenent
    // needs to listen for commands to update QOTD (and hence check for pw to do this, store PW in .env?)
    // needs to respond to reqs with no command.

    // we dont want the function to have ownership.

    Ok(())
}
