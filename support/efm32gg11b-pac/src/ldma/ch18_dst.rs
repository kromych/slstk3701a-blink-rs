#[doc = "Register `CH18_DST` reader"]
pub type R = crate::R<CH18_DST_SPEC>;
#[doc = "Register `CH18_DST` writer"]
pub type W = crate::W<CH18_DST_SPEC>;
#[doc = "Field `DSTADDR` reader - Destination Data Address"]
pub type DSTADDR_R = crate::FieldReader<u32>;
#[doc = "Field `DSTADDR` writer - Destination Data Address"]
pub type DSTADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Destination Data Address"]
    #[inline(always)]
    pub fn dstaddr(&self) -> DSTADDR_R {
        DSTADDR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Destination Data Address"]
    #[inline(always)]
    #[must_use]
    pub fn dstaddr(&mut self) -> DSTADDR_W<CH18_DST_SPEC> {
        DSTADDR_W::new(self, 0)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Channel Descriptor Destination Data Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch18_dst::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch18_dst::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CH18_DST_SPEC;
impl crate::RegisterSpec for CH18_DST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch18_dst::R`](R) reader structure"]
impl crate::Readable for CH18_DST_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ch18_dst::W`](W) writer structure"]
impl crate::Writable for CH18_DST_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CH18_DST to value 0"]
impl crate::Resettable for CH18_DST_SPEC {
    const RESET_VALUE: u32 = 0;
}
