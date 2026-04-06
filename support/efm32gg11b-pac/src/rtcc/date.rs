#[doc = "Register `DATE` reader"]
pub type R = crate::R<DateSpec>;
#[doc = "Register `DATE` writer"]
pub type W = crate::W<DateSpec>;
#[doc = "Field `DAYOMU` reader - Day of Month, Units"]
pub type DayomuR = crate::FieldReader;
#[doc = "Field `DAYOMU` writer - Day of Month, Units"]
pub type DayomuW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `DAYOMT` reader - Day of Month, Tens"]
pub type DayomtR = crate::FieldReader;
#[doc = "Field `DAYOMT` writer - Day of Month, Tens"]
pub type DayomtW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MONTHU` reader - Month, Units"]
pub type MonthuR = crate::FieldReader;
#[doc = "Field `MONTHU` writer - Month, Units"]
pub type MonthuW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `MONTHT` reader - Month, Tens"]
pub type MonthtR = crate::BitReader;
#[doc = "Field `MONTHT` writer - Month, Tens"]
pub type MonthtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `YEARU` reader - Year, Units"]
pub type YearuR = crate::FieldReader;
#[doc = "Field `YEARU` writer - Year, Units"]
pub type YearuW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `YEART` reader - Year, Tens"]
pub type YeartR = crate::FieldReader;
#[doc = "Field `YEART` writer - Year, Tens"]
pub type YeartW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `DAYOW` reader - Day of Week"]
pub type DayowR = crate::FieldReader;
#[doc = "Field `DAYOW` writer - Day of Week"]
pub type DayowW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:3 - Day of Month, Units"]
    #[inline(always)]
    pub fn dayomu(&self) -> DayomuR {
        DayomuR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:5 - Day of Month, Tens"]
    #[inline(always)]
    pub fn dayomt(&self) -> DayomtR {
        DayomtR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 8:11 - Month, Units"]
    #[inline(always)]
    pub fn monthu(&self) -> MonthuR {
        MonthuR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 12 - Month, Tens"]
    #[inline(always)]
    pub fn montht(&self) -> MonthtR {
        MonthtR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 16:19 - Year, Units"]
    #[inline(always)]
    pub fn yearu(&self) -> YearuR {
        YearuR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Year, Tens"]
    #[inline(always)]
    pub fn yeart(&self) -> YeartR {
        YeartR::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:26 - Day of Week"]
    #[inline(always)]
    pub fn dayow(&self) -> DayowR {
        DayowR::new(((self.bits >> 24) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Day of Month, Units"]
    #[inline(always)]
    pub fn dayomu(&mut self) -> DayomuW<'_, DateSpec> {
        DayomuW::new(self, 0)
    }
    #[doc = "Bits 4:5 - Day of Month, Tens"]
    #[inline(always)]
    pub fn dayomt(&mut self) -> DayomtW<'_, DateSpec> {
        DayomtW::new(self, 4)
    }
    #[doc = "Bits 8:11 - Month, Units"]
    #[inline(always)]
    pub fn monthu(&mut self) -> MonthuW<'_, DateSpec> {
        MonthuW::new(self, 8)
    }
    #[doc = "Bit 12 - Month, Tens"]
    #[inline(always)]
    pub fn montht(&mut self) -> MonthtW<'_, DateSpec> {
        MonthtW::new(self, 12)
    }
    #[doc = "Bits 16:19 - Year, Units"]
    #[inline(always)]
    pub fn yearu(&mut self) -> YearuW<'_, DateSpec> {
        YearuW::new(self, 16)
    }
    #[doc = "Bits 20:23 - Year, Tens"]
    #[inline(always)]
    pub fn yeart(&mut self) -> YeartW<'_, DateSpec> {
        YeartW::new(self, 20)
    }
    #[doc = "Bits 24:26 - Day of Week"]
    #[inline(always)]
    pub fn dayow(&mut self) -> DayowW<'_, DateSpec> {
        DayowW::new(self, 24)
    }
}
#[doc = "Date Register\n\nYou can [`read`](crate::Reg::read) this register and get [`date::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`date::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DateSpec;
impl crate::RegisterSpec for DateSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`date::R`](R) reader structure"]
impl crate::Readable for DateSpec {}
#[doc = "`write(|w| ..)` method takes [`date::W`](W) writer structure"]
impl crate::Writable for DateSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DATE to value 0"]
impl crate::Resettable for DateSpec {}
