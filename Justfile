default:
    just --list

fmt:
    trunk fmt

check:
    trunk check

qa: fmt check
