#[test]
fn it_works() {
    ufmt_stdio::init();
    ufmt_stdio::println!("Hellow 世界!");
    ufmt_stdio::eprintln!("Error 世界!");
    ufmt_stdio::d_println!("Debug 世界!");
}

#[test]
fn it_works2() {
    ufmt_stdio::init();
    ufmt_stdio::print!("Hellow World 2!\n");
    ufmt_stdio::eprint!("Error World 2!\n");
    ufmt_stdio::d_print!("Debug World 2!\n");
}
