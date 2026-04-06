#[doc = "Register `DCDCLPCTRL` reader"]
pub type R = crate::R<DcdclpctrlSpec>;
#[doc = "Register `DCDCLPCTRL` writer"]
pub type W = crate::W<DcdclpctrlSpec>;
#[doc = "Field `LPCMPHYSSELEM234H` reader - LP Mode Hysteresis Selection for EM23 and EM4H"]
pub type Lpcmphysselem234hR = crate::FieldReader;
#[doc = "Field `LPCMPHYSSELEM234H` writer - LP Mode Hysteresis Selection for EM23 and EM4H"]
pub type Lpcmphysselem234hW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `LPVREFDUTYEN` reader - LP Mode Duty Cycling Enable"]
pub type LpvrefdutyenR = crate::BitReader;
#[doc = "Field `LPVREFDUTYEN` writer - LP Mode Duty Cycling Enable"]
pub type LpvrefdutyenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPBLANK` reader - Reserved for internal use. Do not change."]
pub type LpblankR = crate::FieldReader;
#[doc = "Field `LPBLANK` writer - Reserved for internal use. Do not change."]
pub type LpblankW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 12:15 - LP Mode Hysteresis Selection for EM23 and EM4H"]
    #[inline(always)]
    pub fn lpcmphysselem234h(&self) -> Lpcmphysselem234hR {
        Lpcmphysselem234hR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bit 24 - LP Mode Duty Cycling Enable"]
    #[inline(always)]
    pub fn lpvrefdutyen(&self) -> LpvrefdutyenR {
        LpvrefdutyenR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 25:26 - Reserved for internal use. Do not change."]
    #[inline(always)]
    pub fn lpblank(&self) -> LpblankR {
        LpblankR::new(((self.bits >> 25) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 12:15 - LP Mode Hysteresis Selection for EM23 and EM4H"]
    #[inline(always)]
    pub fn lpcmphysselem234h(&mut self) -> Lpcmphysselem234hW<'_, DcdclpctrlSpec> {
        Lpcmphysselem234hW::new(self, 12)
    }
    #[doc = "Bit 24 - LP Mode Duty Cycling Enable"]
    #[inline(always)]
    pub fn lpvrefdutyen(&mut self) -> LpvrefdutyenW<'_, DcdclpctrlSpec> {
        LpvrefdutyenW::new(self, 24)
    }
    #[doc = "Bits 25:26 - Reserved for internal use. Do not change."]
    #[inline(always)]
    pub fn lpblank(&mut self) -> LpblankW<'_, DcdclpctrlSpec> {
        LpblankW::new(self, 25)
    }
}
#[doc = "DCDC Low Power Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dcdclpctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcdclpctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DcdclpctrlSpec;
impl crate::RegisterSpec for DcdclpctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dcdclpctrl::R`](R) reader structure"]
impl crate::Readable for DcdclpctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`dcdclpctrl::W`](W) writer structure"]
impl crate::Writable for DcdclpctrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DCDCLPCTRL to value 0x0300_0000"]
impl crate::Resettable for DcdclpctrlSpec {
    const RESET_VALUE: u32 = 0x0300_0000;
}
