#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    ctrl: CTRL,
    rstcause: RSTCAUSE,
    cmd: CMD,
    rst: RST,
    lock: LOCK,
}
impl RegisterBlock {
    #[doc = "0x00 - Control Register"]
    #[inline(always)]
    pub const fn ctrl(&self) -> &CTRL {
        &self.ctrl
    }
    #[doc = "0x04 - Reset Cause Register"]
    #[inline(always)]
    pub const fn rstcause(&self) -> &RSTCAUSE {
        &self.rstcause
    }
    #[doc = "0x08 - Command Register"]
    #[inline(always)]
    pub const fn cmd(&self) -> &CMD {
        &self.cmd
    }
    #[doc = "0x0c - Reset Control Register"]
    #[inline(always)]
    pub const fn rst(&self) -> &RST {
        &self.rst
    }
    #[doc = "0x10 - Configuration Lock Register"]
    #[inline(always)]
    pub const fn lock(&self) -> &LOCK {
        &self.lock
    }
}
#[doc = "CTRL (rw) register accessor: Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl`]
module"]
pub type CTRL = crate::Reg<ctrl::CTRL_SPEC>;
#[doc = "Control Register"]
pub mod ctrl;
#[doc = "RSTCAUSE (r) register accessor: Reset Cause Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rstcause::R`].  See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rstcause`]
module"]
pub type RSTCAUSE = crate::Reg<rstcause::RSTCAUSE_SPEC>;
#[doc = "Reset Cause Register"]
pub mod rstcause;
#[doc = "CMD (w) register accessor: Command Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cmd::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cmd`]
module"]
pub type CMD = crate::Reg<cmd::CMD_SPEC>;
#[doc = "Command Register"]
pub mod cmd;
#[doc = "RST (rw) register accessor: Reset Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rst::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rst::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rst`]
module"]
pub type RST = crate::Reg<rst::RST_SPEC>;
#[doc = "Reset Control Register"]
pub mod rst;
#[doc = "LOCK (rw) register accessor: Configuration Lock Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lock::R`].  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lock::W`]. You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lock`]
module"]
pub type LOCK = crate::Reg<lock::LOCK_SPEC>;
#[doc = "Configuration Lock Register"]
pub mod lock;
