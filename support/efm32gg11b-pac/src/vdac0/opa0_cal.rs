#[doc = "Register `OPA0_CAL` reader"]
pub type R = crate::R<OPA0_CAL_SPEC>;
#[doc = "Register `OPA0_CAL` writer"]
pub type W = crate::W<OPA0_CAL_SPEC>;
#[doc = "Field `CM1` reader - Compensation Cap Cm1 Trim Value"]
pub type CM1_R = crate::FieldReader;
#[doc = "Field `CM1` writer - Compensation Cap Cm1 Trim Value"]
pub type CM1_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CM2` reader - Compensation Cap Cm2 Trim Value"]
pub type CM2_R = crate::FieldReader;
#[doc = "Field `CM2` writer - Compensation Cap Cm2 Trim Value"]
pub type CM2_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CM3` reader - Compensation Cap Cm3 Trim Value"]
pub type CM3_R = crate::FieldReader;
#[doc = "Field `CM3` writer - Compensation Cap Cm3 Trim Value"]
pub type CM3_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `GM` reader - Gm Trim Value"]
pub type GM_R = crate::FieldReader;
#[doc = "Field `GM` writer - Gm Trim Value"]
pub type GM_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `GM3` reader - Gm3 Trim Value"]
pub type GM3_R = crate::FieldReader;
#[doc = "Field `GM3` writer - Gm3 Trim Value"]
pub type GM3_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `OFFSETP` reader - OPAx Non-Inverting Input Offset Configuration Value"]
pub type OFFSETP_R = crate::FieldReader;
#[doc = "Field `OFFSETP` writer - OPAx Non-Inverting Input Offset Configuration Value"]
pub type OFFSETP_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `OFFSETN` reader - OPAx Inverting Input Offset Configuration Value"]
pub type OFFSETN_R = crate::FieldReader;
#[doc = "Field `OFFSETN` writer - OPAx Inverting Input Offset Configuration Value"]
pub type OFFSETN_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:3 - Compensation Cap Cm1 Trim Value"]
    #[inline(always)]
    pub fn cm1(&self) -> CM1_R {
        CM1_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 5:8 - Compensation Cap Cm2 Trim Value"]
    #[inline(always)]
    pub fn cm2(&self) -> CM2_R {
        CM2_R::new(((self.bits >> 5) & 0x0f) as u8)
    }
    #[doc = "Bits 10:11 - Compensation Cap Cm3 Trim Value"]
    #[inline(always)]
    pub fn cm3(&self) -> CM3_R {
        CM3_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 13:15 - Gm Trim Value"]
    #[inline(always)]
    pub fn gm(&self) -> GM_R {
        GM_R::new(((self.bits >> 13) & 7) as u8)
    }
    #[doc = "Bits 17:18 - Gm3 Trim Value"]
    #[inline(always)]
    pub fn gm3(&self) -> GM3_R {
        GM3_R::new(((self.bits >> 17) & 3) as u8)
    }
    #[doc = "Bits 20:24 - OPAx Non-Inverting Input Offset Configuration Value"]
    #[inline(always)]
    pub fn offsetp(&self) -> OFFSETP_R {
        OFFSETP_R::new(((self.bits >> 20) & 0x1f) as u8)
    }
    #[doc = "Bits 26:30 - OPAx Inverting Input Offset Configuration Value"]
    #[inline(always)]
    pub fn offsetn(&self) -> OFFSETN_R {
        OFFSETN_R::new(((self.bits >> 26) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Compensation Cap Cm1 Trim Value"]
    #[inline(always)]
    #[must_use]
    pub fn cm1(&mut self) -> CM1_W<OPA0_CAL_SPEC> {
        CM1_W::new(self, 0)
    }
    #[doc = "Bits 5:8 - Compensation Cap Cm2 Trim Value"]
    #[inline(always)]
    #[must_use]
    pub fn cm2(&mut self) -> CM2_W<OPA0_CAL_SPEC> {
        CM2_W::new(self, 5)
    }
    #[doc = "Bits 10:11 - Compensation Cap Cm3 Trim Value"]
    #[inline(always)]
    #[must_use]
    pub fn cm3(&mut self) -> CM3_W<OPA0_CAL_SPEC> {
        CM3_W::new(self, 10)
    }
    #[doc = "Bits 13:15 - Gm Trim Value"]
    #[inline(always)]
    #[must_use]
    pub fn gm(&mut self) -> GM_W<OPA0_CAL_SPEC> {
        GM_W::new(self, 13)
    }
    #[doc = "Bits 17:18 - Gm3 Trim Value"]
    #[inline(always)]
    #[must_use]
    pub fn gm3(&mut self) -> GM3_W<OPA0_CAL_SPEC> {
        GM3_W::new(self, 17)
    }
    #[doc = "Bits 20:24 - OPAx Non-Inverting Input Offset Configuration Value"]
    #[inline(always)]
    #[must_use]
    pub fn offsetp(&mut self) -> OFFSETP_W<OPA0_CAL_SPEC> {
        OFFSETP_W::new(self, 20)
    }
    #[doc = "Bits 26:30 - OPAx Inverting Input Offset Configuration Value"]
    #[inline(always)]
    #[must_use]
    pub fn offsetn(&mut self) -> OFFSETN_W<OPA0_CAL_SPEC> {
        OFFSETN_W::new(self, 26)
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
#[doc = "Operational Amplifier Calibration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`opa0_cal::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`opa0_cal::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OPA0_CAL_SPEC;
impl crate::RegisterSpec for OPA0_CAL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`opa0_cal::R`](R) reader structure"]
impl crate::Readable for OPA0_CAL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`opa0_cal::W`](W) writer structure"]
impl crate::Writable for OPA0_CAL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OPA0_CAL to value 0x80e7"]
impl crate::Resettable for OPA0_CAL_SPEC {
    const RESET_VALUE: u32 = 0x80e7;
}
