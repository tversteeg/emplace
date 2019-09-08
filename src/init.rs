use std:: {
    env,
    error:: Error
};

pub fn init_main(shell_name: &str) -> Result<(), Box<dyn Error>> {
    let exe_path = env::current_exe()?.into_os_string().into_string().expect("Could not convert path to string");

    let setup_script = match shell_name {
        "bash" => Some(BASH_INIT.replace("## EMPLACE ##", &format!("\"{}\"", exe_path))),
        _ => {
            eprintln!("Shell \"{}\" is not supported at the moment", shell_name);
            None
        }
    };

    if let Some(script) = setup_script {
        print!("{}", script);
    }

    Ok(())
}

const BASH_INIT: &str = r##"
emplace_preexec_invoke_exec () {
    [ -n "$COMP_LINE" ] && return # do nothing if completing
    [ "$BASH_COMMAND" = "$PROMPT_COMMAND" ] && return # don't cause a preexec for $PROMPT_COMMAND
    local this_command=`HISTTIMEFORMAT= history 1 | sed -e "s/^[ ]*[0-9]*[ ]*//"`;
    ## EMPLACE ## catch "$this_command"
}
trap 'emplace_preexec_invoke_exec' DEBUG
"##;
