#[doc = "Register `HC6_INT` reader"]
pub type R = crate::R<Hc6IntSpec>;
#[doc = "Register `HC6_INT` writer"]
pub type W = crate::W<Hc6IntSpec>;
#[doc = "Field `XFERCOMPL` reader - Transfer Completed"]
pub type XfercomplR = crate::BitReader;
#[doc = "Field `XFERCOMPL` writer - Transfer Completed"]
pub type XfercomplW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHHLTD` reader - Channel Halted"]
pub type ChhltdR = crate::BitReader;
#[doc = "Field `CHHLTD` writer - Channel Halted"]
pub type ChhltdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AHBERR` reader - AHB Error"]
pub type AhberrR = crate::BitReader;
#[doc = "Field `AHBERR` writer - AHB Error"]
pub type AhberrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STALL` reader - STALL Response Received Interrupt"]
pub type StallR = crate::BitReader;
#[doc = "Field `STALL` writer - STALL Response Received Interrupt"]
pub type StallW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NAK` reader - NAK Response Received Interrupt"]
pub type NakR = crate::BitReader;
#[doc = "Field `NAK` writer - NAK Response Received Interrupt"]
pub type NakW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACK` reader - ACK Response Received/Transmitted Interrupt"]
pub type AckR = crate::BitReader;
#[doc = "Field `ACK` writer - ACK Response Received/Transmitted Interrupt"]
pub type AckW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XACTERR` reader - Transaction Error"]
pub type XacterrR = crate::BitReader;
#[doc = "Field `XACTERR` writer - Transaction Error"]
pub type XacterrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BBLERR` reader - Babble Error"]
pub type BblerrR = crate::BitReader;
#[doc = "Field `BBLERR` writer - Babble Error"]
pub type BblerrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FRMOVRUN` reader - Frame Overrun"]
pub type FrmovrunR = crate::BitReader;
#[doc = "Field `FRMOVRUN` writer - Frame Overrun"]
pub type FrmovrunW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DATATGLERR` reader - Data Toggle Error"]
pub type DatatglerrR = crate::BitReader;
#[doc = "Field `DATATGLERR` writer - Data Toggle Error"]
pub type DatatglerrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Transfer Completed"]
    #[inline(always)]
    pub fn xfercompl(&self) -> XfercomplR {
        XfercomplR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Channel Halted"]
    #[inline(always)]
    pub fn chhltd(&self) -> ChhltdR {
        ChhltdR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - AHB Error"]
    #[inline(always)]
    pub fn ahberr(&self) -> AhberrR {
        AhberrR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - STALL Response Received Interrupt"]
    #[inline(always)]
    pub fn stall(&self) -> StallR {
        StallR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - NAK Response Received Interrupt"]
    #[inline(always)]
    pub fn nak(&self) -> NakR {
        NakR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - ACK Response Received/Transmitted Interrupt"]
    #[inline(always)]
    pub fn ack(&self) -> AckR {
        AckR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - Transaction Error"]
    #[inline(always)]
    pub fn xacterr(&self) -> XacterrR {
        XacterrR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Babble Error"]
    #[inline(always)]
    pub fn bblerr(&self) -> BblerrR {
        BblerrR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Frame Overrun"]
    #[inline(always)]
    pub fn frmovrun(&self) -> FrmovrunR {
        FrmovrunR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Data Toggle Error"]
    #[inline(always)]
    pub fn datatglerr(&self) -> DatatglerrR {
        DatatglerrR::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Transfer Completed"]
    #[inline(always)]
    pub fn xfercompl(&mut self) -> XfercomplW<'_, Hc6IntSpec> {
        XfercomplW::new(self, 0)
    }
    #[doc = "Bit 1 - Channel Halted"]
    #[inline(always)]
    pub fn chhltd(&mut self) -> ChhltdW<'_, Hc6IntSpec> {
        ChhltdW::new(self, 1)
    }
    #[doc = "Bit 2 - AHB Error"]
    #[inline(always)]
    pub fn ahberr(&mut self) -> AhberrW<'_, Hc6IntSpec> {
        AhberrW::new(self, 2)
    }
    #[doc = "Bit 3 - STALL Response Received Interrupt"]
    #[inline(always)]
    pub fn stall(&mut self) -> StallW<'_, Hc6IntSpec> {
        StallW::new(self, 3)
    }
    #[doc = "Bit 4 - NAK Response Received Interrupt"]
    #[inline(always)]
    pub fn nak(&mut self) -> NakW<'_, Hc6IntSpec> {
        NakW::new(self, 4)
    }
    #[doc = "Bit 5 - ACK Response Received/Transmitted Interrupt"]
    #[inline(always)]
    pub fn ack(&mut self) -> AckW<'_, Hc6IntSpec> {
        AckW::new(self, 5)
    }
    #[doc = "Bit 7 - Transaction Error"]
    #[inline(always)]
    pub fn xacterr(&mut self) -> XacterrW<'_, Hc6IntSpec> {
        XacterrW::new(self, 7)
    }
    #[doc = "Bit 8 - Babble Error"]
    #[inline(always)]
    pub fn bblerr(&mut self) -> BblerrW<'_, Hc6IntSpec> {
        BblerrW::new(self, 8)
    }
    #[doc = "Bit 9 - Frame Overrun"]
    #[inline(always)]
    pub fn frmovrun(&mut self) -> FrmovrunW<'_, Hc6IntSpec> {
        FrmovrunW::new(self, 9)
    }
    #[doc = "Bit 10 - Data Toggle Error"]
    #[inline(always)]
    pub fn datatglerr(&mut self) -> DatatglerrW<'_, Hc6IntSpec> {
        DatatglerrW::new(self, 10)
    }
}
#[doc = "Host Channel x Interrupt Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hc6_int::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hc6_int::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Hc6IntSpec;
impl crate::RegisterSpec for Hc6IntSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hc6_int::R`](R) reader structure"]
impl crate::Readable for Hc6IntSpec {}
#[doc = "`write(|w| ..)` method takes [`hc6_int::W`](W) writer structure"]
impl crate::Writable for Hc6IntSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HC6_INT to value 0"]
impl crate::Resettable for Hc6IntSpec {}
