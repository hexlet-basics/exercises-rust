Изучать язык программирования, по традиции, начинают с программы 'Hello, World!', которая выводит этот текст на экран.

<pre class='hexlet-basics-output'>
  Hello, World!
</pre>

В языке Rust эта программа будет выглядеть так:

```rust
pub fn greet() {
  println!("Hello, World!");
}
```
Функция `println!` принимаем строку `Hello, World!` и выводит её в терминал.
