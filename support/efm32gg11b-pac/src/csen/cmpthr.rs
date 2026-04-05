#[doc = "Register `CMPTHR` reader"]
pub type R = crate::R<CmpthrSpec>;
#[doc = "Register `CMPTHR` writer"]
pub type W = crate::W<CmpthrSpec>;
#[doc = "Field `CMPTHR` reader - Comparator Threshold."]
pub type CmpthrR = crate::FieldReader<u16>;
#[doc = "Field `CMPTHR` writer - Comparator Threshold."]
pub type CmpthrW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Comparator Threshold."]
    #[inline(always)]
    pub fn cmpthr(&self) -> CmpthrR {
        CmpthrR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Comparator Threshold."]
    #[inline(always)]
    pub fn cmpthr(&mut self) -> CmpthrW<'_, CmpthrSpec> {
        CmpthrW::new(self, 0)
    }
}
#[doc = "Comparator Threshold\n\nYou can [`read`](crate::Reg::read) this register and get [`cmpthr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cmpthr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CmpthrSpec;
impl crate::RegisterSpec for CmpthrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cmpthr::R`](R) reader structure"]
impl crate::Readable for CmpthrSpec {}
#[doc = "`write(|w| ..)` method takes [`cmpthr::W`](W) writer structure"]
impl crate::Writable for CmpthrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CMPTHR to value 0"]
impl crate::Resettable for CmpthrSpec {}
