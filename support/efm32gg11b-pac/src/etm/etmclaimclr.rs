#[doc = "Register `ETMCLAIMCLR` reader"]
pub type R = crate::R<EtmclaimclrSpec>;
#[doc = "Register `ETMCLAIMCLR` writer"]
pub type W = crate::W<EtmclaimclrSpec>;
#[doc = "Field `CLRTAG` reader - Tag Bits"]
pub type ClrtagR = crate::BitReader;
#[doc = "Field `CLRTAG` writer - Tag Bits"]
pub type ClrtagW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Tag Bits"]
    #[inline(always)]
    pub fn clrtag(&self) -> ClrtagR {
        ClrtagR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Tag Bits"]
    #[inline(always)]
    pub fn clrtag(&mut self) -> ClrtagW<'_, EtmclaimclrSpec> {
        ClrtagW::new(self, 0)
    }
}
#[doc = "ETM Claim Tag Clear Register\n\nYou can [`read`](crate::Reg::read) this register and get [`etmclaimclr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`etmclaimclr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EtmclaimclrSpec;
impl crate::RegisterSpec for EtmclaimclrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`etmclaimclr::R`](R) reader structure"]
impl crate::Readable for EtmclaimclrSpec {}
#[doc = "`write(|w| ..)` method takes [`etmclaimclr::W`](W) writer structure"]
impl crate::Writable for EtmclaimclrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ETMCLAIMCLR to value 0"]
impl crate::Resettable for EtmclaimclrSpec {}
