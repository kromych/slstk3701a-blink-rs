#[doc = "Register `INDIRECTREADXFERCTRL` reader"]
pub type R = crate::R<IndirectreadxferctrlSpec>;
#[doc = "Register `INDIRECTREADXFERCTRL` writer"]
pub type W = crate::W<IndirectreadxferctrlSpec>;
#[doc = "Field `START` writer - Start Indirect Read"]
pub type StartW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CANCEL` writer - Cancel Indirect Read"]
pub type CancelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RDSTATUS` reader - Indirect Read Status"]
pub type RdstatusR = crate::BitReader;
#[doc = "Field `SRAMFULL` reader - SRAM Full"]
pub type SramfullR = crate::BitReader;
#[doc = "Field `SRAMFULL` writer - SRAM Full"]
pub type SramfullW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RDQUEUED` reader - Two Indirect Read Operations Have Been Queued"]
pub type RdqueuedR = crate::BitReader;
#[doc = "Field `INDOPSDONESTATUS` reader - Indirect Completion Status"]
pub type IndopsdonestatusR = crate::BitReader;
#[doc = "Field `INDOPSDONESTATUS` writer - Indirect Completion Status"]
pub type IndopsdonestatusW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NUMINDOPSDONE` reader - Number Indirect Operations Done"]
pub type NumindopsdoneR = crate::FieldReader;
impl R {
    #[doc = "Bit 2 - Indirect Read Status"]
    #[inline(always)]
    pub fn rdstatus(&self) -> RdstatusR {
        RdstatusR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - SRAM Full"]
    #[inline(always)]
    pub fn sramfull(&self) -> SramfullR {
        SramfullR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Two Indirect Read Operations Have Been Queued"]
    #[inline(always)]
    pub fn rdqueued(&self) -> RdqueuedR {
        RdqueuedR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Indirect Completion Status"]
    #[inline(always)]
    pub fn indopsdonestatus(&self) -> IndopsdonestatusR {
        IndopsdonestatusR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - Number Indirect Operations Done"]
    #[inline(always)]
    pub fn numindopsdone(&self) -> NumindopsdoneR {
        NumindopsdoneR::new(((self.bits >> 6) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Start Indirect Read"]
    #[inline(always)]
    pub fn start(&mut self) -> StartW<'_, IndirectreadxferctrlSpec> {
        StartW::new(self, 0)
    }
    #[doc = "Bit 1 - Cancel Indirect Read"]
    #[inline(always)]
    pub fn cancel(&mut self) -> CancelW<'_, IndirectreadxferctrlSpec> {
        CancelW::new(self, 1)
    }
    #[doc = "Bit 3 - SRAM Full"]
    #[inline(always)]
    pub fn sramfull(&mut self) -> SramfullW<'_, IndirectreadxferctrlSpec> {
        SramfullW::new(self, 3)
    }
    #[doc = "Bit 5 - Indirect Completion Status"]
    #[inline(always)]
    pub fn indopsdonestatus(&mut self) -> IndopsdonestatusW<'_, IndirectreadxferctrlSpec> {
        IndopsdonestatusW::new(self, 5)
    }
}
#[doc = "Indirect Read Transfer Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`indirectreadxferctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`indirectreadxferctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IndirectreadxferctrlSpec;
impl crate::RegisterSpec for IndirectreadxferctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`indirectreadxferctrl::R`](R) reader structure"]
impl crate::Readable for IndirectreadxferctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`indirectreadxferctrl::W`](W) writer structure"]
impl crate::Writable for IndirectreadxferctrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INDIRECTREADXFERCTRL to value 0"]
impl crate::Resettable for IndirectreadxferctrlSpec {}
