use regex::Regex;
use crate::db_operations;
pub enum AdmCommands {
    Password(Vec<String>),
    AddQuote(Vec<String>), 
    ChangeDefault(Vec<String>),
    ChangeIP(Vec<String>),
    ChangePool(Vec<String>),
    ChangeMaxReq(Vec<String>),
    ChangePassword(Vec<String>),
    Unknown {
        _command: String,
        _args: Vec<String>
    }
}

// TODO: need to write errors to the stream. 

// impl syntax is cool! adds a func binding on this enum
impl From<(String, Vec<String>)> for AdmCommands {
    fn from((command, args): (String, Vec<String>)) -> Self {
        // We use &str from command
        match command.as_str() {
            "pw" => Self::Password(args),
            "addquote" => Self::AddQuote(args),
            "changedef" => Self::ChangeDefault(args),
            "changeip" => Self::ChangeIP(args),
            "changepool" => Self::ChangePool(args),
            "changemaxreq" => Self::ChangeMaxReq(args),
            "changepassword" => Self::ChangePassword(args),
            _ => Self::Unknown {
                _command: command,
                _args: args
            }
        }
    }
}


pub fn command_interpreter(commands: &String) -> Result<(), String> {

    // interpret commands sent in stream
    // auto derefernce!
    // we now have an array of this form:
    let comm_iter: Vec<&str> = commands.split("|").collect();
    // where each element is of form:
    // ["command: "arg1" "arg2" ... ", "command2: "arg1" "arg2" ... "]
    // next step is to split each element into command: args. 
    // then, we handle each match to the enum in the corresponding function

    let mut commands_to_parse: Vec<AdmCommands> = vec![];

    for raw_command in comm_iter {

        // skip empty commands. accidental space etc. 
        if raw_command.trim().is_empty() {
            continue
        }
        // ensures we dont accidentally split when we get : in a quote.
        let parts: Vec<&str> = raw_command.splitn(2, ":").collect();
        // now, we have the command, and a undealt with string of args:
        // [com (str), args (str)]
        // pattern  match dbl quotes to turn args into Vec<String>

        if parts.len() < 2 {
            return Err(format!("Malformed command missing colon: {}", raw_command))
        }

        let command = parts[0].trim().to_owned();
        let args_str = parts[1];

        // this is funky, this is the only way rust supports "" in the pattern
        let re = match Regex::new(r#""([^"\\]*(?:\\.[^"\\]*)*)""#) {
            Ok(re) => re,
            Err(e) => {
                return Err(format!("Error loading Regex: {}", e));
            }
        };

        // get args from string
        let args: Vec<String> = re
            .captures_iter(args_str)
            .map(|cap| cap[1].to_owned())
            .collect();

        let sanitised_command = AdmCommands::from((command, args));

        commands_to_parse.push(sanitised_command);

    }
    
    // TODO: Handle this
    let _ = db_operations::exec_commands(&commands_to_parse);
    Ok(())
    

}