#[doc = "Register `BANKSWITCHLOCK` reader"]
pub type R = crate::R<BANKSWITCHLOCK_SPEC>;
#[doc = "Register `BANKSWITCHLOCK` writer"]
pub type W = crate::W<BANKSWITCHLOCK_SPEC>;
#[doc = "Field `BANKSWITCHLOCKKEY` reader - Bank Switching Lock"]
pub type BANKSWITCHLOCKKEY_R = crate::FieldReader<BANKSWITCHLOCKKEY_A>;
#[doc = "Bank Switching Lock\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum BANKSWITCHLOCKKEY_A {
    #[doc = "0: `0`"]
    UNLOCKED = 0,
    #[doc = "1: `1`"]
    LOCKED = 1,
}
impl From<BANKSWITCHLOCKKEY_A> for u16 {
    #[inline(always)]
    fn from(variant: BANKSWITCHLOCKKEY_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for BANKSWITCHLOCKKEY_A {
    type Ux = u16;
}
impl BANKSWITCHLOCKKEY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<BANKSWITCHLOCKKEY_A> {
        match self.bits {
            0 => Some(BANKSWITCHLOCKKEY_A::UNLOCKED),
            1 => Some(BANKSWITCHLOCKKEY_A::LOCKED),
            _ => None,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_unlocked(&self) -> bool {
        *self == BANKSWITCHLOCKKEY_A::UNLOCKED
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        *self == BANKSWITCHLOCKKEY_A::LOCKED
    }
}
#[doc = "Field `BANKSWITCHLOCKKEY` writer - Bank Switching Lock"]
pub type BANKSWITCHLOCKKEY_W<'a, REG> = crate::FieldWriter<'a, REG, 16, BANKSWITCHLOCKKEY_A>;
impl<'a, REG> BANKSWITCHLOCKKEY_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u16>,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn unlocked(self) -> &'a mut crate::W<REG> {
        self.variant(BANKSWITCHLOCKKEY_A::UNLOCKED)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn locked(self) -> &'a mut crate::W<REG> {
        self.variant(BANKSWITCHLOCKKEY_A::LOCKED)
    }
}
impl R {
    #[doc = "Bits 0:15 - Bank Switching Lock"]
    #[inline(always)]
    pub fn bankswitchlockkey(&self) -> BANKSWITCHLOCKKEY_R {
        BANKSWITCHLOCKKEY_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Bank Switching Lock"]
    #[inline(always)]
    #[must_use]
    pub fn bankswitchlockkey(&mut self) -> BANKSWITCHLOCKKEY_W<BANKSWITCHLOCK_SPEC> {
        BANKSWITCHLOCKKEY_W::new(self, 0)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Bank Switching Lock Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bankswitchlock::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bankswitchlock::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BANKSWITCHLOCK_SPEC;
impl crate::RegisterSpec for BANKSWITCHLOCK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bankswitchlock::R`](R) reader structure"]
impl crate::Readable for BANKSWITCHLOCK_SPEC {}
#[doc = "`write(|w| ..)` method takes [`bankswitchlock::W`](W) writer structure"]
impl crate::Writable for BANKSWITCHLOCK_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BANKSWITCHLOCK to value 0x01"]
impl crate::Resettable for BANKSWITCHLOCK_SPEC {
    const RESET_VALUE: u32 = 0x01;
}
