use cipher::*;

fn main() {
    println!("{}", add(2, 3));



    println!("{}", cipher("STICKS IN A BUNDLE ARE UNBREAKABLE.",3));
    println!("{}", decipher("VWLFNV LQ D EXQGOH DUH XQEUHDNDEOH.",3));


    assert_eq!("D",cipher("A",3));

}

