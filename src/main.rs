use c_compiler_in_rust::generate_scanner_artifacts;
use c_compiler_in_rust::scan_source;

const DEFAULT_SCANNER_IMPL_PATH: &str = "src/scanner/generated/scanner_impl.rs";
const DEFAULT_DFA_TABLE_PATH: &str = "src/scanner/generated/dfa_table.rs";

fn main() {
    let mut args = std::env::args().skip(1);

    match args.next().as_deref() {
        Some("generate-scanner") => {
            let scanner_impl_path = args
                .next()
                .unwrap_or_else(|| DEFAULT_SCANNER_IMPL_PATH.to_string());
            let dfa_table_path = args
                .next()
                .unwrap_or_else(|| DEFAULT_DFA_TABLE_PATH.to_string());

            if let Err(err) = generate_scanner_artifacts(&scanner_impl_path, &dfa_table_path) {
                eprintln!("error: {err}");
                std::process::exit(1);
            }

            println!(
                "scanner generated:\n- {}\n- {}",
                scanner_impl_path, dfa_table_path
            );
        }
        Some("scan") => {
            let Some(input_path) = args.next() else {
                eprintln!("usage:\n  cargo run -- scan <input.c>");
                std::process::exit(2);
            };

            let source = match std::fs::read_to_string(&input_path) {
                Ok(s) => s,
                Err(err) => {
                    eprintln!("error reading '{}': {err}", input_path);
                    std::process::exit(1);
                }
            };

            match scan_source(&source) {
                Ok(tokens) => {
                    for token in tokens {
                        println!(
                            "{:?} \t'{}' \t{}:{}",
                            token.kind,
                            token.lexeme,
                            token.span.start.line,
                            token.span.start.column
                        );
                    }
                }
                Err(err) => {
                    eprintln!(
                        "lex error: {:?} at {}:{} - {}",
                        err.kind, err.span.start.line, err.span.start.column, err.message
                    );
                    std::process::exit(1);
                }
            }
        }
        _ => {
            eprintln!(
                "usage:\n  cargo run -- generate-scanner [scanner_impl_path] [dfa_table_path]\n  cargo run -- scan <input.c>"
            );
            std::process::exit(2);
        }
    }
}
