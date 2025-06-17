pub fn inverse_sound(samples :  Vec<i16>)->Vec<i16> {
    let reverse = reverse_vec(&samples);
    reverse
}

fn reverse_vec(vec:  &Vec<i16>) -> Vec<i16>
where
    i16: Clone,
     {
    let mut reversed_vec = vec.clone();
    let len = reversed_vec.len();
    let mid = len / 2;
    for i in 0..mid {
        reversed_vec.swap(i, len - i - 1);
    }
    reversed_vec
}