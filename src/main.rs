use std::{env, fs};
use std::thread;
use std::sync::{Arc, Mutex};
use std::fmt::Display;
use itertools::Itertools;

mod simplified;
mod generate;

use simplified::*;
use generate::*;

#[derive(Debug, Clone, Hash, Eq)]
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

#[derive(Debug, Clone, Hash, Eq)]
pub struct LogicalFunction {
    implicants: Vec<Implicant>,
}

impl Display for LogicalFunction {
    fn fmt(&self, format: &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
        let mut func: Vec<String> = vec![];

        if self.implicants.len() == 0 {
            func.push( String::from("0"));
        } 
        else if self.implicants.len() == 1 {
            func.push( format!("{}", self.implicants[0]) );
        } 
        else {
            for implicant in self.implicants.iter() {
                func.push( format!("({})", implicant) );
            }
        }

        write!(format, "{}", func.join(" | "))
    }
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

    let generated = generate_implicants(cloned_implicants);

    let plug = vec![None].repeat(implicants[0].1.len());
    
    implicants.append(&mut generated.iter().map(|x| (x.clone(), plug.clone())).collect::<Vec<_>>());

    let bind = implicants.clone();
    let expanded = bind
        .iter()
        .enumerate()
        .filter(|(_i, (x, _y))| x.terms.contains(&None));

    expanded.clone()
        .for_each(|(_i, (x, y))| {
            fn unroll(x: Implicant) -> Vec<Implicant> {
                match x.terms.iter().position(|x| *x == None) {
                    Some(i) => {
                        let mut t1 = x.clone();
                        t1.terms[i] = Some(false);

                        let mut t2 = x;
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
                .into_iter()
                .map(|x| (x, y.clone()))
                .for_each(|x| implicants.push(x));
        });
        
    let expanded = expanded.map(|(i, _)| i).collect::<Vec<_>>();

    implicants = implicants.into_iter()
        .enumerate()
        .filter(|(i, _x)| !expanded.contains(i))
        .map(|(_i, x)| x)
        .collect::<Vec<_>>();

    // println!("Initial:");
    // for (i, (imp, fns)) in implicants.iter().enumerate() {
    //     println!("{}, ({{ {} }}, {:?})", i + 1, imp, fns);
    // }
    // println!("");

    let function_variants = Arc::new(Mutex::new(vec![]));

    let threads_quantity = implicants[0].1.len();
    let mut threads = vec![];
    for i in 0..threads_quantity {
        let one_fn: Vec<_> = implicants.clone().iter()
            .map(|(imp, fns)| (imp.clone(), fns[i]))
            .collect();
            
        threads.push(
            thread::spawn({let clone = Arc::clone(&function_variants); move || {
                let simplified: Vec<_> = simplify(&one_fn);
                let (core, others) = construct_func(&one_fn, simplified);

                let mut impicant_variants = vec![];
                let mut implicant_iter = others.into_iter()
                    .map(|x| x.into_iter())
                    .multi_cartesian_product()
                    .peekable();
                
                match implicant_iter.peek() {
                    Some(_) => (),
                    None => impicant_variants.push( LogicalFunction { implicants: core.clone() })
                }

                for combination in implicant_iter {
                    let mut implicants = core.clone();
                    implicants.append(&mut combination.into_iter()
                        .unique()
                        .collect::<Vec<_>>());
                    
                    impicant_variants.push(LogicalFunction { implicants });
                }


                let count = impicant_variants[0].implicants.len();
                let mut shortest_variants = vec![];

                for variant in impicant_variants {
                    if variant.implicants.len() < count {
                        shortest_variants = vec![variant];
                    } else if variant.implicants.len() == count {
                        shortest_variants.push(variant);
                    } 
                }

                println!("{}\n", 
                shortest_variants.iter()
                    .unique()
                    .map(|x| format!("y{} = {}", i + 1, x))
                    .collect::<Vec<_>>()
                    .join(" or\n")
                );

                let mut v = clone.lock().unwrap();
                v.push((i, shortest_variants));
            }})
        );
    }

    for i in threads {
        i.join().unwrap();
    }

    let bind = function_variants.lock().unwrap();
    let with_numbers = bind.iter()
      .map(|(i, x)| (i, x.iter()
        .map(|y| y.implicants.iter()
            .map(|z| z.clone().terms).collect::<Vec<_>>()
        ).collect::<Vec<_>>())
      );
    let without_numbers: Vec<_> = with_numbers.clone()
      .map(|(_i, x)| x).collect(); 

    println!("\nWITH NUMBERS");
    for i in with_numbers.clone() {
        println!("{:?}", i);
    }

    println!("\nWITHOUT NUMBERS");
    for i in without_numbers.clone() {
        println!("{:?}", i);
    }

    let order: Vec<_> = with_numbers.clone().map(|(i, x)| i).collect();

    let mut implicant_iter = with_numbers.clone()
        .map(|(_, x)| x.into_iter())
        .multi_cartesian_product()
        .peekable();

    let combined: Vec<_> = implicant_iter.clone().collect();

    let mut res = combined[0].clone();
    let mut global_counter = 0;

    for (i_j, j) in res.iter().clone().enumerate() {
        for (i_k, k) in  res.iter().clone().enumerate() {
            if i_j == i_k { continue; };
            if j == k { global_counter = global_counter + 1; };
        }
    }

    for (index, i) in implicant_iter.clone().enumerate() {
        let mut counter = 0;
        let mapped = i.iter()
          .map(|x| x[0].clone());
        for (i_j, j) in mapped.clone().enumerate() {
            for (i_k, k) in mapped.clone().enumerate() {
                if i_j == i_k { continue; };
                if j == k { counter = counter + 1; };
            }
        }
        if counter > global_counter {
            global_counter = counter;
            res = i;
        };
    }

    println!("\nRES:");
    for i in order {
        println!("y{}={:?}", i, res[*i]);
    }
}
