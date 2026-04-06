#[doc = "Register `PPUPATD2` reader"]
pub type R = crate::R<Ppupatd2Spec>;
#[doc = "Register `PPUPATD2` writer"]
pub type W = crate::W<Ppupatd2Spec>;
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
impl W {}
#[doc = "PPU Privilege Access Type Descriptor 2\n\nYou can [`read`](crate::Reg::read) this register and get [`ppupatd2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ppupatd2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ppupatd2Spec;
impl crate::RegisterSpec for Ppupatd2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ppupatd2::R`](R) reader structure"]
impl crate::Readable for Ppupatd2Spec {}
#[doc = "`write(|w| ..)` method takes [`ppupatd2::W`](W) writer structure"]
impl crate::Writable for Ppupatd2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PPUPATD2 to value 0"]
impl crate::Resettable for Ppupatd2Spec {}
