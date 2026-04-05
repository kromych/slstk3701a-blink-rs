#[doc = "Register `OPA3_CAL` reader"]
pub type R = crate::R<Opa3CalSpec>;
#[doc = "Register `OPA3_CAL` writer"]
pub type W = crate::W<Opa3CalSpec>;
#[doc = "Field `CM1` reader - Compensation Cap Cm1 Trim Value"]
pub type Cm1R = crate::FieldReader;
#[doc = "Field `CM1` writer - Compensation Cap Cm1 Trim Value"]
pub type Cm1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CM2` reader - Compensation Cap Cm2 Trim Value"]
pub type Cm2R = crate::FieldReader;
#[doc = "Field `CM2` writer - Compensation Cap Cm2 Trim Value"]
pub type Cm2W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CM3` reader - Compensation Cap Cm3 Trim Value"]
pub type Cm3R = crate::FieldReader;
#[doc = "Field `CM3` writer - Compensation Cap Cm3 Trim Value"]
pub type Cm3W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `GM` reader - Gm Trim Value"]
pub type GmR = crate::FieldReader;
#[doc = "Field `GM` writer - Gm Trim Value"]
pub type GmW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `GM3` reader - Gm3 Trim Value"]
pub type Gm3R = crate::FieldReader;
#[doc = "Field `GM3` writer - Gm3 Trim Value"]
pub type Gm3W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `OFFSETP` reader - OPAx Non-Inverting Input Offset Configuration Value"]
pub type OffsetpR = crate::FieldReader;
#[doc = "Field `OFFSETP` writer - OPAx Non-Inverting Input Offset Configuration Value"]
pub type OffsetpW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `OFFSETN` reader - OPAx Inverting Input Offset Configuration Value"]
pub type OffsetnR = crate::FieldReader;
#[doc = "Field `OFFSETN` writer - OPAx Inverting Input Offset Configuration Value"]
pub type OffsetnW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:3 - Compensation Cap Cm1 Trim Value"]
    #[inline(always)]
    pub fn cm1(&self) -> Cm1R {
        Cm1R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 5:8 - Compensation Cap Cm2 Trim Value"]
    #[inline(always)]
    pub fn cm2(&self) -> Cm2R {
        Cm2R::new(((self.bits >> 5) & 0x0f) as u8)
    }
    #[doc = "Bits 10:11 - Compensation Cap Cm3 Trim Value"]
    #[inline(always)]
    pub fn cm3(&self) -> Cm3R {
        Cm3R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 13:15 - Gm Trim Value"]
    #[inline(always)]
    pub fn gm(&self) -> GmR {
        GmR::new(((self.bits >> 13) & 7) as u8)
    }
    #[doc = "Bits 17:18 - Gm3 Trim Value"]
    #[inline(always)]
    pub fn gm3(&self) -> Gm3R {
        Gm3R::new(((self.bits >> 17) & 3) as u8)
    }
    #[doc = "Bits 20:24 - OPAx Non-Inverting Input Offset Configuration Value"]
    #[inline(always)]
    pub fn offsetp(&self) -> OffsetpR {
        OffsetpR::new(((self.bits >> 20) & 0x1f) as u8)
    }
    #[doc = "Bits 26:30 - OPAx Inverting Input Offset Configuration Value"]
    #[inline(always)]
    pub fn offsetn(&self) -> OffsetnR {
        OffsetnR::new(((self.bits >> 26) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Compensation Cap Cm1 Trim Value"]
    #[inline(always)]
    pub fn cm1(&mut self) -> Cm1W<'_, Opa3CalSpec> {
        Cm1W::new(self, 0)
    }
    #[doc = "Bits 5:8 - Compensation Cap Cm2 Trim Value"]
    #[inline(always)]
    pub fn cm2(&mut self) -> Cm2W<'_, Opa3CalSpec> {
        Cm2W::new(self, 5)
    }
    #[doc = "Bits 10:11 - Compensation Cap Cm3 Trim Value"]
    #[inline(always)]
    pub fn cm3(&mut self) -> Cm3W<'_, Opa3CalSpec> {
        Cm3W::new(self, 10)
    }
    #[doc = "Bits 13:15 - Gm Trim Value"]
    #[inline(always)]
    pub fn gm(&mut self) -> GmW<'_, Opa3CalSpec> {
        GmW::new(self, 13)
    }
    #[doc = "Bits 17:18 - Gm3 Trim Value"]
    #[inline(always)]
    pub fn gm3(&mut self) -> Gm3W<'_, Opa3CalSpec> {
        Gm3W::new(self, 17)
    }
    #[doc = "Bits 20:24 - OPAx Non-Inverting Input Offset Configuration Value"]
    #[inline(always)]
    pub fn offsetp(&mut self) -> OffsetpW<'_, Opa3CalSpec> {
        OffsetpW::new(self, 20)
    }
    #[doc = "Bits 26:30 - OPAx Inverting Input Offset Configuration Value"]
    #[inline(always)]
    pub fn offsetn(&mut self) -> OffsetnW<'_, Opa3CalSpec> {
        OffsetnW::new(self, 26)
    }
}
#[doc = "Operational Amplifier Calibration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`opa3_cal::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`opa3_cal::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Opa3CalSpec;
impl crate::RegisterSpec for Opa3CalSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`opa3_cal::R`](R) reader structure"]
impl crate::Readable for Opa3CalSpec {}
#[doc = "`write(|w| ..)` method takes [`opa3_cal::W`](W) writer structure"]
impl crate::Writable for Opa3CalSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OPA3_CAL to value 0x80e7"]
impl crate::Resettable for Opa3CalSpec {
    const RESET_VALUE: u32 = 0x80e7;
}
