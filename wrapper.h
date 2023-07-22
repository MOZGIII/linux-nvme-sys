#include <stdint.h>
#include <stdbool.h>

// In the nvme-cli, the `unlikely` macro is defined at `nvme-cli/nvme.h` -
// but we do not want to include that, so we inline it instead.
// This might hve to be changed when the `nvme-cli` submodule is updated to
// a newer version.
#define unlikely(x) x

#include "nvme-cli/linux/nvme.h"
#include "nvme-cli/linux/nvme_ioctl.h"
#include "nvme-cli/linux/lightnvm.h"
