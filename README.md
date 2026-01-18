# ebpf-test-rs Phase1

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
