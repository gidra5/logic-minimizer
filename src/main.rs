use itertools::Itertools;
use std::fmt::{Display, Error, Formatter};
use std::hash::{Hash, Hasher};
use std::result::Result;
use std::sync::{Arc, Mutex};
use std::thread;
use std::{env, fs};

mod generate;
mod simplified;

use generate::*;
use simplified::*;

#[derive(Debug, Clone, Eq)]
pub struct Implicant {
    naming: Vec<String>,
    terms: Vec<Option<bool>>,
}

impl PartialEq for Implicant {
    fn eq(&self, other: &Self) -> bool {
        // self.terms.iter().zip(other.terms.iter()).map(|(&x, &y)| x == y || x == None).all(|x| x)
        self.terms == other.terms
    }
}

impl Hash for Implicant {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.terms.hash(state)
    }
}

impl Display for Implicant {
    fn fmt(&self, format: &mut Formatter) -> Result<(), Error> {
        let mut implicant = self
            .terms
            .iter()
            .enumerate()
            .filter(|(_, x)| x.is_some())
            .map(|(i, x)| match x {
                Some(false) => format!("!{}", self.naming[i]),
                Some(true) => format!(" {}", self.naming[i]),
                None => unreachable!(),
            })
            .collect::<Vec<_>>()
            .join(" & ");

        if implicant.is_empty() {
            implicant = String::from("1");
        }

        write!(format, "{}", implicant)
    }
}

#[derive(Debug, Clone, Eq)]
pub struct LogicalFunction {
    implicants: Vec<Implicant>,
}

impl Display for LogicalFunction {
    fn fmt(&self, format: &mut Formatter) -> Result<(), Error> {
        let mut func: Vec<String> = vec![];

        match self.implicants.len() {
            0 => func.push(String::from("0")),
            1 => func.push(format!("{}", self.implicants[0])),
            _ => {
                for implicant in self.implicants.iter() {
                    func.push(format!("({})", implicant));
                }
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

impl Hash for LogicalFunction {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.implicants.hash(state)
    }
}

fn main() {
    let namings_x: Vec<String>;
    let namings_y: Vec<String>;

    let mut implicants: Vec<_> = {
        let file = match env::args().collect::<Vec<_>>().as_slice() {
            [] | [_] => {
                println!("\x1b[31mPls select file\x1b[0m");
                panic!("Not enough arguments");
            }
            [_, filename, ..] => filename.clone(),
        };

        let s1 = fs::read_to_string(&file).unwrap();
        let mut s2 = s1
            .lines()
            .filter_map(|line| line.split("//").next())
            .filter(|line| !line.is_empty());

        let mut s3 = s2
            .next()
            .unwrap()
            .split('~')
            .map(|x| x.split(' ').map(String::from).collect::<Vec<_>>());

        namings_x = s3.next().unwrap();
        namings_y = s3.next().unwrap();

        let s2 = s2
            .map(|line| line.chars().filter(|&char| char != ' ').collect::<String>())
            .filter(|line| !line.is_empty());

        s2.map(|x| {
            let (left, right) = match x.split('~').collect::<Vec<_>>().as_slice() {
                [] | [_] => panic!("not enough data in row"),
                [left, right, ..] => (*left, *right),
            };

            let terms: Vec<_> = left
                .split("")
                .filter(|x| !x.is_empty())
                .map(|x| match x {
                    "0" => Some(false),
                    "1" => Some(true),
                    _ => None,
                })
                .collect();

            let functions: Vec<_> = right
                .split("")
                .filter(|x| !x.is_empty())
                .map(|x| match x {
                    "0" => Some(false),
                    "1" => Some(true),
                    _ => None,
                })
                .collect();

            (
                Implicant {
                    terms,
                    naming: namings_x.clone(),
                },
                functions,
            )
        })
        .collect()
    };

    let cloned_implicants: Vec<_> = implicants.iter().map(|(x, _)| x.clone()).collect();

    let generated = generate_implicants(cloned_implicants);

    let plug = vec![None].repeat(implicants[0].1.len());

    implicants.append(&mut generated.iter().map(|x| (x.clone(), plug.clone())).collect::<Vec<_>>());

    let bind = implicants.clone();
    let expanded = bind.iter().enumerate().filter(|(_i, (x, _y))| x.terms.contains(&None));

    expanded.clone().for_each(|(_i, (x, y))| {
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
                }
                None => vec![x],
            }
        }

        unroll(x.clone())
            .into_iter()
            .map(|x| (x, y.clone()))
            .for_each(|x| implicants.push(x));
    });

    let expanded = expanded.map(|(i, _)| i).collect::<Vec<_>>();

    implicants = implicants
        .into_iter()
        .enumerate()
        .filter(|(i, _x)| !expanded.contains(i))
        .map(|(_i, x)| x)
        .collect::<Vec<_>>();

    let function_variants = Arc::new(Mutex::new(vec![]));
    let order = Arc::new(Mutex::new(vec![]));

    let threads_quantity = implicants[0].1.len();
    let mut threads = vec![];
    for i in 0..threads_quantity {
        let one_fn: Vec<_> = implicants
            .clone()
            .iter()
            .map(|(imp, fns)| (imp.clone(), fns[i]))
            .collect();

        threads.push(thread::spawn({
            let clone_fv = Arc::clone(&function_variants);
            let clone_order = Arc::clone(&order);
            let namings_y = namings_y.clone();

            move || {
                let simplified: Vec<_> = simplify(&one_fn);
                let (core, others) = construct_func(&one_fn, simplified);

                let mut impicant_variants = vec![];
                let mut implicant_iter = others
                    .into_iter()
                    .map(|x| x.into_iter())
                    .multi_cartesian_product()
                    .peekable();

                match implicant_iter.peek() {
                    Some(_) => (),
                    None => impicant_variants.push(LogicalFunction {
                        implicants: core.clone(),
                    }),
                }

                for combination in implicant_iter {
                    let mut implicants = core.clone();
                    implicants.append(&mut combination.into_iter().unique().collect::<Vec<_>>());

                    impicant_variants.push(LogicalFunction { implicants });
                }

                let count = impicant_variants[0].implicants.len();
                let mut shortest_variants = vec![];

                for variant in impicant_variants {
                    match variant.implicants.len().cmp(&count) {
                        std::cmp::Ordering::Equal => shortest_variants.push(variant),
                        std::cmp::Ordering::Less => shortest_variants = vec![variant],
                        _ => {}
                    }
                }

                println!(
                    "{}\n",
                    shortest_variants
                        .iter()
                        .unique()
                        .map(|x| format!("{} = {}", namings_y[i], x))
                        .collect::<Vec<_>>()
                        .join(" or\n")
                );

                let mut fv = clone_fv.lock().unwrap();
                let mut order = clone_order.lock().unwrap();
                fv.push(shortest_variants);
                order.push(i);
            }
        }));
    }

    for i in threads {
        i.join().unwrap();
    }

    let bind = function_variants.lock().unwrap();
    let fvs = bind.iter().map(|x| {
        x.iter()
            .map(|y| y.implicants.iter().map(|z| z.clone().terms).collect::<Vec<_>>())
            .collect::<Vec<_>>()
    });

    let implicant_iter = fvs.clone().map(|x| x.into_iter()).multi_cartesian_product().peekable();

    let combined: Vec<_> = implicant_iter.clone().collect();

    let res = combined[0].clone();
    let mut global_counter = 0;

    for (i_j, j) in res.iter().clone().enumerate() {
        for (i_k, k) in res.iter().clone().enumerate() {
            if i_j == i_k {
                continue;
            };
            if j == k {
                global_counter += 1;
            };
        }
    }

    for i in implicant_iter {
        let mut counter = 0;
        let mapped = i.iter().map(|x| x[0].clone());

        for (i_j, j) in mapped.clone().enumerate() {
            for (i_k, k) in mapped.clone().enumerate() {
                if i_j == i_k {
                    continue;
                };
                if j == k {
                    counter += 1;
                };
            }
        }

        if counter > global_counter {
            global_counter = counter;
            // res = i;
        };
    }
}
