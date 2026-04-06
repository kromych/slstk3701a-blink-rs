#[doc = "Register `SYSWAKETIME` reader"]
pub type R = crate::R<SyswaketimeSpec>;
#[doc = "Register `SYSWAKETIME` writer"]
pub type W = crate::W<SyswaketimeSpec>;
#[doc = "Field `SYSWAKETIME` reader - Count of 64ns, 320ns or 3200ns intervals before transmission starts after deassertion of tx_lpi_en"]
pub type SyswaketimeR = crate::FieldReader<u16>;
#[doc = "Field `SYSWAKETIME` writer - Count of 64ns, 320ns or 3200ns intervals before transmission starts after deassertion of tx_lpi_en"]
pub type SyswaketimeW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Count of 64ns, 320ns or 3200ns intervals before transmission starts after deassertion of tx_lpi_en"]
    #[inline(always)]
    pub fn syswaketime(&self) -> SyswaketimeR {
        SyswaketimeR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Count of 64ns, 320ns or 3200ns intervals before transmission starts after deassertion of tx_lpi_en"]
    #[inline(always)]
    pub fn syswaketime(&mut self) -> SyswaketimeW<'_, SyswaketimeSpec> {
        SyswaketimeW::new(self, 0)
    }
}
#[doc = "System wake time\n\nYou can [`read`](crate::Reg::read) this register and get [`syswaketime::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`syswaketime::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SyswaketimeSpec;
impl crate::RegisterSpec for SyswaketimeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`syswaketime::R`](R) reader structure"]
impl crate::Readable for SyswaketimeSpec {}
#[doc = "`write(|w| ..)` method takes [`syswaketime::W`](W) writer structure"]
impl crate::Writable for SyswaketimeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SYSWAKETIME to value 0"]
impl crate::Resettable for SyswaketimeSpec {}
