pub use crate::*;

pub fn generate_implicants(initial: Vec<Implicant>) -> Vec<Implicant> {
  let mut generated = vec![];
  for i in 0..2u32.pow(initial[0].terms.len() as u32) {
      let mut sad = format!("{:#032b}", i);
      sad = String::from(&sad[sad.len() - initial[0].terms.len()..]);

      let terms: Vec<_> = sad.split("")
          .filter(|x| *x != "").map(|x| match x {
              "0" => Some(false),
              "1" => Some(true),
              _ => unreachable!()
          }).collect();

      let imp = Implicant { terms, naming: initial[0].naming.clone() };
      let covered = initial.iter()
          .map(|x| x.terms.iter()
              .zip(imp.terms.iter())
              .map(|(&x, &y)| x == y || x == None)
              .all(|x| x))
          .any(|x| x);

      if !covered {
          generated.push(imp);
      }
  }
  generated
}
