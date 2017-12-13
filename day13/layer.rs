use std::cmp::min;

/// Struct definining a firewall layer.
pub struct Layer {
    depth: u8,
    range: u8,
}

impl Layer {
    /// Constructor.
    pub fn new(depth: u8, range: u8) -> Self {
        Layer {
            depth,
            range,
        }
    }

    /// Get the severity of this layer if the player would be hit.
    fn severity(&self) -> u32 {
        self.depth as u32 * self.range as u32
    }

    /// Get the position this layer would have at the given picosecond.
    ///
    /// Note that the player would reach this position before this layer in the
    /// given picosecond.
    fn position(&self, pico: u32) -> u32 {
        // Determine what the full range with a full roundtrip would be
        let full_range = self.range as u32 * 2 - 2;

        // Wrap/bounce the number
        min(pico % full_range, full_range - pico % full_range)
    }

    /// Get the severity if the player would enter at the given `pico` second.
    ///
    /// If the player wasn't hit, `0` is returned.
    fn severity_at_pos(&self, pico: u8) -> u32 {
        if self.position(pico as u32) == 0 {
            self.severity()
        } else {
            0
        }
    }

    /// Get the severity if the player would enter this layer.
    /// The position of the player and the position of the scanner in the
    /// layer is based on the depth of the scanner.
    pub fn severity_at_depth_pos(&self) -> u32 {
        self.severity_at_pos(self.depth)
    }

    /// Check whether the player would be detected by the scanner on this layer,
    /// at the given `delay`.
    ///
    /// The positon of the scanner is determined based on the layers depth,
    /// and the given `delay`.
    ///
    /// True is returned if the player was detected, false if not.
    pub fn is_detected_at_delay(&self, delay: u32) -> bool {
        self.position(self.depth as u32 + delay) == 0
    }
}
