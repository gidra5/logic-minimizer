pub use crate::Implicant;

pub fn simplify(implicants: Vec<( Implicant, Vec<String> )>) -> Vec<( Implicant, Vec<String> )> {
  let mut simplified_implicants: Vec<Vec<String>> = 
    implicants.iter().map(|(_, vec)| vec.clone()).collect();
  let mut simplified: Vec<( Implicant, Vec<String> )> = vec![];

  for i in 0..implicants.len() {
    let item_i = &implicants[i];

    for j in i + 1..implicants.len() {
      let item_j = &implicants[j];

      let intersect: Vec<String> = item_i.1.iter()
        .zip(item_j.1.iter())
        .map(|(a, b)| a == "1" && b == "1")
        .map(|x| match x {
          true =>  String::from("1"),
          false => String::from("0")
        })
        .collect();

      if intersect.contains(&String::from("1")) {

        let different_at: Vec<usize> = item_i.0.terms.iter()
          .zip(item_j.0.terms.iter())
          .enumerate()
          .filter(|(_, (a, b))| a != b )
          .map(|(i, _)| i)
          .collect();

        if different_at.len() == 1 { 
          println!("{}, {}, {:?}", i, j, intersect);
          let mut simpler = item_i.0.terms.clone();
          simpler[different_at[0]] = None;
  
          let implicant = Implicant { terms: simpler };
          
          let transformed = intersect
            .iter()
            .enumerate()
            .filter(|(_i, x)| **x != String::from("0"))
            .map(|(i, _x)| i);
          for k in transformed {
            simplified_implicants[i][k] = String::from("0");
            simplified_implicants[j][k] = String::from("0");
          };

          simplified.push(( implicant, intersect ));
        };
      };
    };
  };

  if simplified.len() > 0 {
    simplified = simplify(simplified);
  }

  let mut tmp: Vec<( Implicant, Vec<String> )> = implicants.into_iter()
    .enumerate()
    .filter(|(i, _)| simplified_implicants[*i].contains(&String::from("1")))
    .map(|(i, (x, y))| (x, simplified_implicants[i].clone()))
    .collect();

  tmp.append(&mut simplified);
  tmp
}

