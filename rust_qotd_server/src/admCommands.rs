enum AdmCommands {
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
            "changepassword" => Self::ChangePool(args),
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

    for raw_command in comm_iter {
        let full_command: Vec<&str> = raw_command.split(":").collect();
        // now, we have the command, and a undealt with string of args:
        // [com (str), args (str)]
        // pattern  match dbl quotes to turn args into Vec<String>


    }
    

}