#  This file should be sourced only once by session. It is not recommended to
# source it yourself; instead, when starting the KTS server, the binary will
# inject it directly into the session.

# kak-tree-sitter arguments used when invoking the command.
#
# This is mainly used to ensure we use the same arguments when invoking
# kak-tree-sitter from within Kakoune.
declare-option str-list tree_sitter_cli_args

# FIFO buffer path; this is used by Kakoune to write the content of buffers to
# update the tree-sitter representation on KTS side.
#
# Should only be set KTS side by buffer.
declare-option str tree_sitter_buf_fifo_path /dev/null

# Sentinel code used to delimit buffers in FIFOs.
declare-option str tree_sitter_buf_sentinel

# Highlight ranges used when highlighting buffers.
declare-option range-specs tree_sitter_hl_ranges

# Language a buffer uses. That option should be set at the buffer level.
declare-option str tree_sitter_lang

# Last known timestamp of previouses buffer updates.
declare-option int tree_sitter_buf_update_timestamp -1

# Wrapper for kak-tree-sitter.
define-command -hidden kak-tree-sitter -params .. %{
  evaluate-commands -no-hooks %sh{
    kak-tree-sitter $kak_opt_tree_sitter_cli_args -kr "$@"
  }
}

# Create a command to send to Kakoune for the current session.
#
# The parameter is the string to be used as payload.
define-command -hidden tree-sitter-request-with-session -params 1 %{
  kak-tree-sitter "{ ""metadata"": { ""session"": ""%val{session}"" }, ""payload"": { ""type"": ""%arg{1}"" } }"
}

# Create a command to send to Kakoune for the current session and client.
#
# The parameter is the string to be used as payload.
define-command -hidden tree-sitter-request-with-session-client -params 1 %{
  kak-tree-sitter "{ ""metadata"": { ""session"": ""%val{session}"", ""client"": ""%val{client}"" }, ""payload"": %arg{1} }"
}

# Create a command to send to Kakoune for the current session and buffer.
#
# The parameter is the string to be used as payload.
define-command -hidden tree-sitter-request-with-session-buffer -params 1 %{
  kak-tree-sitter "{ ""metadata"": { ""session"": ""%val{session}"", ""buffer"": ""%sh{
    if [ -n ""$kak_buffile"" ]; then
      printf '%s' ""$kak_buffile""
    else
      printf '%s' ""$kak_bufname""
    fi
  }"" }, ""payload"": %arg{1} }"}

# Create a command to send to Kakoune for the current session, client and buffer.
#
# The parameter is the string to be used as payload.
define-command -hidden tree-sitter-request-with-session-client-buffer -params 1 %{
  kak-tree-sitter "{ ""metadata"": { ""session"": ""%val{session}"", ""client"": ""%val{client}"", ""buffer"": ""%sh{
    if [ -n ""$kak_buffile"" ]; then
      printf '%s' ""$kak_buffile""
    else
      printf '%s' ""$kak_bufname""
    fi
  }"" }, ""payload"": %arg{1} }"}

# Notify KTS that a session exists.
define-command tree-sitter-session-begin %{
  tree-sitter-request-with-session 'session_begin'
}

# Notify KTS that the session is exiting.
define-command tree-sitter-session-end %{
  tree-sitter-request-with-session 'session_end'
  tree-sitter-remove-all
}

# Request KTS to reload its configuration (grammar, queries, etc.).
define-command tree-sitter-reload %{
  tree-sitter-request-with-session 'reload'
  tree-sitter-session-end
  tree-sitter-session-begin
}

# Request KTS to completely shutdown.
define-command tree-sitter-shutdown %{
  tree-sitter-request-with-session 'shutdown'
}

# Request KTS to update its metadata regarding a buffer.
define-command tree-sitter-buffer-metadata %{
  tree-sitter-request-with-session-buffer "{ ""type"": ""buffer_metadata"", ""lang"": ""%opt{tree_sitter_lang}"" }"
}

# Request KTS to update its buffer representation of the current buffer.
#
# The parameter is the language the buffer is formatted in.
define-command tree-sitter-buffer-update %{
  evaluate-commands -no-hooks %{
    write "%opt{tree_sitter_buf_fifo_path}"
    echo -to-file "%opt{tree_sitter_buf_fifo_path}" -- "%opt{tree_sitter_buf_sentinel}"
  }
}

# Request KTS to clean up resources of a closed buffer.
define-command tree-sitter-buffer-close %{
  tree-sitter-request-with-session-buffer "{ ""type"": ""buffer_close"" }"
}

# Request the kak-tree-sitter version used by the server.
define-command tree-sitter-version %{
  tree-sitter-request-with-session-client "{ ""type"": ""version"" }"
}

# Request KTS to apply text-objects on selections.
#
# First parameter is the pattern.
# Second parameter is the operation mode.
define-command tree-sitter-text-objects -params 2 %{
  tree-sitter-request-with-session-client-buffer "{ ""type"": ""text_objects"", ""pattern"": ""%arg{1}"", ""selections"": ""%val{selections_desc}"", ""mode"": ""%arg{2}"" }"
}

# Request KTS to apply “object-mode” text-objects on selections.
#
# First parameter is the pattern.
define-command tree-sitter-object-text-objects -params 1 %{
  tree-sitter-request-with-session-client-buffer "{ ""type"": ""text_objects"", ""pattern"": ""%arg{1}"", ""selections"": ""%val{selections_desc}"", ""mode"": { ""object"": { ""mode"": ""%val{select_mode}"", ""flags"": ""%val{object_flags}"" } } }"
}

# Request KTS to navigate the tree-sitter tree on selections.
#
# The first parameter is the direction to move to.
define-command tree-sitter-nav -params 1 %{
  tree-sitter-request-with-session-client-buffer "{ ""type"": ""nav"", ""selections"": ""%val{selections_desc}"", ""dir"": %arg{1} }"
}

# User-overrideable command called right after inserting the tree-sitter
# highlighter.
#
# Useful to introduce a highlighter with higher priority to prevent the
# tree-sitter highlighter from overriding it.
define-command tree-sitter-user-after-highlighter nop

# Install main hooks.
define-command -hidden tree-sitter-hook-install-session %{
  # Hook that runs when the session ends.
  hook -group tree-sitter global KakEnd .* %{
    tree-sitter-session-end
  }
}

# Install a hook that updates buffer content if it has changed.
define-command -hidden tree-sitter-hook-install-update %{
  # Since this hook can be installed several times (after each changes of the
  # tree_sitter_lang option; see tree-sitter-hook-install-main), it’s better
  # to first try to remove the hooks.
  remove-hooks buffer tree-sitter-update

  # Buffer update
  hook -group tree-sitter-update buffer NormalIdle .* %{ tree-sitter-exec-if-changed tree-sitter-buffer-update }
  hook -group tree-sitter-update buffer InsertIdle .* %{ tree-sitter-exec-if-changed tree-sitter-buffer-update }

  # Initial highlight
  tree-sitter-buffer-update

  # Buffer close
  hook -group tree-sitter-update buffer BufClose .* %{ tree-sitter-buffer-close }
}

# Set the tree_sitter_lang buffer-option for all known buffers.
#
# This command should only be used once the session is enabled, and permit to
# dynamically enable tree-sitter for a buffer that was opened and fully displayed
# before the session was KTS-enabled
define-command -hidden tree-sitter-initial-set-buffer-lang %{
  evaluate-commands -buffer "*" %{
    set-option buffer tree_sitter_lang "%opt{filetype}"
  }
}

# A helper function that executes its argument only if the buffer has changed.
define-command -hidden tree-sitter-exec-if-changed -params 1 %{
  set-option -remove buffer tree_sitter_buf_update_timestamp %val{timestamp}

  try %{
    evaluate-commands "tree-sitter-exec-nop-%opt{tree_sitter_buf_update_timestamp}"
    set-option buffer tree_sitter_buf_update_timestamp %val{timestamp}
  } catch %{
    # Actually run the command
    set-option buffer tree_sitter_buf_update_timestamp %val{timestamp}
    evaluate-commands %arg{1}
  }
}

# A helper function that does nothing.
#
# Used with tree-sitter-exec-if-changed to have a fallback when the buffer has
# not changed.
define-command -hidden tree-sitter-exec-nop-0 nop

# Report whether tree-sitter is active for the current buffer, and if so, which
# language is being used.
#
# Three states are possible:
#   - inactive: tree_sitter_lang is not set for this buffer
#   - language set but grammar not loaded: the filetype was recognised but the
#     server did not confirm a successful grammar load (e.g. grammar missing or
#     failed to compile)
#   - active: grammar loaded and highlights are being streamed
define-command tree-sitter-status %{
    evaluate-commands %sh{
        lang=$kak_opt_tree_sitter_lang
        fifo=$kak_opt_tree_sitter_buf_fifo_path
        if [ -z "$lang" ]; then
            printf 'echo -markup "{Information}tree-sitter: inactive"'
        elif [ "$fifo" = "/dev/null" ]; then
            printf 'echo -markup "{Warning}tree-sitter: lang=%s but grammar is not loaded"' "$lang"
        else
            printf 'echo -markup "{Information}tree-sitter: active (lang=%s)"' "$lang"
        fi
    }
}

# Remove every tree-sitter commands, hooks, options, etc.
define-command tree-sitter-remove-all %{
  remove-hooks global tree-sitter

  evaluate-commands -buffer * %{
    try %{
      remove-highlighter buffer/tree-sitter-highlighter
    }

    try %{
      remove-hooks buffer tree-sitter-update
    }

    unset-option buffer tree_sitter_lang
    unset-option buffer tree_sitter_buf_update_timestamp
    unset-option buffer tree_sitter_buf_fifo_path
    unset-option buffer tree_sitter_buf_sentinel
    unset-option buffer tree_sitter_hl_ranges
  }
}
