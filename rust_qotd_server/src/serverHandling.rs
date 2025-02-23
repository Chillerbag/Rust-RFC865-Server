use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::io::{Write, Read};
use std::thread;
use dotenv::dotenv;
// functions for the server

use crate::admCommands;
use crate::dbOperations;

const PORT: u32 = 17;

// <()> says it returns a result, but we dont care what. Either Ok or Error.
// <(i32, String)> could be used for the normal wrapper. 
// I think? 

// note, the <TcpListner> is shorthand for <TcpListner, Std::io::Error> 
pub fn start_server() ->  std::io::Result<TcpListener>{
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
        let mut is_active = adm_thread_status_clone.lock().unwrap();
        
        // dereffing gives us access to inside the MutexGuard
        // first param ofthe mutexGuard is lifetime
        if (*is_active) {
            // lock method locks and gives access to TcpStream
            // TODO remove unwrap. 
            let mut unlocked_stream = stream.lock().unwrap();
            match unlocked_stream.write(b"Admin thread already in use.") {
                Ok(_) => println!("Wrote to Admin thread: Unable to use"),
                Err(e) => println!("Unable to write to admin: {e}")
            }
            return;
        } else {
            admCommands::command_interpreter(&raw_commands);
        }
    });

}

pub fn conn_handler(tcp: &TcpListener) ->  std::io::Result<()>{
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

                // TODO: read in whole stream. This is unreliable.
                match stream.read(&mut buffer) {
                    Ok(n) if n == 0 => {
                        // writing is mutation.
                        dbOperations::serve_quote(&mut stream);
                    },
                    Ok(n) => {
                        // check for admin commands, on a new thread.
                        // admin code should be non blocking.

                        // wrap in an Arc Mutex to keep safe and lock stream.
                        let stream = Arc::new(Mutex::new(stream));

                        // clone for ref safety.
                        let clone_stream = Arc::clone(&stream);

                        use_admin_thread(clone_stream, &mut buffer, &n);
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