#[doc = "Register `CC0_TIME` reader"]
pub type R = crate::R<CC0_TIME_SPEC>;
#[doc = "Register `CC0_TIME` writer"]
pub type W = crate::W<CC0_TIME_SPEC>;
#[doc = "Field `SECU` reader - Seconds, Units"]
pub type SECU_R = crate::FieldReader;
#[doc = "Field `SECU` writer - Seconds, Units"]
pub type SECU_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SECT` reader - Seconds, Tens"]
pub type SECT_R = crate::FieldReader;
#[doc = "Field `SECT` writer - Seconds, Tens"]
pub type SECT_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `MINU` reader - Minutes, Units"]
pub type MINU_R = crate::FieldReader;
#[doc = "Field `MINU` writer - Minutes, Units"]
pub type MINU_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `MINT` reader - Minutes, Tens"]
pub type MINT_R = crate::FieldReader;
#[doc = "Field `MINT` writer - Minutes, Tens"]
pub type MINT_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `HOURU` reader - Hours, Units"]
pub type HOURU_R = crate::FieldReader;
#[doc = "Field `HOURU` writer - Hours, Units"]
pub type HOURU_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `HOURT` reader - Hours, Tens"]
pub type HOURT_R = crate::FieldReader;
#[doc = "Field `HOURT` writer - Hours, Tens"]
pub type HOURT_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:3 - Seconds, Units"]
    #[inline(always)]
    pub fn secu(&self) -> SECU_R {
        SECU_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:6 - Seconds, Tens"]
    #[inline(always)]
    pub fn sect(&self) -> SECT_R {
        SECT_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:11 - Minutes, Units"]
    #[inline(always)]
    pub fn minu(&self) -> MINU_R {
        MINU_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:14 - Minutes, Tens"]
    #[inline(always)]
    pub fn mint(&self) -> MINT_R {
        MINT_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 16:19 - Hours, Units"]
    #[inline(always)]
    pub fn houru(&self) -> HOURU_R {
        HOURU_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:21 - Hours, Tens"]
    #[inline(always)]
    pub fn hourt(&self) -> HOURT_R {
        HOURT_R::new(((self.bits >> 20) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Seconds, Units"]
    #[inline(always)]
    #[must_use]
    pub fn secu(&mut self) -> SECU_W<CC0_TIME_SPEC> {
        SECU_W::new(self, 0)
    }
    #[doc = "Bits 4:6 - Seconds, Tens"]
    #[inline(always)]
    #[must_use]
    pub fn sect(&mut self) -> SECT_W<CC0_TIME_SPEC> {
        SECT_W::new(self, 4)
    }
    #[doc = "Bits 8:11 - Minutes, Units"]
    #[inline(always)]
    #[must_use]
    pub fn minu(&mut self) -> MINU_W<CC0_TIME_SPEC> {
        MINU_W::new(self, 8)
    }
    #[doc = "Bits 12:14 - Minutes, Tens"]
    #[inline(always)]
    #[must_use]
    pub fn mint(&mut self) -> MINT_W<CC0_TIME_SPEC> {
        MINT_W::new(self, 12)
    }
    #[doc = "Bits 16:19 - Hours, Units"]
    #[inline(always)]
    #[must_use]
    pub fn houru(&mut self) -> HOURU_W<CC0_TIME_SPEC> {
        HOURU_W::new(self, 16)
    }
    #[doc = "Bits 20:21 - Hours, Tens"]
    #[inline(always)]
    #[must_use]
    pub fn hourt(&mut self) -> HOURT_W<CC0_TIME_SPEC> {
        HOURT_W::new(self, 20)
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
#[doc = "Capture/Compare Time Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cc0_time::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cc0_time::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CC0_TIME_SPEC;
impl crate::RegisterSpec for CC0_TIME_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cc0_time::R`](R) reader structure"]
impl crate::Readable for CC0_TIME_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cc0_time::W`](W) writer structure"]
impl crate::Writable for CC0_TIME_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CC0_TIME to value 0"]
impl crate::Resettable for CC0_TIME_SPEC {
    const RESET_VALUE: u32 = 0;
}
