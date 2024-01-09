#[doc = "Register `DMCFG` reader"]
pub type R = crate::R<DMCFG_SPEC>;
#[doc = "Register `DMCFG` writer"]
pub type W = crate::W<DMCFG_SPEC>;
#[doc = "Field `DMG` reader - Delta Modulator Gain Step"]
pub type DMG_R = crate::FieldReader;
#[doc = "Field `DMG` writer - Delta Modulator Gain Step"]
pub type DMG_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `DMR` reader - Delta Modulator Gain Reduction Interval"]
pub type DMR_R = crate::FieldReader;
#[doc = "Field `DMR` writer - Delta Modulator Gain Reduction Interval"]
pub type DMR_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `DMCR` reader - Delta Modulator Conversion Rate"]
pub type DMCR_R = crate::FieldReader;
#[doc = "Field `DMCR` writer - Delta Modulator Conversion Rate"]
pub type DMCR_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CRMODE` reader - Delta Modulator Conversion Resolution."]
pub type CRMODE_R = crate::FieldReader<CRMODE_A>;
#[doc = "Delta Modulator Conversion Resolution.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CRMODE_A {
    #[doc = "0: 10-bit delta modulator"]
    DM10 = 0,
    #[doc = "1: 12-bit delta modulator"]
    DM12 = 1,
    #[doc = "2: 14-bit delta modulator"]
    DM14 = 2,
    #[doc = "3: 16-bit delta modulator"]
    DM16 = 3,
}
impl From<CRMODE_A> for u8 {
    #[inline(always)]
    fn from(variant: CRMODE_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CRMODE_A {
    type Ux = u8;
}
impl CRMODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CRMODE_A {
        match self.bits {
            0 => CRMODE_A::DM10,
            1 => CRMODE_A::DM12,
            2 => CRMODE_A::DM14,
            3 => CRMODE_A::DM16,
            _ => unreachable!(),
        }
    }
    #[doc = "10-bit delta modulator"]
    #[inline(always)]
    pub fn is_dm10(&self) -> bool {
        *self == CRMODE_A::DM10
    }
    #[doc = "12-bit delta modulator"]
    #[inline(always)]
    pub fn is_dm12(&self) -> bool {
        *self == CRMODE_A::DM12
    }
    #[doc = "14-bit delta modulator"]
    #[inline(always)]
    pub fn is_dm14(&self) -> bool {
        *self == CRMODE_A::DM14
    }
    #[doc = "16-bit delta modulator"]
    #[inline(always)]
    pub fn is_dm16(&self) -> bool {
        *self == CRMODE_A::DM16
    }
}
#[doc = "Field `CRMODE` writer - Delta Modulator Conversion Resolution."]
pub type CRMODE_W<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, CRMODE_A>;
impl<'a, REG> CRMODE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "10-bit delta modulator"]
    #[inline(always)]
    pub fn dm10(self) -> &'a mut crate::W<REG> {
        self.variant(CRMODE_A::DM10)
    }
    #[doc = "12-bit delta modulator"]
    #[inline(always)]
    pub fn dm12(self) -> &'a mut crate::W<REG> {
        self.variant(CRMODE_A::DM12)
    }
    #[doc = "14-bit delta modulator"]
    #[inline(always)]
    pub fn dm14(self) -> &'a mut crate::W<REG> {
        self.variant(CRMODE_A::DM14)
    }
    #[doc = "16-bit delta modulator"]
    #[inline(always)]
    pub fn dm16(self) -> &'a mut crate::W<REG> {
        self.variant(CRMODE_A::DM16)
    }
}
#[doc = "Field `DMGRDIS` reader - Delta Modulation Gain Step Reduction Disable"]
pub type DMGRDIS_R = crate::BitReader;
#[doc = "Field `DMGRDIS` writer - Delta Modulation Gain Step Reduction Disable"]
pub type DMGRDIS_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - Delta Modulator Gain Step"]
    #[inline(always)]
    pub fn dmg(&self) -> DMG_R {
        DMG_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:11 - Delta Modulator Gain Reduction Interval"]
    #[inline(always)]
    pub fn dmr(&self) -> DMR_R {
        DMR_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Delta Modulator Conversion Rate"]
    #[inline(always)]
    pub fn dmcr(&self) -> DMCR_R {
        DMCR_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:21 - Delta Modulator Conversion Resolution."]
    #[inline(always)]
    pub fn crmode(&self) -> CRMODE_R {
        CRMODE_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bit 28 - Delta Modulation Gain Step Reduction Disable"]
    #[inline(always)]
    pub fn dmgrdis(&self) -> DMGRDIS_R {
        DMGRDIS_R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - Delta Modulator Gain Step"]
    #[inline(always)]
    #[must_use]
    pub fn dmg(&mut self) -> DMG_W<DMCFG_SPEC> {
        DMG_W::new(self, 0)
    }
    #[doc = "Bits 8:11 - Delta Modulator Gain Reduction Interval"]
    #[inline(always)]
    #[must_use]
    pub fn dmr(&mut self) -> DMR_W<DMCFG_SPEC> {
        DMR_W::new(self, 8)
    }
    #[doc = "Bits 16:19 - Delta Modulator Conversion Rate"]
    #[inline(always)]
    #[must_use]
    pub fn dmcr(&mut self) -> DMCR_W<DMCFG_SPEC> {
        DMCR_W::new(self, 16)
    }
    #[doc = "Bits 20:21 - Delta Modulator Conversion Resolution."]
    #[inline(always)]
    #[must_use]
    pub fn crmode(&mut self) -> CRMODE_W<DMCFG_SPEC> {
        CRMODE_W::new(self, 20)
    }
    #[doc = "Bit 28 - Delta Modulation Gain Step Reduction Disable"]
    #[inline(always)]
    #[must_use]
    pub fn dmgrdis(&mut self) -> DMGRDIS_W<DMCFG_SPEC> {
        DMGRDIS_W::new(self, 28)
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
#[doc = "Delta Modulation Configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmcfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmcfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMCFG_SPEC;
impl crate::RegisterSpec for DMCFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmcfg::R`](R) reader structure"]
impl crate::Readable for DMCFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dmcfg::W`](W) writer structure"]
impl crate::Writable for DMCFG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DMCFG to value 0"]
impl crate::Resettable for DMCFG_SPEC {
    const RESET_VALUE: u32 = 0;
}
