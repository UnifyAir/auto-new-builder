use auto_new_builder::auto_new_builder_derive::AutoNewBuilder;


fn main() {
    let hello = Hello::new(21, 55);

    println!("{:?}", hello.world);
    println!("{:?}", hello.next_world);

    let lello = Hello::new(42,  56).with_next_world(32);

    println!("{:?}", lello.world);
    println!("{:?}", lello.next_world);

}

#[derive(AutoNewBuilder, Debug)]
pub struct Hello {
    world: u32,
    #[auto_new_value=34]
    universe: u32,
    #[auto_new_value="\"HELLO\".to_string()"]
    next_next_world: String, 
    next_world: Option<u32>,
    #[auto_new_required]
    next_universe: Option<u32>,
    #[auto_new_value="say()"]
    super_next_next_world: Option<u32>, 
    #[auto_new_value="Some(46)"]
    super_super_next_next_world: Option<u32>, 
    
}

pub fn say() -> Option<u32> {
    Some(42)
}
