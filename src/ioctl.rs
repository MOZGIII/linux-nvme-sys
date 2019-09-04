use super::*;
use nix::{ioctl_none, ioctl_readwrite, ioctl_write_ptr};

pub type nvme_admin_cmd = nvme_passthru_cmd;

// #define NVME_IOCTL_ID		_IO('N', 0x40)
// #define NVME_IOCTL_ADMIN_CMD	_IOWR('N', 0x41, struct nvme_admin_cmd)
// #define NVME_IOCTL_SUBMIT_IO	_IOW('N', 0x42, struct nvme_user_io)
// #define NVME_IOCTL_IO_CMD	_IOWR('N', 0x43, struct nvme_passthru_cmd)
// #define NVME_IOCTL_RESET	_IO('N', 0x44)
// #define NVME_IOCTL_SUBSYS_RESET	_IO('N', 0x45)
// #define NVME_IOCTL_RESCAN	_IO('N', 0x46)

ioctl_none!(nvme_ioctl_id, b'N', 0x40);
ioctl_readwrite!(nvme_ioctl_admin_cmd, b'N', 0x41, nvme_admin_cmd);
ioctl_write_ptr!(nvme_ioctl_submit_io, b'N', 0x42, nvme_user_io);
ioctl_readwrite!(nvme_ioctl_io_cmd, b'N', 0x43, nvme_passthru_cmd);
ioctl_none!(nvme_ioctl_reset, b'N', 0x44);
ioctl_none!(nvme_ioctl_subsys_reset, b'N', 0x45);
ioctl_none!(nvme_ioctl_rescan, b'N', 0x46);
