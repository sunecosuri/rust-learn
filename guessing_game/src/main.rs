use std::io;

fn main() {
    println!("数字当ててみて");
    println!("予想値を入力してください");
    let mut guess = String::new();
    io::stdin().read_line(&mut guess)
        .expect("行の読み取りに失敗しました");
    println!("あなたの予想値: {}", guess);
}
