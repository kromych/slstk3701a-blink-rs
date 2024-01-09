#[doc = "Register `TXDOUBLE` reader"]
pub type R = crate::R<TXDOUBLE_SPEC>;
#[doc = "Register `TXDOUBLE` writer"]
pub type W = crate::W<TXDOUBLE_SPEC>;
#[doc = "Field `TXDATA0` reader - TX Data"]
pub type TXDATA0_R = crate::FieldReader;
#[doc = "Field `TXDATA0` writer - TX Data"]
pub type TXDATA0_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `TXDATA1` reader - TX Data"]
pub type TXDATA1_R = crate::FieldReader;
#[doc = "Field `TXDATA1` writer - TX Data"]
pub type TXDATA1_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - TX Data"]
    #[inline(always)]
    pub fn txdata0(&self) -> TXDATA0_R {
        TXDATA0_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - TX Data"]
    #[inline(always)]
    pub fn txdata1(&self) -> TXDATA1_R {
        TXDATA1_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - TX Data"]
    #[inline(always)]
    #[must_use]
    pub fn txdata0(&mut self) -> TXDATA0_W<TXDOUBLE_SPEC> {
        TXDATA0_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - TX Data"]
    #[inline(always)]
    #[must_use]
    pub fn txdata1(&mut self) -> TXDATA1_W<TXDOUBLE_SPEC> {
        TXDATA1_W::new(self, 8)
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
#[doc = "TX Buffer Double Data Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`txdouble::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`txdouble::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TXDOUBLE_SPEC;
impl crate::RegisterSpec for TXDOUBLE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txdouble::R`](R) reader structure"]
impl crate::Readable for TXDOUBLE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`txdouble::W`](W) writer structure"]
impl crate::Writable for TXDOUBLE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TXDOUBLE to value 0"]
impl crate::Resettable for TXDOUBLE_SPEC {
    const RESET_VALUE: u32 = 0;
}
