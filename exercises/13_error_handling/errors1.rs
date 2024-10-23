pub fn generate_nametag_text(name: String) -> Result<String, String> {
    if name.is_empty() {
        Err("Empty names aren't allowed".to_string()) // 抛出一个可修复带信息的错误
    } else {
        Ok(format!("Hi! My name is {name}"))
    }
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generates_nametag_text_for_a_nonempty_name() {
        let text: String = "ab12".to_string();

        // String 内存固定为24字节(胖指针 = 指针 + 长度 + 容量), 不受字符串长度影响
        assert_eq!(std::mem::size_of_val(&text), 24);

        let text_result: Result<String, String> = generate_nametag_text(text);

        // Rust使用标记联合（tagged union）来实现枚举
        // 24 + 8(标记) = 32
        assert_eq!(std::mem::size_of_val(&text_result), 32);

        assert_eq!(text_result.is_ok(), true);
        assert_eq!(text_result.is_err(), false);

        /*
           as_deref常用于Option和Result的转换
           转为 &str
        */
        assert_eq!(text_result.as_deref(), Ok("Hi! My name is ab12"),);
        assert_eq!(text_result, Ok("Hi! My name is ab12".to_string()),);
    }

    #[test]
    fn explains_why_generating_nametag_text_fails() {
        assert_eq!(
            generate_nametag_text(String::new())
                .as_ref()
                .map_err(|e| e.as_str()),
            Err("Empty names aren't allowed"),
        );
    }
}
