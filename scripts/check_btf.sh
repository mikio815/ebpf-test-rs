set -euo pipefail

if [[ -r /sys/kernel/btf/vmlinux ]]; then
  echo "find /sys/kernel/btf/vmlinux"
  exit 0
fi

echo "BTF disabled or not found /sys/kernel/btf/vmlinux" >&2
exit 1
