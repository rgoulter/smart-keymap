#!/usr/bin/env gawk -f
#
# Gnu AWK script for modifying the .feature files
#  into .md.
#
# See also:
# https://cucumber.io/docs/gherkin/reference

# Implementation:
# Fairly straightforward substitutions:
#   - `Feature:` becomes `###` headings.
#   - `Background:` and `Example:` become `####` headings.
#   - Steps beginning with `Given`/`When`/etc.
#      have a blanck line printed before them
#      (so that these each start a new line in markdown).
#   - Step doc strings become code fences.
#     (Doc strings are always used as code snippets in this codebase).
#     - Indentation is assumed to be aligned with the heredoc `"""`
#       in th .feature file,
#       and is indented an extra 2 spaces in the code fences.

# Keep track of whether in doc string or not,
#  and indentation level.
BEGIN {
    in_docstring = 0
    docstring_indent = 0
}

function trim(str) {
    gsub(/^[ ]+|[ ]+$/, "", str)
    return str
}

function get_indent(str) {
    match(str, /^[ ]*/)
    return RLENGTH
}

function strip_indent(str, indent) {
    if (substr(str, 1, indent) ~ /^[ ]*$/) {
        return substr(str, indent + 1)
    }
    return str
}

# Feature: becomes `###` heading
/^Feature:/ {
    title = gensub(/^Feature:[ ]*(.*)$/, "\\1", 1)
    print "### " title
    next
}

# Background becomes `####` heading
/^[[:space:]]*Background:/ {
    print "\n#### Demonstrative Keymap"
    next
}

# Example becomes `####` heading
/^[[:space:]]*Example:/ {
    title = gensub(/^[[:space:]]*Example:[ ]*(.*)$/, "\\1", 1)
    print "\n#### " title
    next
}

# Handle docstring `"""` start/end
/^[[:space:]]*"""/ {
    if (!in_docstring) {
        docstring_indent = get_indent($0)
        print "\n```"
        in_docstring = 1
    } else {
        print "```"
        in_docstring = 0
    }
    next
}

# Given/When/Then/And step keywords
#
# In the markdown output, each starts its own paragraph.
/^[[:space:]]*(Given|When|Then|And)/ {
    if (!in_docstring) {
        print ""  # Add blank line before keyword
        print trim($0)
        next
    }
}

# All other output in .feature files is Markdown prose.
{
    if (in_docstring) {
        print strip_indent($0, docstring_indent)
    } else {
        print trim($0)
    }
}
