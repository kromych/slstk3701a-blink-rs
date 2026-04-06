#[doc = "Register `SPECTYPE4` reader"]
pub type R = crate::R<Spectype4Spec>;
#[doc = "Register `SPECTYPE4` writer"]
pub type W = crate::W<Spectype4Spec>;
#[doc = "Field `MATCH` reader - Type ID match 4"]
pub type MatchR = crate::FieldReader<u16>;
#[doc = "Field `MATCH` writer - Type ID match 4"]
pub type MatchW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `ENBCOPY` reader - Enable copying of type ID match 4 matched frames."]
pub type EnbcopyR = crate::BitReader;
#[doc = "Field `ENBCOPY` writer - Enable copying of type ID match 4 matched frames."]
pub type EnbcopyW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15 - Type ID match 4"]
    #[inline(always)]
    pub fn match_(&self) -> MatchR {
        MatchR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 31 - Enable copying of type ID match 4 matched frames."]
    #[inline(always)]
    pub fn enbcopy(&self) -> EnbcopyR {
        EnbcopyR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - Type ID match 4"]
    #[inline(always)]
    pub fn match_(&mut self) -> MatchW<'_, Spectype4Spec> {
        MatchW::new(self, 0)
    }
    #[doc = "Bit 31 - Enable copying of type ID match 4 matched frames."]
    #[inline(always)]
    pub fn enbcopy(&mut self) -> EnbcopyW<'_, Spectype4Spec> {
        EnbcopyW::new(self, 31)
    }
}
#[doc = "Type ID Match 4\n\nYou can [`read`](crate::Reg::read) this register and get [`spectype4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spectype4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Spectype4Spec;
impl crate::RegisterSpec for Spectype4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spectype4::R`](R) reader structure"]
impl crate::Readable for Spectype4Spec {}
#[doc = "`write(|w| ..)` method takes [`spectype4::W`](W) writer structure"]
impl crate::Writable for Spectype4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPECTYPE4 to value 0"]
impl crate::Resettable for Spectype4Spec {}
