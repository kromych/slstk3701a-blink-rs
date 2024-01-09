#[doc = "Register `DBGHALT` reader"]
pub type R = crate::R<DBGHALT_SPEC>;
#[doc = "Register `DBGHALT` writer"]
pub type W = crate::W<DBGHALT_SPEC>;
#[doc = "Field `DBGHALT` reader - DMA Debug Halt"]
pub type DBGHALT_R = crate::FieldReader<u32>;
#[doc = "Field `DBGHALT` writer - DMA Debug Halt"]
pub type DBGHALT_W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:23 - DMA Debug Halt"]
    #[inline(always)]
    pub fn dbghalt(&self) -> DBGHALT_R {
        DBGHALT_R::new(self.bits & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:23 - DMA Debug Halt"]
    #[inline(always)]
    #[must_use]
    pub fn dbghalt(&mut self) -> DBGHALT_W<DBGHALT_SPEC> {
        DBGHALT_W::new(self, 0)
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
#[doc = "DMA Channel Debug Halt Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dbghalt::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dbghalt::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DBGHALT_SPEC;
impl crate::RegisterSpec for DBGHALT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dbghalt::R`](R) reader structure"]
impl crate::Readable for DBGHALT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dbghalt::W`](W) writer structure"]
impl crate::Writable for DBGHALT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DBGHALT to value 0"]
impl crate::Resettable for DBGHALT_SPEC {
    const RESET_VALUE: u32 = 0;
}
