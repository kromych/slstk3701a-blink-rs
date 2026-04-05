#[doc = "Register `SDMASYSADDR` reader"]
pub type R = crate::R<SdmasysaddrSpec>;
#[doc = "Register `SDMASYSADDR` writer"]
pub type W = crate::W<SdmasysaddrSpec>;
#[doc = "Field `SDMASYSADDRARG` reader - Physical SYS Memory ADDR Used for DMA Transfers or the Second Argument for the Auto CMD23"]
pub type SdmasysaddrargR = crate::FieldReader<u32>;
#[doc = "Field `SDMASYSADDRARG` writer - Physical SYS Memory ADDR Used for DMA Transfers or the Second Argument for the Auto CMD23"]
pub type SdmasysaddrargW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Physical SYS Memory ADDR Used for DMA Transfers or the Second Argument for the Auto CMD23"]
    #[inline(always)]
    pub fn sdmasysaddrarg(&self) -> SdmasysaddrargR {
        SdmasysaddrargR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Physical SYS Memory ADDR Used for DMA Transfers or the Second Argument for the Auto CMD23"]
    #[inline(always)]
    pub fn sdmasysaddrarg(&mut self) -> SdmasysaddrargW<'_, SdmasysaddrSpec> {
        SdmasysaddrargW::new(self, 0)
    }
}
#[doc = "SDMA System Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sdmasysaddr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdmasysaddr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SdmasysaddrSpec;
impl crate::RegisterSpec for SdmasysaddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sdmasysaddr::R`](R) reader structure"]
impl crate::Readable for SdmasysaddrSpec {}
#[doc = "`write(|w| ..)` method takes [`sdmasysaddr::W`](W) writer structure"]
impl crate::Writable for SdmasysaddrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SDMASYSADDR to value 0"]
impl crate::Resettable for SdmasysaddrSpec {}
