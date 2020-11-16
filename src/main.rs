use std::{env, fs};

#[derive(Debug, Clone)]
struct Implicant {
    terms: Vec<Option<bool>>
}

#[derive(Debug)]
struct LogicalFunction {
    implicants: Vec<Implicant>
}

fn main() {
    let implicants: Vec<(Implicant, Vec<usize>)> = {
        let args: Vec<String> = env::args().collect();
        let file = &args[1];

        fs::read_to_string(&file).unwrap()
            .split("\r\n")
            .map(|x| {
                let collected: Vec<&str> = x.split('~').collect();
                let c1: Vec<Option<bool>> = collected[0].split("")
                    .filter(|x| *x != "")
                    .map(|x| match x {
                        "0" => Some(false),
                        "1" => Some(true),
                        _   => None
                    }).collect();
                let c2: Vec<usize> = collected[1].split("")
                    .filter(|x| *x != "")
                    .enumerate()
                    .map(|(i, x)| match x {
                        "1" => Some(i),
                        _   => None
                    }).filter(|x| match x {
                        Some(_) => true,
                        None => false
                    }).map(|x| match x {
                        Some(i) => i,
                        None => 0
                    }).collect();

                (Implicant { terms: c1 }, c2)
            }).collect()
    };

    // let mut simplified: Vec<(Implicant, Vec<usize>)> = vec![];

    // for i in implicants.iter() {
    //     for j in implicants.iter() {
    //         match intersect(&i.1, &j.1) {
    //             Some(k) => {

    //             },
    //             None => ()
    //         }
    //     }

    //     simplified.push(i.clone());
    // }

    println!("{:?}", implicants);
}

fn intersect<T: Eq + Clone>(arr1: &Vec<T>, arr2: &Vec<T>) -> Option<Vec<T>> {
    let mut intersect: Vec<T> = vec![];
    let mut a2 = arr2.clone();
    
    for i in arr1.iter() {
        let a = a2.clone();
        for (ind, val) in a.iter().enumerate() {
            if i == val {
                intersect.push(i.clone());
                a2.remove(ind);
            }
        }
    }

    if intersect.len() > 0 {
        Some(intersect)
    } else {
        None
    }
}