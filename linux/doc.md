[入門 モダンLinux](https://www.oreilly.co.jp/books/9784814400218/)の知識整理

FIXME：英語の勉強も兼ねて英語化したいね


## CPUアーキテクチャ
### Linuxが支持される理由
多くのCPUアーキテクチャ上で動作すること

### LinuxがどのCPUで動作しているか
lscpuで確認可能
```shell
$ lscpu

Architecture:                    x86_64
CPU op-mode(s):                  32-bit, 64-bit
Byte Order:                      Little Endian
Address sizes:                   46 bit
```

### BIOS(Basic I/O System)とUEFI(Unified Extensible Firmware Interface)とは
Linux起動時に使用．POST(Power On Selg Test)を実行した後，BIOSがハードウェアを初期化してブートローダに引き継ぐ

POST：ハードウェアが指定通りに機能するかを確認

モダンな環境でBIOSからUEFIに置き換わっている
詳細はhttps://www.freecodecamp.org/news/uefi-vs-bios/ を参照

OSの詳細は[osdev.org](https://wiki.osdev.org/Expanded_Main_Page)を参照

