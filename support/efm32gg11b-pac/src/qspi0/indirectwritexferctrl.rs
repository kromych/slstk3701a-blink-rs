#[doc = "Register `INDIRECTWRITEXFERCTRL` reader"]
pub type R = crate::R<IndirectwritexferctrlSpec>;
#[doc = "Register `INDIRECTWRITEXFERCTRL` writer"]
pub type W = crate::W<IndirectwritexferctrlSpec>;
#[doc = "Field `START` writer - Start Indirect Write"]
pub type StartW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CANCEL` writer - Cancel Indirect Write"]
pub type CancelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WRSTATUS` reader - Indirect Write Status"]
pub type WrstatusR = crate::BitReader;
#[doc = "Field `WRQUEUED` reader - Two Indirect Write Operations Have Been Queued"]
pub type WrqueuedR = crate::BitReader;
#[doc = "Field `INDOPSDONESTATUS` reader - Indirect Completion Status"]
pub type IndopsdonestatusR = crate::BitReader;
#[doc = "Field `INDOPSDONESTATUS` writer - Indirect Completion Status"]
pub type IndopsdonestatusW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NUMINDOPSDONE` reader - Indirect Operations Done"]
pub type NumindopsdoneR = crate::FieldReader;
impl R {
    #[doc = "Bit 2 - Indirect Write Status"]
    #[inline(always)]
    pub fn wrstatus(&self) -> WrstatusR {
        WrstatusR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Two Indirect Write Operations Have Been Queued"]
    #[inline(always)]
    pub fn wrqueued(&self) -> WrqueuedR {
        WrqueuedR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Indirect Completion Status"]
    #[inline(always)]
    pub fn indopsdonestatus(&self) -> IndopsdonestatusR {
        IndopsdonestatusR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - Indirect Operations Done"]
    #[inline(always)]
    pub fn numindopsdone(&self) -> NumindopsdoneR {
        NumindopsdoneR::new(((self.bits >> 6) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Start Indirect Write"]
    #[inline(always)]
    pub fn start(&mut self) -> StartW<'_, IndirectwritexferctrlSpec> {
        StartW::new(self, 0)
    }
    #[doc = "Bit 1 - Cancel Indirect Write"]
    #[inline(always)]
    pub fn cancel(&mut self) -> CancelW<'_, IndirectwritexferctrlSpec> {
        CancelW::new(self, 1)
    }
    #[doc = "Bit 5 - Indirect Completion Status"]
    #[inline(always)]
    pub fn indopsdonestatus(&mut self) -> IndopsdonestatusW<'_, IndirectwritexferctrlSpec> {
        IndopsdonestatusW::new(self, 5)
    }
}
#[doc = "Indirect Write Transfer Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`indirectwritexferctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`indirectwritexferctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IndirectwritexferctrlSpec;
impl crate::RegisterSpec for IndirectwritexferctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`indirectwritexferctrl::R`](R) reader structure"]
impl crate::Readable for IndirectwritexferctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`indirectwritexferctrl::W`](W) writer structure"]
impl crate::Writable for IndirectwritexferctrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INDIRECTWRITEXFERCTRL to value 0"]
impl crate::Resettable for IndirectwritexferctrlSpec {}
