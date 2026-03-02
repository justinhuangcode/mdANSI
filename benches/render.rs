use criterion::{black_box, criterion_group, criterion_main, Criterion};
use mdansi::{RenderOptions, Renderer, Theme};

const SAMPLE_MD: &str = r#"
# Benchmark Document

## Introduction

This is a **comprehensive** Markdown document used for benchmarking the `mdANSI`
rendering engine. It contains *various* elements including ~~strikethrough~~.

### Code Block

```rust
fn fibonacci(n: u64) -> u64 {
    match n {
        0 => 0,
        1 => 1,
        n => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

fn main() {
    for i in 0..20 {
        println!("fib({}) = {}", i, fibonacci(i));
    }
}
```

### Table

| Language | Speed   | Safety | Ecosystem |
|----------|---------|--------|-----------|
| Rust     | Fast    | High   | Growing   |
| Go       | Fast    | Medium | Mature    |
| Python   | Slow    | Low    | Massive   |
| C++      | Fastest | Low    | Massive   |

### Lists

- Item one with **bold** text
- Item two with `inline code`
  - Nested item with [link](https://example.com)
  - Another nested item
- Item three

1. First ordered item
2. Second ordered item
3. Third ordered item

### Blockquote

> "The best way to predict the future is to invent it."
> — Alan Kay

---

### Task List

- [x] Design the API
- [x] Implement the parser
- [ ] Write documentation
- [ ] Publish to crates.io

End of benchmark document.
"#;

fn bench_render(c: &mut Criterion) {
    let renderer = Renderer::new(
        Theme::default(),
        RenderOptions {
            width: 80,
            highlight: true,
            ..Default::default()
        },
    );

    c.bench_function("render_full_document", |b| {
        b.iter(|| {
            let output = renderer.render(black_box(SAMPLE_MD));
            black_box(output);
        })
    });
}

fn bench_render_no_highlight(c: &mut Criterion) {
    let renderer = Renderer::new(
        Theme::default(),
        RenderOptions {
            width: 80,
            highlight: false,
            ..Default::default()
        },
    );

    c.bench_function("render_no_highlight", |b| {
        b.iter(|| {
            let output = renderer.render(black_box(SAMPLE_MD));
            black_box(output);
        })
    });
}

fn bench_render_plain(c: &mut Criterion) {
    let renderer = Renderer::new(
        Theme::default(),
        RenderOptions {
            width: 80,
            plain: true,
            highlight: false,
            ..Default::default()
        },
    );

    c.bench_function("render_plain", |b| {
        b.iter(|| {
            let output = renderer.render(black_box(SAMPLE_MD));
            black_box(output);
        })
    });
}

criterion_group!(
    benches,
    bench_render,
    bench_render_no_highlight,
    bench_render_plain
);
criterion_main!(benches);
