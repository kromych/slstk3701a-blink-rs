#[doc = "Register `PWRCTRL` reader"]
pub type R = crate::R<PwrctrlSpec>;
#[doc = "Register `PWRCTRL` writer"]
pub type W = crate::W<PwrctrlSpec>;
#[doc = "Field `ANASW` reader - Analog Switch Selection"]
pub type AnaswR = crate::BitReader;
#[doc = "Field `ANASW` writer - Analog Switch Selection"]
pub type AnaswW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REGPWRSEL` reader - This Field Selects the Input Supply Pin for the Digital LDO"]
pub type RegpwrselR = crate::BitReader;
#[doc = "Field `REGPWRSEL` writer - This Field Selects the Input Supply Pin for the Digital LDO"]
pub type RegpwrselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IMMEDIATEPWRSWITCH` reader - Allows Immediate Switching of ANASW and REGPWRSEL Bitfields"]
pub type ImmediatepwrswitchR = crate::BitReader;
#[doc = "Field `IMMEDIATEPWRSWITCH` writer - Allows Immediate Switching of ANASW and REGPWRSEL Bitfields"]
pub type ImmediatepwrswitchW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 5 - Analog Switch Selection"]
    #[inline(always)]
    pub fn anasw(&self) -> AnaswR {
        AnaswR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 10 - This Field Selects the Input Supply Pin for the Digital LDO"]
    #[inline(always)]
    pub fn regpwrsel(&self) -> RegpwrselR {
        RegpwrselR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 13 - Allows Immediate Switching of ANASW and REGPWRSEL Bitfields"]
    #[inline(always)]
    pub fn immediatepwrswitch(&self) -> ImmediatepwrswitchR {
        ImmediatepwrswitchR::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 5 - Analog Switch Selection"]
    #[inline(always)]
    pub fn anasw(&mut self) -> AnaswW<'_, PwrctrlSpec> {
        AnaswW::new(self, 5)
    }
    #[doc = "Bit 10 - This Field Selects the Input Supply Pin for the Digital LDO"]
    #[inline(always)]
    pub fn regpwrsel(&mut self) -> RegpwrselW<'_, PwrctrlSpec> {
        RegpwrselW::new(self, 10)
    }
    #[doc = "Bit 13 - Allows Immediate Switching of ANASW and REGPWRSEL Bitfields"]
    #[inline(always)]
    pub fn immediatepwrswitch(&mut self) -> ImmediatepwrswitchW<'_, PwrctrlSpec> {
        ImmediatepwrswitchW::new(self, 13)
    }
}
#[doc = "Power Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pwrctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwrctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PwrctrlSpec;
impl crate::RegisterSpec for PwrctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pwrctrl::R`](R) reader structure"]
impl crate::Readable for PwrctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`pwrctrl::W`](W) writer structure"]
impl crate::Writable for PwrctrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PWRCTRL to value 0"]
impl crate::Resettable for PwrctrlSpec {}
