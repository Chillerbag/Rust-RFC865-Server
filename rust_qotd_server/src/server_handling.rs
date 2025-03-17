use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::io::{Write, Read};
use std::thread;
use dotenv::dotenv;
use threadpool::ThreadPool;
// functions for the server
use crate::adm_commands;
use crate::db_operations;

const PORT: u32 = 17;

// note, the <TcpListner> is shorthand for <TcpListner, Std::io::Error> 
pub fn start_server() ->  std::io::Result<TcpListener>{
    match dotenv() {
        Ok(_) => println!("Loaded env vars successfully."),
        Err(e) => println!("Error loading env vars: {}", e)
    }
    // get IP.
    let ip_raw: String = match dotenv::var("SERVER_IP") {
        Ok(ipstr) => ipstr,
        Err(e) => {
            println!("Unable to load server IP {}", e);
            "127.0.0.1".to_string()
        }
    };

    let ip: String = format!("{}:{}", ip_raw, PORT);
    println!("IP set up: {}", ip);
    // failing here
    let tcp_listener: TcpListener = TcpListener::bind(&ip)?;
    println!("Server is listening at: {}", ip);
    Ok(tcp_listener)
}

fn use_admin_thread(stream: Arc<Mutex<TcpStream>>, buffer: &mut [u8; 1024], buf_fill: &usize)  {

    // Arc ensures the reference to thread_status is shared across threads
    // so we can ensure only one admin thread exists at any given time. 
    // Mutex is what we use to lock. 
    // this mutex is unlocked.
    // init this one as false. 

    // we should probably do this in the thread but its ok for now
    // buf_string is how much raw data we read, hence why it is sliced.

    // not great for memory, but creates a COW (clone on write, and makes it owned.)    
    // not a big deal because its cloned.
    let raw_commands = String::from_utf8_lossy(&buffer[..*buf_fill]).into_owned();

    let adm_thread_status = Arc::new(Mutex::new(false));

    // clone to keep consistency. 
    let adm_thread_status_clone = Arc::clone(&adm_thread_status);

    // closure. move keyword means we take ownership of the vars inside the 
    // env. In this case, adm_thread_status
    thread::spawn(move || {

        // lock the admin thread.
        // TODO: fix unwrap!
        let is_active = adm_thread_status_clone.lock().unwrap();
        
        // dereffing gives us access to inside the MutexGuard
        // first param ofthe mutexGuard is lifetime
        if *is_active {
            // lock method locks and gives access to TcpStream
            // TODO remove unwrap. 
            let mut unlocked_stream = stream.lock().unwrap();
            match unlocked_stream.write(b"Admin thread already in use.") {
                Ok(_) => println!("Wrote to Admin thread: Unable to use"),
                Err(e) => println!("Unable to write to admin: {e}")
            }
            return;
        } else {
            // fix errors or change this
            adm_commands::command_interpreter(&raw_commands).unwrap();
        }
    });

}

fn is_admin_command(data: &[u8]) -> bool {
    // Convert bytes to string, ignoring invalid UTF-8
    if let Ok(command_str) = std::str::from_utf8(data) {
        // Check if it starts with an admin command marker
        // Adjust this logic based on your actual admin command format
        command_str.trim().starts_with("pw")
    } else {
        false
    }
}

// TODO: rate limit max client requests per hour
pub fn conn_handler(tcp: &TcpListener, pool: &ThreadPool) ->  std::io::Result<()>{
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
                let _ = stream.set_read_timeout(Some(std::time::Duration::from_secs(2)));

                // TODO: read in whole stream. This is unreliable.
                match stream.read(&mut buffer) {
                    Ok(n) => {
                        if n == 0 {
                            println!("No data sent. bug!")
                        }
                        if n > 0 && is_admin_command(&buffer[..n]) {
                            println!("Admin command detected.");
                            // check for admin commands, on a new thread.
                            // admin code should be non blocking.

                            // wrap in an Arc Mutex to keep safe and lock stream.
                            let stream = Arc::new(Mutex::new(stream));

                            // clone for ref safety.
                            let clone_stream = Arc::clone(&stream);

                            use_admin_thread(clone_stream, &mut buffer, &n);
                        } else {
                            // writing is mutation.
                            // todo, not sure if move is good here
                            pool.execute(move || {
                                let quote: db_operations::Quote = match db_operations::serve_quote() {
                                    Ok(quote) => quote,
                                    Err(e) => {
                                        eprintln!("Error serving quote: {:?}", e);
                                        db_operations::Quote {
                                            quote: "The server encountered an error, but errors are just opportunities in disguise.".to_string(),
                                            author: "Error Handler".to_string(),
                                        }
                                    }
                                };
                                let quote_str = format!("{} - {}", quote.quote, quote.author);
                                match stream.write_all(quote_str.as_bytes()) {
                                    Ok(_) => {
                                        // Write succeeded, proceed with flush and shutdown
                                        if let Err(e) = stream.flush() {
                                            eprintln!("Error flushing stream: {}", e);
                                        }
                                        if let Err(e) = stream.shutdown(std::net::Shutdown::Both) {
                                            eprintln!("Error shutting down stream: {}", e);
                                        }
                                        println!("Served client a quote");
                                    },
                                    Err(e) => {
                                        eprintln!("Failed to write to stream: {}", e);
                                        // Attempt to shutdown the connection anyway
                                        let _ = stream.shutdown(std::net::Shutdown::Both);
                                    }
                                }
                            });
                        }
                    },
                    Err(e) => { 
                        let _ = stream.flush();
                        let _ = stream.shutdown(std::net::Shutdown::Both);
                        println!("Error reading stream: {}", e)
                    }
                }
                


            },
            Err(e) => println!("Error receiving stream: {}", e)
        }
    }

    Ok(())

}
