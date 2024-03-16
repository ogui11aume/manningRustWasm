fn main() {
  let hello : String = String::from("Hello");
  take(hello); // From take(): Hello WasmEdge!
  // The following will fail since hello is already taken by take() and no longer available here
  // println!("From main(): {}", hello);

  let hello : String = String::from("Hello");
  take(hello.clone()); // From take(): Hello WasmEdge!
  println!("From main(): {}", hello); // From main(): Hello

  let hello : String = String::from("Hello");
  let hello_returned : String = borrow_and_return(&hello); // From borrow(): Hello WasmEdge!
  println!("From main() after Borrow returns: {}", hello_returned); // From main(): Hello

  let mut hello : String = String::from("Hello");
  borrow(&mut hello); // From borrow(): Hello WasmEdge!
  println!("From main() after Borrow mutated: {}", hello); // From main(): Hello
}

fn take (mut s: String) {
  s.push_str(" WasmEdge!");
  println!("From take(): {}", s);
}

fn borrow_and_return (s: &String) -> String {
  let mut buf = String::from(s);
  buf.push_str(" WasmEdge!");
  println!("From borrow(): {}", buf);
  buf
}

fn borrow (s: &mut String) {
  // Ok, * is a dereference operator. Also implicity works without it.
  (*s).push_str(" WasmEdge!");
  println!("From borrow(): {}", s);
}
