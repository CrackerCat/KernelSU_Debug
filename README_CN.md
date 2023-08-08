[English](README.md) | **简体中文** 

# KernelSU Debug

一个 Android 上基于内核的 root 方案。为调试用途做了一些修改。

## 特性

1. 基于内核的 su 和权限管理。
2. ~基于 overlayfs 的模块系统。~ (即将删除)
3. [App Profile](https://kernelsu.org/guide/app-profile.html): 把 Root 权限关进笼子里。


4. 去除无用的管理器签名验证，仅保留 package name 验证作为基础的安全校验，允许用户自行修改管理器 app。
5. 默认为 adb shell 提供 su 授权。
6. 默认设置 SELinux 为宽容模式，为使用 bootconfig 作为启动参数的设备提供一种永久获取**最高权限**的方式。
7. 默认授予全部应用 root 权限功能，仅在 Debug 构建中提供，供方便调试使用。

## 兼容状态

KernelSU 官方支持 GKI 2.0 的设备（内核版本5.10以上）；旧内核也是兼容的（最低4.14+），不过需要自己编译内核。

WSA, ChromeOS 和运行在容器上的 Android 也可以与 KernelSU 一起工作。

目前支持架构 : `arm64-v8a` 和 `x86_64`

## 讨论

- Telegram: [@MlgmXyysd_bibilailai](https://t.me/MlgmXyysd_bibilailai) (频道评论群)

## To-Dos

- 删除多余的模块功能
- Root 授权对话框
- 使用内核模块方式安装 KernelSU
- 允许在管理器中修改管理器包名（转移权限）
- 允许在管理器中开启严格执行的 SELinux

## 修改解释

- 在获取 root 权限的应用中限制用户自行修改分发版管理器是没有意义的，限制用户也违背了解锁刷机的初衷。仅保留包名验证做为基础安全校验，使管理器授权处于一种先装先得的状态已足够。

- 仅获取到 root 权限，在一些情况下执行操作仍然会被严格执行的 SELinux 策略挡掉，只有关闭 SELinux 或设置为宽容模式才允许执行，而 Android 无法直接关闭 SELinux。所以我们认为，在取得 root 的同时将 SELinux 设置为宽容模式（ROOT + Permissive），才是真正获取到了设备的最高权限。谷歌并不认为开启 SELinux Permissive 后关闭 seccomp 是安全问题，因此我们也不关心。

- KernelSU **只应该**提供 su 功能。模块功能是多余的，并且兼容性不佳。所以去除模块功能（保留启动脚本），让其只做应该做的事，还可以提升运行性能，没什么好解释的。

- KernelSU 不提供询问授权弹窗，在授权时会很麻烦。默认授权 adb shell 既保留了相对的应用安全，也方便用户操作终端调试。同理，默认授权全部应用 root 权限，仅供方便调试用，但因为安全风险，单独作为 Debug 构建的附加功能提供。


	如果您认为上述功能破坏了已刷入自定义固件设备的安全性，请先回顾一下 BootLoader 的解锁警告：

> <!>
> 
> By unlocking the bootloader, you will be able to install custom operating system on this phone. A custom OS is not subject to the same level of testing as the original OS, and can cause your phone and installed applications to stop working properly.
> 
> **Software integrity cannot be guaranteed with a custom OS, so any data stored on the phone while the bootloader is unlocked may be at risk.**
> 
> To prevent unauthorized access to your personal data, unlocking the bootloader will also delete all personal data on your phone.
> 
> Press the Volume keys to select whether to unlock the bootloader, then the Power Button to continue.
> 
> __________
> DO NOT UNLOCK THE BOOTLOADER
> __________
> UNLOCK THE BOOTLOADER
> __________

	以及

> <!>
> 
> The boot loader is unlocked and software integrity cannot be guaranteed. **Any data stored on the device may be available to attackers. Do not store any sensitive data on the device.**
> 
> Visit this link on another device:
> 
> g.co/ABH

	如果这仍然不能解决您的疑惑，请您点击窗口右上角的 X 按钮

## 使用方法

- [安装教程](https://kernelsu.org/zh_CN/guide/installation.html)
- [如何构建？](https://kernelsu.org/zh_CN/guide/how-to-build.html)

## 许可证

- 目录 `kernel` 下所有文件为 [GPL-2](https://www.gnu.org/licenses/old-licenses/gpl-2.0.en.html)
- 除 `kernel` 目录的其他部分均为 [GPL-3](https://www.gnu.org/licenses/gpl-3.0.html)

## 鸣谢

- [KernelSU](https://github.com/tiann/KernelSU): fork 源
- [kernel-assisted-superuser](https://git.zx2c4.com/kernel-assisted-superuser/about/): KernelSU 的灵感
- [Magisk](https://github.com/topjohnwu/Magisk): 强大的 root 工具箱
- [Diamorphine](https://github.com/m0nad/Diamorphine): 一些 rootkit 技巧
