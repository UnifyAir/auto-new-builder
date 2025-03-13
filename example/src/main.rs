use auto_new_builder::auto_new_builder_derive::AutoNewBuilder;


fn main() {
    let hello = Hello::new(21);

    println!("{:?}", hello.world);
    println!("{:?}", hello.next_world);

    let lello = Hello::new(42).with_next_world(32);

    println!("{:?}", lello.world);
    println!("{:?}", lello.next_world);

}

#[derive(AutoNewBuilder, Debug)]
pub struct Hello {
    world: u32,
    next_world: Option<u32>
}
