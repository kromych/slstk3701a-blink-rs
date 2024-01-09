#[doc = "Register `PB_CTRL` reader"]
pub type R = crate::R<PB_CTRL_SPEC>;
#[doc = "Register `PB_CTRL` writer"]
pub type W = crate::W<PB_CTRL_SPEC>;
#[doc = "Field `DRIVESTRENGTH` reader - Drive Strength for Port"]
pub type DRIVESTRENGTH_R = crate::BitReader;
#[doc = "Field `DRIVESTRENGTH` writer - Drive Strength for Port"]
pub type DRIVESTRENGTH_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLEWRATE` reader - Slewrate Limit for Port"]
pub type SLEWRATE_R = crate::FieldReader;
#[doc = "Field `SLEWRATE` writer - Slewrate Limit for Port"]
pub type SLEWRATE_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `DINDIS` reader - Data in Disable"]
pub type DINDIS_R = crate::BitReader;
#[doc = "Field `DINDIS` writer - Data in Disable"]
pub type DINDIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DRIVESTRENGTHALT` reader - Alternate Drive Strength for Port"]
pub type DRIVESTRENGTHALT_R = crate::BitReader;
#[doc = "Field `DRIVESTRENGTHALT` writer - Alternate Drive Strength for Port"]
pub type DRIVESTRENGTHALT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLEWRATEALT` reader - Alternate Slewrate Limit for Port"]
pub type SLEWRATEALT_R = crate::FieldReader;
#[doc = "Field `SLEWRATEALT` writer - Alternate Slewrate Limit for Port"]
pub type SLEWRATEALT_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `DINDISALT` reader - Alternate Data in Disable"]
pub type DINDISALT_R = crate::BitReader;
#[doc = "Field `DINDISALT` writer - Alternate Data in Disable"]
pub type DINDISALT_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Drive Strength for Port"]
    #[inline(always)]
    pub fn drivestrength(&self) -> DRIVESTRENGTH_R {
        DRIVESTRENGTH_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 4:6 - Slewrate Limit for Port"]
    #[inline(always)]
    pub fn slewrate(&self) -> SLEWRATE_R {
        SLEWRATE_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 12 - Data in Disable"]
    #[inline(always)]
    pub fn dindis(&self) -> DINDIS_R {
        DINDIS_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 16 - Alternate Drive Strength for Port"]
    #[inline(always)]
    pub fn drivestrengthalt(&self) -> DRIVESTRENGTHALT_R {
        DRIVESTRENGTHALT_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 20:22 - Alternate Slewrate Limit for Port"]
    #[inline(always)]
    pub fn slewratealt(&self) -> SLEWRATEALT_R {
        SLEWRATEALT_R::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bit 28 - Alternate Data in Disable"]
    #[inline(always)]
    pub fn dindisalt(&self) -> DINDISALT_R {
        DINDISALT_R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Drive Strength for Port"]
    #[inline(always)]
    #[must_use]
    pub fn drivestrength(&mut self) -> DRIVESTRENGTH_W<PB_CTRL_SPEC> {
        DRIVESTRENGTH_W::new(self, 0)
    }
    #[doc = "Bits 4:6 - Slewrate Limit for Port"]
    #[inline(always)]
    #[must_use]
    pub fn slewrate(&mut self) -> SLEWRATE_W<PB_CTRL_SPEC> {
        SLEWRATE_W::new(self, 4)
    }
    #[doc = "Bit 12 - Data in Disable"]
    #[inline(always)]
    #[must_use]
    pub fn dindis(&mut self) -> DINDIS_W<PB_CTRL_SPEC> {
        DINDIS_W::new(self, 12)
    }
    #[doc = "Bit 16 - Alternate Drive Strength for Port"]
    #[inline(always)]
    #[must_use]
    pub fn drivestrengthalt(&mut self) -> DRIVESTRENGTHALT_W<PB_CTRL_SPEC> {
        DRIVESTRENGTHALT_W::new(self, 16)
    }
    #[doc = "Bits 20:22 - Alternate Slewrate Limit for Port"]
    #[inline(always)]
    #[must_use]
    pub fn slewratealt(&mut self) -> SLEWRATEALT_W<PB_CTRL_SPEC> {
        SLEWRATEALT_W::new(self, 20)
    }
    #[doc = "Bit 28 - Alternate Data in Disable"]
    #[inline(always)]
    #[must_use]
    pub fn dindisalt(&mut self) -> DINDISALT_W<PB_CTRL_SPEC> {
        DINDISALT_W::new(self, 28)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Port Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pb_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pb_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PB_CTRL_SPEC;
impl crate::RegisterSpec for PB_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pb_ctrl::R`](R) reader structure"]
impl crate::Readable for PB_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pb_ctrl::W`](W) writer structure"]
impl crate::Writable for PB_CTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PB_CTRL to value 0x0050_0050"]
impl crate::Resettable for PB_CTRL_SPEC {
    const RESET_VALUE: u32 = 0x0050_0050;
}
