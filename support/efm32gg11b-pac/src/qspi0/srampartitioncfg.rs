#[doc = "Register `SRAMPARTITIONCFG` reader"]
pub type R = crate::R<SrampartitioncfgSpec>;
#[doc = "Register `SRAMPARTITIONCFG` writer"]
pub type W = crate::W<SrampartitioncfgSpec>;
#[doc = "Field `ADDR` reader - Indirect Read Partition Size"]
pub type AddrR = crate::FieldReader;
#[doc = "Field `ADDR` writer - Indirect Read Partition Size"]
pub type AddrW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Indirect Read Partition Size"]
    #[inline(always)]
    pub fn addr(&self) -> AddrR {
        AddrR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Indirect Read Partition Size"]
    #[inline(always)]
    pub fn addr(&mut self) -> AddrW<'_, SrampartitioncfgSpec> {
        AddrW::new(self, 0)
    }
}
#[doc = "SRAM Partition Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`srampartitioncfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`srampartitioncfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SrampartitioncfgSpec;
impl crate::RegisterSpec for SrampartitioncfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`srampartitioncfg::R`](R) reader structure"]
impl crate::Readable for SrampartitioncfgSpec {}
#[doc = "`write(|w| ..)` method takes [`srampartitioncfg::W`](W) writer structure"]
impl crate::Writable for SrampartitioncfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SRAMPARTITIONCFG to value 0x80"]
impl crate::Resettable for SrampartitioncfgSpec {
    const RESET_VALUE: u32 = 0x80;
}
