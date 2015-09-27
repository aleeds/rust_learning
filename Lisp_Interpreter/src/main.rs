use std::io;

struct Value {
    iv : i32,
    b_iv : bool,

}

impl Value {
    fn new(v : &str) -> Value {
        let v: i32 = v.trim().parse()
         .ok()
         .expect("Values must be a number");

        Value {
            iv : v,b_iv : true

        }
    }
}

enum SyntaxTree {
    Err,
    Val{v : Value},
    Node{fnc : String, child : Vec<SyntaxTree>},
}

fn parse_to_vec(expr : &String) -> Vec<String> {
  let mut v : Vec<String> = Vec::new();
  let mut st = String::new();
  for c in expr.chars() {
    if c == '(' || c == ')' {
      if !st.is_empty() {
        v.push(st);
      }
      st = String::new();
      st.push(c);
      v.push(st);
      st = String::new();
    } else if c == ' ' {
      if !st.is_empty() {
        v.push(st);
      }
      st = String::new();
    } else {
      st.push(c);
    }
  }

  v
}

fn several_vecs_to_trees(vs : Vec<Vec<String>>) -> Vec<SyntaxTree> {
    let mut ret : Vec<SyntaxTree> = Vec::new();
    for v in vs.iter() {
        ret.push(vec_to_tree(v));
    }
    ret
}

fn vec_to_tree(tree : &Vec<String>) -> SyntaxTree {

   for i in 0..(tree.len()) {
       print!("{} ", tree[i]);
   }
   println!("");

   let mut num_right = 0;
   for c in tree.iter() {
       if c.clone() == "(".to_string() {
           num_right += 1;
       }
   }

   if tree.len() == 1 {
       return SyntaxTree::Val{v : Value::new(&tree[0])};
   } else if num_right == 1 {
      let mut vs : Vec<Vec<String>> = Vec::new();
      for i in  2..(tree.len() - 1) {
          vs.push(vec![tree[i].clone()]);
      }
      // need to find more elegant solution to this
      let vs = several_vecs_to_trees(vs);
      return SyntaxTree::Node{fnc : tree[1].clone(), child : vs};
   } else {
       let mut vecs : Vec<Vec<String>> = Vec::new();
       let mut i = 2;
       let mut right = 0;
       let mut add_v : Vec<String> = Vec::new();
       while i < tree.len() {
           if tree[i] == "(".to_string() {
               right += 1;
           } else if tree[i] == ")".to_string() {
               right += -1;
           }
           if right == 0 {
               add_v.push(")".to_string());
               vecs.push(add_v.clone());
               add_v = Vec::new();
           }
           else {
               add_v.push(tree[i].clone());
           }
           i += 1;
       }
       return SyntaxTree::Node{fnc : tree[1].clone(),child : several_vecs_to_trees(vecs)};

   }
   SyntaxTree::Err
}

fn parse_to_syntax_tree(expr : &String) -> SyntaxTree {
    let v = parse_to_vec(&expr);
    vec_to_tree(&v)
}

fn is_balanced(expr : &String) -> bool {
    let mut c = 0;
    for s in expr.chars() {
        if s == '(' {
            c += 1;
        }
        else if s == ')' {
            c += -1;
        }
        if c < 0 {
            break;
        }
    }
    c == 0
}

fn main() {
    let mut inp = String::new();
    // I need to change this so the loop will properly halt.
    // For now you exit program with ctrl c.


    while inp != "Exit" {
        io::stdin().read_line(&mut inp)
           .ok()
            .expect("Failed to read line");
        inp = String::from(inp.trim());
        let is_b = is_balanced(&inp);
        println!("{}",is_b);
        let parsed = parse_to_vec(&inp);
        vec_to_tree(&parsed);
        inp = String::from("");
    }
}
