/// 明示的なライフタイム注釈を持つ関数シグネチャ
/// 
/// * `i` - ライフタイムaを持つ、i32型への参照
/// * `j` - ライフタイムbを持つ、i32型への参照
fn add_with_lifetimes<'a, 'b>(i: &'a i32, j: &'b i32) -> i32 {
    *i + *j
}

/// "トレイト境界"を持つジェネリック関数の型シグネチャ
/// 同じ型の引数を2つ受け取り、その型の値を1つ返す
fn add<T: std::ops::Add<Output = T>>(i: T, j: T) -> T {
    i + j
}

fn main() {
    let a = 2;
    let b = 3;
    let return_value = add_with_lifetimes(&a, &b);
    println!("{}", return_value);

    println!("{} + {} = {}", a, b, add(a, 2));
}
