default:
    @just --list

# Run pre-commit hooks on all files, including autoformatting
pre-commit-all:
    pre-commit run --all-files

# Run 'cargo run' on the project
run *ARGS:
    cargo run -- -i ../icfp2006-rust-interpreter/target/release/icfp2006-rust ../icfp2006-rust-interpreter/local/cmu.um

# Run 'bacon' to test and make docs (auto-recompiles)
watch *ARGS:
    bacon --job test-doc
