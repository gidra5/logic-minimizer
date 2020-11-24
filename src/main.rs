use std::{env, fs};
use std::fmt::Display;
mod simplified;
use simplified::*;

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

    let mut generated = vec![];
    let initial: Vec<_> = implicants.iter().map(|(x, _)| x.clone()).collect();

    for i in 0..2u32.pow(implicants[0].0.terms.len() as u32) {
        let mut sad = format!("{:#032b}", i);
        sad = String::from(&sad[sad.len() - implicants[0].0.terms.len()..]);

        let terms: Vec<_> = sad.split("")
            .filter(|x| *x != "").map(|x| match x {
                "0" => Some(false),
                "1" => Some(true),
                _ => unreachable!()
            }).collect();

        let imp = Implicant { terms };
        let covered = initial.iter()
            .map(|x| x.terms.iter()
                .zip(imp.terms.iter())
                .map(|(&x, &y)| x == y || x == None)
                .all(|x| x))
            .any(|x| x);

        if !covered {
            generated.push(imp);
        }
    }

    let mut tmp = implicants[0].1.clone();

    for y in tmp.iter_mut() {
        *y = None;
    }

    implicants.append(&mut generated.iter().map(|x| (x.clone(), tmp.clone())).collect::<Vec<_>>());

    let asd = implicants.clone();
    let it = asd
        .iter()
        .enumerate()
        .filter(|(_i, (x, _y))| x.terms.contains(&None));

    it.clone()
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
        
    let it = it.map(|(i, _)| i).collect::<Vec<_>>();

    implicants = implicants.iter()
        .enumerate()
        .filter(|(i, _x)| !it.contains(i))
        .map(|(_i, x)| x.clone())
        .collect::<Vec<_>>();

    println!("Initial:");
    for (i, (imp, fns)) in implicants.iter().enumerate() {
        println!("{}, ({{ {} }}, {:?})", i + 1, imp, fns);
    }
    println!("");

    let simplified: Vec<_> = simplify(&implicants);
    let fns = construct_func(&implicants, simplified);

    for (i, LogicalFunction { implicants }) in fns.iter().enumerate() {
        let mut func: Vec<String> = vec![];

        for implicant in implicants {
            func.push( format!("({})", implicant) );
        }

        println!("y{} = {}", i + 1, func.join(" | "));
    }
}
