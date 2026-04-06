#[doc = "Register `WRPROTCTRL` reader"]
pub type R = crate::R<WrprotctrlSpec>;
#[doc = "Register `WRPROTCTRL` writer"]
pub type W = crate::W<WrprotctrlSpec>;
#[doc = "Field `INV` reader - Write Protection Inversion Bit"]
pub type InvR = crate::BitReader;
#[doc = "Field `INV` writer - Write Protection Inversion Bit"]
pub type InvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENB` reader - Write Protection Enable Bit"]
pub type EnbR = crate::BitReader;
#[doc = "Field `ENB` writer - Write Protection Enable Bit"]
pub type EnbW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Write Protection Inversion Bit"]
    #[inline(always)]
    pub fn inv(&self) -> InvR {
        InvR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Write Protection Enable Bit"]
    #[inline(always)]
    pub fn enb(&self) -> EnbR {
        EnbR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Write Protection Inversion Bit"]
    #[inline(always)]
    pub fn inv(&mut self) -> InvW<'_, WrprotctrlSpec> {
        InvW::new(self, 0)
    }
    #[doc = "Bit 1 - Write Protection Enable Bit"]
    #[inline(always)]
    pub fn enb(&mut self) -> EnbW<'_, WrprotctrlSpec> {
        EnbW::new(self, 1)
    }
}
#[doc = "Write Protection Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`wrprotctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wrprotctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WrprotctrlSpec;
impl crate::RegisterSpec for WrprotctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wrprotctrl::R`](R) reader structure"]
impl crate::Readable for WrprotctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`wrprotctrl::W`](W) writer structure"]
impl crate::Writable for WrprotctrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets WRPROTCTRL to value 0"]
impl crate::Resettable for WrprotctrlSpec {}
