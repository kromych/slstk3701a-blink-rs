#[doc = "Register `SCANFIFOCLEAR` writer"]
pub type W = crate::W<ScanfifoclearSpec>;
#[doc = "Field `SCANFIFOCLEAR` writer - Clear Scan FIFO Content"]
pub type ScanfifoclearW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Clear Scan FIFO Content"]
    #[inline(always)]
    pub fn scanfifoclear(&mut self) -> ScanfifoclearW<'_, ScanfifoclearSpec> {
        ScanfifoclearW::new(self, 0)
    }
}
#[doc = "Scan FIFO Clear Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scanfifoclear::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ScanfifoclearSpec;
impl crate::RegisterSpec for ScanfifoclearSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`scanfifoclear::W`](W) writer structure"]
impl crate::Writable for ScanfifoclearSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SCANFIFOCLEAR to value 0"]
impl crate::Resettable for ScanfifoclearSpec {}
