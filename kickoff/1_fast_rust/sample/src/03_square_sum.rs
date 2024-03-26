

// isize型は「符号付き整数型の一種」
fn square_sum(n: isize) -> isize {
    // FizzBuzzと同じくレンジリテラル
    (0..n)
      // 高階関数の`filter`とクロージャリテラルの`|i| i % 2 == 0`
      .filter(|i| i % 2 == 0) //(i) => i % 2 == 0みたいなもんかな
      // 同じく高階関数の`map`
      .map(|i| i * i)
      // イテレータへの演算`sum`
      .sum()
      // returnを書かなくても最後の値が返り値になる。
}
// コンパイル時に -0 オプションを指定することで最適化されるd


/*
    関数の基本形は以下のとおりであり、void型を返却する場合には型を省略できる
    fn 関数名(引数) -> 返り値の型 {
    関数本体
    }
 */
fn main() {
    println!("{}", square_sum(5));
}