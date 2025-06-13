#include "hidapi.h"
#include <wchar.h>

int hid_init(void) {
    return 0;
}

int hid_exit(void) {
    return 0;
}

struct hid_device_info *hid_enumerate(unsigned short vendor_id, unsigned short product_id) {
    return NULL;
}

void hid_free_enumeration(struct hid_device_info *devs) {
    // Nothing to do for a NULL list
}
