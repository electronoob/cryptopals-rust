fn main ()
{
    let hex = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
    println!("---------\n\ninitial hex string\n {}", hex);
    let slc = &hex[..2];
    let hex = &hex[2..];
    println!("first byte\n {}", slc);
    let x = u8::from_str_radix(slc, 16).unwrap();
    println!("first byte as u8\n {}", x);
    println!("hm\n {:?}", x as char);
    println!("modified hex string\n {}", hex);
}