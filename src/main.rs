mod beep;
mod gui;

fn main() {
    gui::main().expect("gui error");

    /*
    beep::beep(beep::Beep{
        amplitude: 1.0,
        freq: 200.0,
    }).expect("could not beep!");
    */
}
