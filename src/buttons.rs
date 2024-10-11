//! Traits and structs for handling the button data of the controllers.

/// Many controllers have the same set of buttons (Square, Circle, L3, R1, etc).
/// The devices that do have these buttons implement this trait. Depite the original
/// Controller not having L3 and R3, they are brought out regardless and just considered
/// unpressable.
pub trait HasStandardButtons {
    /// This does require a clone operation of the bytes inside the controller.
    /// To save yourself the copy, you can access the button data directly via `buttons`
    fn buttons(&self) -> GamepadButtons;
}

/// A collection of helper functions to take the button bitfield and make them more
/// ergonomic to use.
pub trait Buttons {
    /// Constant representing the SELECT button.
    const SELECT: u16;
    /// Constant representing the L3 button.
    const L3: u16;
    /// Constant representing the R3 button.
    const R3: u16;
    /// Constant representing the START button.
    const START: u16;
    /// Constant representing the UP button.
    const UP: u16;
    /// Constant representing the RIGHT button.
    const RIGHT: u16;
    /// Constant representing the DOWN button.
    const DOWN: u16;
    /// Constant representing the LEFT button.
    const LEFT: u16;
    /// Constant representing the L2 button.
    const L2: u16;
    /// Constant representing the R2 button.
    const R2: u16;
    /// Constant representing the L1 button.
    const L1: u16;
    /// Constant representing the R1 button.
    const R1: u16;
    /// Constant representing the TRIANGLE button.
    const TRIANGLE: u16;
    /// Constant representing the CIRCLE button.
    const CIRCLE: u16;
    /// Constant representing the CROSS button.
    const CROSS: u16;
    /// Constant representing the SQUARE button.
    const SQUARE: u16;

    /// Returns the raw data representing the state of the buttons.
    fn data(&self) -> u16;

    /// Returns `true` if the SELECT button is pressed.
    fn select(&self) -> bool {
        self.data() & Self::SELECT == 0
    }

    /// Returns `true` if the L3 button is pressed.
    fn l3(&self) -> bool {
        self.data() & Self::L3 == 0
    }

    /// Returns `true` if the R3 button is pressed.
    fn r3(&self) -> bool {
        self.data() & Self::R3 == 0
    }

    /// Returns `true` if the START button is pressed.
    fn start(&self) -> bool {
        self.data() & Self::START == 0
    }

    /// Returns `true` if the UP button is pressed.
    fn up(&self) -> bool {
        self.data() & Self::UP == 0
    }

    /// Returns `true` if the RIGHT button is pressed.
    fn right(&self) -> bool {
        self.data() & Self::RIGHT == 0
    }

    /// Returns `true` if the DOWN button is pressed.
    fn down(&self) -> bool {
        self.data() & Self::DOWN == 0
    }

    /// Returns `true` if the LEFT button is pressed.
    fn left(&self) -> bool {
        self.data() & Self::LEFT == 0
    }

    /// Returns `true` if the L2 button is pressed.
    fn l2(&self) -> bool {
        self.data() & Self::L2 == 0
    }

    /// Returns `true` if the R2 button is pressed.
    fn r2(&self) -> bool {
        self.data() & Self::R2 == 0
    }

    /// Returns `true` if the L1 button is pressed.
    fn l1(&self) -> bool {
        self.data() & Self::L1 == 0
    }

    /// Returns `true` if the R1 button is pressed.
    fn r1(&self) -> bool {
        self.data() & Self::R1 == 0
    }

    /// Returns `true` if the TRIANGLE button is pressed.
    fn triangle(&self) -> bool {
        self.data() & Self::TRIANGLE == 0
    }

    /// Returns `true` if the CIRCLE button is pressed.
    fn circle(&self) -> bool {
        self.data() & Self::CIRCLE == 0
    }

    /// Returns `true` if the CROSS button is pressed.
    fn cross(&self) -> bool {
        self.data() & Self::CROSS == 0
    }

    /// Returns `true` if the SQUARE button is pressed.
    fn square(&self) -> bool {
        self.data() & Self::SQUARE == 0
    }

    /// Returns the raw data representing the state of the buttons.
    fn bits(&self) -> u16 {
        self.data()
    }
}

/// The digital buttons of the gamepad
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct GamepadButtons {
    data: u16,
}

impl Buttons for GamepadButtons {
    const SELECT: u16 = 0x0001;
    const L3: u16 = 0x0002;
    const R3: u16 = 0x0004;
    const START: u16 = 0x0008;
    const UP: u16 = 0x0010;
    const RIGHT: u16 = 0x0020;
    const DOWN: u16 = 0x0040;
    const LEFT: u16 = 0x0080;

    const L2: u16 = 0x0100;
    const R2: u16 = 0x0200;
    const L1: u16 = 0x0400;
    const R1: u16 = 0x0800;
    const TRIANGLE: u16 = 0x1000;
    const CIRCLE: u16 = 0x2000;
    const CROSS: u16 = 0x4000;
    const SQUARE: u16 = 0x8000;

    fn data(&self) -> u16 {
        self.data
    }
}

/// The digital buttons of the analog joystick (SCPH-1110) or analog controller (SCPH-1150) in green mode
#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct AnalogJoystickButtons {
    data: u16,
}

impl Buttons for AnalogJoystickButtons {
    const SELECT: u16 = 0x0001;
    const L3: u16 = 0x0002;
    const R3: u16 = 0x0004;
    const START: u16 = 0x0008;
    const UP: u16 = 0x0010;
    const RIGHT: u16 = 0x0020;
    const DOWN: u16 = 0x0040;
    const LEFT: u16 = 0x0080;

    const L2: u16 = 0x0100;
    const L1: u16 = 0x0200;
    const SQUARE: u16 = 0x0400;
    const TRIANGLE: u16 = 0x0800;
    const R1: u16 = 0x1000;
    const CIRCLE: u16 = 0x2000;
    const CROSS: u16 = 0x4000;
    const R2: u16 = 0x8000;

    fn data(&self) -> u16 {
        self.data
    }
}
