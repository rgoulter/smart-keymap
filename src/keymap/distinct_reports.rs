use core::fmt::Debug;

/// For tracking distinct HID reports from the keymap.
#[cfg(feature = "std")]
#[derive(Clone, Eq)]
pub struct DistinctReports(Vec<[u8; 8]>);

#[cfg(feature = "std")]
struct ReportDebugHelper<'a>(&'a [u8; 8]);

#[cfg(feature = "std")]
impl Debug for ReportDebugHelper<'_> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "[{:02X} {:02X} {:02X} {:02X} {:02X} {:02X} {:02X} {:02X}]",
            self.0[0], self.0[1], self.0[2], self.0[3], self.0[4], self.0[5], self.0[6], self.0[7]
        )
    }
}

#[cfg(feature = "std")]
impl Debug for DistinctReports {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        // Debug fmt with each of the [u8; 8] on one line.
        f.debug_tuple("DistinctReports")
            .field(
                &self
                    .0
                    .iter()
                    .map(|r| ReportDebugHelper(r))
                    .collect::<Vec<_>>(),
            )
            .finish()
    }
}

#[cfg(feature = "std")]
impl Default for DistinctReports {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(feature = "std")]
impl core::cmp::PartialEq for DistinctReports {
    fn eq(&self, other: &Self) -> bool {
        // First element in DistinctReports should be [0; 8].
        if self.0[0] != other.0[0] {
            return false;
        }

        let mut i: usize = 1;
        let mut j: usize = 1;

        let self_len = self.0.len();
        let other_len = other.0.len();

        // Compare the rest of the elements.
        while i < self_len && j < other_len {
            // Ignore [0; 8] elements.
            // (The reports are distinct; so, no two elements should be equal)
            while (i < self_len - 1) && self.0[i] == [0; 8] {
                i += 1;
            }
            while (j < other_len - 1) && other.0[j] == [0; 8] {
                j += 1;
            }

            if self.0[i] != other.0[j] {
                // Special cases
                //  - comparing "modifier pressed" in two reports, vs one report.
                //  - comparing "modifier released" in two reports, vs one report.
                if i > 0
                    && i < self_len - 1
                    && self.0[i + 1] == other.0[j]
                    && ((self.0[i - 1][0] == self.0[i][0] && self.0[i][2..] == self.0[i + 1][2..])
                        || (self.0[i - 1][2..] == self.0[i][2..]
                            && self.0[i][0] == self.0[i + 1][0]))
                {
                    // self uses two reports for equivalent of one report in other
                    i += 1;
                } else if j > 0
                    && j < other_len - 1
                    && self.0[i] == other.0[j + 1]
                    && ((other.0[j - 1][0] == other.0[j][0]
                        && other.0[j][2..] == other.0[j + 1][2..])
                        || (other.0[j - 1][2..] == other.0[j][2..]
                            && other.0[j][0] == other.0[j + 1][0]))
                {
                    // other uses two reports for equivalent of one report in self
                    j += 1;
                } else {
                    return false;
                }
            }

            i += 1;
            j += 1;
        }

        i == self_len && j == other_len
    }
}

#[cfg(feature = "std")]
impl DistinctReports {
    /// Constructs a new DistinctReports.
    pub fn new() -> Self {
        Self(vec![[0; 8]])
    }

    /// Adds the report to the distinct reports.
    pub fn update(&mut self, report: [u8; 8]) {
        match self.0.last() {
            Some(last_report) if last_report == &report => {}
            _ => self.0.push(report),
        }
    }

    /// Access reports as slice of reports.
    pub fn reports(&self) -> &[[u8; 8]] {
        self.0.as_slice()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_distinct_reports_equal() {
        // Assemble
        let lhs = DistinctReports(vec![[0, 0, 0, 0, 0, 0, 0, 0], [0, 0, 0x04, 0, 0, 0, 0, 0]]);
        let rhs = DistinctReports(vec![[0, 0, 0, 0, 0, 0, 0, 0], [0, 0, 0x04, 0, 0, 0, 0, 0]]);

        // Act

        // Assert
        assert!(lhs == rhs);
    }

    #[test]
    fn test_distinct_reports_not_equal() {
        // Assemble
        let lhs = DistinctReports(vec![[0, 0, 0, 0, 0, 0, 0, 0], [0, 0, 0x04, 0, 0, 0, 0, 0]]);
        let rhs = DistinctReports(vec![[0, 0, 0, 0, 0, 0, 0, 0], [0, 0, 0x05, 0, 0, 0, 0, 0]]);

        // Act

        // Assert
        assert!(lhs != rhs);
    }

    #[test]
    fn test_distinct_reports_not_equal_modif() {
        // Assemble
        let lhs = DistinctReports(vec![[0, 0, 0, 0, 0, 0, 0, 0], [0, 0, 0x04, 0, 0, 0, 0, 0]]);
        let rhs = DistinctReports(vec![
            [0, 0, 0, 0, 0, 0, 0, 0],
            [0x01, 0, 0x04, 0, 0, 0, 0, 0],
        ]);

        // Act

        // Assert
        assert!(lhs != rhs);
    }

    #[test]
    fn test_distinct_reports_equal_ignores_0_between() {
        // Assemble
        let lhs = DistinctReports(vec![
            [0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0x04, 0, 0, 0, 0, 0],
            [0, 0, 0x05, 0, 0, 0, 0, 0],
        ]);
        let rhs = DistinctReports(vec![
            [0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0x04, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0x05, 0, 0, 0, 0, 0],
        ]);

        // Act

        // Assert
        assert!(lhs == rhs);
    }

    #[test]
    fn test_distinct_reports_not_equal_respects_trailing_0() {
        // Assemble
        let lhs = DistinctReports(vec![
            [0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0x04, 0, 0, 0, 0, 0],
            [0, 0, 0x05, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0],
        ]);
        let rhs = DistinctReports(vec![
            [0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0x04, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0x05, 0, 0, 0, 0, 0],
        ]);

        // Act

        // Assert
        assert!(lhs != rhs);
    }

    #[test]
    fn test_distinct_reports_update_ignores_consecutive_duplicate() {
        // Assemble
        let lhs = DistinctReports(vec![[0, 0, 0, 0, 0, 0, 0, 0], [0, 0, 0x04, 0, 0, 0, 0, 0]]);

        // Act
        let mut rhs = DistinctReports::new();
        rhs.update([0, 0, 0x04, 0, 0, 0, 0, 0]);
        rhs.update([0, 0, 0x04, 0, 0, 0, 0, 0]);
        rhs.update([0, 0, 0x04, 0, 0, 0, 0, 0]);

        // Assert
        assert!(lhs == rhs);
    }

    #[test]
    fn test_distinct_reports_allows_modifier_press_equivalence() {
        // Assemble
        let lhs = DistinctReports(vec![
            [0, 0, 0, 0, 0, 0, 0, 0],
            [0x01, 0, 0x04, 0, 0, 0, 0, 0],
        ]);
        let rhs = DistinctReports(vec![
            [0, 0, 0, 0, 0, 0, 0, 0],
            [0x01, 0, 0, 0, 0, 0, 0, 0],
            [0x01, 0, 0x04, 0, 0, 0, 0, 0],
        ]);

        // Act

        // Assert
        assert!(lhs == rhs);
    }

    #[test]
    fn test_distinct_reports_allows_modifier_release_equivalence() {
        // Assemble
        let lhs = DistinctReports(vec![
            [0, 0, 0, 0, 0, 0, 0, 0],
            [0x01, 0, 0x04, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0],
        ]);
        let rhs = DistinctReports(vec![
            [0, 0, 0, 0, 0, 0, 0, 0],
            [0x01, 0, 0, 0, 0, 0, 0, 0],
            [0x01, 0, 0x04, 0, 0, 0, 0, 0],
            [0x01, 0, 0, 0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0, 0],
        ]);

        // Act

        // Assert
        assert!(lhs == rhs);
    }
}
