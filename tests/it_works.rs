#[test]
fn it_works() {
    ufmt_stdio::init();
    ufmt_stdio::println!("Hellow 世界!");
}

#[test]
fn it_works2() {
    ufmt_stdio::init();
    ufmt_stdio::print!("Hellow World 2!\n");
}
