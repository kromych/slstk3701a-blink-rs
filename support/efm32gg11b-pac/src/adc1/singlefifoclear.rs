#[doc = "Register `SINGLEFIFOCLEAR` writer"]
pub type W = crate::W<SinglefifoclearSpec>;
#[doc = "Field `SINGLEFIFOCLEAR` writer - Clear Single FIFO Content"]
pub type SinglefifoclearW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Clear Single FIFO Content"]
    #[inline(always)]
    pub fn singlefifoclear(&mut self) -> SinglefifoclearW<'_, SinglefifoclearSpec> {
        SinglefifoclearW::new(self, 0)
    }
}
#[doc = "Single FIFO Clear Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`singlefifoclear::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SinglefifoclearSpec;
impl crate::RegisterSpec for SinglefifoclearSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`singlefifoclear::W`](W) writer structure"]
impl crate::Writable for SinglefifoclearSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SINGLEFIFOCLEAR to value 0"]
impl crate::Resettable for SinglefifoclearSpec {}
