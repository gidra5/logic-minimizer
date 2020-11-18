pub fn intersect<T: Eq + Clone>(arr1: &Vec<T>, arr2: &Vec<T>) -> Option<Vec<T>> {
  let mut intersect: Vec<T> = vec![];
  let mut a2 = arr2.clone();
  
  for i in arr1.iter() {
    let a = a2.clone();
    for (ind, val) in a.iter().enumerate() {
      if i == val {
        intersect.push(i.clone());
        a2.remove(ind);
      }
    }
  }

  if intersect.len() > 0 {
    Some(intersect)
  } else {
    None
  }
}