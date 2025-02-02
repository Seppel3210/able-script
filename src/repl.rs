use logos::Source;
use rustyline::Editor;

use crate::{interpret::ExecEnv, parser::Parser};

pub fn repl() {
    let mut rl = Editor::<()>::new();
    let mut env = ExecEnv::new();
    loop {
        let readline = rl.readline(":: ");
        match readline {
            Ok(line) => {
                if &line == "exit" {
                    println!("bye");
                    break;
                }
                let mut parser = Parser::new(&line);
                let ast = parser.init();
                match ast {
                    Ok(ast) => {
                        println!("{:?}", ast);
                        println!("{:?}", env.eval_items(&ast));
                    },
                    Err(e) => {
                        println!(
                            "Error `{:?}` occured at span: {:?} = `{:?}`",
                            e.kind,
                            e.position.clone(),
                            line.slice(e.position)
                        );
                    }
                }
            }
            Err(rustyline::error::ReadlineError::Eof) => {
                println!("bye");
                break;
            }
            Err(rustyline::error::ReadlineError::Interrupted) => (),
            Err(e) => {
                println!("Error: {:?}", e);
                break;
            }
        }
    }
}
