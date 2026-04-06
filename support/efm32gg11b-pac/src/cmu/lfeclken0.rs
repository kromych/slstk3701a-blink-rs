#[doc = "Register `LFECLKEN0` reader"]
pub type R = crate::R<Lfeclken0Spec>;
#[doc = "Register `LFECLKEN0` writer"]
pub type W = crate::W<Lfeclken0Spec>;
#[doc = "Field `RTCC` reader - Real-Time Counter and Calendar Clock Enable"]
pub type RtccR = crate::BitReader;
#[doc = "Field `RTCC` writer - Real-Time Counter and Calendar Clock Enable"]
pub type RtccW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Real-Time Counter and Calendar Clock Enable"]
    #[inline(always)]
    pub fn rtcc(&self) -> RtccR {
        RtccR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Real-Time Counter and Calendar Clock Enable"]
    #[inline(always)]
    pub fn rtcc(&mut self) -> RtccW<'_, Lfeclken0Spec> {
        RtccW::new(self, 0)
    }
}
#[doc = "Low Frequency E Clock Enable Register 0 (Async Reg)\n\nYou can [`read`](crate::Reg::read) this register and get [`lfeclken0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lfeclken0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Lfeclken0Spec;
impl crate::RegisterSpec for Lfeclken0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lfeclken0::R`](R) reader structure"]
impl crate::Readable for Lfeclken0Spec {}
#[doc = "`write(|w| ..)` method takes [`lfeclken0::W`](W) writer structure"]
impl crate::Writable for Lfeclken0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LFECLKEN0 to value 0"]
impl crate::Resettable for Lfeclken0Spec {}
