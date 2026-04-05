#[doc = "Register `BUF15_DATA` reader"]
pub type R = crate::R<Buf15DataSpec>;
#[doc = "Register `BUF15_DATA` writer"]
pub type W = crate::W<Buf15DataSpec>;
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
    pub fn data(&mut self) -> DataW<'_, Buf15DataSpec> {
        DataW::new(self, 0)
    }
}
#[doc = "Scan Results\n\nYou can [`read`](crate::Reg::read) this register and get [`buf15_data::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`buf15_data::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Buf15DataSpec;
impl crate::RegisterSpec for Buf15DataSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`buf15_data::R`](R) reader structure"]
impl crate::Readable for Buf15DataSpec {}
#[doc = "`write(|w| ..)` method takes [`buf15_data::W`](W) writer structure"]
impl crate::Writable for Buf15DataSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BUF15_DATA to value 0"]
impl crate::Resettable for Buf15DataSpec {}
