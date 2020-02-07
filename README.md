# expr-executor

Rust

## コマンド

```
// 実行
cargo run
```

## 動作

```
in [0] > 1 + 2
out[0] > 3
in [1] > 1204 + 4 + 10
out[1] > 1218
in [2] > (10+4) * (12/4) + 11
out[2] > 53
in [3] > exit
```

## 実行できる式

- 四則演算(+, -, *, /)を含む式
- ()を含む式
