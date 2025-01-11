# axhal_crates

Reusable crates used for [ArceOS](https://github.com/arceos-org/arceos) Hardware Abstraction Layer (HAL).

## Common crates

* [axhal_cpu](./axhal_cpu)
* [axhal_plat](./axhal_plat)

## Platform-specific crates

* [axplat-x86-pc](./platforms/axplat-x86-pc)
* [axplat-riscv-qemu-virt](./platforms/axplat-riscv-qemu-virt)
* [axplat-aarch64-qemu-virt](./platforms/axplat-riscv-qemu-virt)
* [axplat-aarch64-raspi](./platforms/axplat-aarch64-raspi)
* [axplat-aarch64-phytium-pi](./platforms/axplat-aarch64-raspi)
* [axplat-aarch64-bsta1000b](./platforms/axplat-aarch64-bsta1000b)

## Utility crates

* [axplat-cli](./axplat-cli): A CLI tool to manage hardware platform packages using [axhal_plat](https://github.com/arceos-org/axhal_crates/tree/main/axhal_plat).

