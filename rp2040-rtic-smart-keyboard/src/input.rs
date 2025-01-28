use rp2040_hal as hal;

use hal::gpio::{DynPinId, FunctionNull, FunctionSio, Pin, PullDown, PullUp, SioInput, SioOutput};

/// An unconfigured pin, for the given RP2040 pin ID.
pub type UnconfiguredPin<I> = Pin<I, FunctionNull, PullDown>;

/// An ID-erased PullUp input.
pub type Input = Pin<DynPinId, FunctionSio<SioInput>, PullUp>;

/// An ID-erased output.
pub type Output = Pin<DynPinId, FunctionSio<SioOutput>, PullDown>;
