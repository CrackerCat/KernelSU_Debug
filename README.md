**English** | [简体中文](README_CN.md)

# KernelSU Debug

A Kernel based root solution for Android devices. Modified for development debugging.

## Features

1. Kernel-based `su` and root access management.
2. [App Profile](https://kernelsu.org/guide/app-profile.html): Lock up the root power in a cage.

3. Remove redundant module functions, to make the root access management cleaner. [1]
4. Remove the unnecessary manager signature validation, only keep package name validation as basic security. Now you can modify the manager as much as you want. [2]
5. Set SELinux Permissive by default for **highest privileges**. [3]
6. Disabled the seccomp **non-security** Privilege Escalation, even if SELinux is set to Permissive on boot. [3]
7. Forced spoofing of SELinux state to Enforcing, even if it was actually Permissive, used to spoof authentication through Play Integrity.  [3]
8. Always allow adb shell root. [4]
9. Allow all APPs root, same as Magisk's auto response feature.
10. `init.d` support，ksud will parse or execute [stage].sh/system.prop/sepolicy.rule under `/data/ksu/common`.
11. Allow user to modify the target manager package name via the cli command.

## Compatibility State

KernelSU officially supports Android GKI 2.0 devices(with kernel 5.10+), old kernels(4.14+) are also compatible, but you need to build the kernel yourself.

WSA, ChromeOS and container-based Android can also work with KernelSU integrated.

And the current supported ABIs are: `arm64-v8a` and `x86_64`

## Discussion

- Telegram: [@MlgmXyysd_bibilailai](https://t.me/MlgmXyysd_bibilailai) (Channel Discussion Group)

## To-Dos

- Root access authorization dialog
- Allow modify manager package name in manager
- Allow enable SELinux Enforcing in manager
- Allow disable seccomp in manager
- Allow disable SELinux Enforcing proof in manager
- Install KernelSU as kernel module

## Workaround

1. KernelSU **SHOULD ONLY** provide root access related features, module features are redundant and poorly implemented for compatibility. So drop the redundant modules feature (but keep boot stage scripts) to keep it clean and improves runtime performance.

2. Restricting users from modifying the distribution manager in root access tools is pointless, and defeats the purpose of unlocking and rooting devices. Users should be able to verify that the software they are installing is trustworthy, otherwise any security measures are meaningless. It's enough to keep only the package name verification in the kernel as a basic security check, and leave the signature verification to the Android's PackageManager, which puts the manager's license in a first-install-first-out state. What? Core patch? Why do I need to consider users installing untrusted software on their own?

3. If you only got the root access, in some cases you will still be prevented by the Enforcing SELinux policy. The only way to do this is to disable SELinux (not allowed in Android) or set it to Permissive. So we believe that setting SELinux to Permissive mode (ROOT + Permissive) is the only way to really get the highest privileges on Android device. Google doesn't consider automatically disabling seccomp after setting SELinux Permissive to be a security issue, so we don't care too.

4. KernelSU does not provide a dialog for authorization, which is annoying. Adb shell is one of the core components of Android, there is no need to confirm its security. Trusting and authorizing it preserves relative application security as well as convenient user operation for terminal debugging.


	If you believe that the above features undermine the security of a device that has been flashed with custom firmware, please review the BootLoader's unlock warning first:

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

	And

> <!>
> 
> The boot loader is unlocked and software integrity cannot be guaranteed. **Any data stored on the device may be available to attackers. Do not store any sensitive data on the device.**
> 
> Visit this link on another device:
> 
> g.co/ABH

	If this still does not solve your problem, please click on the X button in the upper right corner of the window.

## Usage

- [Installation Instruction](https://kernelsu.org/guide/installation.html)
- [How to build?](https://kernelsu.org/guide/how-to-build.html)

## License

- Files under the `kernel` directory are [GPL-2](https://www.gnu.org/licenses/old-licenses/gpl-2.0.en.html)
- All other parts except the `kernel` directory are [GPL-3](https://www.gnu.org/licenses/gpl-3.0.html)

## Credits

- [KernelSU](https://github.com/tiann/KernelSU): fork source
- [kernel-assisted-superuser](https://git.zx2c4.com/kernel-assisted-superuser/about/): the KernelSU idea
- [Magisk](https://github.com/topjohnwu/Magisk): the powerful root tool
- [Diamorphine](https://github.com/m0nad/Diamorphine): some rootkit skills
