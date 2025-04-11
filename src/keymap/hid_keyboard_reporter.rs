use core::fmt::Debug;

use crate::key;

use super::MAX_PRESSED_KEYS;

/// Transforms output from the keymap so it's suitable for HID keyboard reports.
///
/// e.g. limits output to one new pressed key per sent report,
///  so that the USB host doesn't confuse the sequence of pressed keys.
#[derive(Debug)]
pub struct HIDKeyboardReporter {
    pressed_key_outputs: heapless::Vec<key::KeyOutput, { MAX_PRESSED_KEYS }>,
    num_reportable_keys: u8,
}

impl Default for HIDKeyboardReporter {
    fn default() -> Self {
        Self::new()
    }
}

impl HIDKeyboardReporter {
    /// Constructs a new HIDKeyboardReporter.
    pub const fn new() -> Self {
        Self {
            pressed_key_outputs: heapless::Vec::new(),
            num_reportable_keys: 1,
        }
    }

    /// Transforms the keymap output to a HID keyboard report.
    pub fn init(&mut self) {
        self.pressed_key_outputs.clear();
        self.num_reportable_keys = 1;
    }

    /// Updates the state of the HIDKeyboardReporter with the given pressed key outputs.
    pub fn update(
        &mut self,
        pressed_key_outputs: heapless::Vec<key::KeyOutput, { MAX_PRESSED_KEYS }>,
    ) {
        // e.g.
        //  WAS: A B C
        //  NOW: A   C D
        //   -> released B, pressed D
        let mut prev_iter = self.pressed_key_outputs.iter();
        let new_iter = pressed_key_outputs.iter();

        for new_key_output in new_iter {
            for prev_key_output in prev_iter.by_ref() {
                if prev_key_output == new_key_output {
                    // Same key output in both
                    break;
                } else {
                    // The key in the previous report doesn't match key in new report;
                    //  hence, it has been released.
                    if self.num_reportable_keys > 1 {
                        self.num_reportable_keys -= 1;
                    }
                }
            }
        }

        for _ in prev_iter {
            // The key in the previous report, but not in new report.
            //  hence, it has been released.
            if self.num_reportable_keys > 1 {
                self.num_reportable_keys -= 1;
            }
        }

        self.pressed_key_outputs = pressed_key_outputs;
    }

    /// Indicate an HID report was sent. Allows reporting one more key in the next report.
    pub fn report_sent(&mut self) {
        if self.pressed_key_outputs.len() > self.num_reportable_keys.into() {
            self.num_reportable_keys += 1;
        }
    }

    /// Gets the filtered pressed key outputs, suitable for sending for HID reports.
    pub fn reportable_key_outputs(&self) -> heapless::Vec<key::KeyOutput, { MAX_PRESSED_KEYS }> {
        self.pressed_key_outputs
            .clone()
            .into_iter()
            .take(self.num_reportable_keys as usize)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::super::*;
    use super::*;

    #[test]
    fn test_hid_keyboard_reporter_reports_single_keypress() {
        // Assemble
        let mut input: heapless::Vec<key::KeyOutput, { MAX_PRESSED_KEYS }> = heapless::Vec::new();
        input.push(key::KeyOutput::from_key_code(0x04)).unwrap();

        let mut reporter = HIDKeyboardReporter::new();

        // Act
        reporter.update(input);
        let actual_outputs = reporter.reportable_key_outputs();

        // Assert
        let expected_outputs: heapless::Vec<key::KeyOutput, { MAX_PRESSED_KEYS }> = [0x04]
            .iter()
            .map(|&kc| key::KeyOutput::from_key_code(kc))
            .collect();
        assert_eq!(expected_outputs, actual_outputs);
    }

    #[test]
    fn test_hid_keyboard_reporter_reports_single_new_keypress_per_report_sent() {
        // Assemble
        let input: heapless::Vec<key::KeyOutput, { MAX_PRESSED_KEYS }> = [0x04, 0x05]
            .iter()
            .map(|&kc| key::KeyOutput::from_key_code(kc))
            .collect();

        let mut reporter = HIDKeyboardReporter::new();

        // Act
        reporter.update(input);
        let actual_outputs = reporter.reportable_key_outputs();

        // Assert
        let expected_outputs: heapless::Vec<key::KeyOutput, { MAX_PRESSED_KEYS }> = [0x04]
            .iter()
            .map(|&kc| key::KeyOutput::from_key_code(kc))
            .collect();
        assert_eq!(expected_outputs, actual_outputs);
    }

    #[test]
    fn test_hid_keyboard_reporter_reports_more_keypresses_after_report_sent() {
        // Assemble
        let input: heapless::Vec<key::KeyOutput, { MAX_PRESSED_KEYS }> = [0x04, 0x05]
            .iter()
            .map(|&kc| key::KeyOutput::from_key_code(kc))
            .collect();

        let mut reporter = HIDKeyboardReporter::new();

        // Act
        reporter.update(input);
        reporter.report_sent();
        let actual_outputs = reporter.reportable_key_outputs();

        // Assert
        let expected_outputs: heapless::Vec<key::KeyOutput, { MAX_PRESSED_KEYS }> = [0x04, 0x05]
            .iter()
            .map(|&kc| key::KeyOutput::from_key_code(kc))
            .collect();
        assert_eq!(expected_outputs, actual_outputs);
    }

    #[test]
    fn test_hid_keyboard_reporter_reports_updates_for_key_releases() {
        // Assemble
        let input: heapless::Vec<key::KeyOutput, { MAX_PRESSED_KEYS }> = [0x04, 0x05]
            .iter()
            .map(|&kc| key::KeyOutput::from_key_code(kc))
            .collect();
        let input_after_key_released: heapless::Vec<key::KeyOutput, { MAX_PRESSED_KEYS }> = [0x05]
            .iter()
            .map(|&kc| key::KeyOutput::from_key_code(kc))
            .collect();
        let input_after_more_keys_pressed: heapless::Vec<key::KeyOutput, { MAX_PRESSED_KEYS }> =
            [0x05, 0x06, 0x07]
                .iter()
                .map(|&kc| key::KeyOutput::from_key_code(kc))
                .collect();

        let mut reporter = HIDKeyboardReporter::new();

        // Act
        reporter.update(input);
        reporter.report_sent(); // now may report 2 keys
        assert_eq!(2, reporter.num_reportable_keys);
        reporter.update(input_after_key_released); // 1 key released; so, only may report 1 key
        assert_eq!(1, reporter.num_reportable_keys);
        reporter.report_sent();
        assert_eq!(1, reporter.num_reportable_keys);
        reporter.update(input_after_more_keys_pressed); // 1+2 new pressed in KM; only 2 should reported
        reporter.report_sent();
        assert_eq!(2, reporter.num_reportable_keys);
        let actual_outputs = reporter.reportable_key_outputs();

        // Assert
        let expected_outputs: heapless::Vec<key::KeyOutput, { MAX_PRESSED_KEYS }> = [0x05, 0x06]
            .iter()
            .map(|&kc| key::KeyOutput::from_key_code(kc))
            .collect();
        assert_eq!(
            KeymapOutput::new(expected_outputs).as_hid_boot_keyboard_report(),
            KeymapOutput::new(actual_outputs).as_hid_boot_keyboard_report(),
        );
    }
}
