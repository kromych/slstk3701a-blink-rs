#[doc = "Register `PI_CTRL` reader"]
pub type R = crate::R<PiCtrlSpec>;
#[doc = "Register `PI_CTRL` writer"]
pub type W = crate::W<PiCtrlSpec>;
#[doc = "Field `DRIVESTRENGTH` reader - Drive Strength for Port"]
pub type DrivestrengthR = crate::BitReader;
#[doc = "Field `DRIVESTRENGTH` writer - Drive Strength for Port"]
pub type DrivestrengthW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLEWRATE` reader - Slewrate Limit for Port"]
pub type SlewrateR = crate::FieldReader;
#[doc = "Field `SLEWRATE` writer - Slewrate Limit for Port"]
pub type SlewrateW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `DINDIS` reader - Data in Disable"]
pub type DindisR = crate::BitReader;
#[doc = "Field `DINDIS` writer - Data in Disable"]
pub type DindisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DRIVESTRENGTHALT` reader - Alternate Drive Strength for Port"]
pub type DrivestrengthaltR = crate::BitReader;
#[doc = "Field `DRIVESTRENGTHALT` writer - Alternate Drive Strength for Port"]
pub type DrivestrengthaltW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLEWRATEALT` reader - Alternate Slewrate Limit for Port"]
pub type SlewratealtR = crate::FieldReader;
#[doc = "Field `SLEWRATEALT` writer - Alternate Slewrate Limit for Port"]
pub type SlewratealtW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `DINDISALT` reader - Alternate Data in Disable"]
pub type DindisaltR = crate::BitReader;
#[doc = "Field `DINDISALT` writer - Alternate Data in Disable"]
pub type DindisaltW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Drive Strength for Port"]
    #[inline(always)]
    pub fn drivestrength(&self) -> DrivestrengthR {
        DrivestrengthR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 4:6 - Slewrate Limit for Port"]
    #[inline(always)]
    pub fn slewrate(&self) -> SlewrateR {
        SlewrateR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 12 - Data in Disable"]
    #[inline(always)]
    pub fn dindis(&self) -> DindisR {
        DindisR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 16 - Alternate Drive Strength for Port"]
    #[inline(always)]
    pub fn drivestrengthalt(&self) -> DrivestrengthaltR {
        DrivestrengthaltR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 20:22 - Alternate Slewrate Limit for Port"]
    #[inline(always)]
    pub fn slewratealt(&self) -> SlewratealtR {
        SlewratealtR::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bit 28 - Alternate Data in Disable"]
    #[inline(always)]
    pub fn dindisalt(&self) -> DindisaltR {
        DindisaltR::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Drive Strength for Port"]
    #[inline(always)]
    pub fn drivestrength(&mut self) -> DrivestrengthW<'_, PiCtrlSpec> {
        DrivestrengthW::new(self, 0)
    }
    #[doc = "Bits 4:6 - Slewrate Limit for Port"]
    #[inline(always)]
    pub fn slewrate(&mut self) -> SlewrateW<'_, PiCtrlSpec> {
        SlewrateW::new(self, 4)
    }
    #[doc = "Bit 12 - Data in Disable"]
    #[inline(always)]
    pub fn dindis(&mut self) -> DindisW<'_, PiCtrlSpec> {
        DindisW::new(self, 12)
    }
    #[doc = "Bit 16 - Alternate Drive Strength for Port"]
    #[inline(always)]
    pub fn drivestrengthalt(&mut self) -> DrivestrengthaltW<'_, PiCtrlSpec> {
        DrivestrengthaltW::new(self, 16)
    }
    #[doc = "Bits 20:22 - Alternate Slewrate Limit for Port"]
    #[inline(always)]
    pub fn slewratealt(&mut self) -> SlewratealtW<'_, PiCtrlSpec> {
        SlewratealtW::new(self, 20)
    }
    #[doc = "Bit 28 - Alternate Data in Disable"]
    #[inline(always)]
    pub fn dindisalt(&mut self) -> DindisaltW<'_, PiCtrlSpec> {
        DindisaltW::new(self, 28)
    }
}
#[doc = "Port Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pi_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pi_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PiCtrlSpec;
impl crate::RegisterSpec for PiCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pi_ctrl::R`](R) reader structure"]
impl crate::Readable for PiCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`pi_ctrl::W`](W) writer structure"]
impl crate::Writable for PiCtrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PI_CTRL to value 0x0050_0050"]
impl crate::Resettable for PiCtrlSpec {
    const RESET_VALUE: u32 = 0x0050_0050;
}
