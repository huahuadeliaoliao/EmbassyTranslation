#Embassy应用基础
原文:https://embassy.dev/dev/basic_application.html

在成功运行Examples中的示例之后，下一步该何去何从？我们将通过nRF52 DK的简易Embassy应用示例，深入浅出地探索其底层原理。

---

##主干逻辑

完整的示例代码[见此](https://github.com/embassy-rs/embassy/tree/main/docs/modules/ROOT/examples/basic)。

---

##Rust Nightly 特性

Embassy需要依赖于一些nightly特性，声明如下：

'''
#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]
'''

---

##错误处理

接着是一些有关panic和故障处理的声明。在开发过程中，一个好的做法是使用 defmt-rtt 和 panic-probe 显示诊断信息：

'''
use {defmt_rtt as _, panic_probe as _}; // 全局日志记录
'''

---

##任务声明

完成基本的导入声明后，接下来是应用程序要运行的任务声明：

'''
#[embassy_executor::task]
async fn blinker(mut led: Output<'static, P0_13>, interval: Duration) {
    loop {
        led.set_high();
        Timer::after(interval).await;
        led.set_low();
        Timer::after(interval).await;
    }
}
'''

Embassy任务需声明为 async 形式，且不支持泛型参数。本例中，我们接收到了需要闪烁的LED及其闪烁间隔。

值得注意的是，此任务中避免了忙等操作。它利用Embassy计时器进行执行权让渡，允许微控制器在闪烁间隔中进入休眠。

---

##主函数

Embassy应用的主入口通过 #[embassy_executor::main] 宏进行定义，并且需要接收 Spawner 和 Peripherals 参数。

Spawner 是主应用生成其他任务的途径。Peripherals 类型来源于HAL，包含应用程序可能使用的所有外设。在此案例中，我们配置了一个作为GPIO输出的引脚来驱动LED：

'''
#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_nrf::init(Default::default());

    let led = Output::new(p.P0_13, Level::Low, OutputDrive::Standard);
    unwrap!(spawner.spawn(blinker(led, Duration::from_millis(300))));
}
'''

当 blinker 任务启动且主函数返回后会如何？实际上，主入口仅是一个特殊的任务，它需要一些特定类型的参数。 #[embassy::main] 宏的精髓在于：

'''
    创建Embassy执行器。
    初始化微控制器HAL以获取 Peripherals。
    为入口点定义主任务。
    启动执行器并运行主任务。
'''

也有一种不使用宏而直接运行执行器的方式，这时需要手动创建 Executor 实例。

---

##Cargo.toml

项目定义需包含Embassy的依赖项：

'''
embassy-executor = { version = "0.1.0", path = "../../../../../embassy-executor", features = ["defmt", "nightly", "integrated-timers"] }
embassy-time = { version = "0.1.0", path = "../../../../../embassy-time", features = ["defmt", "nightly"] }
embassy-nrf = { version = "0.1.0", path = "../../../../../embassy-nrf", features = ["defmt", "nrf52840", "time-driver-rtc1", "gpiote", "nightly"] }
'''

根据您的微控制器类型，可能需要将 embassy-nrf 替换为其他库（如对于STM32，使用 embassy-stm32）。记得同时更新功能标记。