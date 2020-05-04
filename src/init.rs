use crate::public_clap_app;
use anyhow::Result;
use std::{env, io};

pub fn init_main(shell_name: &str) -> Result<()> {
    let exe_path = env::current_exe()?
        .into_os_string()
        .into_string()
        .expect("Could not convert path to string");

    let (setup_script, shell) = match shell_name {
        "bash" => (BASH_INIT, clap::Shell::Bash),
        "zsh" => (ZSH_INIT, clap::Shell::Zsh),
        "fish" => (FISH_INIT, clap::Shell::Fish),
        _ => panic!("Shell \"{}\" is not supported at the moment", shell_name),
    };

    println!(
        "{}",
        setup_script.replace("## EMPLACE ##", &format!("\"{}\"", exe_path))
    );

    // Print the completions
    public_clap_app().gen_completions_to("emplace", shell, &mut io::stdout());

    Ok(())
}

const BASH_INIT: &str = r##"
emplace_postexec_invoke_exec () {
    # quit when the previous command failed
    [ -z "$!" ] && exit $?

    local this_command=`HISTTIMEFORMAT= echo `history 1` | sed -e "s/^[ ]*[0-9]*[ ]*//"`;
    ## EMPLACE ## catch "$this_command"
}
PROMPT_COMMAND="emplace_postexec_invoke_exec;$PROMPT_COMMAND"
"##;

const ZSH_INIT: &str = r##"
emplace_precmd() {
    # quit when the previous command failed
    [ -z "$!" ] && exit

    local this_command=`HISTTIMEFORMAT= echo `history -1` | sed -e "s/^[ ]*[0-9]*[ ]*//"`;
    ## EMPLACE ## catch "$this_command"
}
# Don't hook them double in nested shells
if [[ ${precmd_functions[(ie)emplace_precmd]} -gt ${#precmd_functions} ]]; then
    precmd_functions+=(emplace_precmd)
fi
"##;

const FISH_INIT: &str = r##"
function emplace_postcmd --on-event fish_postexec
    # quit when the previous command failed
    if test $status -gt 0
        return
    end

    ## EMPLACE ## catch "$argv"
end
"##;
