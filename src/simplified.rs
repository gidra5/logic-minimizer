pub fn simplify(implicants: Vec<(Implicant, Vec<String>)>) -> Vec<(Implicant, Vec<String>)> {
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

  match simplified {
    1 => simplified
    2 => simplify(simplified)
  }
}


