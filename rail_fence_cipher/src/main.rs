use std::io::{self, Write};

pub fn rail_fence_encrypt(plain_text: &str, key: usize) -> String {
    let mut cipher = vec![Vec::new(); key];

    for (c, i) in plain_text.chars().zip(zigzag(key)) {
        cipher[i].push(c);
    }

    cipher.iter().flatten().collect::<String>()
}

pub fn rail_fence_decrypt(cipher: &str, key: usize) -> String {
    let mut indices: Vec<_> = zigzag(key).zip(1..).take(cipher.len()).collect();
    indices.sort();

    let mut cipher_text: Vec<_> = cipher
        .chars()
        .zip(indices)
        .map(|(c, (_, i))| (i, c))
        .collect();

    cipher_text.sort();

    cipher_text.iter().map(|(_, c)| c).collect()
}

fn zigzag(n: usize) -> impl Iterator<Item = usize> {
    (0..n - 1).chain((1..n).rev()).cycle()
}

fn main() {
    println!("篱笆密码法(栅栏密码)");

    println!("篱笆密码法是一种置换式密码,它从编码的方式得到它的名字。在栅栏密码中,信息向下写在虚构的篱笆的连续”rail”上，然后当我们到达底部时，又向上移动(像锯齿形)。最后,消息以行读取.");

    let content = read_input("请输入你要加密的字符串");
    let key = read_input("请输入rails(栅栏)数");

    let key: usize = match key.trim().parse() {
        Ok(num) => {
            if num < 2 {
                eprint!("Rails (栅栏) 数必须大于等于 2");
                return;
            }
            num
        }
        Err(_) => {
            eprint!("错误: 请输入一个有效的数字作为rails(栅栏)数");
            return;
        }
    };

    let rail_fence_encrypt_result = rail_fence_encrypt(&content, key);
    println!("加密后的结果：{}", rail_fence_encrypt_result);

    let rail_fence_decrypt_result = rail_fence_decrypt(&rail_fence_encrypt_result, key);
    println!("解密后的结果：{}", rail_fence_decrypt_result);
}

fn read_input(prompt: &str) -> String {
    print!("{}\n>", prompt);
    io::stdout().flush().expect("无法刷新输出");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("读取输入失败");
    let input = input.trim();
    input.to_string()
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn rails_fence_test() {
        assert_eq!(rail_fence_encrypt("attack at once", 2), "atc toctaka ne");
        assert_eq!(rail_fence_decrypt("atc toctaka ne", 2), "attack at once");
    }
}
