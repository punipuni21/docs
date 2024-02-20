fn main() {
    {
        let s = String::from("hello");
        println!("{:?}", s);
    }
    // println!("{:?}", s); スコープを抜ける時点でdrop関数を呼び出す．OSにメモリを返却している
}
