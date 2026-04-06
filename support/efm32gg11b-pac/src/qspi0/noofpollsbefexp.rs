#[doc = "Register `NOOFPOLLSBEFEXP` reader"]
pub type R = crate::R<NoofpollsbefexpSpec>;
#[doc = "Register `NOOFPOLLSBEFEXP` writer"]
pub type W = crate::W<NoofpollsbefexpSpec>;
#[doc = "Field `NOOFPOLLSBEFEXP` reader - Number of Polls Cycles Before Expiration"]
pub type NoofpollsbefexpR = crate::FieldReader<u32>;
#[doc = "Field `NOOFPOLLSBEFEXP` writer - Number of Polls Cycles Before Expiration"]
pub type NoofpollsbefexpW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Number of Polls Cycles Before Expiration"]
    #[inline(always)]
    pub fn noofpollsbefexp(&self) -> NoofpollsbefexpR {
        NoofpollsbefexpR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Number of Polls Cycles Before Expiration"]
    #[inline(always)]
    pub fn noofpollsbefexp(&mut self) -> NoofpollsbefexpW<'_, NoofpollsbefexpSpec> {
        NoofpollsbefexpW::new(self, 0)
    }
}
#[doc = "Polling Expiration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`noofpollsbefexp::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`noofpollsbefexp::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NoofpollsbefexpSpec;
impl crate::RegisterSpec for NoofpollsbefexpSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`noofpollsbefexp::R`](R) reader structure"]
impl crate::Readable for NoofpollsbefexpSpec {}
#[doc = "`write(|w| ..)` method takes [`noofpollsbefexp::W`](W) writer structure"]
impl crate::Writable for NoofpollsbefexpSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets NOOFPOLLSBEFEXP to value 0xffff_ffff"]
impl crate::Resettable for NoofpollsbefexpSpec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
