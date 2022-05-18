use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Op {
  op: String,
  count: Option<usize>,
  chars: Option<String>
}

fn main() {
  assert!(is_valid(
    "Repl.it uses operational transformations to keep everyone in a multiplayer repl in sync.",
    "Repl.it uses operational transformations.",
    r#"[{"op": "skip", "count": 40}, {"op": "delete", "count": 47}]"#
  )); // true

  assert!(!is_valid(
    "Repl.it uses operational transformations to keep everyone in a multiplayer repl in sync.",
    "Repl.it uses operational transformations.",
    r#"[{"op": "skip", "count": 45}, {"op": "delete", "count": 47}]"#
  )); // false, delete past end

  assert!(!is_valid(
    "Repl.it uses operational transformations to keep everyone in a multiplayer repl in sync.",
    "Repl.it uses operational transformations.",
    r#"[{"op": "skip", "count": 40}, {"op": "delete", "count": 47}, {"op": "skip", "count": 2}]"#
  )); // false, skip past end

  assert!(is_valid(
    "Repl.it uses operational transformations to keep everyone in a multiplayer repl in sync.",
    "We use operational transformations to keep everyone in a multiplayer repl in sync.",
    r#"[{"op": "delete", "count": 7}, {"op": "insert", "chars": "We"}, {"op": "skip", "count": 4}, {"op": "delete", "count": 1}]"#
  )); // true

  assert!(is_valid(
    "Repl.it uses operational transformations to keep everyone in a multiplayer repl in sync.",
    "Repl.it uses operational transformations to keep everyone in a multiplayer repl in sync.",
    "[]"
  )); // true
  
  println!("Done!");
}

fn is_valid(stale: &str, latest: &str, otjson: &str) -> bool {
  let ops: Vec<Op> = serde_json::from_str(otjson).unwrap();
  let mut i = 0;
  let mut buf = stale.to_string();

  for op in ops.iter() {
    // println!("{:?}", op);

    let len = buf.chars().count();
    let count = op.count.unwrap_or(0);

    if i + count > len {
      return false;
    }
    
    match op.op.as_str() {
      "skip" => i = i + count,
      "insert" => match &op.chars {
        Some(str) => {
          buf.insert_str(i, str);
          i = i + str.chars().count();
        },
        None => ()
      },
      "delete" => buf.replace_range(i..(i + count), ""),
      _ => println!("Unknown op \"{}\"", op.op)
    }

    // println!("i = {}", i);
    // println!("buf = \"{}\"\n", buf);
  }

  // println!("Result = {}", buf);
  
  buf == latest
}