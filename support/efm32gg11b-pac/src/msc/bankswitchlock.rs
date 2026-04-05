#[doc = "Register `BANKSWITCHLOCK` reader"]
pub type R = crate::R<BankswitchlockSpec>;
#[doc = "Register `BANKSWITCHLOCK` writer"]
pub type W = crate::W<BankswitchlockSpec>;
#[doc = "Bank Switching Lock\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum Bankswitchlockkey {
    #[doc = "0: `0`"]
    Unlocked = 0,
    #[doc = "1: `1`"]
    Locked = 1,
}
impl From<Bankswitchlockkey> for u16 {
    #[inline(always)]
    fn from(variant: Bankswitchlockkey) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Bankswitchlockkey {
    type Ux = u16;
}
impl crate::IsEnum for Bankswitchlockkey {}
#[doc = "Field `BANKSWITCHLOCKKEY` reader - Bank Switching Lock"]
pub type BankswitchlockkeyR = crate::FieldReader<Bankswitchlockkey>;
impl BankswitchlockkeyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Bankswitchlockkey> {
        match self.bits {
            0 => Some(Bankswitchlockkey::Unlocked),
            1 => Some(Bankswitchlockkey::Locked),
            _ => None,
        }
    }
    #[doc = "`0`"]
    #[inline(always)]
    pub fn is_unlocked(&self) -> bool {
        *self == Bankswitchlockkey::Unlocked
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn is_locked(&self) -> bool {
        *self == Bankswitchlockkey::Locked
    }
}
#[doc = "Field `BANKSWITCHLOCKKEY` writer - Bank Switching Lock"]
pub type BankswitchlockkeyW<'a, REG> = crate::FieldWriter<'a, REG, 16, Bankswitchlockkey>;
impl<'a, REG> BankswitchlockkeyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u16>,
{
    #[doc = "`0`"]
    #[inline(always)]
    pub fn unlocked(self) -> &'a mut crate::W<REG> {
        self.variant(Bankswitchlockkey::Unlocked)
    }
    #[doc = "`1`"]
    #[inline(always)]
    pub fn locked(self) -> &'a mut crate::W<REG> {
        self.variant(Bankswitchlockkey::Locked)
    }
}
impl R {
    #[doc = "Bits 0:15 - Bank Switching Lock"]
    #[inline(always)]
    pub fn bankswitchlockkey(&self) -> BankswitchlockkeyR {
        BankswitchlockkeyR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Bank Switching Lock"]
    #[inline(always)]
    pub fn bankswitchlockkey(&mut self) -> BankswitchlockkeyW<'_, BankswitchlockSpec> {
        BankswitchlockkeyW::new(self, 0)
    }
}
#[doc = "Bank Switching Lock Register\n\nYou can [`read`](crate::Reg::read) this register and get [`bankswitchlock::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bankswitchlock::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BankswitchlockSpec;
impl crate::RegisterSpec for BankswitchlockSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`bankswitchlock::R`](R) reader structure"]
impl crate::Readable for BankswitchlockSpec {}
#[doc = "`write(|w| ..)` method takes [`bankswitchlock::W`](W) writer structure"]
impl crate::Writable for BankswitchlockSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BANKSWITCHLOCK to value 0x01"]
impl crate::Resettable for BankswitchlockSpec {
    const RESET_VALUE: u32 = 0x01;
}
