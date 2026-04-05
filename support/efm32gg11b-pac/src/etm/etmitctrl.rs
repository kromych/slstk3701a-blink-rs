#[doc = "Register `ETMITCTRL` reader"]
pub type R = crate::R<EtmitctrlSpec>;
#[doc = "Register `ETMITCTRL` writer"]
pub type W = crate::W<EtmitctrlSpec>;
#[doc = "Field `ITEN` reader - Integration Mode Enable"]
pub type ItenR = crate::BitReader;
#[doc = "Field `ITEN` writer - Integration Mode Enable"]
pub type ItenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Integration Mode Enable"]
    #[inline(always)]
    pub fn iten(&self) -> ItenR {
        ItenR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Integration Mode Enable"]
    #[inline(always)]
    pub fn iten(&mut self) -> ItenW<'_, EtmitctrlSpec> {
        ItenW::new(self, 0)
    }
}
#[doc = "ETM Integration Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`etmitctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`etmitctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EtmitctrlSpec;
impl crate::RegisterSpec for EtmitctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`etmitctrl::R`](R) reader structure"]
impl crate::Readable for EtmitctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`etmitctrl::W`](W) writer structure"]
impl crate::Writable for EtmitctrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ETMITCTRL to value 0"]
impl crate::Resettable for EtmitctrlSpec {}
