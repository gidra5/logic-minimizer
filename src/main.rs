use std::{env, fs};
mod simplified;
use simplified::simplify;

#[derive(Debug, Clone)]
pub struct Implicant {
    terms: Vec<Option<bool>>,
}

impl PartialEq for Implicant {
    fn eq(&self, other: &Self) -> bool {
        self.terms == other.terms
    }
}

#[derive(Debug, Clone)]
struct LogicalFunction {
    implicants: Vec<Implicant>,
}

fn main() {
    let implicants: Vec<_> = {
        let file = match env::args().collect::<Vec<_>>().as_slice() {
            [] | [_] => {
                println!("\x1b[31mPls select file\x1b[0m");
                panic!("Not enough arguments");
            },
            [_, filename, ..] => filename.clone()
        };

        fs::read_to_string(&file)
            .unwrap()
            .lines()
            .map(|x| {
                let (left, right) = match x.split('~').collect::<Vec<_>>().as_slice() {
                    [] | [_] => panic!("not enough data in row"),
                    [left, right, ..] => (*left, *right)
                };

                let terms: Vec<_> = left
                    .split("")
                    .filter(|x| *x != "")
                    .map(|x| match x {
                        "0" => Some(false),
                        "1" => Some(true),
                        _ => None,
                    })
                    .collect();

                let functions: Vec<_> = right
                    .split("")
                    .filter(|x| *x != "")
                    .map(|x| String::from(x))
                    .collect();

                (Implicant { terms }, functions)
            })
            .collect()
    };

    println!("Initial:");
    for (i, x) in implicants.iter().enumerate() {
        println!("{}, {:?}", i, x);
    }
    println!("");

    let simplified: Vec<_> = simplify(implicants);

    println!("Simplified:");
    for (i, x) in simplified.iter().enumerate() {
        println!("{}, {:?}", i, x);
    }
    println!("");

    for (i, (implicant, functions)) in simplified.iter().enumerate() {
        let res: Vec<_> = implicant
            .terms
            .iter()
            .enumerate()
            .map(|(i, x)| {
                match x {
                    Some(false) => format!("!x{}", i),
                    Some(true) => format!(" x{}", i),
                    None => String::from(" - ")
                }
            })
            .collect();

        let indexes: Vec<_> = functions
            .iter()
            .enumerate()
            .filter(|(_, x)| **x == "1")
            .map(|(i, _)| i)
            .collect();

        println!("{}-th Result: {} Functions: {:?}", i, res.join(" "), indexes);
    }

    let functions: Vec<LogicalFunction> = {
        let mut fns: Vec<LogicalFunction> = vec![];

        for (implicant, functions) in &simplified {
            let indexes = functions
                .iter()
                .enumerate()
                .filter(|(_, x)| **x == "1")
                .map(|(i, _)| i);

            for i in indexes {
                if fns.len() <= i {
                    fns.resize(i + 1, LogicalFunction { implicants: vec![] });
                }

                fns[i].implicants.push(implicant.clone());
            }
        }

        fns
    };

    for (i, LogicalFunction { implicants }) in functions.iter().enumerate() {
        let mut func: Vec<String> = vec![];

        for Implicant { terms } in implicants {
            let implicant = terms
            .iter()
            .enumerate()
            .filter(|(_, x)| match x {
                Some(_) => true,
                None => false
            })
            .map(|(i, x)| {
                match x {
                    Some(false) => format!("!x{}", i),
                    Some(true) => format!(" x{}", i),
                    None => String::from("")
                }
            })
            .collect::<Vec<_>>()
            .join(" & ");

            func.push( format!("({})", implicant) );
        }

        println!("y{} = {}", i, func.join(" | "));
    }
}
