fn main() {
    let _x = do_stuff(2.0, 12.5);
}


fn do_stuff(qty: f64, oz: f64 ) -> f64 {
    println!("{} {} - oz sarparilla(s)!", qty, oz);
    qty * oz
}