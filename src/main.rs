mod beep;

fn main() {
    println!("hans");
    beep::beep().expect("could not beep!");
}
