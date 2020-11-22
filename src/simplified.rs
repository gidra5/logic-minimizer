// #![feature(non_ascii_idents)]
pub use crate::Implicant;

pub fn simplify(implicants: Vec<( Implicant, Vec<Option<bool>> )>) -> Vec<( Implicant, Vec<Option<bool>> )> {
  let mut simplified_functions: Vec<_> = implicants.iter()
      .map(|(_, vec)| vec.clone())
      .collect();

  let mut simplified: Vec<( Implicant, Vec<Option<bool>> )> = vec![];

  for i in 0..implicants.len() {
    let item_i = &implicants[i];

    for j in i + 1..implicants.len() {
      let item_j = &implicants[j];

      let intersect: Vec<_> = item_i.1.iter()
        .zip(item_j.1.iter())
        .map(|x| match x {
          (_, Some(false)) | (Some(false), _) => Some(false),
          _ => Some(true)
        })
        .collect();

      let different_at: Vec<usize> = item_i.0.terms.iter()
        .zip(item_j.0.terms.iter())
        .enumerate()
        .filter(|(_, (a, b))| a != b )
        .map(|(i, _)| i)
        .collect();
      
      if different_at.len() == 1 && intersect.contains(&Some(true)) {
        println!("{}, {}, {:?}", i, j, intersect);
        let mut simpler = item_i.0.terms.clone();
        simpler[different_at[0]] = None;

        let implicant = Implicant { terms: simpler };
        
        let transformed = intersect
          .iter()
          .enumerate()
          .filter(|(_i, &x)| x != Some(false))
          .map(|(i, _x)| i);

        for k in transformed {
          simplified_functions[i][k] = Some(false);
          simplified_functions[j][k] = Some(false);
        };

        simplified.push(( implicant, intersect ));
      }
    }
  }

  if simplified.len() > 0 {
    simplified = simplify(simplified);
  }

  let mut tmp: Vec<_> = implicants.into_iter()
    .enumerate()
    .filter(|&(i, _)| simplified_functions[i].contains(&Some(true)))
    .map(|(i, (x, _))| (x, simplified_functions[i].clone()))
    .collect();

  tmp.append(&mut simplified);
  tmp
}

