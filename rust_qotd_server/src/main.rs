use std::io::Read;
use std::net::{TcpListener, TcpStream};
use dotenv::dotenv;
use std::fmt;
use rusqlite::{params, Connection, Result};
use std::thread;
use std::collections::HashMap;


// TODO: change this to the correct port later.
const PORT: u32 = 17;

struct Quote {
    quote: String,
    author: String
}

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


// <()> says it returns a result, but we dont care what. Either Ok or Error.
// <(i32, String)> could be used for the normal wrapper. 
// I think? 

// note, the <TcpListner> is shorthand for <TcpListner, Std::io::Error> 
fn start_server() ->  std::io::Result<TcpListener>{
    match dotenv() {
        Ok(_) => println!("Loaded env vars successfully."),
        Err(e) => println!("Error loading env vars: {}", e)
    }
    // get the sqllite file on startup, or make it. 
    // also get the default_quote. 

    // TODO - does this mean that main needs to be in a while true loop?

    let ip: String = format!("127.0.0.1:{}", PORT);
    let tcp_listener: TcpListener = TcpListener::bind(&ip)?;
    println!("Server is listening at: {}", ip);
    Ok(tcp_listener)
}

fn update_qotd(quote: &str) {
    // update sqllite file with new qotd.
}

fn serve_quote(stream: &mut TcpStream) {
    // gives us a quote. If no quotes avaliable, AND sqllite is empty,
    // serve the default quote.
    // otherwise, wrap around, get latest date quote, find offset from that date
    // and wrap around array. 

    // TODO: dont unwrap! fix this.
    let default_quote = Quote {
        quote: dotenv::var("DEFAULT_QOTD").unwrap(),
        author: dotenv::var("DEFAULT_AUTH").unwrap()
    };

}

fn conn_handler(tcp: &TcpListener) ->  std::io::Result<()>{
    // handle connections generically. if they send a command, do sumthin 
    // TODO, startup should take an IP to run on, right? 
    // ? propogates errors. returns err if fails, but unwraps if Ok()

    // else, serve quote and sever connection. 


    for stream in tcp.incoming() {
        match stream {
            // you can bind variables here. thats why mut stream works.
            Ok(mut stream) => {
                // TODO - what happens if ? produces an error here? does it kill the whole func? 
                // do we handle that result somehow? What does it do? output to stderr?
                println!("Connection from: {}", stream.peer_addr()?);
                let mut buffer = [0; 1024];

                match stream.read(&mut buffer) {
                    Ok(n) if n == 0 => {
                        // writing is mutation.
                        serve_quote(&mut stream);
                    },
                    Ok(n) => {
                        // check for admin commands.
                    },
                    Err(e) => println!("Error reading stream: {}", e)
                }
                


            },
            Err(e) => println!("Error receiving stream: {}", e)
        }

        // this is array syntax.


        // if adm-pw sent in initial message, treat user as adm in seperate thread. 
        // else, send the quote and close their connection.

    }

    Ok(())

}

fn command_interpreter() {
    // interpret commands sent in stream
}

fn main() -> std::io::Result<()>{

    // ? propogates errors. returns err if fails, but unwraps if Ok()
    let tcp_listener: TcpListener = start_server()?;



    // TODO, startup should take an IP to run on, right? 
    

    conn_handler(&tcp_listener);

    // implmenent
    // needs to listen for commands to update QOTD (and hence check for pw to do this, store PW in .env?)
    // needs to respond to reqs with no command.

    // we dont want the function to have ownership.

    Ok(())
}
