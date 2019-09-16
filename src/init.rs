use strum::IntoEnumIterator;
use std:: {
    env,
    error:: Error
};
use crate::catch::PackageSource;

pub fn init_main(shell_name: &str) -> Result<(), Box<dyn Error>> {
    let exe_path = env::current_exe()?.into_os_string().into_string().expect("Could not convert path to string");

    let check_str = match shell_name {
        "bash" => BASH_CHECK,
        _ => panic!("Shell \"{}\" is not supported at the moment", shell_name)
    };
    // Get all the different package sources and replace them into the check strings
    let checks: String = PackageSource::iter()
        .map(|s| check_str.replace("## EMPLACE ##", s.command()))
        .collect::<Vec<String>>()
        .join("\n");

    let setup_script = match shell_name {
        "bash" => BASH_INIT,
        _ => panic!("Shell \"{}\" is not supported at the moment", shell_name)
    };

    let script = setup_script
        .replace("## EMPLACE ##", &format!("\"{}\"", exe_path))
        .replace("## EMPLACE_CHECKS ##", &*checks);
    print!("{}", script);

    Ok(())
}

const BASH_INIT: &str = r##"
emplace_preexec_invoke_exec () {
    [ -n "$COMP_LINE" ] && return # do nothing if completing
    [ "$BASH_COMMAND" = "$PROMPT_COMMAND" ] && return # don't cause a preexec for $PROMPT_COMMAND

    local hist=`history 1`
    # optimization to check quickly for strings
## EMPLACE_CHECKS ##

    local this_command=`HISTTIMEFORMAT= history 1 | sed -e "s/^[ ]*[0-9]*[ ]*//"`;
    ## EMPLACE ## catch "$this_command"
}
trap 'emplace_preexec_invoke_exec' DEBUG
"##;

const BASH_CHECK: &str = "    [[ $hist == *\"## EMPLACE ##\"* ]] || return;";
