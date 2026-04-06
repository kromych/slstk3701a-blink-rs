#[doc = "Register `CAPAB2` reader"]
pub type R = crate::R<Capab2Spec>;
#[doc = "Field `SDR50SUP` reader - SDR50 Support"]
pub type Sdr50supR = crate::BitReader;
#[doc = "Field `SDR104SUP` reader - SDR104 Support"]
pub type Sdr104supR = crate::BitReader;
#[doc = "Field `DDR50SUP` reader - DDR50 Support"]
pub type Ddr50supR = crate::BitReader;
#[doc = "Field `DRVTYPASUP` reader - Driver Type a Support"]
pub type DrvtypasupR = crate::BitReader;
#[doc = "Field `DRVTYPCSUP` reader - Driver Type C Support"]
pub type DrvtypcsupR = crate::BitReader;
#[doc = "Field `DRVTYPDSUP` reader - Driver Type D Support"]
pub type DrvtypdsupR = crate::BitReader;
#[doc = "Field `TIMCNTRETUN` reader - Timer Count for Re-Tuning"]
pub type TimcntretunR = crate::FieldReader;
#[doc = "Field `USETUNSDR50` reader - Use Tuning for SDR50"]
pub type Usetunsdr50R = crate::BitReader;
#[doc = "Field `RETUNEMODES` reader - Re-tuning Modes"]
pub type RetunemodesR = crate::FieldReader;
#[doc = "Field `CLOCKKMUL` reader - Clock Multiplier"]
pub type ClockkmulR = crate::FieldReader;
#[doc = "Field `SPIMODE` reader - SPI Mode Support"]
pub type SpimodeR = crate::BitReader;
#[doc = "Field `SPIBLOCKMODE` reader - SPI Block Mode Support"]
pub type SpiblockmodeR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - SDR50 Support"]
    #[inline(always)]
    pub fn sdr50sup(&self) -> Sdr50supR {
        Sdr50supR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SDR104 Support"]
    #[inline(always)]
    pub fn sdr104sup(&self) -> Sdr104supR {
        Sdr104supR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DDR50 Support"]
    #[inline(always)]
    pub fn ddr50sup(&self) -> Ddr50supR {
        Ddr50supR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Driver Type a Support"]
    #[inline(always)]
    pub fn drvtypasup(&self) -> DrvtypasupR {
        DrvtypasupR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Driver Type C Support"]
    #[inline(always)]
    pub fn drvtypcsup(&self) -> DrvtypcsupR {
        DrvtypcsupR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Driver Type D Support"]
    #[inline(always)]
    pub fn drvtypdsup(&self) -> DrvtypdsupR {
        DrvtypdsupR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 8:11 - Timer Count for Re-Tuning"]
    #[inline(always)]
    pub fn timcntretun(&self) -> TimcntretunR {
        TimcntretunR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 13 - Use Tuning for SDR50"]
    #[inline(always)]
    pub fn usetunsdr50(&self) -> Usetunsdr50R {
        Usetunsdr50R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 14:15 - Re-tuning Modes"]
    #[inline(always)]
    pub fn retunemodes(&self) -> RetunemodesR {
        RetunemodesR::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:23 - Clock Multiplier"]
    #[inline(always)]
    pub fn clockkmul(&self) -> ClockkmulR {
        ClockkmulR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 24 - SPI Mode Support"]
    #[inline(always)]
    pub fn spimode(&self) -> SpimodeR {
        SpimodeR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - SPI Block Mode Support"]
    #[inline(always)]
    pub fn spiblockmode(&self) -> SpiblockmodeR {
        SpiblockmodeR::new(((self.bits >> 25) & 1) != 0)
    }
}
#[doc = "Capabilities Register to Hold Bits 63~32\n\nYou can [`read`](crate::Reg::read) this register and get [`capab2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Capab2Spec;
impl crate::RegisterSpec for Capab2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`capab2::R`](R) reader structure"]
impl crate::Readable for Capab2Spec {}
#[doc = "`reset()` method sets CAPAB2 to value 0"]
impl crate::Resettable for Capab2Spec {}
