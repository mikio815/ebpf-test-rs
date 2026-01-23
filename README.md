# ebpf-test-rs

## できること

- BTF有効環境のチェック: `scripts/check_btf.sh`
- LSM BPFハローワールド: `bpf/lsm_block.bpf.c` が `cheat.dat` への書き込み/実行を拒否
- Rustローダー: `loader` が BPFオブジェクトを読み込み LSMとしてアタッチ

## ビルド

```bash
cd bpf
make
cd ..
cargo build -p loader
```

`BPF_OBJECT` 環境変数で別の .bpf.o を指定可能 (デフォルト: `target/bpf/lsm_block.bpf.o`)

## 実行

```bash
sudo target/debug/loader
```

`cheat.dat` に対する書き込み/実行アクセスが -EPERM になる

## Lima-vm

[Lima-vm](https://github.com/lima-vm/lima)を使って実行する

### VMの立ち上げ

```bash
limactl start ./ebpf-lima.yaml

```

### 入る

```bash
limactl shell ebpf-lima
```

### セットアップ

```bash
sudo apt-get update
sudo apt-get install -y clang llvm lld make gcc pkg-config libelf-dev libbpf-dev \
    linux-tools-common linux-tools-$(uname -r) linux-headers-$(uname -r) build-essential strace git
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
sudo sed -i 's/^GRUB_CMDLINE_LINUX=.*/GRUB_CMDLINE_LINUX="lsm=landlock,lockdown,yama,integrity,apparmor,bpf"/' /etc/default/grub
sudo update-grub
sudo reboot
```

### ビルド

```bash
make -C bpf
cargo build -p loader
```

### 実行

```bash
sudo ./target/debug/loader 
```

### テスト

```bash
echo test > cheat.dat
```

`bash: echo: write error: Operation not permitted`が出ていれば成功
