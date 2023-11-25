# Rust中并发性和Futures

并发性介绍:
    Rust 提供了高级的并发性支持。
    异步代码在 Rust 中的应用和限制。
    为什么需要 Rust 的运行时库。

理解 Futures:
    什么是 Future?: 表示将来某个时候完成的操作。
    Futures 的工作原理:
        轮询阶段: Future 被执行直到阻塞。
        等待阶段: 等待事件发生并唤醒 Future。
        唤醒阶段: 事件发生后，Future 被唤醒并再次轮询直到完成。

Futures 的类型:
    Leaf futures: 由运行时创建，代表资源（如套接字），操作非阻塞。
    Non-leaf futures: 使用 async 关键字创建，代表可暂停的计算，由一系列 await 的leaf-future 组成。

Rust的运行时(Runtimes):
    Rust 不自带并发运行时，需要使用库。
    运行时复杂性源于 Futures 的管理。
    异步运行时分为执行器和reactor。
    流行的运行时包括 async-std 和 Tokio。

Rust 标准库的角色:
    提供 Future trait 和 Waker 接口。
    通过 async 和 await 关键字暂停和恢复 Future。
    不包括异步 I/O 的定义和运行机制。

I/O密集型与CPU密集型任务:
    需要注意的是，在非叶子 Future 之间的代码将与执行器在同一线程上运行。
    对 CPU 密集型任务，有多种方法处理，如将任务发送到线程池。