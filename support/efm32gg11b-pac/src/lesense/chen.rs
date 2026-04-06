#[doc = "Register `CHEN` reader"]
pub type R = crate::R<ChenSpec>;
#[doc = "Register `CHEN` writer"]
pub type W = crate::W<ChenSpec>;
#[doc = "Field `CHEN` reader - Enable Scan Channel"]
pub type ChenR = crate::FieldReader<u16>;
#[doc = "Field `CHEN` writer - Enable Scan Channel"]
pub type ChenW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Enable Scan Channel"]
    #[inline(always)]
    pub fn chen(&self) -> ChenR {
        ChenR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Enable Scan Channel"]
    #[inline(always)]
    pub fn chen(&mut self) -> ChenW<'_, ChenSpec> {
        ChenW::new(self, 0)
    }
}
#[doc = "Channel Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`chen::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chen::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ChenSpec;
impl crate::RegisterSpec for ChenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`chen::R`](R) reader structure"]
impl crate::Readable for ChenSpec {}
#[doc = "`write(|w| ..)` method takes [`chen::W`](W) writer structure"]
impl crate::Writable for ChenSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CHEN to value 0"]
impl crate::Resettable for ChenSpec {}
