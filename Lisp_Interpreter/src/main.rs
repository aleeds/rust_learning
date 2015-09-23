use std::io;

enum SyntaxTree {
    Expression(String),
    Node(String,Vec<SyntaxTree>),
}

// fn Parse(expr : &str) -> SyntaxTree {
//
// }

fn is_balanced(expr : String) -> bool {
    let mut c = 0;
    for s in expr.chars() {
        if s == '(' {
            c += 1;
        }
        else if s == ')' {
            c += -1;
        }
        if c < 0 {
            // Why can't I just say false?
            // That is what the book says, but it doesn't do that
            return false;
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
        let is_b = is_balanced(inp.clone());
        println!("  {}  {}",inp,is_b);
        inp = String::from("");
    }
}
