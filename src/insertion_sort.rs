pub fn sort(v: Vec<i32>) -> Vec<i32> {
    let mut sorted = v.to_vec();
    let mut i = 1;
    let len = v.len();

    while i < len {
      let mut j = 1;
      while j > 0 && sorted[j-1] > sorted[j] {
        let a = sorted[j];
        let b = sorted[j-1];

        sorted[j] = b;
        sorted[j-1] = a;
        j = j - 1;
      }
      i = i + 1;
    }
    return sorted;
}