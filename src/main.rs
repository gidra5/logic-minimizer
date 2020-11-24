use std::{env, fs};
use std::thread;
use std::fmt::Display;

mod simplified;
mod generate;

use simplified::*;
use generate::*;

#[derive(Debug, Clone)]
pub struct Implicant {
    terms: Vec<Option<bool>>,
}

impl PartialEq for Implicant {
    fn eq(&self, other: &Self) -> bool {
        // self.terms.iter().zip(other.terms.iter()).map(|(&x, &y)| x == y || x == None).all(|x| x)
        self.terms == other.terms
    }
}

impl Display for Implicant {
    fn fmt(&self, format: &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
        let mut implicant = self.terms
            .iter()
            .enumerate()
            .filter(|(_, x)| match x {
                Some(_) => true,
                None => false
            })
            .map(|(i, x)| {
                match x {
                    Some(false) => format!("!x{}", i + 1),
                    Some(true) => format!(" x{}", i + 1),
                    None => String::from("")
                }
            })
            .collect::<Vec<_>>()
            .join(" & ");

        if implicant.len() == 0 {
            implicant = String::from("1");
        }

        write!(format, "{}", implicant)
    }
}

#[derive(Debug, Clone)]
pub struct LogicalFunction {
    implicants: Vec<Implicant>,
}

impl PartialEq for LogicalFunction {
    fn eq(&self, other: &Self) -> bool {
        self.implicants == other.implicants
    }
}

fn main() {
    let mut implicants: Vec<_> = {
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
                    .map(|x| match x {
                        "0" => Some(false),
                        "1" => Some(true),
                        _ => None
                    })
                    .collect();

                (Implicant { terms }, functions)
            })
            .collect()
    };

    let cloned_implicants: Vec<_> = implicants.iter().map(|(x, _)| x.clone()).collect();

    let mut generated = generate_implicants(cloned_implicants);

    let mut plug = vec![None].repeat(implicants[0].1.len());
    
    implicants.append(&mut generated.iter().map(|x| (x.clone(), plug.clone())).collect::<Vec<_>>());

    let bind = implicants.clone();
    let abbreviated = bind
        .iter()
        .enumerate()
        .filter(|(_i, (x, _y))| x.terms.contains(&None));

    abbreviated.clone()
        .for_each(|(_i, (x, y))| {
            fn unroll(x: Implicant) -> Vec<Implicant> {
                match x.terms.iter().position(|x| *x == None) {
                    Some(i) => {
                        let mut t1 = x.clone();
                        t1.terms[i] = Some(false);

                        let mut t2 = x.clone();
                        t2.terms[i] = Some(true);

                        let mut t3 = vec![];
                        t3.append(&mut unroll(t1));
                        t3.append(&mut unroll(t2));
                        
                        t3
                    },
                    None => vec![x]
                }
            };

            unroll(x.clone())
                .iter()
                .map(|x| (x.clone(), y.clone()))
                .for_each(|x| implicants.push(x));
        });
        
    let abbreviated = abbreviated.map(|(i, _)| i).collect::<Vec<_>>();

    implicants = implicants.iter()
        .enumerate()
        .filter(|(i, _x)| !abbreviated.contains(i))
        .map(|(_i, x)| x.clone())
        .collect::<Vec<_>>();

    println!("Initial:");
    for (i, (imp, fns)) in implicants.iter().enumerate() {
        println!("{}, ({{ {} }}, {:?})", i + 1, imp, fns);
    }
    println!("");

    let threads_quantity = implicants[0].1.len();

    let mut threads = vec![];
    for i in 0..threads_quantity {
        let one_fn: Vec<_> = implicants.clone().iter()
            .map(|(imp, fns)| (imp.clone(), vec![fns[i]]))
            .collect();
        threads.push(
            thread::spawn(move || {
                let simplified: Vec<_> = simplify(&one_fn);
                let fns = construct_func(&one_fn, simplified);

                for (j, LogicalFunction { implicants }) in fns.iter().enumerate() {
                    let mut func: Vec<String> = vec![];

                    for implicant in implicants {
                        func.push( format!("({})", implicant) );
                    }

                    println!("y{} = {}", i + j + 1, func.join(" | "));
                }
            })
        );
    }

    for i in threads {
        i.join().unwrap();
    }
    
}
