// functions for dealing with db, called from adminCommands
// also called by serverHandling in the case of serveQuote.
use rusqlite::{params, Connection, Result};
use std::net::TcpStream;
use crate::admCommands::AdmCommands;

struct Quote {
    quote: String,
    author: String
}

pub fn serve_quote(stream: &mut TcpStream) {
    // gives us a quote. If no quotes avaliable, AND sqllite is empty,
    // serve the default quote.
    // otherwise, wrap around, get latest date quote, find offset from that date
    // and wrap around array. 

    // TODO: dont unwrap! fix this.
    let default_quote = Quote {
        quote: dotenv::var("DEFAULT_QOTD").unwrap(),
        author: dotenv::var("DEFAULT_AUTH").unwrap()
    };

    // todo: finish this! 

}

pub fn exec_commands(commands: &Vec<AdmCommands>) -> std::io::Result<()> {


    // TODO: dont unwrap! tell user that they need to put in the correct env vars.
    let conf_pw = dotenv::var("ADM_PW").unwrap();

    // TODO we should probably do this better. time complexity is 2n

    // check password is present and matches dotenv password.
    // TODO: probs dont use any. We only want one password command.
    let has_valid_password = commands.iter().any(|cmd| {
        if let AdmCommands::Password(pw_args) = cmd {
            // TODO: assert that pw_args should only be 1 long.
            !pw_args.is_empty() && pw_args[0] == conf_pw
        } else {
            false
        }
    });
    
    // return error if no password or invalid
    if !has_valid_password {
        return Err(std::io::Error::new(
            std::io::ErrorKind::PermissionDenied,
            "Valid password required",
        ));
    }

    for cmd in commands {
        let cmd_res = match cmd {
            AdmCommands::Password(_) => Ok(()),
            AdmCommands::AddQuote(args) => add_quote(args),
            // any of these are an env var change.
            AdmCommands::ChangeDefault(_) | 
            AdmCommands::ChangeIP(_) | 
            AdmCommands::ChangeMaxReq(_) | 
            AdmCommands::ChangePassword(_) | 
            AdmCommands::ChangePool(_) => change_env_var(cmd),
            AdmCommands::Unknown {..} => 
                return Err(std::io::Error::new(
                    std::io::ErrorKind::NotFound,
                    "Invalid command sent",
                ))
        };
    
        if let Err(err) = cmd_res {
            return Err(err);
        }
    }

    Ok(())
}

fn change_env_var(command: &AdmCommands) -> std::io::Result<()> {

    Ok(())

}

fn add_quote(command: &Vec<String>) -> std::io::Result<()> {

    // convert to a quote (confirm shape)
    Ok(())
}

// startup db operation (create if not exists)

// add quote

// change default quote

// change hosted ip, then restart server on that IP.

// change pool 

// change max req

// change password

// generic function to change env vars? 