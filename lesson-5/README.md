# lesson-5

## 知识

### ink! vs solidity

* 溢出保护: ink!默认开启，solidity没有
* constructor Functions: ink!多个，solidity单个
* 存储：ink!(kv), solidity(槽256bit)
* 类型：ink!支持更多

### ink!能做以及不能做什么

### 如何使用ink!编写、部署、测试智能合约

* 环境初始化设置

https://substrate.dev/substrate-contracts-workshop/#0/setup

rustup component add rust-src --toolchain nightly

rustup target add wasm32-unknown-unknown --toolchain stable

mac brew install binaryen

centos 从https://github.com/WebAssembly/binaryen/releases下载二进制，并放到path中。


* 安装substrate contracts node 节点

git clone https://github.com/paritytech/substrate-contracts-node.git

cd substrate-contracts-node && git checkout 35ee && cargo build --release

* 安装contract插件

cargo install cargo-contract --vers *0.14 --force --locked

* ink版本tag https://github.com/paritytech/ink/tree/v3.0.0-rc5

* 创建合约: cargo contract new flipper

* 编译合约： cargo +nightly contract build --release
编译分为5步
** 第一步编译合约(no-sdt)
** 第二步、第三步处理wasm文件(no-sdt)
** 第四步生成metadata(sdt)
** 第五步生成bundle,合约+metadata ABI

* 运行substrate-contracts-node: ./target/release/substrate-contracts-node --dev --tmp

* 打开 paritytech.github.io/canvas-ui/#/instantiate 连接本地substrate node

* 上传flipper/target/ink/flipper.contract，使用default默认值上传transaction

* 一份代码可以实例化多个合约

* 页面执行合约

### ink!合约代码解读

* 合约架构

存储#[ink(storage)]

合约实例化方法#[ink(constructor)]

公共方法（用户可调用）#[ink(message)]

事件#[ink(event)]

* ink!支持的存储类型

一般类型：bool, u{8,16,32,64,128}, i{8,16,32,64,128}, String, Tuples, arrays.

ink!类型：Vec, HashMap, Stash, Bitvec

Substrate类型：AccountId, Balance, Hash

### ink!的一些注意事项

### erc20
* 创建项目 cargo contract new erc20


## 作业

* 自己完成并部署一个erc20的智能合约

### 合约代码

[lib.rs](https://github.com/zongxunjie/SA/blob/main/lesson-5/erc20/lib.rs)

### 编译后wasm文件

[erc20.contract](https://github.com/zongxunjie/SA/blob/main/lesson-5/erc20.contract)

### 运行截图

* 网页初始化，使用本地canvas-ui

![1.png](https://github.com/zongxunjie/SA/blob/main/lesson-5/1.png)

* 上传合约

![2.png](https://github.com/zongxunjie/SA/blob/main/lesson-5/2.png)

* 初始化供应总量并实例化

![3.png](https://github.com/zongxunjie/SA/blob/main/lesson-5/3.png)

![4.png](https://github.com/zongxunjie/SA/blob/main/lesson-5/4.png)

![5.png](https://github.com/zongxunjie/SA/blob/main/lesson-5/5.png)

* balance_of 函数验证

![6.png](https://github.com/zongxunjie/SA/blob/main/lesson-5/6.png)

* total-supply 函数验证

![7.png](https://github.com/zongxunjie/SA/blob/main/lesson-5/7.png)

* transfer 函数验证

![8.png](https://github.com/zongxunjie/SA/blob/main/lesson-5/8.png)

![9.png](https://github.com/zongxunjie/SA/blob/main/lesson-5/9.png)

![10.png](https://github.com/zongxunjie/SA/blob/main/lesson-5/10.png)

![11.png](https://github.com/zongxunjie/SA/blob/main/lesson-5/11.png)

* approve 函数验证

![12.png](https://github.com/zongxunjie/SA/blob/main/lesson-5/12.png)

![13.png](https://github.com/zongxunjie/SA/blob/main/lesson-5/13.png)

![14.png](https://github.com/zongxunjie/SA/blob/main/lesson-5/14.png)

* transfer_from allowance 函数验证

![15.png](https://github.com/zongxunjie/SA/blob/main/lesson-5/15.png)

![16.png](https://github.com/zongxunjie/SA/blob/main/lesson-5/16.png)

![17.png](https://github.com/zongxunjie/SA/blob/main/lesson-5/17.png)

![18.png](https://github.com/zongxunjie/SA/blob/main/lesson-5/18.png)

![19.png](https://github.com/zongxunjie/SA/blob/main/lesson-5/19.png)

![20.png](https://github.com/zongxunjie/SA/blob/main/lesson-5/20.png)

![21.png](https://github.com/zongxunjie/SA/blob/main/lesson-5/21.png)

![22.png](https://github.com/zongxunjie/SA/blob/main/lesson-5/22.png)

![23.png](https://github.com/zongxunjie/SA/blob/main/lesson-5/23.png)

![24.png](https://github.com/zongxunjie/SA/blob/main/lesson-5/24.png)
