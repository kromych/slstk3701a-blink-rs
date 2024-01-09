#[doc = "Register `DATE` reader"]
pub type R = crate::R<DATE_SPEC>;
#[doc = "Register `DATE` writer"]
pub type W = crate::W<DATE_SPEC>;
#[doc = "Field `DAYOMU` reader - Day of Month, Units"]
pub type DAYOMU_R = crate::FieldReader;
#[doc = "Field `DAYOMU` writer - Day of Month, Units"]
pub type DAYOMU_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `DAYOMT` reader - Day of Month, Tens"]
pub type DAYOMT_R = crate::FieldReader;
#[doc = "Field `DAYOMT` writer - Day of Month, Tens"]
pub type DAYOMT_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MONTHU` reader - Month, Units"]
pub type MONTHU_R = crate::FieldReader;
#[doc = "Field `MONTHU` writer - Month, Units"]
pub type MONTHU_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `MONTHT` reader - Month, Tens"]
pub type MONTHT_R = crate::BitReader;
#[doc = "Field `MONTHT` writer - Month, Tens"]
pub type MONTHT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `YEARU` reader - Year, Units"]
pub type YEARU_R = crate::FieldReader;
#[doc = "Field `YEARU` writer - Year, Units"]
pub type YEARU_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `YEART` reader - Year, Tens"]
pub type YEART_R = crate::FieldReader;
#[doc = "Field `YEART` writer - Year, Tens"]
pub type YEART_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `DAYOW` reader - Day of Week"]
pub type DAYOW_R = crate::FieldReader;
#[doc = "Field `DAYOW` writer - Day of Week"]
pub type DAYOW_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:3 - Day of Month, Units"]
    #[inline(always)]
    pub fn dayomu(&self) -> DAYOMU_R {
        DAYOMU_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:5 - Day of Month, Tens"]
    #[inline(always)]
    pub fn dayomt(&self) -> DAYOMT_R {
        DAYOMT_R::new(((self.bits >> 4) & 3) as u8)
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
    #[doc = "Bits 16:19 - Year, Units"]
    #[inline(always)]
    pub fn yearu(&self) -> YEARU_R {
        YEARU_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Year, Tens"]
    #[inline(always)]
    pub fn yeart(&self) -> YEART_R {
        YEART_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:26 - Day of Week"]
    #[inline(always)]
    pub fn dayow(&self) -> DAYOW_R {
        DAYOW_R::new(((self.bits >> 24) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Day of Month, Units"]
    #[inline(always)]
    #[must_use]
    pub fn dayomu(&mut self) -> DAYOMU_W<DATE_SPEC> {
        DAYOMU_W::new(self, 0)
    }
    #[doc = "Bits 4:5 - Day of Month, Tens"]
    #[inline(always)]
    #[must_use]
    pub fn dayomt(&mut self) -> DAYOMT_W<DATE_SPEC> {
        DAYOMT_W::new(self, 4)
    }
    #[doc = "Bits 8:11 - Month, Units"]
    #[inline(always)]
    #[must_use]
    pub fn monthu(&mut self) -> MONTHU_W<DATE_SPEC> {
        MONTHU_W::new(self, 8)
    }
    #[doc = "Bit 12 - Month, Tens"]
    #[inline(always)]
    #[must_use]
    pub fn montht(&mut self) -> MONTHT_W<DATE_SPEC> {
        MONTHT_W::new(self, 12)
    }
    #[doc = "Bits 16:19 - Year, Units"]
    #[inline(always)]
    #[must_use]
    pub fn yearu(&mut self) -> YEARU_W<DATE_SPEC> {
        YEARU_W::new(self, 16)
    }
    #[doc = "Bits 20:23 - Year, Tens"]
    #[inline(always)]
    #[must_use]
    pub fn yeart(&mut self) -> YEART_W<DATE_SPEC> {
        YEART_W::new(self, 20)
    }
    #[doc = "Bits 24:26 - Day of Week"]
    #[inline(always)]
    #[must_use]
    pub fn dayow(&mut self) -> DAYOW_W<DATE_SPEC> {
        DAYOW_W::new(self, 24)
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
#[doc = "Date Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`date::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`date::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DATE_SPEC;
impl crate::RegisterSpec for DATE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`date::R`](R) reader structure"]
impl crate::Readable for DATE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`date::W`](W) writer structure"]
impl crate::Writable for DATE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DATE to value 0"]
impl crate::Resettable for DATE_SPEC {
    const RESET_VALUE: u32 = 0;
}
