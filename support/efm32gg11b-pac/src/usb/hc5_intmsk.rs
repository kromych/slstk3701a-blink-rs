#[doc = "Register `HC5_INTMSK` reader"]
pub type R = crate::R<Hc5IntmskSpec>;
#[doc = "Register `HC5_INTMSK` writer"]
pub type W = crate::W<Hc5IntmskSpec>;
#[doc = "Field `XFERCOMPLMSK` reader - Transfer Completed Mask"]
pub type XfercomplmskR = crate::BitReader;
#[doc = "Field `XFERCOMPLMSK` writer - Transfer Completed Mask"]
pub type XfercomplmskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHHLTDMSK` reader - Channel Halted Mask"]
pub type ChhltdmskR = crate::BitReader;
#[doc = "Field `CHHLTDMSK` writer - Channel Halted Mask"]
pub type ChhltdmskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AHBERRMSK` reader - AHB Error Mask"]
pub type AhberrmskR = crate::BitReader;
#[doc = "Field `AHBERRMSK` writer - AHB Error Mask"]
pub type AhberrmskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STALLMSK` reader - STALL Response Received Interrupt Mask"]
pub type StallmskR = crate::BitReader;
#[doc = "Field `STALLMSK` writer - STALL Response Received Interrupt Mask"]
pub type StallmskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NAKMSK` reader - NAK Response Received Interrupt Mask"]
pub type NakmskR = crate::BitReader;
#[doc = "Field `NAKMSK` writer - NAK Response Received Interrupt Mask"]
pub type NakmskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACKMSK` reader - ACK Response Received/Transmitted Interrupt Mask"]
pub type AckmskR = crate::BitReader;
#[doc = "Field `ACKMSK` writer - ACK Response Received/Transmitted Interrupt Mask"]
pub type AckmskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XACTERRMSK` reader - Transaction Error Mask"]
pub type XacterrmskR = crate::BitReader;
#[doc = "Field `XACTERRMSK` writer - Transaction Error Mask"]
pub type XacterrmskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BBLERRMSK` reader - Babble Error Mask"]
pub type BblerrmskR = crate::BitReader;
#[doc = "Field `BBLERRMSK` writer - Babble Error Mask"]
pub type BblerrmskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FRMOVRUNMSK` reader - Frame Overrun Mask"]
pub type FrmovrunmskR = crate::BitReader;
#[doc = "Field `FRMOVRUNMSK` writer - Frame Overrun Mask"]
pub type FrmovrunmskW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DATATGLERRMSK` reader - Data Toggle Error Mask"]
pub type DatatglerrmskR = crate::BitReader;
#[doc = "Field `DATATGLERRMSK` writer - Data Toggle Error Mask"]
pub type DatatglerrmskW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Transfer Completed Mask"]
    #[inline(always)]
    pub fn xfercomplmsk(&self) -> XfercomplmskR {
        XfercomplmskR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Channel Halted Mask"]
    #[inline(always)]
    pub fn chhltdmsk(&self) -> ChhltdmskR {
        ChhltdmskR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - AHB Error Mask"]
    #[inline(always)]
    pub fn ahberrmsk(&self) -> AhberrmskR {
        AhberrmskR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - STALL Response Received Interrupt Mask"]
    #[inline(always)]
    pub fn stallmsk(&self) -> StallmskR {
        StallmskR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - NAK Response Received Interrupt Mask"]
    #[inline(always)]
    pub fn nakmsk(&self) -> NakmskR {
        NakmskR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - ACK Response Received/Transmitted Interrupt Mask"]
    #[inline(always)]
    pub fn ackmsk(&self) -> AckmskR {
        AckmskR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - Transaction Error Mask"]
    #[inline(always)]
    pub fn xacterrmsk(&self) -> XacterrmskR {
        XacterrmskR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Babble Error Mask"]
    #[inline(always)]
    pub fn bblerrmsk(&self) -> BblerrmskR {
        BblerrmskR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Frame Overrun Mask"]
    #[inline(always)]
    pub fn frmovrunmsk(&self) -> FrmovrunmskR {
        FrmovrunmskR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Data Toggle Error Mask"]
    #[inline(always)]
    pub fn datatglerrmsk(&self) -> DatatglerrmskR {
        DatatglerrmskR::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Transfer Completed Mask"]
    #[inline(always)]
    pub fn xfercomplmsk(&mut self) -> XfercomplmskW<'_, Hc5IntmskSpec> {
        XfercomplmskW::new(self, 0)
    }
    #[doc = "Bit 1 - Channel Halted Mask"]
    #[inline(always)]
    pub fn chhltdmsk(&mut self) -> ChhltdmskW<'_, Hc5IntmskSpec> {
        ChhltdmskW::new(self, 1)
    }
    #[doc = "Bit 2 - AHB Error Mask"]
    #[inline(always)]
    pub fn ahberrmsk(&mut self) -> AhberrmskW<'_, Hc5IntmskSpec> {
        AhberrmskW::new(self, 2)
    }
    #[doc = "Bit 3 - STALL Response Received Interrupt Mask"]
    #[inline(always)]
    pub fn stallmsk(&mut self) -> StallmskW<'_, Hc5IntmskSpec> {
        StallmskW::new(self, 3)
    }
    #[doc = "Bit 4 - NAK Response Received Interrupt Mask"]
    #[inline(always)]
    pub fn nakmsk(&mut self) -> NakmskW<'_, Hc5IntmskSpec> {
        NakmskW::new(self, 4)
    }
    #[doc = "Bit 5 - ACK Response Received/Transmitted Interrupt Mask"]
    #[inline(always)]
    pub fn ackmsk(&mut self) -> AckmskW<'_, Hc5IntmskSpec> {
        AckmskW::new(self, 5)
    }
    #[doc = "Bit 7 - Transaction Error Mask"]
    #[inline(always)]
    pub fn xacterrmsk(&mut self) -> XacterrmskW<'_, Hc5IntmskSpec> {
        XacterrmskW::new(self, 7)
    }
    #[doc = "Bit 8 - Babble Error Mask"]
    #[inline(always)]
    pub fn bblerrmsk(&mut self) -> BblerrmskW<'_, Hc5IntmskSpec> {
        BblerrmskW::new(self, 8)
    }
    #[doc = "Bit 9 - Frame Overrun Mask"]
    #[inline(always)]
    pub fn frmovrunmsk(&mut self) -> FrmovrunmskW<'_, Hc5IntmskSpec> {
        FrmovrunmskW::new(self, 9)
    }
    #[doc = "Bit 10 - Data Toggle Error Mask"]
    #[inline(always)]
    pub fn datatglerrmsk(&mut self) -> DatatglerrmskW<'_, Hc5IntmskSpec> {
        DatatglerrmskW::new(self, 10)
    }
}
#[doc = "Host Channel x Interrupt Mask Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hc5_intmsk::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hc5_intmsk::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Hc5IntmskSpec;
impl crate::RegisterSpec for Hc5IntmskSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hc5_intmsk::R`](R) reader structure"]
impl crate::Readable for Hc5IntmskSpec {}
#[doc = "`write(|w| ..)` method takes [`hc5_intmsk::W`](W) writer structure"]
impl crate::Writable for Hc5IntmskSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HC5_INTMSK to value 0"]
impl crate::Resettable for Hc5IntmskSpec {}
