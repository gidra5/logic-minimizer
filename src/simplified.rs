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
          
          //TODO set to "0" elements that are "1" in 'intersect'
          // simplified_implicants[i] = true;
          // simplified_implicants[j] = true;

          simplified.push(( implicant, intersect ));
        };
      };
    };
  };

  //for now to debug
  // if simplified.len() > 0 {
  //   simplified = simplify(simplified);
  // }

  let mut tmp: Vec<( Implicant, Vec<String> )> = implicants.into_iter()
    .enumerate()
    .filter(|(i, _)| simplified_implicants[*i].iter().fold(true, |acc, x| acc || x == "1"))
    .map(|(_, x)| x)
    .collect();

  tmp.append(&mut simplified);
  tmp
}


