use std::{env, fs};
mod intersect;
use intersect::intersect;

#[derive(Debug, Clone)]
struct Implicant {
    terms: Vec<Option<bool>>
}

impl PartialEq for Implicant {
    fn eq(&self, other: &Self) -> bool {
        self.terms == other.terms
    }
}

#[derive(Debug)]
struct LogicalFunction {
    implicants: Vec<Implicant>
}

fn main() {
    let implicants: Vec<(Implicant, Vec<String>)> = {
        let args: Vec<String> = env::args().collect();
        let file = &args[1];

        fs::read_to_string(&file).unwrap()
            .split("\r\n")
            .filter(|x| *x != "")
            .map(|x| {
                let collected: Vec<&str> = x.split('~').collect();
                let c1: Vec<Option<bool>> = collected[0].split("")
                    .filter(|x| *x != "")
                    .map(|x| match x {
                        "0" => Some(false),
                        "1" => Some(true),
                        _   => None
                    }).collect();
                let c2: Vec<String> = collected[1].split("")
                    .filter(|x| *x != "")
                    .map(|x| String::from(x))
                    .collect();

                (Implicant { terms: c1 }, c2)
            }).collect()
    };

    println!("Initial:");
    for i in &implicants {
        println!("{:?}", i);
    }    
    println!("");

    let mut simplified: Vec<(Implicant, Vec<String>)> = vec![];

    for i in 0..implicants.len() {
        let itemI = &implicants[i];
        let mut found = false;
        for j in i + 1..implicants.len() {
            let itemJ = &implicants[j];
            let compared: Vec<bool> = itemI.1.iter().zip(itemJ.1.iter())
                .map(|(a, b)| a == "1" && b == "1")
                .collect();
            if (compared.contains(&true)) {
                let pairs = itemI.0.terms.iter().zip(itemJ.0.terms.iter());
                let mut counter = vec![];
                for (i, (a, b)) in pairs.enumerate() {
                    if (a != b) { counter.push(i); }         
                };
                if (counter.len() == 1) { 
                    let mut simpler = itemI.0.terms.clone();
                    let index = counter[0];
                    simpler[index] = None;
                    let implicant = Implicant { terms: simpler };
                    let intersect = compared.iter()
                        .map(|x| match x {
                            true =>  String::from("1"),
                            false => String::from("0")
                        })
                        .collect();
                    found = true;
                    simplified.push((implicant, intersect ));
                };
            };
        };

        if !found { simplified.push(itemI.clone()); };
    };

    println!("Simplified:");
    for i in &simplified {
        println!("{:?}", i);
    }
}

