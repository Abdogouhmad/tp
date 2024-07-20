#compdef tp

autoload -U is-at-least

# Function to handle the auto-completion for the `tp` command
_tp() {
    typeset -A opt_args
    typeset -a _arguments_options
    local ret=1

    # Set the options for the `_arguments` function based on the Zsh version
    if is-at-least 5.2; then
        _arguments_options=(-s -S -C)
    else
        _arguments_options=(-s -C)
    fi

    local context curcontext="$curcontext" state line
    _arguments "${_arguments_options[@]}" : \
        '-c+[Check the file config in a specific path]:FILE CONFIG:_files' \
        '--check=[Check the file config in a specific path]:FILE CONFIG:_files' \
        '-g[Generate an absolute config]' \
        '--generate[Generate an absolute config]' \
        '-h[Print help (see more with '\''--help'\'')]' \
        '--help[Print help (see more with '\''--help'\'')]' \
        '-V[Print version]' \
        '--version[Print version]' \
        && ret=0
}

# Function to handle the command descriptions
(( $+functions[_tp_commands] )) ||
_tp_commands() {
    local commands; commands=()
    _describe -t commands 'tp commands' commands "$@"
}

# Define the completion function for `tp` command
if [ "$funcstack[1]" = "_tp" ]; then
    _tp "$@"
else
    compdef _tp tp
fi
