/// Auto generated from the original `wowm` in file [`wow_message_parser/wowm/world/movement/common_movement_3_3_5.wowm:11`](https://github.com/gtker/wow_messages/tree/main/wow_message_parser/wowm/world/movement/common_movement_3_3_5.wowm#L11):
/// ```text
/// flag MovementFlags : u32 {
///     NONE = 0x00000000;
///     FORWARD = 0x00000001;
///     BACKWARD = 0x00000002;
///     STRAFE_LEFT = 0x00000004;
///     STRAFE_RIGHT = 0x00000008;
///     LEFT = 0x00000010;
///     RIGHT = 0x00000020;
///     PITCH_UP = 0x00000040;
///     PITCH_DOWN = 0x00000080;
///     WALKING = 0x00000100;
///     ON_TRANSPORT = 0x00000200;
///     DISABLE_GRAVITY = 0x00000400;
///     ROOT = 0x00000800;
///     FALLING = 0x00001000;
///     FALLING_FAR = 0x00002000;
///     PENDING_STOP = 0x00004000;
///     PENDING_STRAFE_STOP = 0x00008000;
///     PENDING_FORWARD = 0x00010000;
///     PENDING_BACKWARD = 0x00020000;
///     PENDING_STRAFE_LEFT = 0x00040000;
///     PENDING_STRAFE_RIGHT = 0x00080000;
///     PENDING_ROOT = 0x00100000;
///     SWIMMING = 0x00200000;
///     ASCENDING = 0x00400000;
///     DESCENDING = 0x00800000;
///     CAN_FLY = 0x01000000;
///     FLYING = 0x02000000;
///     SPLINE_ELEVATION = 0x04000000;
///     SPLINE_ENABLED = 0x08000000;
///     WATERWALKING = 0x10000000;
///     FALLING_SLOW = 0x20000000;
///     HOVER = 0x40000000;
/// }
/// ```
#[derive(Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Copy, Clone, Default)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct MovementFlags {
    inner: u32,
}

impl MovementFlags {
    pub const fn new(inner: u32) -> Self {
        Self { inner }
    }

    pub const NONE: u32 = 0x00;
    pub const FORWARD: u32 = 0x01;
    pub const BACKWARD: u32 = 0x02;
    pub const STRAFE_LEFT: u32 = 0x04;
    pub const STRAFE_RIGHT: u32 = 0x08;
    pub const LEFT: u32 = 0x10;
    pub const RIGHT: u32 = 0x20;
    pub const PITCH_UP: u32 = 0x40;
    pub const PITCH_DOWN: u32 = 0x80;
    pub const WALKING: u32 = 0x100;
    pub const ON_TRANSPORT: u32 = 0x200;
    pub const DISABLE_GRAVITY: u32 = 0x400;
    pub const ROOT: u32 = 0x800;
    pub const FALLING: u32 = 0x1000;
    pub const FALLING_FAR: u32 = 0x2000;
    pub const PENDING_STOP: u32 = 0x4000;
    pub const PENDING_STRAFE_STOP: u32 = 0x8000;
    pub const PENDING_FORWARD: u32 = 0x10000;
    pub const PENDING_BACKWARD: u32 = 0x20000;
    pub const PENDING_STRAFE_LEFT: u32 = 0x40000;
    pub const PENDING_STRAFE_RIGHT: u32 = 0x80000;
    pub const PENDING_ROOT: u32 = 0x100000;
    pub const SWIMMING: u32 = 0x200000;
    pub const ASCENDING: u32 = 0x400000;
    pub const DESCENDING: u32 = 0x800000;
    pub const CAN_FLY: u32 = 0x1000000;
    pub const FLYING: u32 = 0x2000000;
    pub const SPLINE_ELEVATION: u32 = 0x4000000;
    pub const SPLINE_ENABLED: u32 = 0x8000000;
    pub const WATERWALKING: u32 = 0x10000000;
    pub const FALLING_SLOW: u32 = 0x20000000;
    pub const HOVER: u32 = 0x40000000;

    pub const fn empty() -> Self {
        Self { inner: 0 }
    }

    pub const fn is_empty(&self) -> bool {
        self.inner == 0
    }

    pub const fn all() -> Self {
        Self {
            inner: Self::NONE
                | Self::FORWARD
                | Self::BACKWARD
                | Self::STRAFE_LEFT
                | Self::STRAFE_RIGHT
                | Self::LEFT
                | Self::RIGHT
                | Self::PITCH_UP
                | Self::PITCH_DOWN
                | Self::WALKING
                | Self::ON_TRANSPORT
                | Self::DISABLE_GRAVITY
                | Self::ROOT
                | Self::FALLING
                | Self::FALLING_FAR
                | Self::PENDING_STOP
                | Self::PENDING_STRAFE_STOP
                | Self::PENDING_FORWARD
                | Self::PENDING_BACKWARD
                | Self::PENDING_STRAFE_LEFT
                | Self::PENDING_STRAFE_RIGHT
                | Self::PENDING_ROOT
                | Self::SWIMMING
                | Self::ASCENDING
                | Self::DESCENDING
                | Self::CAN_FLY
                | Self::FLYING
                | Self::SPLINE_ELEVATION
                | Self::SPLINE_ENABLED
                | Self::WATERWALKING
                | Self::FALLING_SLOW
                | Self::HOVER
        }
    }

    pub const fn is_FORWARD(&self) -> bool {
        (self.inner & Self::FORWARD) != 0
    }

    pub const fn new_FORWARD() -> Self {
        Self { inner: Self::FORWARD }
    }

    pub fn set_FORWARD(&mut self) -> Self {
        self.inner |= Self::FORWARD;
        *self
    }

    pub fn clear_FORWARD(&mut self) -> Self {
        self.inner &= Self::FORWARD.reverse_bits();
        *self
    }

    pub const fn is_BACKWARD(&self) -> bool {
        (self.inner & Self::BACKWARD) != 0
    }

    pub const fn new_BACKWARD() -> Self {
        Self { inner: Self::BACKWARD }
    }

    pub fn set_BACKWARD(&mut self) -> Self {
        self.inner |= Self::BACKWARD;
        *self
    }

    pub fn clear_BACKWARD(&mut self) -> Self {
        self.inner &= Self::BACKWARD.reverse_bits();
        *self
    }

    pub const fn is_STRAFE_LEFT(&self) -> bool {
        (self.inner & Self::STRAFE_LEFT) != 0
    }

    pub const fn new_STRAFE_LEFT() -> Self {
        Self { inner: Self::STRAFE_LEFT }
    }

    pub fn set_STRAFE_LEFT(&mut self) -> Self {
        self.inner |= Self::STRAFE_LEFT;
        *self
    }

    pub fn clear_STRAFE_LEFT(&mut self) -> Self {
        self.inner &= Self::STRAFE_LEFT.reverse_bits();
        *self
    }

    pub const fn is_STRAFE_RIGHT(&self) -> bool {
        (self.inner & Self::STRAFE_RIGHT) != 0
    }

    pub const fn new_STRAFE_RIGHT() -> Self {
        Self { inner: Self::STRAFE_RIGHT }
    }

    pub fn set_STRAFE_RIGHT(&mut self) -> Self {
        self.inner |= Self::STRAFE_RIGHT;
        *self
    }

    pub fn clear_STRAFE_RIGHT(&mut self) -> Self {
        self.inner &= Self::STRAFE_RIGHT.reverse_bits();
        *self
    }

    pub const fn is_LEFT(&self) -> bool {
        (self.inner & Self::LEFT) != 0
    }

    pub const fn new_LEFT() -> Self {
        Self { inner: Self::LEFT }
    }

    pub fn set_LEFT(&mut self) -> Self {
        self.inner |= Self::LEFT;
        *self
    }

    pub fn clear_LEFT(&mut self) -> Self {
        self.inner &= Self::LEFT.reverse_bits();
        *self
    }

    pub const fn is_RIGHT(&self) -> bool {
        (self.inner & Self::RIGHT) != 0
    }

    pub const fn new_RIGHT() -> Self {
        Self { inner: Self::RIGHT }
    }

    pub fn set_RIGHT(&mut self) -> Self {
        self.inner |= Self::RIGHT;
        *self
    }

    pub fn clear_RIGHT(&mut self) -> Self {
        self.inner &= Self::RIGHT.reverse_bits();
        *self
    }

    pub const fn is_PITCH_UP(&self) -> bool {
        (self.inner & Self::PITCH_UP) != 0
    }

    pub const fn new_PITCH_UP() -> Self {
        Self { inner: Self::PITCH_UP }
    }

    pub fn set_PITCH_UP(&mut self) -> Self {
        self.inner |= Self::PITCH_UP;
        *self
    }

    pub fn clear_PITCH_UP(&mut self) -> Self {
        self.inner &= Self::PITCH_UP.reverse_bits();
        *self
    }

    pub const fn is_PITCH_DOWN(&self) -> bool {
        (self.inner & Self::PITCH_DOWN) != 0
    }

    pub const fn new_PITCH_DOWN() -> Self {
        Self { inner: Self::PITCH_DOWN }
    }

    pub fn set_PITCH_DOWN(&mut self) -> Self {
        self.inner |= Self::PITCH_DOWN;
        *self
    }

    pub fn clear_PITCH_DOWN(&mut self) -> Self {
        self.inner &= Self::PITCH_DOWN.reverse_bits();
        *self
    }

    pub const fn is_WALKING(&self) -> bool {
        (self.inner & Self::WALKING) != 0
    }

    /// Walking
    ///
    pub const fn new_WALKING() -> Self {
        Self { inner: Self::WALKING }
    }

    pub fn set_WALKING(&mut self) -> Self {
        self.inner |= Self::WALKING;
        *self
    }

    pub fn clear_WALKING(&mut self) -> Self {
        self.inner &= Self::WALKING.reverse_bits();
        *self
    }

    pub const fn is_ON_TRANSPORT(&self) -> bool {
        (self.inner & Self::ON_TRANSPORT) != 0
    }

    /// `AzerothCore`: Used for flying on some creatures
    ///
    pub const fn new_ON_TRANSPORT() -> Self {
        Self { inner: Self::ON_TRANSPORT }
    }

    pub fn set_ON_TRANSPORT(&mut self) -> Self {
        self.inner |= Self::ON_TRANSPORT;
        *self
    }

    pub fn clear_ON_TRANSPORT(&mut self) -> Self {
        self.inner &= Self::ON_TRANSPORT.reverse_bits();
        *self
    }

    pub const fn is_DISABLE_GRAVITY(&self) -> bool {
        (self.inner & Self::DISABLE_GRAVITY) != 0
    }

    /// `AzerothCore`: Former `MOVEMENTFLAG_LEVITATING`. This is used when walking is not possible.
    ///
    pub const fn new_DISABLE_GRAVITY() -> Self {
        Self { inner: Self::DISABLE_GRAVITY }
    }

    pub fn set_DISABLE_GRAVITY(&mut self) -> Self {
        self.inner |= Self::DISABLE_GRAVITY;
        *self
    }

    pub fn clear_DISABLE_GRAVITY(&mut self) -> Self {
        self.inner &= Self::DISABLE_GRAVITY.reverse_bits();
        *self
    }

    pub const fn is_ROOT(&self) -> bool {
        (self.inner & Self::ROOT) != 0
    }

    /// `AzerothCore`: Must not be set along with `MOVEMENTFLAG_MASK_MOVING`
    ///
    pub const fn new_ROOT() -> Self {
        Self { inner: Self::ROOT }
    }

    pub fn set_ROOT(&mut self) -> Self {
        self.inner |= Self::ROOT;
        *self
    }

    pub fn clear_ROOT(&mut self) -> Self {
        self.inner &= Self::ROOT.reverse_bits();
        *self
    }

    pub const fn is_FALLING(&self) -> bool {
        (self.inner & Self::FALLING) != 0
    }

    /// `AzerothCore`: damage dealt on that type of falling
    ///
    pub const fn new_FALLING() -> Self {
        Self { inner: Self::FALLING }
    }

    pub fn set_FALLING(&mut self) -> Self {
        self.inner |= Self::FALLING;
        *self
    }

    pub fn clear_FALLING(&mut self) -> Self {
        self.inner &= Self::FALLING.reverse_bits();
        *self
    }

    pub const fn is_FALLING_FAR(&self) -> bool {
        (self.inner & Self::FALLING_FAR) != 0
    }

    pub const fn new_FALLING_FAR() -> Self {
        Self { inner: Self::FALLING_FAR }
    }

    pub fn set_FALLING_FAR(&mut self) -> Self {
        self.inner |= Self::FALLING_FAR;
        *self
    }

    pub fn clear_FALLING_FAR(&mut self) -> Self {
        self.inner &= Self::FALLING_FAR.reverse_bits();
        *self
    }

    pub const fn is_PENDING_STOP(&self) -> bool {
        (self.inner & Self::PENDING_STOP) != 0
    }

    pub const fn new_PENDING_STOP() -> Self {
        Self { inner: Self::PENDING_STOP }
    }

    pub fn set_PENDING_STOP(&mut self) -> Self {
        self.inner |= Self::PENDING_STOP;
        *self
    }

    pub fn clear_PENDING_STOP(&mut self) -> Self {
        self.inner &= Self::PENDING_STOP.reverse_bits();
        *self
    }

    pub const fn is_PENDING_STRAFE_STOP(&self) -> bool {
        (self.inner & Self::PENDING_STRAFE_STOP) != 0
    }

    pub const fn new_PENDING_STRAFE_STOP() -> Self {
        Self { inner: Self::PENDING_STRAFE_STOP }
    }

    pub fn set_PENDING_STRAFE_STOP(&mut self) -> Self {
        self.inner |= Self::PENDING_STRAFE_STOP;
        *self
    }

    pub fn clear_PENDING_STRAFE_STOP(&mut self) -> Self {
        self.inner &= Self::PENDING_STRAFE_STOP.reverse_bits();
        *self
    }

    pub const fn is_PENDING_FORWARD(&self) -> bool {
        (self.inner & Self::PENDING_FORWARD) != 0
    }

    pub const fn new_PENDING_FORWARD() -> Self {
        Self { inner: Self::PENDING_FORWARD }
    }

    pub fn set_PENDING_FORWARD(&mut self) -> Self {
        self.inner |= Self::PENDING_FORWARD;
        *self
    }

    pub fn clear_PENDING_FORWARD(&mut self) -> Self {
        self.inner &= Self::PENDING_FORWARD.reverse_bits();
        *self
    }

    pub const fn is_PENDING_BACKWARD(&self) -> bool {
        (self.inner & Self::PENDING_BACKWARD) != 0
    }

    pub const fn new_PENDING_BACKWARD() -> Self {
        Self { inner: Self::PENDING_BACKWARD }
    }

    pub fn set_PENDING_BACKWARD(&mut self) -> Self {
        self.inner |= Self::PENDING_BACKWARD;
        *self
    }

    pub fn clear_PENDING_BACKWARD(&mut self) -> Self {
        self.inner &= Self::PENDING_BACKWARD.reverse_bits();
        *self
    }

    pub const fn is_PENDING_STRAFE_LEFT(&self) -> bool {
        (self.inner & Self::PENDING_STRAFE_LEFT) != 0
    }

    pub const fn new_PENDING_STRAFE_LEFT() -> Self {
        Self { inner: Self::PENDING_STRAFE_LEFT }
    }

    pub fn set_PENDING_STRAFE_LEFT(&mut self) -> Self {
        self.inner |= Self::PENDING_STRAFE_LEFT;
        *self
    }

    pub fn clear_PENDING_STRAFE_LEFT(&mut self) -> Self {
        self.inner &= Self::PENDING_STRAFE_LEFT.reverse_bits();
        *self
    }

    pub const fn is_PENDING_STRAFE_RIGHT(&self) -> bool {
        (self.inner & Self::PENDING_STRAFE_RIGHT) != 0
    }

    pub const fn new_PENDING_STRAFE_RIGHT() -> Self {
        Self { inner: Self::PENDING_STRAFE_RIGHT }
    }

    pub fn set_PENDING_STRAFE_RIGHT(&mut self) -> Self {
        self.inner |= Self::PENDING_STRAFE_RIGHT;
        *self
    }

    pub fn clear_PENDING_STRAFE_RIGHT(&mut self) -> Self {
        self.inner &= Self::PENDING_STRAFE_RIGHT.reverse_bits();
        *self
    }

    pub const fn is_PENDING_ROOT(&self) -> bool {
        (self.inner & Self::PENDING_ROOT) != 0
    }

    pub const fn new_PENDING_ROOT() -> Self {
        Self { inner: Self::PENDING_ROOT }
    }

    pub fn set_PENDING_ROOT(&mut self) -> Self {
        self.inner |= Self::PENDING_ROOT;
        *self
    }

    pub fn clear_PENDING_ROOT(&mut self) -> Self {
        self.inner &= Self::PENDING_ROOT.reverse_bits();
        *self
    }

    pub const fn is_SWIMMING(&self) -> bool {
        (self.inner & Self::SWIMMING) != 0
    }

    /// `AzerothCore`: appears with fly flag also
    ///
    pub const fn new_SWIMMING() -> Self {
        Self { inner: Self::SWIMMING }
    }

    pub fn set_SWIMMING(&mut self) -> Self {
        self.inner |= Self::SWIMMING;
        *self
    }

    pub fn clear_SWIMMING(&mut self) -> Self {
        self.inner &= Self::SWIMMING.reverse_bits();
        *self
    }

    pub const fn is_ASCENDING(&self) -> bool {
        (self.inner & Self::ASCENDING) != 0
    }

    /// `AzerothCore`: press 'space' when flying
    ///
    pub const fn new_ASCENDING() -> Self {
        Self { inner: Self::ASCENDING }
    }

    pub fn set_ASCENDING(&mut self) -> Self {
        self.inner |= Self::ASCENDING;
        *self
    }

    pub fn clear_ASCENDING(&mut self) -> Self {
        self.inner &= Self::ASCENDING.reverse_bits();
        *self
    }

    pub const fn is_DESCENDING(&self) -> bool {
        (self.inner & Self::DESCENDING) != 0
    }

    pub const fn new_DESCENDING() -> Self {
        Self { inner: Self::DESCENDING }
    }

    pub fn set_DESCENDING(&mut self) -> Self {
        self.inner |= Self::DESCENDING;
        *self
    }

    pub fn clear_DESCENDING(&mut self) -> Self {
        self.inner &= Self::DESCENDING.reverse_bits();
        *self
    }

    pub const fn is_CAN_FLY(&self) -> bool {
        (self.inner & Self::CAN_FLY) != 0
    }

    /// `AzerothCore`: Appears when unit can fly AND also walk
    ///
    pub const fn new_CAN_FLY() -> Self {
        Self { inner: Self::CAN_FLY }
    }

    pub fn set_CAN_FLY(&mut self) -> Self {
        self.inner |= Self::CAN_FLY;
        *self
    }

    pub fn clear_CAN_FLY(&mut self) -> Self {
        self.inner &= Self::CAN_FLY.reverse_bits();
        *self
    }

    pub const fn is_FLYING(&self) -> bool {
        (self.inner & Self::FLYING) != 0
    }

    /// `AzerothCore`: unit is actually flying. pretty sure this is only used for players. creatures use `disable_gravity`
    ///
    pub const fn new_FLYING() -> Self {
        Self { inner: Self::FLYING }
    }

    pub fn set_FLYING(&mut self) -> Self {
        self.inner |= Self::FLYING;
        *self
    }

    pub fn clear_FLYING(&mut self) -> Self {
        self.inner &= Self::FLYING.reverse_bits();
        *self
    }

    pub const fn is_SPLINE_ELEVATION(&self) -> bool {
        (self.inner & Self::SPLINE_ELEVATION) != 0
    }

    /// `AzerothCore`: used for flight paths
    ///
    pub const fn new_SPLINE_ELEVATION() -> Self {
        Self { inner: Self::SPLINE_ELEVATION }
    }

    pub fn set_SPLINE_ELEVATION(&mut self) -> Self {
        self.inner |= Self::SPLINE_ELEVATION;
        *self
    }

    pub fn clear_SPLINE_ELEVATION(&mut self) -> Self {
        self.inner &= Self::SPLINE_ELEVATION.reverse_bits();
        *self
    }

    pub const fn is_SPLINE_ENABLED(&self) -> bool {
        (self.inner & Self::SPLINE_ENABLED) != 0
    }

    /// `AzerothCore`: used for flight paths
    ///
    pub const fn new_SPLINE_ENABLED() -> Self {
        Self { inner: Self::SPLINE_ENABLED }
    }

    pub fn set_SPLINE_ENABLED(&mut self) -> Self {
        self.inner |= Self::SPLINE_ENABLED;
        *self
    }

    pub fn clear_SPLINE_ENABLED(&mut self) -> Self {
        self.inner &= Self::SPLINE_ENABLED.reverse_bits();
        *self
    }

    pub const fn is_WATERWALKING(&self) -> bool {
        (self.inner & Self::WATERWALKING) != 0
    }

    /// `AzerothCore`: prevent unit from falling through water
    ///
    pub const fn new_WATERWALKING() -> Self {
        Self { inner: Self::WATERWALKING }
    }

    pub fn set_WATERWALKING(&mut self) -> Self {
        self.inner |= Self::WATERWALKING;
        *self
    }

    pub fn clear_WATERWALKING(&mut self) -> Self {
        self.inner &= Self::WATERWALKING.reverse_bits();
        *self
    }

    pub const fn is_FALLING_SLOW(&self) -> bool {
        (self.inner & Self::FALLING_SLOW) != 0
    }

    /// `AzerothCore`: active rogue safe fall spell (passive)
    ///
    pub const fn new_FALLING_SLOW() -> Self {
        Self { inner: Self::FALLING_SLOW }
    }

    pub fn set_FALLING_SLOW(&mut self) -> Self {
        self.inner |= Self::FALLING_SLOW;
        *self
    }

    pub fn clear_FALLING_SLOW(&mut self) -> Self {
        self.inner &= Self::FALLING_SLOW.reverse_bits();
        *self
    }

    pub const fn is_HOVER(&self) -> bool {
        (self.inner & Self::HOVER) != 0
    }

    /// `AzerothCore`: hover, cannot jump
    ///
    pub const fn new_HOVER() -> Self {
        Self { inner: Self::HOVER }
    }

    pub fn set_HOVER(&mut self) -> Self {
        self.inner |= Self::HOVER;
        *self
    }

    pub fn clear_HOVER(&mut self) -> Self {
        self.inner &= Self::HOVER.reverse_bits();
        *self
    }

    pub const fn as_int(&self) -> u32 {
        self.inner
    }

}

impl std::fmt::UpperHex for MovementFlags {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::UpperHex::fmt(&self.inner, f)
    }
}

impl std::fmt::LowerHex for MovementFlags {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::LowerHex::fmt(&self.inner, f)
    }
}

impl std::fmt::Octal for MovementFlags {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Octal::fmt(&self.inner, f)
    }
}

impl std::fmt::Binary for MovementFlags {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Binary::fmt(&self.inner, f)
    }
}

impl std::ops::BitAnd for MovementFlags {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self::Output {
        Self { inner: self.inner.bitand(rhs.inner), }
    }
}

impl std::ops::BitAndAssign for MovementFlags {
    fn bitand_assign(&mut self, rhs: Self) {
        self.inner.bitand_assign(rhs.inner)
    }
}

impl std::ops::BitOr for MovementFlags {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self::Output {
        Self { inner: self.inner.bitor(rhs.inner), }
    }
}

impl std::ops::BitOrAssign for MovementFlags {
    fn bitor_assign(&mut self, rhs: Self) {
        self.inner.bitor_assign(rhs.inner)
    }
}

impl std::ops::BitXor for MovementFlags {
    type Output = Self;
    fn bitxor(self, rhs: Self) -> Self::Output {
        Self { inner: self.inner.bitxor(rhs.inner), }
    }
}

impl std::ops::BitXorAssign for MovementFlags {
    fn bitxor_assign(&mut self, rhs: Self) {
        self.inner.bitxor_assign(rhs.inner)
    }
}

