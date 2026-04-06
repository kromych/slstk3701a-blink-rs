#[doc = "Register `BUF10_DATA` reader"]
pub type R = crate::R<Buf10DataSpec>;
#[doc = "Register `BUF10_DATA` writer"]
pub type W = crate::W<Buf10DataSpec>;
#[doc = "Field `DATA` reader - Scan Result Buffer"]
pub type DataR = crate::FieldReader<u16>;
#[doc = "Field `DATA` writer - Scan Result Buffer"]
pub type DataW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `DATASRC` reader - Result Data Source"]
pub type DatasrcR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:15 - Scan Result Buffer"]
    #[inline(always)]
    pub fn data(&self) -> DataR {
        DataR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:19 - Result Data Source"]
    #[inline(always)]
    pub fn datasrc(&self) -> DatasrcR {
        DatasrcR::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:15 - Scan Result Buffer"]
    #[inline(always)]
    pub fn data(&mut self) -> DataW<'_, Buf10DataSpec> {
        DataW::new(self, 0)
    }
}
#[doc = "Scan Results\n\nYou can [`read`](crate::Reg::read) this register and get [`buf10_data::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`buf10_data::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Buf10DataSpec;
impl crate::RegisterSpec for Buf10DataSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`buf10_data::R`](R) reader structure"]
impl crate::Readable for Buf10DataSpec {}
#[doc = "`write(|w| ..)` method takes [`buf10_data::W`](W) writer structure"]
impl crate::Writable for Buf10DataSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BUF10_DATA to value 0"]
impl crate::Resettable for Buf10DataSpec {}
