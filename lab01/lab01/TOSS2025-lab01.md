# TOSS2025-lab01

## task 1

### 安装

在 [RUSTUP-INIT](https://www.rust-lang.org/learn/get-started) 下载系统相对应的 Rust 安装程序

```
Current installation options:

   default host triple: x86_64-pc-windows-msvc
     default toolchain: stable (default)
               profile: default
  modify PATH variable: yes

1) Proceed with installation (default)
2) Customize installation
3) Cancel installation

```

### 检查安装是否成功

在终端输入以下命令

```
rustc -V
rustc 1.86.0 (05f9846f8 2025-03-31)
cargo -V
cargo 1.86.0 (adf9b6ad1 2025-02-28)

```

此时能看见版本号即为安装成功

## task2

从github中下载一个小项目，在vs code中打开

（本报告使用项目：https://github.com/youngoing/RustProjects）

### 1.自动编译并运行项目

在终端中输入

```
cargo run
```

系统会自动编译并运行项目

![](.\p\1.png)

### 2.手动编译项目以及运行

编译：

```
cargo build
```

![](.\p\2.png)

运行：

```
./target/debug/game
```

![](.\p\3.png)

### 3.验证代码正确性

使用

```
cargo check
```

命令可以快速检查编译代码是否通过

![](.\p\4.png)

## task3

### 给rust安装包

在crate.io中找到感兴趣的包（本例为rand和clap）

打开终端并输入，自动下载并安装相对应包

```
cargo add clap
```

![](.\p\5.png)

```
cargo add rand
```

![](.\p\6.png)

### 示例

```
use clap::Parser;
use rand::Rng;

/// 生成指定范围内的随机整数
#[derive(Parser, Debug)]
struct Args {
    /// 最小值（包含）
    #[arg(short = 'n', long, default_value_t = 1)] // 修改短选项为 -n
    min: i32,

    /// 最大值（包含）
    #[arg(short = 'x', long, default_value_t = 100)] // 修改短选项为 -x
    max: i32,

    /// 生成数量
    #[arg(short, long, default_value_t = 1)]
    count: usize,
}

fn main() {
    let args = Args::parse();

    // 范围校验
    if args.min > args.max {
        eprintln!("错误：最小值 {} 不能大于最大值 {}", args.min, args.max);
        std::process::exit(1);
    }

    let mut rng = rand::thread_rng();

    println!("生成的随机数：");
    for _ in 0..args.count {
        let num = rng.gen_range(args.min..=args.max); // 确保这里实际使用了 rng
        println!("{}", num);
    }
}
```

输入用例：

```
cargo run -- -n 10 -x 50 -c 3
```

结果：

![](.\p\7.png)

### 笔记

在本程序中使用了clap和rand包

首先定义了一个结构体，其中将最小值的短选项设置为-n，长选项设置为min，最大值的短选项设置为-m，长选项设置为max，方便在运行的时候设置最大值和最小值的区间。

其次在主函数中先设置一个args自动进行参数的解析，然后手动判断最大值和最小值的取值是否合理。然后初始化随机数生成器，然后进行count次的for循环随机在最大值和最小值之间生成数字

## task4

### 1

在执行代码时被//注释的内容会自动忽略掉

![](.\p\8.png)

### 2

![](.\p\9.png)

```
//为行注释
/*   */为块注释，可以直接注释一块的内容
```

### 3

1.println中数字不加后缀默认为i32类型

2.在println中可以使用命名参数，如：

```
println!("{subject} {verb} {object}",
             object="the lazy dog",
             subject="the quick brown fox",
             verb="jumps over");
```

3.在println中在**：**后边指定特殊格式

```
println!("{} of {:b} people know binary, the other half don't", 1, 2);
```

4.println指定宽度：

```
println!("{number:>width$}", number=1, width=6);
//指定宽度来对齐右文本,输出 "     1"，5 个空格后面连着 1。
```

5.println会简餐使用到参数数量是否正确

错误：

```
println!("My name is {0}, {1} {0}", "Bond");
//缺少参数
```

更正后：

```
 println!("My name is {0}, {1} {0}", "Bond","James");
```

6.println无法输出自定义类型的结构体