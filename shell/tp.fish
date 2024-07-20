complete -c tp -s c -l check -d 'Check the file config in a specific dir -c=path/ --check=path' -r -F
complete -c tp -l completion -d 'Generate autocompletion script for the specified shell' -r -f -a "{bash\t'',elvish\t'',fish\t'',powershell\t'',zsh\t''}"
complete -c tp -s g -l generate -d 'Generate an absolute config'
complete -c tp -s h -l help -d 'Print help (see more with \'--help\')'
complete -c tp -s V -l version -d 'Print version'