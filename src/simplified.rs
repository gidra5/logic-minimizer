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
        println!("{}, {}, {:?}", i + 1, j + 1, intersect);
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

  let mut table: Vec<Vec<Vec<Implicant>>> = Vec::with_capacity(implicants[0].1.len());
  table.resize(implicants[0].1.len(), vec![]);

  for i in table.iter_mut() {
    *i = Vec::with_capacity(implicants.len());
    i.resize(implicants.len(), vec![]);
    for j in i.iter_mut() {
      *j = vec![];
    }
  }

  //table[func_index][implicant_index].push(simplified_implicant);

  for (creativnij_index, i) in implicants.iter().enumerate() {
    for j in simplified.iter() {
      let zipped: Vec<_> = i.0.terms.iter().zip(j.0.terms.iter())
        .map(|(a, b)| a == b || (*b == None && *a != None))
        .collect();

      let intersect = i.1.iter()
        .zip(j.1.iter())
        .enumerate()
        .filter(|(i, x)| match x {
            (Some(true), Some(true)) => true,
            _ => false
        })
        .map(|(i, _x)| i);

      if !zipped.contains(&false) { 
        for k in intersect {
          table[k][creativnij_index].push(j.0.clone());
        };
      };
    };
  };

  let mut functions = vec![]; 
  for i in &table {
    let mut function = vec![];
    let (mut unsorted_core, mut other_simples): (Vec<_>, Vec<_>) = i
      .iter()
      .partition(|&j| j.len() == 1); 
    let mut core: Vec<_> = unsorted_core.iter()
      .map(|x| x[0].clone())
      .collect();
    let mut others: Vec<_> = vec![];
    for j in other_simples {
      let mut shortest = None;
      let mut count = 0;
      for k in j {
        if core.contains(k) { shortest = Some(k.clone()); break; };
        let quantity = k.terms
          .iter()
          .filter(|x| **x == None)
          .collect::<Vec<_>>().len();
        if quantity > count { count = quantity; shortest = Some(k.clone()); };
      }
      others.push(shortest);
    }
    function.append(&mut core);
    function.append(&mut others.iter().filter(|x| **x != None).map(|x| x.clone().unwrap()).collect());
    functions.push(function);
  };

  println!("RES:");
  for (creativnij_index, i) in functions.iter().enumerate() {
    println!("{:?}", creativnij_index);
    for k in i {
      println!("{:?}", k);
    }
  };

  let mut tmp: Vec<_> = implicants.into_iter()
    .enumerate()
    .filter(|&(i, _)| simplified_functions[i].contains(&Some(true)))
    .map(|(i, (x, _))| (x, simplified_functions[i].clone()))
    .collect();

  tmp.append(&mut simplified);
  tmp
}

