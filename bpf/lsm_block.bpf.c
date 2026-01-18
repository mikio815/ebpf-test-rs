#include "vmlinux.h"
#include <bpf/bpf_core_read.h>
#include <bpf/bpf_helpers.h>
#include <bpf/bpf_tracing.h>

char LICENSE[] SEC("license") = "Dual BSD/GPL";

const volatile int deny_mask = MAY_WRITE | MAY_EXEC;
const volatile char target_name[] = "cheat.dat";

SEC("lsm/file_permission")
int BPF_PROG(block_file_permission, struct file *file, int mask)
{
	struct dentry *d;
	struct qstr name;
	char buf[16];

	if (!(mask & deny_mask))
		return 0;

	d = BPF_CORE_READ(file, f_path.dentry);
	name = BPF_CORE_READ(d, d_name);

	if (name.len != sizeof(target_name) - 1)
		return 0;

	__builtin_memset(buf, 0, sizeof(buf));
	bpf_core_read_str(buf, sizeof(buf), name.name);

	if (__builtin_memcmp(buf, target_name, sizeof(target_name) - 1) == 0)
		return -EPERM;

	return 0;
}
