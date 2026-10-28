#pragma once

// Consumer control usages surfaced in KeymapHidReport.consumer[].
// Firmware reads pressed codes from report->consumer (up to
// KEYMAP_HID_REPORT_CONSUMER_LEN entries; unused slots are zero).
#define CONSUMER_PLAY_PAUSE 0xCD
