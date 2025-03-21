// functions for dealing with db, called from adminCommands
// also called by serverHandling in the case of serveQuote.
use rusqlite::{params, Connection, Result};
use chrono::prelude::*;
use crate::adm_commands::AdmCommands;

pub struct Quote {
    pub quote: String,
    pub author: String
}

// TODO - memoize so we only need to call once per day
pub fn serve_quote() -> Result<Quote, rusqlite::Error> {
    // gives us a quote. If no quotes avaliable, AND sqllite is empty,
    // serve the default quote.
    // otherwise, wrap around, get latest date quote, find offset from that date
    // and wrap around array. 

    // TODO: dont unwrap! fix this.
    let default_quote = Quote {
        quote: dotenv::var("DEFAULT_QOTD").unwrap_or_else(|_| {
            eprintln!("WARNING: DEFAULT_QOTD environment variable not found, using fallback quote");
            "The server encountered an error, but errors are just opportunities in disguise.".to_string()
        }),
        author: dotenv::var("DEFAULT_QOTD_AUTH").unwrap_or_else(|_| {
            eprintln!("WARNING: DEFAULT_QOTD_AUTH environment variable not found, using fallback author");
            "Error Handler".to_string()
        })
    };

    let conn =  match Connection::open("qotd.db") {
        Ok(conn) => conn,
        Err(err) =>  {
            // TODO change all of these to eprintln
            // TODO this should propogate to caller like ? does. 
            eprintln!("Failed to open database: {}", err);
            return Err(err)
        }
    };

    let local: DateTime<Local> = Local::now();
    let local_date = format!("{}{}{}", local.year(), local.month(), local.day());
    let date_int = local_date.parse::<i32>().unwrap();

    let sql = format!("select quote, author from qotd where returned_on = {}", date_int.to_string());

    // TODO: this will panic if the table doesnt exist. If it doesm serve the default quote.
    let mut quote_stmt = match conn.prepare(&sql) {
        Ok(stmt) => stmt,
        Err(e) => {
            eprintln!("Table doesnt exist yet in db: {}", e);
            return Ok(default_quote)
        }
    };

    let todays_quote_result: Result<Vec<(String, String)>, rusqlite::Error> = quote_stmt.query_map([],
        |row| {
            let quote: String = row.get(0)?;
            let author: String = row.get(1)?;
            Ok((quote, author)) // Return a tuple of (quote, author)
        },
    ).unwrap().collect(); // Collect the results into a Vec
    
    let todays_quote = match todays_quote_result {
        Ok(quotes) if !quotes.is_empty() => {
            let (quote, author) = quotes[0].clone(); 
            Quote { 
                quote:quote,
                author:author
            }
        },
        _ => {
            eprintln!("No quote found for today.");
            return Ok(default_quote)
        }
    };
    
    // ... use todays_quote ...
    return Ok(todays_quote); // Return the tuple

    // todo: finish this! 

}

pub fn exec_commands(commands: &Vec<AdmCommands>) -> Result<(), String> {

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
        // TODO - fix error handling here or alternatively make all eprintln
        return Err("Error: invalid password.".to_string());
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
                // todo. Why are strings borrowed by default.
                return Err("Error: unknown command".to_string())
        };
    
        if let Err(err) = cmd_res {
            return Err(err) // TODO dont forget to remove semicolon for returns
        }
    }

    Ok(())
}

// This is really bad. lets fix this later.
fn change_env_var(command: &AdmCommands) -> Result<(), String> {

    // ok using .env was probably not smart. need to move to something else.
    // for time being, modify just for this session.

    match command {
        AdmCommands::ChangeDefault(new_quote) => {
            // TODO. not great. Thread safety is important here.
            unsafe { std::env::set_var("DEFAULT_QOTD", &new_quote[0]) };
            unsafe { std::env::set_var("DEFAULT_QOTD_AUTH", &new_quote[1]) };
        },
        // TODO: probably make this not a vec. 
        AdmCommands::ChangeIP(ip) => {
            unsafe { std::env::set_var("SERVER_IP", &ip[0]) };
        },
        AdmCommands::ChangeMaxReq(reqnum) => {
            // dont forget, all env vars are strings. need to handle these.
            unsafe { std::env::set_var("MAX_REQS_PER_HOUR", &reqnum[0]) };
        },
        AdmCommands::ChangePassword(newpw) => {
            unsafe { std::env::set_var("ADM_PW", &newpw[0]) };
        },
        AdmCommands::ChangePool(poolval) => {
            unsafe { std::env::set_var("POOL", &poolval[0]) };
        }
        AdmCommands::AddQuote(_) |
        AdmCommands::Password(_) | 
        AdmCommands::Unknown { .. } => ()
    }

    Ok(())

}

fn add_quote(args: &Vec<String>) -> Result<(), String> {
    if args.len() != 2 {
        return Err("Incorrect args for quote.".to_string());
    }

    // cloning here is less than ideal. Fine to give ownership?
    let quote_to_add = Quote {
        quote: args[0].clone(),
        author: args[1].clone()
    };

    return insert_into_db(quote_to_add)
}


// this function is super unsafe!!!!
fn insert_into_db(quote: Quote) -> Result<(), String> {

    let conn =  match Connection::open("qotd.db") {
        Ok(conn) => conn,
        Err(err) =>  {
            // TODO change all of these to eprintln
            // TODO this should propogate to caller like ? does. 
            eprintln!("Failed to open database: {}", err);
            return Err(err.to_string())
        }
    };

    // get todays date
    let local: DateTime<Local> = Local::now();
    let local_date = format!("{}{}{}", local.year(), local.month(), local.day());
    let date_int = local_date.parse::<i32>().unwrap();


    conn.execute(
        "create table if not exists qotd (
             quote text not null,
             author text not null,
             returned_on integer not null
         )",
        [],
    ).unwrap();

    // get max date from the table and increment it by one to insert the 
    // next qotd. 

    // TODO naughty unwrapping
    let mut max_stmt = conn.prepare(
        "select max(returned_on) from qotd",
    ).unwrap();

    // if returns empty, just do tomorrow. 
    let max_date_result: Result<Vec<i32>, rusqlite::Error> = max_stmt.query_map([], |row| {
        let max_date_int: i32 = row.get(0)?;
        Ok(max_date_int)
    }).unwrap().collect(); 

    let max_date = match max_date_result {
        Ok(dates) if !dates.is_empty() => dates[0] + 1,
        _ => date_int, // today
    };

    // todo this is a result, deal with it
    conn.execute(
        "INSERT INTO qotd (quote, author, returned_on) values (?1, ?2, ?3)",
        params![quote.quote, quote.author, max_date],
    ).unwrap();



    Ok(())

}
