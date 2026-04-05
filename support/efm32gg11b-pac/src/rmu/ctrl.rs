#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CtrlSpec>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CtrlSpec>;
#[doc = "WDOG Reset Mode\n\nValue on reset: 4"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Wdogrmode {
    #[doc = "0: Reset request is blocked. This disable bit is redundant with enable/disable bit in WDOG"]
    Disabled = 0,
    #[doc = "1: The CRYOTIMER, DEBUGGER, RTCC, are not reset."]
    Limited = 1,
    #[doc = "2: The CRYOTIMER, DEBUGGER are not reset. RTCC is reset."]
    Extended = 2,
    #[doc = "4: The entire device is reset except some EMU and RMU registers."]
    Full = 4,
}
impl From<Wdogrmode> for u8 {
    #[inline(always)]
    fn from(variant: Wdogrmode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Wdogrmode {
    type Ux = u8;
}
impl crate::IsEnum for Wdogrmode {}
#[doc = "Field `WDOGRMODE` reader - WDOG Reset Mode"]
pub type WdogrmodeR = crate::FieldReader<Wdogrmode>;
impl WdogrmodeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Wdogrmode> {
        match self.bits {
            0 => Some(Wdogrmode::Disabled),
            1 => Some(Wdogrmode::Limited),
            2 => Some(Wdogrmode::Extended),
            4 => Some(Wdogrmode::Full),
            _ => None,
        }
    }
    #[doc = "Reset request is blocked. This disable bit is redundant with enable/disable bit in WDOG"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Wdogrmode::Disabled
    }
    #[doc = "The CRYOTIMER, DEBUGGER, RTCC, are not reset."]
    #[inline(always)]
    pub fn is_limited(&self) -> bool {
        *self == Wdogrmode::Limited
    }
    #[doc = "The CRYOTIMER, DEBUGGER are not reset. RTCC is reset."]
    #[inline(always)]
    pub fn is_extended(&self) -> bool {
        *self == Wdogrmode::Extended
    }
    #[doc = "The entire device is reset except some EMU and RMU registers."]
    #[inline(always)]
    pub fn is_full(&self) -> bool {
        *self == Wdogrmode::Full
    }
}
#[doc = "Field `WDOGRMODE` writer - WDOG Reset Mode"]
pub type WdogrmodeW<'a, REG> = crate::FieldWriter<'a, REG, 3, Wdogrmode>;
impl<'a, REG> WdogrmodeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Reset request is blocked. This disable bit is redundant with enable/disable bit in WDOG"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Wdogrmode::Disabled)
    }
    #[doc = "The CRYOTIMER, DEBUGGER, RTCC, are not reset."]
    #[inline(always)]
    pub fn limited(self) -> &'a mut crate::W<REG> {
        self.variant(Wdogrmode::Limited)
    }
    #[doc = "The CRYOTIMER, DEBUGGER are not reset. RTCC is reset."]
    #[inline(always)]
    pub fn extended(self) -> &'a mut crate::W<REG> {
        self.variant(Wdogrmode::Extended)
    }
    #[doc = "The entire device is reset except some EMU and RMU registers."]
    #[inline(always)]
    pub fn full(self) -> &'a mut crate::W<REG> {
        self.variant(Wdogrmode::Full)
    }
}
#[doc = "Core LOCKUP Reset Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Lockuprmode {
    #[doc = "0: Reset request is blocked."]
    Disabled = 0,
    #[doc = "1: The CRYOTIMER, DEBUGGER, RTCC, are not reset."]
    Limited = 1,
    #[doc = "2: The CRYOTIMER, DEBUGGER are not reset. RTCC is reset."]
    Extended = 2,
    #[doc = "4: The entire device is reset except some EMU and RMU registers."]
    Full = 4,
}
impl From<Lockuprmode> for u8 {
    #[inline(always)]
    fn from(variant: Lockuprmode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Lockuprmode {
    type Ux = u8;
}
impl crate::IsEnum for Lockuprmode {}
#[doc = "Field `LOCKUPRMODE` reader - Core LOCKUP Reset Mode"]
pub type LockuprmodeR = crate::FieldReader<Lockuprmode>;
impl LockuprmodeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Lockuprmode> {
        match self.bits {
            0 => Some(Lockuprmode::Disabled),
            1 => Some(Lockuprmode::Limited),
            2 => Some(Lockuprmode::Extended),
            4 => Some(Lockuprmode::Full),
            _ => None,
        }
    }
    #[doc = "Reset request is blocked."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Lockuprmode::Disabled
    }
    #[doc = "The CRYOTIMER, DEBUGGER, RTCC, are not reset."]
    #[inline(always)]
    pub fn is_limited(&self) -> bool {
        *self == Lockuprmode::Limited
    }
    #[doc = "The CRYOTIMER, DEBUGGER are not reset. RTCC is reset."]
    #[inline(always)]
    pub fn is_extended(&self) -> bool {
        *self == Lockuprmode::Extended
    }
    #[doc = "The entire device is reset except some EMU and RMU registers."]
    #[inline(always)]
    pub fn is_full(&self) -> bool {
        *self == Lockuprmode::Full
    }
}
#[doc = "Field `LOCKUPRMODE` writer - Core LOCKUP Reset Mode"]
pub type LockuprmodeW<'a, REG> = crate::FieldWriter<'a, REG, 3, Lockuprmode>;
impl<'a, REG> LockuprmodeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Reset request is blocked."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Lockuprmode::Disabled)
    }
    #[doc = "The CRYOTIMER, DEBUGGER, RTCC, are not reset."]
    #[inline(always)]
    pub fn limited(self) -> &'a mut crate::W<REG> {
        self.variant(Lockuprmode::Limited)
    }
    #[doc = "The CRYOTIMER, DEBUGGER are not reset. RTCC is reset."]
    #[inline(always)]
    pub fn extended(self) -> &'a mut crate::W<REG> {
        self.variant(Lockuprmode::Extended)
    }
    #[doc = "The entire device is reset except some EMU and RMU registers."]
    #[inline(always)]
    pub fn full(self) -> &'a mut crate::W<REG> {
        self.variant(Lockuprmode::Full)
    }
}
#[doc = "Core Sysreset Reset Mode\n\nValue on reset: 2"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Sysrmode {
    #[doc = "0: Reset request is blocked."]
    Disabled = 0,
    #[doc = "1: The CRYOTIMER, DEBUGGER, RTCC, are not reset."]
    Limited = 1,
    #[doc = "2: The CRYOTIMER, DEBUGGER are not reset. RTCC is reset."]
    Extended = 2,
    #[doc = "4: The entire device is reset except some EMU and RMU registers."]
    Full = 4,
}
impl From<Sysrmode> for u8 {
    #[inline(always)]
    fn from(variant: Sysrmode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Sysrmode {
    type Ux = u8;
}
impl crate::IsEnum for Sysrmode {}
#[doc = "Field `SYSRMODE` reader - Core Sysreset Reset Mode"]
pub type SysrmodeR = crate::FieldReader<Sysrmode>;
impl SysrmodeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Sysrmode> {
        match self.bits {
            0 => Some(Sysrmode::Disabled),
            1 => Some(Sysrmode::Limited),
            2 => Some(Sysrmode::Extended),
            4 => Some(Sysrmode::Full),
            _ => None,
        }
    }
    #[doc = "Reset request is blocked."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Sysrmode::Disabled
    }
    #[doc = "The CRYOTIMER, DEBUGGER, RTCC, are not reset."]
    #[inline(always)]
    pub fn is_limited(&self) -> bool {
        *self == Sysrmode::Limited
    }
    #[doc = "The CRYOTIMER, DEBUGGER are not reset. RTCC is reset."]
    #[inline(always)]
    pub fn is_extended(&self) -> bool {
        *self == Sysrmode::Extended
    }
    #[doc = "The entire device is reset except some EMU and RMU registers."]
    #[inline(always)]
    pub fn is_full(&self) -> bool {
        *self == Sysrmode::Full
    }
}
#[doc = "Field `SYSRMODE` writer - Core Sysreset Reset Mode"]
pub type SysrmodeW<'a, REG> = crate::FieldWriter<'a, REG, 3, Sysrmode>;
impl<'a, REG> SysrmodeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Reset request is blocked."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Sysrmode::Disabled)
    }
    #[doc = "The CRYOTIMER, DEBUGGER, RTCC, are not reset."]
    #[inline(always)]
    pub fn limited(self) -> &'a mut crate::W<REG> {
        self.variant(Sysrmode::Limited)
    }
    #[doc = "The CRYOTIMER, DEBUGGER are not reset. RTCC is reset."]
    #[inline(always)]
    pub fn extended(self) -> &'a mut crate::W<REG> {
        self.variant(Sysrmode::Extended)
    }
    #[doc = "The entire device is reset except some EMU and RMU registers."]
    #[inline(always)]
    pub fn full(self) -> &'a mut crate::W<REG> {
        self.variant(Sysrmode::Full)
    }
}
#[doc = "PIN Reset Mode\n\nValue on reset: 4"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pinrmode {
    #[doc = "0: Reset request is blocked."]
    Disabled = 0,
    #[doc = "1: The CRYOTIMER, DEBUGGER, RTCC, are not reset."]
    Limited = 1,
    #[doc = "2: The CRYOTIMER, DEBUGGER are not reset. RTCC is reset."]
    Extended = 2,
    #[doc = "4: The entire device is reset except some EMU and RMU registers."]
    Full = 4,
}
impl From<Pinrmode> for u8 {
    #[inline(always)]
    fn from(variant: Pinrmode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pinrmode {
    type Ux = u8;
}
impl crate::IsEnum for Pinrmode {}
#[doc = "Field `PINRMODE` reader - PIN Reset Mode"]
pub type PinrmodeR = crate::FieldReader<Pinrmode>;
impl PinrmodeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Pinrmode> {
        match self.bits {
            0 => Some(Pinrmode::Disabled),
            1 => Some(Pinrmode::Limited),
            2 => Some(Pinrmode::Extended),
            4 => Some(Pinrmode::Full),
            _ => None,
        }
    }
    #[doc = "Reset request is blocked."]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Pinrmode::Disabled
    }
    #[doc = "The CRYOTIMER, DEBUGGER, RTCC, are not reset."]
    #[inline(always)]
    pub fn is_limited(&self) -> bool {
        *self == Pinrmode::Limited
    }
    #[doc = "The CRYOTIMER, DEBUGGER are not reset. RTCC is reset."]
    #[inline(always)]
    pub fn is_extended(&self) -> bool {
        *self == Pinrmode::Extended
    }
    #[doc = "The entire device is reset except some EMU and RMU registers."]
    #[inline(always)]
    pub fn is_full(&self) -> bool {
        *self == Pinrmode::Full
    }
}
#[doc = "Field `PINRMODE` writer - PIN Reset Mode"]
pub type PinrmodeW<'a, REG> = crate::FieldWriter<'a, REG, 3, Pinrmode>;
impl<'a, REG> PinrmodeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Reset request is blocked."]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Pinrmode::Disabled)
    }
    #[doc = "The CRYOTIMER, DEBUGGER, RTCC, are not reset."]
    #[inline(always)]
    pub fn limited(self) -> &'a mut crate::W<REG> {
        self.variant(Pinrmode::Limited)
    }
    #[doc = "The CRYOTIMER, DEBUGGER are not reset. RTCC is reset."]
    #[inline(always)]
    pub fn extended(self) -> &'a mut crate::W<REG> {
        self.variant(Pinrmode::Extended)
    }
    #[doc = "The entire device is reset except some EMU and RMU registers."]
    #[inline(always)]
    pub fn full(self) -> &'a mut crate::W<REG> {
        self.variant(Pinrmode::Full)
    }
}
#[doc = "Field `RESETSTATE` reader - System Software Reset State"]
pub type ResetstateR = crate::FieldReader;
#[doc = "Field `RESETSTATE` writer - System Software Reset State"]
pub type ResetstateW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:2 - WDOG Reset Mode"]
    #[inline(always)]
    pub fn wdogrmode(&self) -> WdogrmodeR {
        WdogrmodeR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:6 - Core LOCKUP Reset Mode"]
    #[inline(always)]
    pub fn lockuprmode(&self) -> LockuprmodeR {
        LockuprmodeR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:10 - Core Sysreset Reset Mode"]
    #[inline(always)]
    pub fn sysrmode(&self) -> SysrmodeR {
        SysrmodeR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 12:14 - PIN Reset Mode"]
    #[inline(always)]
    pub fn pinrmode(&self) -> PinrmodeR {
        PinrmodeR::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 24:25 - System Software Reset State"]
    #[inline(always)]
    pub fn resetstate(&self) -> ResetstateR {
        ResetstateR::new(((self.bits >> 24) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - WDOG Reset Mode"]
    #[inline(always)]
    pub fn wdogrmode(&mut self) -> WdogrmodeW<'_, CtrlSpec> {
        WdogrmodeW::new(self, 0)
    }
    #[doc = "Bits 4:6 - Core LOCKUP Reset Mode"]
    #[inline(always)]
    pub fn lockuprmode(&mut self) -> LockuprmodeW<'_, CtrlSpec> {
        LockuprmodeW::new(self, 4)
    }
    #[doc = "Bits 8:10 - Core Sysreset Reset Mode"]
    #[inline(always)]
    pub fn sysrmode(&mut self) -> SysrmodeW<'_, CtrlSpec> {
        SysrmodeW::new(self, 8)
    }
    #[doc = "Bits 12:14 - PIN Reset Mode"]
    #[inline(always)]
    pub fn pinrmode(&mut self) -> PinrmodeW<'_, CtrlSpec> {
        PinrmodeW::new(self, 12)
    }
    #[doc = "Bits 24:25 - System Software Reset State"]
    #[inline(always)]
    pub fn resetstate(&mut self) -> ResetstateW<'_, CtrlSpec> {
        ResetstateW::new(self, 24)
    }
}
#[doc = "Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtrlSpec;
impl crate::RegisterSpec for CtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl::R`](R) reader structure"]
impl crate::Readable for CtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`ctrl::W`](W) writer structure"]
impl crate::Writable for CtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTRL to value 0x4204"]
impl crate::Resettable for CtrlSpec {
    const RESET_VALUE: u32 = 0x4204;
}
