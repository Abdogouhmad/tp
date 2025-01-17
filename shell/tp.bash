_tp() {
    local i cur prev opts cmd
    COMPREPLY=()
    cur="${COMP_WORDS[COMP_CWORD]}"
    prev="${COMP_WORDS[COMP_CWORD-1]}"
    cmd=""
    opts=""

    for i in "${COMP_WORDS[@]}"
    do
        case "${cmd},${i}" in
            ",$1")
                cmd="tp"
                ;;
            *)
                ;;
        esac
    done

    case "${cmd}" in
        tp)
            opts="-c -g -h -V --check --generate --clean --help --version"
            if [[ ${cur} == -* || ${COMP_CWORD} -eq 1 ]] ; then
                mapfile -t COMPREPLY < <(compgen -W "${opts}" -- "${cur}")
                return 0
            fi
            case "${prev}" in
                --check)
                    mapfile -t COMPREPLY < <(compgen -f "${cur}")
                    return 0
                    ;;
                -c)
                    mapfile -t COMPREPLY < <(compgen -f "${cur}")
                    return 0
                    ;;
                *)
                    COMPREPLY=()
                    ;;
            esac
            mapfile -t COMPREPLY < <(compgen -W "${opts}" -- "${cur}")
            return 0
            ;;
    esac
}

if [[ "${BASH_VERSINFO[0]}" -eq 4 && "${BASH_VERSINFO[1]}" -ge 4 || "${BASH_VERSINFO[0]}" -gt 4 ]]; then
    complete -F _tp -o nosort -o bashdefault -o default tp
else
    complete -F _tp -o bashdefault -o default tp
fi