use crate::package::PackageSource;
use std::{env, error::Error};
use strum::IntoEnumIterator;

pub fn init_main(shell_name: &str) -> Result<(), Box<dyn Error>> {
    let exe_path = env::current_exe()?
        .into_os_string()
        .into_string()
        .expect("Could not convert path to string");

    let check_str = match shell_name {
        "bash" => BASH_CHECK,
        "zsh" => ZSH_CHECK,
        _ => panic!("Shell \"{}\" is not supported at the moment", shell_name),
    };
    // Get all the different package sources and replace them into the check strings
    let checks: String = PackageSource::iter()
        .map(|s| check_str.replace("## EMPLACE ##", s.command()))
        .collect::<Vec<String>>()
        .join(" || ");

    let setup_script = match shell_name {
        "bash" => BASH_INIT,
        "zsh" => ZSH_INIT,
        _ => panic!("Shell \"{}\" is not supported at the moment", shell_name),
    };

    let script = setup_script
        .replace("## EMPLACE ##", &format!("\"{}\"", exe_path))
        .replace("## EMPLACE_CHECKS ##", &*checks);
    print!("{}", script);

    Ok(())
}

const BASH_INIT: &str = r##"
emplace_postexec_invoke_exec () {
    # quit when the previous command failed
    [ -z "$!" ] && exit

    local hist=`history 1`
    # optimization to check quickly for strings
    ## EMPLACE_CHECKS ## || return;

    local this_command=`HISTTIMEFORMAT= echo $hist | sed -e "s/^[ ]*[0-9]*[ ]*//"`;
    ## EMPLACE ## catch "$this_command"
}
PROMPT_COMMAND="emplace_postexec_invoke_exec;$PROMPT_COMMAND"
"##;

const BASH_CHECK: &str = "[[ $hist == *\"## EMPLACE ##\"* ]]";

const ZSH_INIT: &str = r##"
emplace_precmd() {
    # quit when the previous command failed
    [ -z "$!" ] && exit

    local hist=`history -1`
    # optimization to check quickly for strings
    ## EMPLACE_CHECKS ## || return;

    local this_command=`HISTTIMEFORMAT= echo $hist | sed -e "s/^[ ]*[0-9]*[ ]*//"`;
    ## EMPLACE ## catch "$this_command"
}
# Don't hook them double in nested shells
if [[ ${precmd_functions[(ie)emplace_precmd]} -gt ${#precmd_functions} ]]; then
    precmd_functions+=(emplace_precmd)
fi
"##;

const ZSH_CHECK: &str = "[[ $hist == *\"## EMPLACE ##\"* ]]";
