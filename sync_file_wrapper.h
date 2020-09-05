#include "kernel-hdr/sync_file.h"

#define MAKE_CONST(req_name) const __u64 MK_##req_name = req_name;

MAKE_CONST(SYNC_IOC_MERGE)
MAKE_CONST(SYNC_IOC_FILE_INFO)
