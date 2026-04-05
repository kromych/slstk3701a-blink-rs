#[doc = "Register `TFTPOLARITY` reader"]
pub type R = crate::R<TftpolaritySpec>;
#[doc = "Register `TFTPOLARITY` writer"]
pub type W = crate::W<TftpolaritySpec>;
#[doc = "Field `CSPOL` reader - TFT Chip Select Polarity"]
pub type CspolR = crate::BitReader;
#[doc = "Field `CSPOL` writer - TFT Chip Select Polarity"]
pub type CspolW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCLKPOL` reader - TFT DCLK Polarity"]
pub type DclkpolR = crate::BitReader;
#[doc = "Field `DCLKPOL` writer - TFT DCLK Polarity"]
pub type DclkpolW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DATAENPOL` reader - TFT DATAEN Polarity"]
pub type DataenpolR = crate::BitReader;
#[doc = "Field `DATAENPOL` writer - TFT DATAEN Polarity"]
pub type DataenpolW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSYNCPOL` reader - Address Latch Polarity"]
pub type HsyncpolR = crate::BitReader;
#[doc = "Field `HSYNCPOL` writer - Address Latch Polarity"]
pub type HsyncpolW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VSYNCPOL` reader - VSYNC Polarity"]
pub type VsyncpolR = crate::BitReader;
#[doc = "Field `VSYNCPOL` writer - VSYNC Polarity"]
pub type VsyncpolW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - TFT Chip Select Polarity"]
    #[inline(always)]
    pub fn cspol(&self) -> CspolR {
        CspolR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TFT DCLK Polarity"]
    #[inline(always)]
    pub fn dclkpol(&self) -> DclkpolR {
        DclkpolR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - TFT DATAEN Polarity"]
    #[inline(always)]
    pub fn dataenpol(&self) -> DataenpolR {
        DataenpolR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Address Latch Polarity"]
    #[inline(always)]
    pub fn hsyncpol(&self) -> HsyncpolR {
        HsyncpolR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - VSYNC Polarity"]
    #[inline(always)]
    pub fn vsyncpol(&self) -> VsyncpolR {
        VsyncpolR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TFT Chip Select Polarity"]
    #[inline(always)]
    pub fn cspol(&mut self) -> CspolW<'_, TftpolaritySpec> {
        CspolW::new(self, 0)
    }
    #[doc = "Bit 1 - TFT DCLK Polarity"]
    #[inline(always)]
    pub fn dclkpol(&mut self) -> DclkpolW<'_, TftpolaritySpec> {
        DclkpolW::new(self, 1)
    }
    #[doc = "Bit 2 - TFT DATAEN Polarity"]
    #[inline(always)]
    pub fn dataenpol(&mut self) -> DataenpolW<'_, TftpolaritySpec> {
        DataenpolW::new(self, 2)
    }
    #[doc = "Bit 3 - Address Latch Polarity"]
    #[inline(always)]
    pub fn hsyncpol(&mut self) -> HsyncpolW<'_, TftpolaritySpec> {
        HsyncpolW::new(self, 3)
    }
    #[doc = "Bit 4 - VSYNC Polarity"]
    #[inline(always)]
    pub fn vsyncpol(&mut self) -> VsyncpolW<'_, TftpolaritySpec> {
        VsyncpolW::new(self, 4)
    }
}
#[doc = "TFT Polarity Register\n\nYou can [`read`](crate::Reg::read) this register and get [`tftpolarity::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tftpolarity::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TftpolaritySpec;
impl crate::RegisterSpec for TftpolaritySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tftpolarity::R`](R) reader structure"]
impl crate::Readable for TftpolaritySpec {}
#[doc = "`write(|w| ..)` method takes [`tftpolarity::W`](W) writer structure"]
impl crate::Writable for TftpolaritySpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TFTPOLARITY to value 0"]
impl crate::Resettable for TftpolaritySpec {}
