#[doc = "Register `LOCK` reader"]
pub type R = crate::R<LockSpec>;
#[doc = "Register `LOCK` writer"]
pub type W = crate::W<LockSpec>;
#[doc = "Timer Lock Key\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum Timerlockkey {
    #[doc = "0: `0`"]
    Unlocked = 0,
    #[doc = "1: `1`"]
    Locked = 1,
}
impl From<Timerlockkey> for u16 {
    #[inline(always)]
    fn from(variant: Timerlockkey) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Timerlockkey {
    type Ux = u16;
}
impl crate::IsEnum for Timerlockkey {}
#[doc = "Field `TIMERLOCKKEY` reader - Timer Lock Key"]
pub type TimerlockkeyR = crate::FieldReader<Timerlockkey>;
impl TimerlockkeyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Timerlockkey> {
        match self.bits {
            0 => Some(Timerlockkey::Unlocked),
            1 => Some(Timerlockkey::Locked),
            _ => None,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_unlocked(&self) -> bool {
        *self == Timerlockkey::Unlocked
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        *self == Timerlockkey::Locked
    }
}
#[doc = "Field `TIMERLOCKKEY` writer - Timer Lock Key"]
pub type TimerlockkeyW<'a, REG> = crate::FieldWriter<'a, REG, 16, Timerlockkey>;
impl<'a, REG> TimerlockkeyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u16>,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn unlocked(self) -> &'a mut crate::W<REG> {
        self.variant(Timerlockkey::Unlocked)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn locked(self) -> &'a mut crate::W<REG> {
        self.variant(Timerlockkey::Locked)
    }
}
impl R {
    #[doc = "Bits 0:15 - Timer Lock Key"]
    #[inline(always)]
    pub fn timerlockkey(&self) -> TimerlockkeyR {
        TimerlockkeyR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Timer Lock Key"]
    #[inline(always)]
    pub fn timerlockkey(&mut self) -> TimerlockkeyW<'_, LockSpec> {
        TimerlockkeyW::new(self, 0)
    }
}
#[doc = "TIMER Configuration Lock Register\n\nYou can [`read`](crate::Reg::read) this register and get [`lock::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lock::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LockSpec;
impl crate::RegisterSpec for LockSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lock::R`](R) reader structure"]
impl crate::Readable for LockSpec {}
#[doc = "`write(|w| ..)` method takes [`lock::W`](W) writer structure"]
impl crate::Writable for LockSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LOCK to value 0"]
impl crate::Resettable for LockSpec {}
