pub use crate::*;

pub fn simplify(implicants: &[(Implicant, Option<bool>)]) -> Vec<(Implicant, Option<bool>)> {
    let mut simplified_functions: Vec<_> = implicants.iter().cloned().map(|(_, vec)| vec).collect();

    let mut simplified: Vec<(Implicant, Option<bool>)> = vec![];

    for i in 0..implicants.len() {
        let item_i = &implicants[i];

        for j in i + 1..implicants.len() {
            let item_j = &implicants[j];

            let intersect = !matches!((item_i.1, item_j.1), (_, Some(false)) | (Some(false), _));

            let different_at: Vec<usize> = item_i
                .0
                .terms
                .iter()
                .zip(item_j.0.terms.iter())
                .enumerate()
                .filter(|(_, (a, b))| a != b)
                .map(|(i, _)| i)
                .collect();

            if different_at.len() == 1 && intersect {
                let mut simpler = item_i.0.terms.clone();
                simpler[different_at[0]] = None;

                let implicant = Implicant {
                    terms: simpler,
                    naming: implicants[0].0.naming.clone(),
                };

                simplified_functions[i] = Some(false);
                simplified_functions[j] = Some(false);

                simplified.push((implicant, Some(true)));
            }
        }
    }

    if !simplified.is_empty() {
        simplified.dedup();
        simplified = simplify(&simplified);
    }

    #[allow(unused_mut)]
    let mut tmp: Vec<_> = implicants
        .iter()
        .enumerate()
        .filter(|&(i, _)| simplified_functions[i] == Some(true))
        .map(|(i, (x, _))| (x.clone(), simplified_functions[i]))
        .collect();

    tmp.append(&mut simplified);
    tmp
}

pub fn construct_func(
    initial: &[(Implicant, Option<bool>)],
    simplified: Vec<(Implicant, Option<bool>)>,
) -> (Vec<Implicant>, Vec<Vec<Implicant>>) {
    let mut table: Vec<Vec<Implicant>> = Vec::with_capacity(initial.len());
    table.resize(initial.len(), vec![]);

    for (initial_id, i) in initial.iter().enumerate() {
        for j in &simplified {
            let mut zipped =
                i.0.terms
                    .iter()
                    .zip(j.0.terms.iter())
                    .map(|(&a, &b)| a == b || b == None);

            let intersect = matches!((i.1, j.1), (Some(true), Some(true)));

            if zipped.all(|x| x) && !table[initial_id].contains(&j.0) && intersect {
                table[initial_id].push(j.0.clone());
            }
        }
    }

    let mut function = vec![];
    let (unsorted_core, other_simples): (Vec<_>, Vec<_>) =
        table.into_iter().enumerate().partition(|(_i, j)| j.len() == 1);

    for item in unsorted_core.into_iter().map(|(_i, x)| x[0].clone()) {
        if !function.contains(&item) {
            function.push(item);
        }
    }

    let mut others: Vec<Vec<_>> = vec![];
    for (impl_id, j) in other_simples {
        if initial[impl_id].1 == None {
            continue;
        }

        let mut shortest = vec![];
        let mut count = 0;

        for k in j {
            if function.contains(&k) {
                shortest = vec![];
                break;
            };

            let quantity = k.terms.iter().filter(|&&x| x == None).count();

            match quantity.cmp(&count) {
                std::cmp::Ordering::Greater => {
                    count = quantity;
                    shortest = vec![k]
                }
                std::cmp::Ordering::Equal => shortest.push(k),
                _ => {}
            }
        }

        if !shortest.is_empty() && !others.contains(&shortest) {
            others.push(shortest);
        }
    }
    // let mut others = others.iter().map(|x| x[0].clone()).unique().collect::<Vec<_>>();

    // function.append(&mut others);
    // LogicalFunction { implicants: function }
    (function, others)
}
