use std::cmp;

fn main() {
  let min_step = get_min_step_to_one(6);
  println!("min step: {}", min_step);
}

fn get_min_step_to_one(mut num: i8) ->i8 {
  if num == 1{
    return 0
  };
  let mut res = get_min_step_to_one(num-1);

  if num%2== 0 {
    res = cmp::min(res, get_min_step_to_one(num/2));
  }
  if num%3== 0 {
    res = cmp::min(res, get_min_step_to_one(num/3));
  }
  res + 1 
}
