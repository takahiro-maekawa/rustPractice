fn main() {
  // イミュータブルな変数
  let x = 1 + 2;
  // ミュータブルな変数に束縛できる。
  let mut y = x;
  y = 5;
  // さらにイミュータブルな変数に束縛できる。
  let z = y;
  // z = 10; // これはエラーになる。

  println!("{}", z); // => 5
}