// run-rustfix

fn main() {
    println!("Testing option_take_on_temporary");
    let x = Some(3);
    let y = x.as_ref().take();
}