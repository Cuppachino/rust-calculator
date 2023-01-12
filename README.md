# Shunting Yard Calculator

## build debug + execute

```ps1
cargo run
```

## build release

```ps1
cargo build -p
```

## execute release

### windows

```ps1
.\target\release\calculator.exe
```

### posix

```ps1
.\target\release\calculator
```

## Blurb

 I followed along with [this video](https://www.youtube.com/watch?v=KJwfZ06Z6og).
 This is mostly his implementation, but I made a few changes that reduced reptition and tested
 my understanding of `match`, `if let`, `while let`, `loop`, and `Some<T>` / `Option<T>`.

## There are a few problems with the implementation

- It doesn't handle negatives or floating points.
- it doesn't actually handle parentheses correctly. e.g. 2 * (2 + 4) = 12, but this gives 8.

I still learned a lot thanks to his video and I'm excited to try more with Rust.
