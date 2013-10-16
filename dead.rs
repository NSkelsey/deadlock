# please compile using rust 0.7 not sure if .8 will work
use std::task::spawn;
use std::pipes::{stream, Port, Chan};

fn main() {

    let (portx, chanx): (Port<int>, Chan<int>) = stream();
    let (porty, chany): (Port<int>, Chan<int>) = stream();

    
    do spawn {
        porty.recv();
        chanx.send(20)
    }
    
    do spawn {
        portx.recv();
        chany.send(20);
    }
}

