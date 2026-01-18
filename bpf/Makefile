BPF_CLANG ?= clang
BPF_STRIP ?= llvm-strip
VMLINUX ?= /sys/kernel/btf/vmlinux
BPF_OBJ := ../target/bpf/lsm_block.bpf.o

all: $(BPF_OBJ)

vmlinux.h:
	bpftool btf dump file $(VMLINUX) format c > vmlinux.h

../target/bpf:
	mkdir -p ../target/bpf

$(BPF_OBJ): lsm_block.bpf.c vmlinux.h | ../target/bpf
	$(BPF_CLANG) -g -O2 -target bpf -D__TARGET_ARCH_x86 -Wall -Werror \
		-I. -c $< -o $@
	$(BPF_STRIP) -g $@

clean:
	rm -f $(BPF_OBJ) vmlinux.h

.PHONY: all clean
