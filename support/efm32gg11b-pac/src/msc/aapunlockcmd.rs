#[doc = "Register `AAPUNLOCKCMD` writer"]
pub type W = crate::W<AapunlockcmdSpec>;
#[doc = "Field `UNLOCKAAP` writer - Software Unlock AAP Command"]
pub type UnlockaapW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Software Unlock AAP Command"]
    #[inline(always)]
    pub fn unlockaap(&mut self) -> UnlockaapW<'_, AapunlockcmdSpec> {
        UnlockaapW::new(self, 0)
    }
}
#[doc = "Software Unlock AAP Command Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`aapunlockcmd::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AapunlockcmdSpec;
impl crate::RegisterSpec for AapunlockcmdSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`aapunlockcmd::W`](W) writer structure"]
impl crate::Writable for AapunlockcmdSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AAPUNLOCKCMD to value 0"]
impl crate::Resettable for AapunlockcmdSpec {}
