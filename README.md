# Latex Assignment Template

A template I'm using for my college

## Use

```sh
# cargo run [config] [template] [output]
# ./target/release/latex-assignment-template-engine [config] [template] [output]
cargo run "src/config.toml" "src/latex-assignment-template/document.template.tex" "document.tex"
```

## Personal Use

```sh
cargo build --release
./target/release/latex-assignment-template-engine "src/latex-assignment-template-engine.config/config.toml" "src/latex-assignment-template/document.template.tex" "document.tex"
```
