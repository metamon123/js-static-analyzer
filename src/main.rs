use anyhow::Result;
use clap::Parser;
use std::fs;
use std::path::PathBuf;
use swc_common::errors::Handler;
use swc_common::sync::Lrc;
use swc_common::{FileName, SourceMap};
use swc_ecma_ast::Module;
use swc_ecma_parser::{lexer::Lexer, Parser as SwcParser, StringInput, Syntax};

#[derive(Parser)]
#[command(name = "js-static-analyzer")]
#[command(about = "A simple JavaScript static analyzer")]
struct Args {
    #[arg(short, long)]
    file: PathBuf,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let ast = parse_javascript(&args.file)?;

    analyze_ast(&ast);

    Ok(())
}

fn parse_javascript(file_path: &PathBuf) -> Result<Module> {
    let source_code = fs::read_to_string(file_path)?;

    let cm: Lrc<SourceMap> = Default::default();
    let handler = Handler::with_emitter_writer(Box::new(std::io::stderr()), Some(cm.clone()));

    let fm = cm.new_source_file(FileName::Real(file_path.clone()).into(), source_code);

    let lexer = Lexer::new(
        Syntax::Es(Default::default()),
        Default::default(),
        StringInput::from(&*fm),
        None,
    );

    let mut parser = SwcParser::new_from(lexer);

    match parser.parse_module() {
        Ok(module) => Ok(module),
        Err(e) => {
            e.into_diagnostic(&handler).emit();
            Err(anyhow::anyhow!("Failed to parse JavaScript"))
        }
    }
}

fn analyze_ast(module: &Module) {
    println!("JavaScript AST Analysis:");
    println!("========================");

    let counts = count_top_level_items(module);

    println!("Functions: {}", counts.functions);
    println!("Variables: {}", counts.variables);
    println!("Imports: {}", counts.imports);
}

struct AnalysisCounts {
    functions: usize,
    variables: usize,
    imports: usize,
}

fn count_top_level_items(module: &Module) -> AnalysisCounts {
    let mut function_count = 0;
    let mut var_count = 0;
    let mut import_count = 0;

    for item in &module.body {
        match item {
            swc_ecma_ast::ModuleItem::Stmt(stmt) => {
                if let swc_ecma_ast::Stmt::Decl(decl) = stmt {
                    match decl {
                        swc_ecma_ast::Decl::Fn(_) => function_count += 1,
                        swc_ecma_ast::Decl::Var(_) => var_count += 1,
                        _ => {}
                    }
                }
            }
            swc_ecma_ast::ModuleItem::ModuleDecl(decl) => match decl {
                swc_ecma_ast::ModuleDecl::Import(_) => import_count += 1,
                _ => {}
            },
        }
    }

    AnalysisCounts {
        functions: function_count,
        variables: var_count,
        imports: import_count,
    }
}

#[cfg(test)]
mod tests {
    use super::{count_top_level_items, parse_javascript};
    use std::io::Write;

    #[test]
    fn counts_functions_variables_imports() {
        let source = r#"
function greet(name) {
    const message = "Hello, " + name + "!";
    console.log(message);
}

let counter = 0;
const PI = 3.14159;

import { utils } from './utils.js';
"#;
        let mut file = tempfile::NamedTempFile::new().expect("create temp file");
        file.write_all(source.as_bytes())
            .expect("write test source");
        let module = parse_javascript(&file.path().to_path_buf()).expect("parse test source");
        let counts = count_top_level_items(&module);

        assert_eq!(counts.functions, 1);
        assert_eq!(counts.variables, 2);
        assert_eq!(counts.imports, 1);
    }
}
