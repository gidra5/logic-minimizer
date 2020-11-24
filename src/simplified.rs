pub use crate::*;

pub fn simplify(implicants: &Vec<( Implicant, Option<bool> )>) -> Vec<( Implicant, Option<bool> )> {
  let mut simplified_functions: Vec<_> = implicants.iter()
      .map(|(_, vec)| vec.clone())
      .collect();

  let mut simplified: Vec<( Implicant, Option<bool> )> = vec![];

  for i in 0..implicants.len() {
    let item_i = &implicants[i];

    for j in i + 1..implicants.len() {
      let item_j = &implicants[j];

      let intersect = match (item_i.1, item_j.1) {
        (_, Some(false)) | (Some(false), _) => false,
         _ => true
      };

      let different_at: Vec<usize> = item_i.0.terms.iter()
        .zip(item_j.0.terms.iter())
        .enumerate()
        .filter(|(_, (a, b))| a != b )
        .map(|(i, _)| i)
        .collect();
      
      if different_at.len() == 1 && intersect {
        let mut simpler = item_i.0.terms.clone();
        simpler[different_at[0]] = None;

        let implicant = Implicant { terms: simpler };
        
        simplified_functions[i] = Some(false);
        simplified_functions[j] = Some(false);

        simplified.push(( implicant, Some(true)));
      }
    }
  }

  if simplified.len() > 0 {
    simplified = simplify(&simplified);
  }

  #[allow(unused_mut)]
  let mut tmp: Vec<_> = implicants.into_iter()
    .enumerate()
    .filter(|&(i, _)| simplified_functions[i]== Some(true))
    .map(|(i, (x, _))| (x.clone(), simplified_functions[i].clone()))
    .collect();

  tmp.append(&mut simplified);
  tmp
}

pub fn construct_func(
  initial:    &Vec<( Implicant, Option<bool> )>, 
  simplified:  Vec<( Implicant, Option<bool> )>
) -> LogicalFunction {
  let mut table: Vec<Vec<Implicant>> = Vec::with_capacity(initial.len());
  table.resize(initial.len(), vec![]);

  for (initial_id, i) in initial.iter().enumerate() {
    for j in &simplified {
      let mut zipped = i.0.terms.iter().zip(j.0.terms.iter())
        .map(|(&a, &b)| a == b || b == None );

      let intersect = match (i.1, j.1) {
        (Some(true), Some(true)) => true,
        _ => false
      };
      if 
        zipped.all(|x| x) &&
        !table[initial_id].contains(&j.0) &&
        intersect 
      {
        table[initial_id].push(j.0.clone());
      }
    };
  };

  let mut function = vec![];
  let (unsorted_core, other_simples): (Vec<_>, Vec<_>) = table
    .iter()
    .enumerate()
    .partition(|&(_i, j)| j.len() == 1); 
    
  for item in unsorted_core.iter()
    .map(|(_i, x)| x[0].clone()) 
  {
    if !function.contains(&item) {
      function.push(item);
    }
  }

  let mut others: Vec<_> = vec![];
  for (impl_id, j) in other_simples {
    if initial[impl_id].1 == None {
      continue;
    }

    let mut shortest = None;
    let mut count = 0;

    for k in j {
      if function.contains(k) { shortest = None; break; };

      let quantity = k.terms
        .iter()
        .filter(|&&x| x == None)
        .collect::<Vec<_>>()
        .len();

      if quantity > count { count = quantity; shortest = Some(k.clone()); };
    }

    if shortest != None {
      let tmp = shortest.unwrap();
      if !others.contains(&tmp) {
        others.push(tmp);
      }
    }
  }

  function.append(&mut others);
  LogicalFunction { implicants: function }

}