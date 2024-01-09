#[doc = "Register `CAL` reader"]
pub type R = crate::R<CAL_SPEC>;
#[doc = "Register `CAL` writer"]
pub type W = crate::W<CAL_SPEC>;
#[doc = "Field `SINGLEOFFSET` reader - Single Mode Offset Calibration Value for Differential or Positive Single-ended Mode"]
pub type SINGLEOFFSET_R = crate::FieldReader;
#[doc = "Field `SINGLEOFFSET` writer - Single Mode Offset Calibration Value for Differential or Positive Single-ended Mode"]
pub type SINGLEOFFSET_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SINGLEOFFSETINV` reader - Single Mode Offset Calibration Value for Negative Single-ended Mode"]
pub type SINGLEOFFSETINV_R = crate::FieldReader;
#[doc = "Field `SINGLEOFFSETINV` writer - Single Mode Offset Calibration Value for Negative Single-ended Mode"]
pub type SINGLEOFFSETINV_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SINGLEGAIN` reader - Single Mode Gain Calibration Value"]
pub type SINGLEGAIN_R = crate::FieldReader;
#[doc = "Field `SINGLEGAIN` writer - Single Mode Gain Calibration Value"]
pub type SINGLEGAIN_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `OFFSETINVMODE` reader - Negative Single-ended Offset Calibration is Enabled"]
pub type OFFSETINVMODE_R = crate::BitReader;
#[doc = "Field `OFFSETINVMODE` writer - Negative Single-ended Offset Calibration is Enabled"]
pub type OFFSETINVMODE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCANOFFSET` reader - Scan Mode Offset Calibration Value for Differential or Positive Single-ended Mode"]
pub type SCANOFFSET_R = crate::FieldReader;
#[doc = "Field `SCANOFFSET` writer - Scan Mode Offset Calibration Value for Differential or Positive Single-ended Mode"]
pub type SCANOFFSET_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SCANOFFSETINV` reader - Scan Mode Offset Calibration Value for Negative Single-ended Mode"]
pub type SCANOFFSETINV_R = crate::FieldReader;
#[doc = "Field `SCANOFFSETINV` writer - Scan Mode Offset Calibration Value for Negative Single-ended Mode"]
pub type SCANOFFSETINV_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SCANGAIN` reader - Scan Mode Gain Calibration Value"]
pub type SCANGAIN_R = crate::FieldReader;
#[doc = "Field `SCANGAIN` writer - Scan Mode Gain Calibration Value"]
pub type SCANGAIN_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `CALEN` reader - Calibration Mode is Enabled"]
pub type CALEN_R = crate::BitReader;
#[doc = "Field `CALEN` writer - Calibration Mode is Enabled"]
pub type CALEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - Single Mode Offset Calibration Value for Differential or Positive Single-ended Mode"]
    #[inline(always)]
    pub fn singleoffset(&self) -> SINGLEOFFSET_R {
        SINGLEOFFSET_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Single Mode Offset Calibration Value for Negative Single-ended Mode"]
    #[inline(always)]
    pub fn singleoffsetinv(&self) -> SINGLEOFFSETINV_R {
        SINGLEOFFSETINV_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:14 - Single Mode Gain Calibration Value"]
    #[inline(always)]
    pub fn singlegain(&self) -> SINGLEGAIN_R {
        SINGLEGAIN_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bit 15 - Negative Single-ended Offset Calibration is Enabled"]
    #[inline(always)]
    pub fn offsetinvmode(&self) -> OFFSETINVMODE_R {
        OFFSETINVMODE_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:19 - Scan Mode Offset Calibration Value for Differential or Positive Single-ended Mode"]
    #[inline(always)]
    pub fn scanoffset(&self) -> SCANOFFSET_R {
        SCANOFFSET_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Scan Mode Offset Calibration Value for Negative Single-ended Mode"]
    #[inline(always)]
    pub fn scanoffsetinv(&self) -> SCANOFFSETINV_R {
        SCANOFFSETINV_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:30 - Scan Mode Gain Calibration Value"]
    #[inline(always)]
    pub fn scangain(&self) -> SCANGAIN_R {
        SCANGAIN_R::new(((self.bits >> 24) & 0x7f) as u8)
    }
    #[doc = "Bit 31 - Calibration Mode is Enabled"]
    #[inline(always)]
    pub fn calen(&self) -> CALEN_R {
        CALEN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Single Mode Offset Calibration Value for Differential or Positive Single-ended Mode"]
    #[inline(always)]
    #[must_use]
    pub fn singleoffset(&mut self) -> SINGLEOFFSET_W<CAL_SPEC> {
        SINGLEOFFSET_W::new(self, 0)
    }
    #[doc = "Bits 4:7 - Single Mode Offset Calibration Value for Negative Single-ended Mode"]
    #[inline(always)]
    #[must_use]
    pub fn singleoffsetinv(&mut self) -> SINGLEOFFSETINV_W<CAL_SPEC> {
        SINGLEOFFSETINV_W::new(self, 4)
    }
    #[doc = "Bits 8:14 - Single Mode Gain Calibration Value"]
    #[inline(always)]
    #[must_use]
    pub fn singlegain(&mut self) -> SINGLEGAIN_W<CAL_SPEC> {
        SINGLEGAIN_W::new(self, 8)
    }
    #[doc = "Bit 15 - Negative Single-ended Offset Calibration is Enabled"]
    #[inline(always)]
    #[must_use]
    pub fn offsetinvmode(&mut self) -> OFFSETINVMODE_W<CAL_SPEC> {
        OFFSETINVMODE_W::new(self, 15)
    }
    #[doc = "Bits 16:19 - Scan Mode Offset Calibration Value for Differential or Positive Single-ended Mode"]
    #[inline(always)]
    #[must_use]
    pub fn scanoffset(&mut self) -> SCANOFFSET_W<CAL_SPEC> {
        SCANOFFSET_W::new(self, 16)
    }
    #[doc = "Bits 20:23 - Scan Mode Offset Calibration Value for Negative Single-ended Mode"]
    #[inline(always)]
    #[must_use]
    pub fn scanoffsetinv(&mut self) -> SCANOFFSETINV_W<CAL_SPEC> {
        SCANOFFSETINV_W::new(self, 20)
    }
    #[doc = "Bits 24:30 - Scan Mode Gain Calibration Value"]
    #[inline(always)]
    #[must_use]
    pub fn scangain(&mut self) -> SCANGAIN_W<CAL_SPEC> {
        SCANGAIN_W::new(self, 24)
    }
    #[doc = "Bit 31 - Calibration Mode is Enabled"]
    #[inline(always)]
    #[must_use]
    pub fn calen(&mut self) -> CALEN_W<CAL_SPEC> {
        CALEN_W::new(self, 31)
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
#[doc = "Calibration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cal::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cal::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CAL_SPEC;
impl crate::RegisterSpec for CAL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cal::R`](R) reader structure"]
impl crate::Readable for CAL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cal::W`](W) writer structure"]
impl crate::Writable for CAL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CAL to value 0x4078_4078"]
impl crate::Resettable for CAL_SPEC {
    const RESET_VALUE: u32 = 0x4078_4078;
}
