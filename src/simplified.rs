// #![feature(non_ascii_idents)]
pub use crate::*;

fn partition_implicants(implicants: &Vec<( Implicant, Vec<Option<bool>> )>, n: Option<usize>) 
  -> Vec<Vec<( Implicant, Vec<Option<bool>> )>>
{
  let k = match n {Some(x) => x, _ => implicants[0].0.terms.len() };
  if k == 0 { return vec![]; }

  let (left, right): (Vec<_>, Vec<_>) = implicants.iter()
    .partition(|x| x.0.terms
      .iter()
      .filter(|&&x| x == None)
      .collect::<Vec<_>>()
      .len() >= k - 1);

  let mut t = vec![left.iter().map(|&x| x.clone()).collect::<Vec<_>>()];

  t.append(&mut partition_implicants(&right.iter().map(|&x| x.clone()).collect::<Vec<_>>(), Some(k - 1)));
  t
}

pub fn simplify(implicants: &Vec<( Implicant, Vec<Option<bool>> )>) -> Vec<( Implicant, Vec<Option<bool>> )> {
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
        // println!("{}, {}, {:?}", i + 1, j + 1, intersect);
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
    simplified = simplify(&simplified);
  }

  #[allow(unused_mut)]
  let mut tmp: Vec<_> = implicants.into_iter()
    .enumerate()
    .filter(|&(i, _)| simplified_functions[i].contains(&Some(true)))
    .map(|(i, (x, _))| (x.clone(), simplified_functions[i].clone()))
    .collect();

  tmp.append(&mut simplified);
  tmp
}

pub fn construct_func(initial:    &Vec<( Implicant, Vec<Option<bool>> )>, 
                      simplified: &Vec<( Implicant, Vec<Option<bool>> )>
  ) -> Vec<LogicalFunction> 
{
  let mut table: Vec<Vec<Vec<Implicant>>> = Vec::with_capacity(initial[0].1.len());
  table.resize(initial[0].1.len(), vec![]);

  for i in table.iter_mut() {
    *i = Vec::with_capacity(initial.len());
    i.resize(initial.len(), vec![]);
    for j in i.iter_mut() {
      *j = vec![];
    }
  }

  //table[func_index][implicant_index].push(simplified_implicant);

  for (initial_id, i) in initial.iter().enumerate() {
    for j in simplified.iter() {
      let zipped: Vec<_> = i.0.terms.iter().zip(j.0.terms.iter())
        .map(|(a, b)| a == b || *b == None )
        .collect();

      // let covered = i.0.terms.iter()
      //   .zip(j.0.terms.iter())
      //   .map(|(&x, &y)| x == y || x == None)
      //   .all(|x| x);

        // if !covered {
        //     generated.push(imp);
        // }

      let intersect = i.1.iter()
        .zip(j.1.iter())
        .enumerate()
        .filter(|(_i, x)| match x {
            (Some(true), Some(true)) => true,
            _ => false
        })
        .map(|(i, _x)| i);

      if !zipped.contains(&false) { 
        for k in intersect {
          if !table[k][initial_id].contains(&j.0.clone()) {
            table[k][initial_id].push(j.0.clone());
          }
        };
      };
    };
  };

  let mut functions = vec![]; 

  for (fn_id, i) in table.iter().enumerate() {
    let mut function = vec![];
    let (unsorted_core, other_simples): (Vec<_>, Vec<_>) = i
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

    // println!("asdd");
    // for k in function.iter() {
    //   println!("{:?}", k);
    // }
    // for k in other_simples.iter() {
    //   println!("{:?}", k);
    // }

    let mut others: Vec<_> = vec![];
    for (impl_id, j) in other_simples {
      if initial[impl_id].1[fn_id] == None {
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
    functions.push(LogicalFunction { implicants: function });
  };

  // println!("RES:");
  // for (fn_id, i) in functions.iter().enumerate() {
  //   println!("{:?}", fn_id);
  //   for k in i.implicants.iter() {
  //     println!("{:?}", k);
  //   }
  // };

  functions
}