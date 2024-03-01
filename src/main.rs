use bfjit::AsmFunction;

fn main() {
    let bytes = include_bytes!("hello.bin");

    let f = AsmFunction::<_, ()>::new(bytes);

    let msg = "This appears to be working!\n".as_bytes();
    let msg_len = msg.len();

    unsafe {
        f.run((msg.as_ptr(), msg_len));
    }
}
