#[doc = "Register `CC0_DATE` reader"]
pub type R = crate::R<CC0_DATE_SPEC>;
#[doc = "Register `CC0_DATE` writer"]
pub type W = crate::W<CC0_DATE_SPEC>;
#[doc = "Field `DAYU` reader - Day of Month/week, Units"]
pub type DAYU_R = crate::FieldReader;
#[doc = "Field `DAYU` writer - Day of Month/week, Units"]
pub type DAYU_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `DAYT` reader - Day of Month/week, Tens"]
pub type DAYT_R = crate::FieldReader;
#[doc = "Field `DAYT` writer - Day of Month/week, Tens"]
pub type DAYT_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `MONTHU` reader - Month, Units"]
pub type MONTHU_R = crate::FieldReader;
#[doc = "Field `MONTHU` writer - Month, Units"]
pub type MONTHU_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `MONTHT` reader - Month, Tens"]
pub type MONTHT_R = crate::BitReader;
#[doc = "Field `MONTHT` writer - Month, Tens"]
pub type MONTHT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:3 - Day of Month/week, Units"]
    #[inline(always)]
    pub fn dayu(&self) -> DAYU_R {
        DAYU_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:5 - Day of Month/week, Tens"]
    #[inline(always)]
    pub fn dayt(&self) -> DAYT_R {
        DAYT_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 8:11 - Month, Units"]
    #[inline(always)]
    pub fn monthu(&self) -> MONTHU_R {
        MONTHU_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 12 - Month, Tens"]
    #[inline(always)]
    pub fn montht(&self) -> MONTHT_R {
        MONTHT_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Day of Month/week, Units"]
    #[inline(always)]
    #[must_use]
    pub fn dayu(&mut self) -> DAYU_W<CC0_DATE_SPEC, 0> {
        DAYU_W::new(self)
    }
    #[doc = "Bits 4:5 - Day of Month/week, Tens"]
    #[inline(always)]
    #[must_use]
    pub fn dayt(&mut self) -> DAYT_W<CC0_DATE_SPEC, 4> {
        DAYT_W::new(self)
    }
    #[doc = "Bits 8:11 - Month, Units"]
    #[inline(always)]
    #[must_use]
    pub fn monthu(&mut self) -> MONTHU_W<CC0_DATE_SPEC, 8> {
        MONTHU_W::new(self)
    }
    #[doc = "Bit 12 - Month, Tens"]
    #[inline(always)]
    #[must_use]
    pub fn montht(&mut self) -> MONTHT_W<CC0_DATE_SPEC, 12> {
        MONTHT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Capture/Compare Date Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cc0_date::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cc0_date::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CC0_DATE_SPEC;
impl crate::RegisterSpec for CC0_DATE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cc0_date::R`](R) reader structure"]
impl crate::Readable for CC0_DATE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cc0_date::W`](W) writer structure"]
impl crate::Writable for CC0_DATE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CC0_DATE to value 0"]
impl crate::Resettable for CC0_DATE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
