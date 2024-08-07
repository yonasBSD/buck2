#!/usr/bin/env zsh
# Copyright (c) Meta Platforms, Inc. and affiliates.
#
# This source code is licensed under both the MIT license found in the
# LICENSE-MIT file in the root directory of this source tree and the Apache
# License, Version 2.0 found in the LICENSE-APACHE file in the root directory
# of this source tree.

# %INSERT_GENERATED_LINE%

# clap_complete generated content BEGINS
# %INSERT_OPTION_COMPLETION%
# clap_complete generated content ENDS

compdef -d buck2

_BUCK_COMPLETE_BIN="${_BUCK_COMPLETE_BIN:-buck2}"

__buck2_takes_target()
{
    case "$1" in
    build|ctargets|install|run|targets|test|utargets)
        return 0
        ;;
    *)
        return 1
        ;;
    esac
}

__buck2_subcommand()
{
    local subcommand=
    for w in "${COMP_WORDS[@]:1:$COMP_CWORD - 1}"; do
        case "$w" in
        --)
            # This marker should only occur after certain subcommands
            exit 1
            ;;
        -*|@*)
            ;;
        *)
            if [[ -z $subcommand ]]; then
                subcommand="$w"
            fi
            ;;
        esac
    done
    if [[ -n $subcommand ]]; then
        echo "$subcommand"
    fi
}

__buck2_add_target_completions()
{
    local completions=()
    while read -r; do
        completions+="$REPLY"
    done < <("${_BUCK_COMPLETE_BIN[@]}" complete --target="$1" 2>/dev/null)

    compadd -S '' -- "${completions[@]}"
}

__buck2_fix()
{
    local cur="${words[CURRENT]}"
    local prev="${words[CURRENT-1]}"
    local pprev="${words[CURRENT-2]}"
    if [[ $cur = : ]]; then
        cur="$prev:"
    elif [[ $prev = : ]]; then
        cur="$pprev:$cur"
    fi

    if __buck2_takes_target "$(__buck2_subcommand)"; then
        if [[ $cur =~ ^- ]]; then
            _buck2 "$@"
            return
        elif [[ -z $cur ]]; then
            _buck2 "$@"
            __buck2_add_target_completions "$cur"
        else
            __buck2_add_target_completions "$cur"
        fi
    else
        _buck2 "$@"
    fi
}

compdef __buck2_fix buck buck2
