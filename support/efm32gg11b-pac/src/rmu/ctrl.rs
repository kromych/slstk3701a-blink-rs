#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CTRL_SPEC>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CTRL_SPEC>;
#[doc = "Field `WDOGRMODE` reader - WDOG Reset Mode"]
pub type WDOGRMODE_R = crate::FieldReader<WDOGRMODE_A>;
#[doc = "WDOG Reset Mode\n\nValue on reset: 4"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum WDOGRMODE_A {
    #[doc = "0: Reset request is blocked. This disable bit is redundant with enable/disable bit in WDOG"]
    DISABLED = 0,
    #[doc = "1: The CRYOTIMER, DEBUGGER, RTCC, are not reset."]
    LIMITED = 1,
    #[doc = "2: The CRYOTIMER, DEBUGGER are not reset. RTCC is reset."]
    EXTENDED = 2,
    #[doc = "4: The entire device is reset except some EMU and RMU registers."]
    FULL = 4,
}
impl From<WDOGRMODE_A> for u8 {
    #[inline(always)]
    fn from(variant: WDOGRMODE_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for WDOGRMODE_A {
    type Ux = u8;
}
impl WDOGRMODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<WDOGRMODE_A> {
        match self.bits {
            0 => Some(WDOGRMODE_A::DISABLED),
            1 => Some(WDOGRMODE_A::LIMITED),
            2 => Some(WDOGRMODE_A::EXTENDED),
            4 => Some(WDOGRMODE_A::FULL),
            _ => None,
        }
    }
    #[doc = "Reset request is blocked. This disable bit is redundant with enable/disable bit in WDOG"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == WDOGRMODE_A::DISABLED
    }
    #[doc = "The CRYOTIMER, DEBUGGER, RTCC, are not reset."]
    #[inline(always)]
    pub fn is_limited(&self) -> bool {
        *self == WDOGRMODE_A::LIMITED
    }
    #[doc = "The CRYOTIMER, DEBUGGER are not reset. RTCC is reset."]
    #[inline(always)]
    pub fn is_extended(&self) -> bool {
        *self == WDOGRMODE_A::EXTENDED
    }
    #[doc = "The entire device is reset except some EMU and RMU registers."]
    #[inline(always)]
    pub fn is_full(&self) -> bool {
        *self == WDOGRMODE_A::FULL
    }
}
#[doc = "Field `WDOGRMODE` writer - WDOG Reset Mode"]
pub type WDOGRMODE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O, WDOGRMODE_A>;
impl<'a, REG, const O: u8> WDOGRMODE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Reset request is blocked. This disable bit is redundant with enable/disable bit in WDOG"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(WDOGRMODE_A::DISABLED)
    }
    #[doc = "The CRYOTIMER, DEBUGGER, RTCC, are not reset."]
    #[inline(always)]
    pub fn limited(self) -> &'a mut crate::W<REG> {
        self.variant(WDOGRMODE_A::LIMITED)
    }
    #[doc = "The CRYOTIMER, DEBUGGER are not reset. RTCC is reset."]
    #[inline(always)]
    pub fn extended(self) -> &'a mut crate::W<REG> {
        self.variant(WDOGRMODE_A::EXTENDED)
    }
    #[doc = "The entire device is reset except some EMU and RMU registers."]
    #[inline(always)]
    pub fn full(self) -> &'a mut crate::W<REG> {
        self.variant(WDOGRMODE_A::FULL)
    }
}
#[doc = "Field `LOCKUPRMODE` reader - Core LOCKUP Reset Mode"]
pub type LOCKUPRMODE_R = crate::FieldReader<LOCKUPRMODE_A>;
#[doc = "Core LOCKUP Reset Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LOCKUPRMODE_A {
    #[doc = "0: Reset request is blocked."]
    DISABLED = 0,
    #[doc = "1: The CRYOTIMER, DEBUGGER, RTCC, are not reset."]
    LIMITED = 1,
    #[doc = "2: The CRYOTIMER, DEBUGGER are not reset. RTCC is reset."]
    EXTENDED = 2,
    #[doc = "4: The entire device is reset except some EMU and RMU registers."]
    FULL = 4,
}
impl From<LOCKUPRMODE_A> for u8 {
    #[inline(always)]
    fn from(variant: LOCKUPRMODE_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for LOCKUPRMODE_A {
    type Ux = u8;
}
impl LOCKUPRMODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<LOCKUPRMODE_A> {
        match self.bits {
            0 => Some(LOCKUPRMODE_A::DISABLED),
            1 => Some(LOCKUPRMODE_A::LIMITED),
            2 => Some(LOCKUPRMODE_A::EXTENDED),
            4 => Some(LOCKUPRMODE_A::FULL),
            _ => None,
        }
    }
    #[doc = "Reset request is blocked."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == LOCKUPRMODE_A::DISABLED
    }
    #[doc = "The CRYOTIMER, DEBUGGER, RTCC, are not reset."]
    #[inline(always)]
    pub fn is_limited(&self) -> bool {
        *self == LOCKUPRMODE_A::LIMITED
    }
    #[doc = "The CRYOTIMER, DEBUGGER are not reset. RTCC is reset."]
    #[inline(always)]
    pub fn is_extended(&self) -> bool {
        *self == LOCKUPRMODE_A::EXTENDED
    }
    #[doc = "The entire device is reset except some EMU and RMU registers."]
    #[inline(always)]
    pub fn is_full(&self) -> bool {
        *self == LOCKUPRMODE_A::FULL
    }
}
#[doc = "Field `LOCKUPRMODE` writer - Core LOCKUP Reset Mode"]
pub type LOCKUPRMODE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O, LOCKUPRMODE_A>;
impl<'a, REG, const O: u8> LOCKUPRMODE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Reset request is blocked."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(LOCKUPRMODE_A::DISABLED)
    }
    #[doc = "The CRYOTIMER, DEBUGGER, RTCC, are not reset."]
    #[inline(always)]
    pub fn limited(self) -> &'a mut crate::W<REG> {
        self.variant(LOCKUPRMODE_A::LIMITED)
    }
    #[doc = "The CRYOTIMER, DEBUGGER are not reset. RTCC is reset."]
    #[inline(always)]
    pub fn extended(self) -> &'a mut crate::W<REG> {
        self.variant(LOCKUPRMODE_A::EXTENDED)
    }
    #[doc = "The entire device is reset except some EMU and RMU registers."]
    #[inline(always)]
    pub fn full(self) -> &'a mut crate::W<REG> {
        self.variant(LOCKUPRMODE_A::FULL)
    }
}
#[doc = "Field `SYSRMODE` reader - Core Sysreset Reset Mode"]
pub type SYSRMODE_R = crate::FieldReader<SYSRMODE_A>;
#[doc = "Core Sysreset Reset Mode\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SYSRMODE_A {
    #[doc = "0: Reset request is blocked."]
    DISABLED = 0,
    #[doc = "1: The CRYOTIMER, DEBUGGER, RTCC, are not reset."]
    LIMITED = 1,
    #[doc = "2: The CRYOTIMER, DEBUGGER are not reset. RTCC is reset."]
    EXTENDED = 2,
    #[doc = "4: The entire device is reset except some EMU and RMU registers."]
    FULL = 4,
}
impl From<SYSRMODE_A> for u8 {
    #[inline(always)]
    fn from(variant: SYSRMODE_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SYSRMODE_A {
    type Ux = u8;
}
impl SYSRMODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SYSRMODE_A> {
        match self.bits {
            0 => Some(SYSRMODE_A::DISABLED),
            1 => Some(SYSRMODE_A::LIMITED),
            2 => Some(SYSRMODE_A::EXTENDED),
            4 => Some(SYSRMODE_A::FULL),
            _ => None,
        }
    }
    #[doc = "Reset request is blocked."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SYSRMODE_A::DISABLED
    }
    #[doc = "The CRYOTIMER, DEBUGGER, RTCC, are not reset."]
    #[inline(always)]
    pub fn is_limited(&self) -> bool {
        *self == SYSRMODE_A::LIMITED
    }
    #[doc = "The CRYOTIMER, DEBUGGER are not reset. RTCC is reset."]
    #[inline(always)]
    pub fn is_extended(&self) -> bool {
        *self == SYSRMODE_A::EXTENDED
    }
    #[doc = "The entire device is reset except some EMU and RMU registers."]
    #[inline(always)]
    pub fn is_full(&self) -> bool {
        *self == SYSRMODE_A::FULL
    }
}
#[doc = "Field `SYSRMODE` writer - Core Sysreset Reset Mode"]
pub type SYSRMODE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O, SYSRMODE_A>;
impl<'a, REG, const O: u8> SYSRMODE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Reset request is blocked."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(SYSRMODE_A::DISABLED)
    }
    #[doc = "The CRYOTIMER, DEBUGGER, RTCC, are not reset."]
    #[inline(always)]
    pub fn limited(self) -> &'a mut crate::W<REG> {
        self.variant(SYSRMODE_A::LIMITED)
    }
    #[doc = "The CRYOTIMER, DEBUGGER are not reset. RTCC is reset."]
    #[inline(always)]
    pub fn extended(self) -> &'a mut crate::W<REG> {
        self.variant(SYSRMODE_A::EXTENDED)
    }
    #[doc = "The entire device is reset except some EMU and RMU registers."]
    #[inline(always)]
    pub fn full(self) -> &'a mut crate::W<REG> {
        self.variant(SYSRMODE_A::FULL)
    }
}
#[doc = "Field `PINRMODE` reader - PIN Reset Mode"]
pub type PINRMODE_R = crate::FieldReader<PINRMODE_A>;
#[doc = "PIN Reset Mode\n\nValue on reset: 4"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PINRMODE_A {
    #[doc = "0: Reset request is blocked."]
    DISABLED = 0,
    #[doc = "1: The CRYOTIMER, DEBUGGER, RTCC, are not reset."]
    LIMITED = 1,
    #[doc = "2: The CRYOTIMER, DEBUGGER are not reset. RTCC is reset."]
    EXTENDED = 2,
    #[doc = "4: The entire device is reset except some EMU and RMU registers."]
    FULL = 4,
}
impl From<PINRMODE_A> for u8 {
    #[inline(always)]
    fn from(variant: PINRMODE_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PINRMODE_A {
    type Ux = u8;
}
impl PINRMODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PINRMODE_A> {
        match self.bits {
            0 => Some(PINRMODE_A::DISABLED),
            1 => Some(PINRMODE_A::LIMITED),
            2 => Some(PINRMODE_A::EXTENDED),
            4 => Some(PINRMODE_A::FULL),
            _ => None,
        }
    }
    #[doc = "Reset request is blocked."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PINRMODE_A::DISABLED
    }
    #[doc = "The CRYOTIMER, DEBUGGER, RTCC, are not reset."]
    #[inline(always)]
    pub fn is_limited(&self) -> bool {
        *self == PINRMODE_A::LIMITED
    }
    #[doc = "The CRYOTIMER, DEBUGGER are not reset. RTCC is reset."]
    #[inline(always)]
    pub fn is_extended(&self) -> bool {
        *self == PINRMODE_A::EXTENDED
    }
    #[doc = "The entire device is reset except some EMU and RMU registers."]
    #[inline(always)]
    pub fn is_full(&self) -> bool {
        *self == PINRMODE_A::FULL
    }
}
#[doc = "Field `PINRMODE` writer - PIN Reset Mode"]
pub type PINRMODE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O, PINRMODE_A>;
impl<'a, REG, const O: u8> PINRMODE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Reset request is blocked."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(PINRMODE_A::DISABLED)
    }
    #[doc = "The CRYOTIMER, DEBUGGER, RTCC, are not reset."]
    #[inline(always)]
    pub fn limited(self) -> &'a mut crate::W<REG> {
        self.variant(PINRMODE_A::LIMITED)
    }
    #[doc = "The CRYOTIMER, DEBUGGER are not reset. RTCC is reset."]
    #[inline(always)]
    pub fn extended(self) -> &'a mut crate::W<REG> {
        self.variant(PINRMODE_A::EXTENDED)
    }
    #[doc = "The entire device is reset except some EMU and RMU registers."]
    #[inline(always)]
    pub fn full(self) -> &'a mut crate::W<REG> {
        self.variant(PINRMODE_A::FULL)
    }
}
#[doc = "Field `RESETSTATE` reader - System Software Reset State"]
pub type RESETSTATE_R = crate::FieldReader;
#[doc = "Field `RESETSTATE` writer - System Software Reset State"]
pub type RESETSTATE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
impl R {
    #[doc = "Bits 0:2 - WDOG Reset Mode"]
    #[inline(always)]
    pub fn wdogrmode(&self) -> WDOGRMODE_R {
        WDOGRMODE_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:6 - Core LOCKUP Reset Mode"]
    #[inline(always)]
    pub fn lockuprmode(&self) -> LOCKUPRMODE_R {
        LOCKUPRMODE_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:10 - Core Sysreset Reset Mode"]
    #[inline(always)]
    pub fn sysrmode(&self) -> SYSRMODE_R {
        SYSRMODE_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 12:14 - PIN Reset Mode"]
    #[inline(always)]
    pub fn pinrmode(&self) -> PINRMODE_R {
        PINRMODE_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 24:25 - System Software Reset State"]
    #[inline(always)]
    pub fn resetstate(&self) -> RESETSTATE_R {
        RESETSTATE_R::new(((self.bits >> 24) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - WDOG Reset Mode"]
    #[inline(always)]
    #[must_use]
    pub fn wdogrmode(&mut self) -> WDOGRMODE_W<CTRL_SPEC, 0> {
        WDOGRMODE_W::new(self)
    }
    #[doc = "Bits 4:6 - Core LOCKUP Reset Mode"]
    #[inline(always)]
    #[must_use]
    pub fn lockuprmode(&mut self) -> LOCKUPRMODE_W<CTRL_SPEC, 4> {
        LOCKUPRMODE_W::new(self)
    }
    #[doc = "Bits 8:10 - Core Sysreset Reset Mode"]
    #[inline(always)]
    #[must_use]
    pub fn sysrmode(&mut self) -> SYSRMODE_W<CTRL_SPEC, 8> {
        SYSRMODE_W::new(self)
    }
    #[doc = "Bits 12:14 - PIN Reset Mode"]
    #[inline(always)]
    #[must_use]
    pub fn pinrmode(&mut self) -> PINRMODE_W<CTRL_SPEC, 12> {
        PINRMODE_W::new(self)
    }
    #[doc = "Bits 24:25 - System Software Reset State"]
    #[inline(always)]
    #[must_use]
    pub fn resetstate(&mut self) -> RESETSTATE_W<CTRL_SPEC, 24> {
        RESETSTATE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTRL_SPEC;
impl crate::RegisterSpec for CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl::R`](R) reader structure"]
impl crate::Readable for CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctrl::W`](W) writer structure"]
impl crate::Writable for CTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTRL to value 0x4204"]
impl crate::Resettable for CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0x4204;
}
