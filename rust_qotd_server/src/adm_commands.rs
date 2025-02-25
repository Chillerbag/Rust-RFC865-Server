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
        command: String,
        args: Vec<String>
    }
}

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
                command,
                args
            }
        }
    }
}


pub fn command_interpreter(commands: &String) {

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
        let full_command: Vec<&str> = raw_command.split(":").collect();
        // now, we have the command, and a undealt with string of args:
        // [com (str), args (str)]
        // pattern  match dbl quotes to turn args into Vec<String>

        // this is funky, this is the only way rust supports "" in the pattern
        let re = match Regex::new(r#""(\S*)""#) {
            Ok(re) => re,
            Err(e) => {
                println!("Error loading Regex: {}", e);
                // TODO, what should we do here? 
                panic!("Panicked: Failed to compile regex");
            }
        };

        // get args from string
        let args: Vec<String> = re.find_iter(full_command[1]).map(|m| m.as_str().to_owned()).collect();
        let command: String = full_command[0].to_owned();

        let sanitised_command = AdmCommands::from((command, args));

        commands_to_parse.push(sanitised_command);

    }
    
    // TODO: Handle this
    db_operations::exec_commands(&commands_to_parse);
    

}