mod block;
mod blockchain;

use blockchain::BlockChain;
use std::io;

fn main() {
    let mut blockchain = BlockChain::new(5);

    loop {
        println!("\n请选择操作:");
        println!("1. 添加区块");
        println!("2. 查看区块链");
        println!("3. 验证区块");
        println!("4. Exist");

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        match input.trim() {
            "1" => {
                println!("输入的区块数据:");
                let mut data = String::new();
                io::stdin().read_line(&mut data).unwrap();
                blockchain.add_block(data.trim().to_string());
                println!("区块数据添加成功");
            }
            "2" => {
                for block in &blockchain.chain {
                    println!("{:?}", block);
                }
            }
            "3" => {
                if blockchain.is_valid() {
                    println!("区块链有效");
                } else {
                    println!("区块链无效");
                }
            }
            "4" => {
                println!("退出");
                break;
            }

            _ => println!("无效操作，请重试"),
        }
    }
}
