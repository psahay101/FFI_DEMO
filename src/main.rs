
use std::io;

#[link(name = "ffidemo", kind = "static")]
extern "C" {
    fn Play() -> ();
    fn Pause() -> ();
    fn Stop() -> ();
}


fn main() {
    println!("Press a to Play, Press b to Pause and Press c to Stop");
    let mut input_string = String::new();

    while input_string.trim() != "c" {
        input_string.clear();
        io::stdin().read_line(&mut input_string).unwrap();

        match input_string.trim() {
            "a" => unsafe {Play();},
            "b" => unsafe {Pause();},
            "c" => unsafe {Stop();},
            _ => println!("Invalid Input")
        }
    }
}
