#[doc = "Register `BOOTTOCTRL` reader"]
pub type R = crate::R<BoottoctrlSpec>;
#[doc = "Register `BOOTTOCTRL` writer"]
pub type W = crate::W<BoottoctrlSpec>;
#[doc = "Field `BOOTDATTOCNT` reader - Boot Data Timeout Counter Value"]
pub type BootdattocntR = crate::FieldReader<u32>;
#[doc = "Field `BOOTDATTOCNT` writer - Boot Data Timeout Counter Value"]
pub type BootdattocntW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Boot Data Timeout Counter Value"]
    #[inline(always)]
    pub fn bootdattocnt(&self) -> BootdattocntR {
        BootdattocntR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Boot Data Timeout Counter Value"]
    #[inline(always)]
    pub fn bootdattocnt(&mut self) -> BootdattocntW<'_, BoottoctrlSpec> {
        BootdattocntW::new(self, 0)
    }
}
#[doc = "Boot Timeout Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`boottoctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`boottoctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BoottoctrlSpec;
impl crate::RegisterSpec for BoottoctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`boottoctrl::R`](R) reader structure"]
impl crate::Readable for BoottoctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`boottoctrl::W`](W) writer structure"]
impl crate::Writable for BoottoctrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BOOTTOCTRL to value 0"]
impl crate::Resettable for BoottoctrlSpec {}
