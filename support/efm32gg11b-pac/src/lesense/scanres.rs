#[doc = "Register `SCANRES` reader"]
pub type R = crate::R<ScanresSpec>;
#[doc = "Register `SCANRES` writer"]
pub type W = crate::W<ScanresSpec>;
#[doc = "Field `SCANRES` reader - Scan Results"]
pub type ScanresR = crate::FieldReader<u16>;
#[doc = "Field `SCANRES` writer - Scan Results"]
pub type ScanresW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `STEPDIR` reader - Direction of Previous Step Detection"]
pub type StepdirR = crate::FieldReader<u16>;
#[doc = "Field `STEPDIR` writer - Direction of Previous Step Detection"]
pub type StepdirW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Scan Results"]
    #[inline(always)]
    pub fn scanres(&self) -> ScanresR {
        ScanresR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Direction of Previous Step Detection"]
    #[inline(always)]
    pub fn stepdir(&self) -> StepdirR {
        StepdirR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Scan Results"]
    #[inline(always)]
    pub fn scanres(&mut self) -> ScanresW<'_, ScanresSpec> {
        ScanresW::new(self, 0)
    }
    #[doc = "Bits 16:31 - Direction of Previous Step Detection"]
    #[inline(always)]
    pub fn stepdir(&mut self) -> StepdirW<'_, ScanresSpec> {
        StepdirW::new(self, 16)
    }
}
#[doc = "Scan Result Register\n\nYou can [`read`](crate::Reg::read) this register and get [`scanres::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scanres::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ScanresSpec;
impl crate::RegisterSpec for ScanresSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`scanres::R`](R) reader structure"]
impl crate::Readable for ScanresSpec {}
#[doc = "`write(|w| ..)` method takes [`scanres::W`](W) writer structure"]
impl crate::Writable for ScanresSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCANRES to value 0"]
impl crate::Resettable for ScanresSpec {}
