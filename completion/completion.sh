#!/bin/bash
# -----
# tab-completion for system76-power tool
# -----
#
# Changelog:
# 2022.01.28 - change-tresholds
# 2020.06.30 - rename intel to integrated mode
# 2019.01.14 - add hybrid mode
# 2018.10.30 - initial version <github.com/szydell/system76-power>
#

_system76-power ()
{
    local cur prev opts 
    COMPREPLY=()
    cur="${COMP_WORDS[COMP_CWORD]}"
    prev="${COMP_WORDS[COMP_CWORD-1]}"

    # 1st level options
    opts="charge-thresholds daemon graphics help profile --version --help"

    # 2nd/3rd level options
    case "${prev}" in
        graphics)
            local _opts="compute integrated hybrid nvidia power switchable --help"
            COMPREPLY=( $(compgen -W "${_opts}" -- ${cur}) )
            return 0
            ;;

        battery|balanced|compute|integrated|hybrid|nvidia|performance|switchable|on|off|auto)
            local _opts="--help"
            COMPREPLY=( $(compgen -W "${_opts}" -- ${cur}) )
            return 0
            ;;

        ?(--)help|-[hv]|--version)
            # Do not reply more
            return 0
            ;;

	     daemon)
	          local _opts="--quiet --verbose --help"
            COMPREPLY=( $(compgen -W "${_opts}" -- ${cur}) )
            return 0
            ;;

        profile)
            local _opts="battery balanced performance --help"
            COMPREPLY=( $(compgen -W "${_opts}" -- ${cur}) )
            return 0
            ;;

	      power)
	          local _opts="auto on off --help"
            COMPREPLY=( $(compgen -W "${_opts}" -- ${cur}) )
            return 0
            ;;
        charge-thresholds)
            local _opts="--profile --list-profiles --help"
            COMPREPLY=( $(compgen -W "${_opts}" -- ${cur}) )
            return 0
            ;;
        --profile)
            local _opts="full_charge balanced max_lifespan --help"
            COMPREPLY=( $(compgen -W "${_opts}" -- ${cur}) )
            return 0
            ;;
        *)
            ;;
  esac

   COMPREPLY=($(compgen -W "${opts}" -- ${cur}))  
   return 0
}

complete -F _system76-power system76-power

