#[doc = "Register `SPECTYPE3` reader"]
pub type R = crate::R<Spectype3Spec>;
#[doc = "Register `SPECTYPE3` writer"]
pub type W = crate::W<Spectype3Spec>;
#[doc = "Field `MATCH` reader - Type ID match 3"]
pub type MatchR = crate::FieldReader<u16>;
#[doc = "Field `MATCH` writer - Type ID match 3"]
pub type MatchW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `ENBCOPY` reader - Enable copying of type ID match 3 matched frames."]
pub type EnbcopyR = crate::BitReader;
#[doc = "Field `ENBCOPY` writer - Enable copying of type ID match 3 matched frames."]
pub type EnbcopyW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15 - Type ID match 3"]
    #[inline(always)]
    pub fn match_(&self) -> MatchR {
        MatchR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 31 - Enable copying of type ID match 3 matched frames."]
    #[inline(always)]
    pub fn enbcopy(&self) -> EnbcopyR {
        EnbcopyR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - Type ID match 3"]
    #[inline(always)]
    pub fn match_(&mut self) -> MatchW<'_, Spectype3Spec> {
        MatchW::new(self, 0)
    }
    #[doc = "Bit 31 - Enable copying of type ID match 3 matched frames."]
    #[inline(always)]
    pub fn enbcopy(&mut self) -> EnbcopyW<'_, Spectype3Spec> {
        EnbcopyW::new(self, 31)
    }
}
#[doc = "Type ID Match 3\n\nYou can [`read`](crate::Reg::read) this register and get [`spectype3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spectype3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spectype3Spec;
impl crate::RegisterSpec for Spectype3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spectype3::R`](R) reader structure"]
impl crate::Readable for Spectype3Spec {}
#[doc = "`write(|w| ..)` method takes [`spectype3::W`](W) writer structure"]
impl crate::Writable for Spectype3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPECTYPE3 to value 0"]
impl crate::Resettable for Spectype3Spec {}
