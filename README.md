**English** | [简体中文](README_CN.md)

# KernelSU Debug

A Kernel based root solution for Android devices. Modified for debugging.

## Features

1. Kernel-based `su` and root access management.
2. ~Module system based on overlayfs.~ (Remove soon)
3. [App Profile](https://kernelsu.org/guide/app-profile.html): Lock up the root power in a cage.
4. <Modify> Remove the unnecessary manager signature validation, only keep package name validation as basic security. Now you can modify the manager as much as you want.
5. <Modify> Always allow adb shell root.
6. <Modify> Set SELinux Permissive by default for **highest privileges**.
7. <Modify> Allow all APPs root, only available on Debug variant.

## Compatibility State

KernelSU officially supports Android GKI 2.0 devices(with kernel 5.10+), old kernels(4.14+) is also compatible, but you need to build kernel yourself.

WSA, ChromeOS and containter-based Android can also work with KernelSU integrated.

And the current supported ABIs are : `arm64-v8a` and `x86_64`

## To-Dos

- Drop redundant modules feature
- Root access authorization dialog
- Install KernelSU as kernel module
- Allow modify manager package name in manager

## Workaround

- Restricting users from modifying the distribution manager in root access tools is pointless, and defeats the purpose of unlocking and rooting devices. It is enough to keep only the package name validation as a basic security to keep the manager authorization in a first-installed-first-served state.

- If you only got the root access, in some cases you will still be prevented by the Enforcing SELinux policy. The only way to do this is to disable SELinux (not allowed in Android) or set it to Permissive. So we believe that setting SELinux to Permissive mode (ROOT + Permissive) is the only way to really get the highest privileges on Android device. Google doesn't consider automatically disabling seccomp after setting SELinux Permissive to be a security issue, so we don't care too.

- KernelSU **SHOULD ONLY** provide root access related features, module features are redundant and poorly implemented for compatibility. So drop the redundant modules feature (but keep boot stage scripts) to keep it clean and improves runtime performance.

- KernelSU does not provide a dialog for authorization, which is annoying. Allow adb shell root by default preserves both relative application security and the convenience of terminal debugging. Similarly, root privileges are provided for all applications by default for debugging purposes only, but are provided separately for Debug variant due to security risks.


- If you believe that the above features undermine the security of a device that has been flashed with custom firmware, please review the BootLoader's unlock warning first:

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


> <!>
> 
> The boot loader is unlocked and software integrity cannot be guaranteed. **Any data stored on the device may be available to attackers. Do not store any sensitive data on the device.**
> 
> Visit this link on another device:
> 
> g.co/ABH

	- If this still does not solve your problem, please click on the X button in the upper right corner of the window.

## Usage

- [Installation Instruction](https://kernelsu.org/guide/installation.html)
- [How to build?](https://kernelsu.org/guide/how-to-build.html)

## License

- Files under `kernel` directory are [GPL-2](https://www.gnu.org/licenses/old-licenses/gpl-2.0.en.html)
- All other parts except `kernel` directory are [GPL-3](https://www.gnu.org/licenses/gpl-3.0.html)

## Credits

- [KernelSU](https://github.com/tiann/KernelSU): fork source
- [kernel-assisted-superuser](https://git.zx2c4.com/kernel-assisted-superuser/about/): the KernelSU idea
- [Magisk](https://github.com/topjohnwu/Magisk): the powerful root tool
- [Diamorphine](https://github.com/m0nad/Diamorphine): some rootkit skills
