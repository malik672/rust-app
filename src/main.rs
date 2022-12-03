use std::io;

fn main() {
    //mutable string
    let mut index = String::new();

    io::stdin().read_line(&mut index).expect("Failed to read line");
    let index: usize = index
          .trim()
          .parse()
          .expect("write a valid number");

    fn convertfahenreit(f: i32) -> i32 {
       let a = 5/9 ;
        let b = f - 32;
       let c = a * b;
       return c;
    }

    fn convertcelsius(c: i32)  -> i32{
    
        let a = 9/5 ;
        let b = c + 32;
        let f = a * b;
        return f;
    }

}
