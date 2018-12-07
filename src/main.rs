use std::io;

fn main(){
    io::copy(&mut io::stdin(), &mut io::sink()).unwrap();
}
