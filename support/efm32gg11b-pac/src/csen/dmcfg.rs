#[doc = "Register `DMCFG` reader"]
pub type R = crate::R<DmcfgSpec>;
#[doc = "Register `DMCFG` writer"]
pub type W = crate::W<DmcfgSpec>;
#[doc = "Field `DMG` reader - Delta Modulator Gain Step"]
pub type DmgR = crate::FieldReader;
#[doc = "Field `DMG` writer - Delta Modulator Gain Step"]
pub type DmgW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `DMR` reader - Delta Modulator Gain Reduction Interval"]
pub type DmrR = crate::FieldReader;
#[doc = "Field `DMR` writer - Delta Modulator Gain Reduction Interval"]
pub type DmrW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `DMCR` reader - Delta Modulator Conversion Rate"]
pub type DmcrR = crate::FieldReader;
#[doc = "Field `DMCR` writer - Delta Modulator Conversion Rate"]
pub type DmcrW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Delta Modulator Conversion Resolution.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Crmode {
    #[doc = "0: 10-bit delta modulator"]
    Dm10 = 0,
    #[doc = "1: 12-bit delta modulator"]
    Dm12 = 1,
    #[doc = "2: 14-bit delta modulator"]
    Dm14 = 2,
    #[doc = "3: 16-bit delta modulator"]
    Dm16 = 3,
}
impl From<Crmode> for u8 {
    #[inline(always)]
    fn from(variant: Crmode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Crmode {
    type Ux = u8;
}
impl crate::IsEnum for Crmode {}
#[doc = "Field `CRMODE` reader - Delta Modulator Conversion Resolution."]
pub type CrmodeR = crate::FieldReader<Crmode>;
impl CrmodeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Crmode {
        match self.bits {
            0 => Crmode::Dm10,
            1 => Crmode::Dm12,
            2 => Crmode::Dm14,
            3 => Crmode::Dm16,
            _ => unreachable!(),
        }
    }
    #[doc = "10-bit delta modulator"]
    #[inline(always)]
    pub fn is_dm10(&self) -> bool {
        *self == Crmode::Dm10
    }
    #[doc = "12-bit delta modulator"]
    #[inline(always)]
    pub fn is_dm12(&self) -> bool {
        *self == Crmode::Dm12
    }
    #[doc = "14-bit delta modulator"]
    #[inline(always)]
    pub fn is_dm14(&self) -> bool {
        *self == Crmode::Dm14
    }
    #[doc = "16-bit delta modulator"]
    #[inline(always)]
    pub fn is_dm16(&self) -> bool {
        *self == Crmode::Dm16
    }
}
#[doc = "Field `CRMODE` writer - Delta Modulator Conversion Resolution."]
pub type CrmodeW<'a, REG> = crate::FieldWriter<'a, REG, 2, Crmode, crate::Safe>;
impl<'a, REG> CrmodeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "10-bit delta modulator"]
    #[inline(always)]
    pub fn dm10(self) -> &'a mut crate::W<REG> {
        self.variant(Crmode::Dm10)
    }
    #[doc = "12-bit delta modulator"]
    #[inline(always)]
    pub fn dm12(self) -> &'a mut crate::W<REG> {
        self.variant(Crmode::Dm12)
    }
    #[doc = "14-bit delta modulator"]
    #[inline(always)]
    pub fn dm14(self) -> &'a mut crate::W<REG> {
        self.variant(Crmode::Dm14)
    }
    #[doc = "16-bit delta modulator"]
    #[inline(always)]
    pub fn dm16(self) -> &'a mut crate::W<REG> {
        self.variant(Crmode::Dm16)
    }
}
#[doc = "Field `DMGRDIS` reader - Delta Modulation Gain Step Reduction Disable"]
pub type DmgrdisR = crate::BitReader;
#[doc = "Field `DMGRDIS` writer - Delta Modulation Gain Step Reduction Disable"]
pub type DmgrdisW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - Delta Modulator Gain Step"]
    #[inline(always)]
    pub fn dmg(&self) -> DmgR {
        DmgR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:11 - Delta Modulator Gain Reduction Interval"]
    #[inline(always)]
    pub fn dmr(&self) -> DmrR {
        DmrR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Delta Modulator Conversion Rate"]
    #[inline(always)]
    pub fn dmcr(&self) -> DmcrR {
        DmcrR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:21 - Delta Modulator Conversion Resolution."]
    #[inline(always)]
    pub fn crmode(&self) -> CrmodeR {
        CrmodeR::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bit 28 - Delta Modulation Gain Step Reduction Disable"]
    #[inline(always)]
    pub fn dmgrdis(&self) -> DmgrdisR {
        DmgrdisR::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Delta Modulator Gain Step"]
    #[inline(always)]
    pub fn dmg(&mut self) -> DmgW<'_, DmcfgSpec> {
        DmgW::new(self, 0)
    }
    #[doc = "Bits 8:11 - Delta Modulator Gain Reduction Interval"]
    #[inline(always)]
    pub fn dmr(&mut self) -> DmrW<'_, DmcfgSpec> {
        DmrW::new(self, 8)
    }
    #[doc = "Bits 16:19 - Delta Modulator Conversion Rate"]
    #[inline(always)]
    pub fn dmcr(&mut self) -> DmcrW<'_, DmcfgSpec> {
        DmcrW::new(self, 16)
    }
    #[doc = "Bits 20:21 - Delta Modulator Conversion Resolution."]
    #[inline(always)]
    pub fn crmode(&mut self) -> CrmodeW<'_, DmcfgSpec> {
        CrmodeW::new(self, 20)
    }
    #[doc = "Bit 28 - Delta Modulation Gain Step Reduction Disable"]
    #[inline(always)]
    pub fn dmgrdis(&mut self) -> DmgrdisW<'_, DmcfgSpec> {
        DmgrdisW::new(self, 28)
    }
}
#[doc = "Delta Modulation Configuration\n\nYou can [`read`](crate::Reg::read) this register and get [`dmcfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmcfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmcfgSpec;
impl crate::RegisterSpec for DmcfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmcfg::R`](R) reader structure"]
impl crate::Readable for DmcfgSpec {}
#[doc = "`write(|w| ..)` method takes [`dmcfg::W`](W) writer structure"]
impl crate::Writable for DmcfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DMCFG to value 0"]
impl crate::Resettable for DmcfgSpec {}
