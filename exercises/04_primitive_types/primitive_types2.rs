// Characters (`char`)

fn main() {
    let my_first_initial: char = 'C'; // 字符
    let len: usize = std::mem::size_of_val(&my_first_initial); // 占用长度
    let addr: usize = std::ptr::addr_of!(my_first_initial) as usize; // 内存地址
    let code_point: Vec<u8> = my_first_initial.to_string().into_bytes(); // 字码
    assert_eq!(len, 4);
    assert_eq!(code_point[0], 67);
    println!("{}", len);
    println!("{:p}", addr as *const ());
    println!("{:?}", code_point[0]);

    assert_eq!(my_first_initial.is_alphabetic(), true);
    assert_eq!(my_first_initial.is_numeric(), false);

    let your_character: char = '😉';
    assert_eq!(your_character.is_alphabetic(), false);
    assert_eq!(your_character.is_numeric(), false);
}
