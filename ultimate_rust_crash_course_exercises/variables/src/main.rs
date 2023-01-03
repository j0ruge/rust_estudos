const STARTING_MISSILES : i32 = 8;
const READY_AMOUNT : i32 = 2;

fn main() {    
    let (missiles, ready) = (STARTING_MISSILES, READY_AMOUNT);
    let _gabiru = "nervoso";
    println!("Firing {} of my {} missiles...", ready, missiles);    
    println!("{} missiles left", missiles - ready);
}
