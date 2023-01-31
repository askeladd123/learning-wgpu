pub mod window;

fn main(){
    let mut data = window::Data::new();

    window::run(&mut data);
}