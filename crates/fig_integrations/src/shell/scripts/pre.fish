command mkdir -p ~/.local/bin >/dev/null

builtin contains $HOME/.local/bin $PATH
or set --append PATH $HOME/.local/bin

if not test -z "$Q_NEW_SESSION"
    set --erase QTERM_SESSION_ID
    set --erase Q_TERM
    set --erase Q_NEW_SESSION
end

# Load parent from env variables
if test -n "$Q_PARENT"; and test -z "$Q_SET_PARENT"
    set --export Q_PARENT $Q_SET_PARENT
end

if test -z "$SHOULD_QTERM_LAUNCH"
    # 0 = Yes, 1 = No, 2 = Fallback to Q_TERM
    q _ should-figterm-launch 1>/dev/null 2>&1
    set SHOULD_QTERM_LAUNCH $status
end

if test -t 1
    and test -z "$PROCESS_LAUNCHED_BY_Q"
    and command -v qterm 1>/dev/null 2>/dev/null
    and test "$SHOULD_QTERM_LAUNCH" -eq 0 -o \( "$SHOULD_QTERM_LAUNCH" -eq 2 -a \( -z "$Q_TERM" -o \( -z "$Q_TERM_TMUX" -a -n "$TMUX" \) \) \)

    if test -z "$Q_SHELL"
        set Q_SHELL (q _ get-shell)
    end
    set Q_IS_LOGIN_SHELL 0
    if status --is-login
        set Q_IS_LOGIN_SHELL 1
    end

    # Do not launch qterm in non-interactive shells (like VSCode Tasks)
    if status --is-interactive
        set Q_TERM_NAME (command basename "$Q_SHELL")" (qterm)"
        if not set -q Q_TERM_PATH
            if test -x "$HOME/.local/bin/$Q_TERM_NAME"
                set Q_TERM_PATH "$HOME/.local/bin/$Q_TERM_NAME"
            else
                set Q_TERM_PATH (command -v qterm || echo "$HOME/.local/bin/qterm")
            end
        end

        # Need to exec bash because we're using 'exec -a <name>'
        # to set argv[0] and fish's exec doesn't have this option
        exec bash -c "Q_PARENT=$Q_PARENT Q_SHELL=$Q_SHELL Q_IS_LOGIN_SHELL=$Q_IS_LOGIN_SHELL exec -a \"$Q_TERM_NAME\" \"$Q_TERM_PATH\""
    end
end