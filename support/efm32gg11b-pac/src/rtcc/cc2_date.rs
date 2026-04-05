#[doc = "Register `CC2_DATE` reader"]
pub type R = crate::R<Cc2DateSpec>;
#[doc = "Register `CC2_DATE` writer"]
pub type W = crate::W<Cc2DateSpec>;
#[doc = "Field `DAYU` reader - Day of Month/week, Units"]
pub type DayuR = crate::FieldReader;
#[doc = "Field `DAYU` writer - Day of Month/week, Units"]
pub type DayuW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `DAYT` reader - Day of Month/week, Tens"]
pub type DaytR = crate::FieldReader;
#[doc = "Field `DAYT` writer - Day of Month/week, Tens"]
pub type DaytW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MONTHU` reader - Month, Units"]
pub type MonthuR = crate::FieldReader;
#[doc = "Field `MONTHU` writer - Month, Units"]
pub type MonthuW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `MONTHT` reader - Month, Tens"]
pub type MonthtR = crate::BitReader;
#[doc = "Field `MONTHT` writer - Month, Tens"]
pub type MonthtW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - Day of Month/week, Units"]
    #[inline(always)]
    pub fn dayu(&self) -> DayuR {
        DayuR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:5 - Day of Month/week, Tens"]
    #[inline(always)]
    pub fn dayt(&self) -> DaytR {
        DaytR::new(((self.bits >> 4) & 3) as u8)
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
}
impl W {
    #[doc = "Bits 0:3 - Day of Month/week, Units"]
    #[inline(always)]
    pub fn dayu(&mut self) -> DayuW<'_, Cc2DateSpec> {
        DayuW::new(self, 0)
    }
    #[doc = "Bits 4:5 - Day of Month/week, Tens"]
    #[inline(always)]
    pub fn dayt(&mut self) -> DaytW<'_, Cc2DateSpec> {
        DaytW::new(self, 4)
    }
    #[doc = "Bits 8:11 - Month, Units"]
    #[inline(always)]
    pub fn monthu(&mut self) -> MonthuW<'_, Cc2DateSpec> {
        MonthuW::new(self, 8)
    }
    #[doc = "Bit 12 - Month, Tens"]
    #[inline(always)]
    pub fn montht(&mut self) -> MonthtW<'_, Cc2DateSpec> {
        MonthtW::new(self, 12)
    }
}
#[doc = "Capture/Compare Date Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cc2_date::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cc2_date::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cc2DateSpec;
impl crate::RegisterSpec for Cc2DateSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cc2_date::R`](R) reader structure"]
impl crate::Readable for Cc2DateSpec {}
#[doc = "`write(|w| ..)` method takes [`cc2_date::W`](W) writer structure"]
impl crate::Writable for Cc2DateSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CC2_DATE to value 0"]
impl crate::Resettable for Cc2DateSpec {}
