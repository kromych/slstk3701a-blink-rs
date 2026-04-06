#[doc = "Register `CTRL` reader"]
pub type R = crate::R<CtrlSpec>;
#[doc = "Register `CTRL` writer"]
pub type W = crate::W<CtrlSpec>;
#[doc = "Field `SYNCPRSSETEN` reader - Synchronization PRS Set Enable"]
pub type SyncprssetenR = crate::FieldReader;
#[doc = "Field `SYNCPRSSETEN` writer - Synchronization PRS Set Enable"]
pub type SyncprssetenW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SYNCPRSCLREN` reader - Synchronization PRS Clear Enable"]
pub type SyncprsclrenR = crate::FieldReader;
#[doc = "Field `SYNCPRSCLREN` writer - Synchronization PRS Clear Enable"]
pub type SyncprsclrenW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `NUMFIXED` reader - Number of Fixed Priority Channels"]
pub type NumfixedR = crate::FieldReader;
#[doc = "Field `NUMFIXED` writer - Number of Fixed Priority Channels"]
pub type NumfixedW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:7 - Synchronization PRS Set Enable"]
    #[inline(always)]
    pub fn syncprsseten(&self) -> SyncprssetenR {
        SyncprssetenR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Synchronization PRS Clear Enable"]
    #[inline(always)]
    pub fn syncprsclren(&self) -> SyncprsclrenR {
        SyncprsclrenR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 24:28 - Number of Fixed Priority Channels"]
    #[inline(always)]
    pub fn numfixed(&self) -> NumfixedR {
        NumfixedR::new(((self.bits >> 24) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Synchronization PRS Set Enable"]
    #[inline(always)]
    pub fn syncprsseten(&mut self) -> SyncprssetenW<'_, CtrlSpec> {
        SyncprssetenW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Synchronization PRS Clear Enable"]
    #[inline(always)]
    pub fn syncprsclren(&mut self) -> SyncprsclrenW<'_, CtrlSpec> {
        SyncprsclrenW::new(self, 8)
    }
    #[doc = "Bits 24:28 - Number of Fixed Priority Channels"]
    #[inline(always)]
    pub fn numfixed(&mut self) -> NumfixedW<'_, CtrlSpec> {
        NumfixedW::new(self, 24)
    }
}
#[doc = "DMA Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtrlSpec;
impl crate::RegisterSpec for CtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrl::R`](R) reader structure"]
impl crate::Readable for CtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`ctrl::W`](W) writer structure"]
impl crate::Writable for CtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CTRL to value 0x1700_0000"]
impl crate::Resettable for CtrlSpec {
    const RESET_VALUE: u32 = 0x1700_0000;
}
