pub use crate::*;

pub fn generate_implicants(initial: Vec<Implicant>) -> Vec<Implicant> {
    let mut generated = vec![];
    let terms_count = initial[0].terms.len();
    for i in 0..2u32 << (terms_count - 1) {
        let bit_terms = format!("{:#032b}", i);
        println!("{}", bit_terms);
        let bit_terms = String::from(&bit_terms[bit_terms.len() - terms_count..]);

        let terms: Vec<_> = bit_terms
            .split("")
            .filter(|x| !x.is_empty())
            .map(|x| match x {
                "0" => Some(false),
                "1" => Some(true),
                _ => unreachable!(),
            })
            .collect();

        let imp = Implicant {
            terms,
            naming: initial[0].naming.clone(),
        };
        let covered = initial
            .iter()
            .map(|x| {
                x.terms
                    .iter()
                    .zip(imp.terms.iter())
                    .map(|(&x, &y)| x == y || x == None)
                    .all(|x| x)
            })
            .any(|x| x);

        if !covered {
            generated.push(imp);
        }
    }
    generated
}
