use staticvec::StaticVec;

pub fn main() {
  let mut v = StaticVec::<&f32, 24>::new();
  for _i in 0..v.capacity() {
    v.push(&24.5);
  }
  for f in &v {
    println!("{}", f);
  }
  println!("{}", "Clearing!");
  v.clear();
  v.insert(0, &47.6);
  v.insert(1, &48.6);
  v.insert(2, &49.6);
  v.insert(v.len() - 1, &50.6);
  v.insert(v.len() - 2, &51.6);
  v.insert(v.len() - 3, &52.6);
  for f in &v {
    println!("{}", f);
  }
  for f in 0..v.len() {
    println!("{}", v[f]);
  }
  v.remove(1);
  v.remove(2);
  for f in &v {
    println!("{}", f);
  }
  let mut va = StaticVec::<usize, 65536>::new();
  for i in 0..va.capacity() {
    va.push(i);
  }
  va.remove(10);
  va.remove(10);
  va.remove(10);
  va.remove(10);
  va.remove(10);
  va.remove(10);
  va.insert(10, 99);
  va.insert(10, 99);
  va.insert(10, 99);
  va.insert(10, 99);
  va.insert(10, 99);
  va.insert(10, 99);
  for i in 0..va.len() {
    println!("{}", va[i])
  }
  for i in &va {
    println!("{}", i)
  }
  while va.is_not_empty() {
    println!("{}", va.pop());
  }
  println!("{}", "Clearing!");
  va.clear();
  let mut vb = StaticVec::<&'static str, 24>::new();
  vb.push("a");
  vb.push("b");
  vb.push("c");
  vb.push("d");
  vb.push("e");
  vb.push("f");
  vb.push("g");
  vb.push("h");
  vb.remove(2);
  vb.remove(2);
  vb.remove(vb.len() - 1);
  for i in 0..vb.len() {
    println!("{}", vb[i]);
  }
  for s in &vb {
    println!("{}", s);
  }
  let pb = vb.as_mut_ptr();
  unsafe {
    println!("{}", *pb);
    println!("{}", *pb.add(1).add(1));
  }
  let pbc = vb.as_ptr();
  unsafe {
    println!("{}", *pbc);
    println!("{}", *pbc.add(1).add(1));
  }
  vb.clear();
  for _i in 0..vb.capacity() {
    vb.push("hello");
  }
  println!("{}", "pop");
  while vb.is_not_empty() {
    println!("{}", vb.remove(0));
  }
  println!("{}", "slice");
  vb.push("g");
  vb.push("f");
  vb.push("e");
  vb.push("d");
  vb.push("c");
  vb.push("b");
  vb.push("a");
  let vbm = vb.as_mut_slice();
  vbm.sort();
  for s in vbm {
    println!("{}", s);
  }
  let vbmb = vb.as_mut_slice();
  vbmb.reverse();
  for s in vbmb {
    println!("{}", s);
  }
  for s in vb.sorted() {
    println!("{}", s);
  }
  for s in vb.reversed() {
    println!("{}", s);
  }
  vb.clear();
}