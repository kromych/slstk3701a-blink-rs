#[doc = "Register `TSUTIMERADJUST` reader"]
pub type R = crate::R<TSUTIMERADJUST_SPEC>;
#[doc = "Register `TSUTIMERADJUST` writer"]
pub type W = crate::W<TSUTIMERADJUST_SPEC>;
#[doc = "Field `INCREMENTVAL` reader - Timer increment value"]
pub type INCREMENTVAL_R = crate::FieldReader<u32>;
#[doc = "Field `INCREMENTVAL` writer - Timer increment value"]
pub type INCREMENTVAL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 30, O, u32>;
#[doc = "Field `ADDSUBTRACT` reader - Write as one to subtract from the 1588 timer"]
pub type ADDSUBTRACT_R = crate::BitReader;
#[doc = "Field `ADDSUBTRACT` writer - Write as one to subtract from the 1588 timer"]
pub type ADDSUBTRACT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:29 - Timer increment value"]
    #[inline(always)]
    pub fn incrementval(&self) -> INCREMENTVAL_R {
        INCREMENTVAL_R::new(self.bits & 0x3fff_ffff)
    }
    #[doc = "Bit 31 - Write as one to subtract from the 1588 timer"]
    #[inline(always)]
    pub fn addsubtract(&self) -> ADDSUBTRACT_R {
        ADDSUBTRACT_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:29 - Timer increment value"]
    #[inline(always)]
    #[must_use]
    pub fn incrementval(&mut self) -> INCREMENTVAL_W<TSUTIMERADJUST_SPEC, 0> {
        INCREMENTVAL_W::new(self)
    }
    #[doc = "Bit 31 - Write as one to subtract from the 1588 timer"]
    #[inline(always)]
    #[must_use]
    pub fn addsubtract(&mut self) -> ADDSUBTRACT_W<TSUTIMERADJUST_SPEC, 31> {
        ADDSUBTRACT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "This register returns all zeroes when read.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tsutimeradjust::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tsutimeradjust::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TSUTIMERADJUST_SPEC;
impl crate::RegisterSpec for TSUTIMERADJUST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tsutimeradjust::R`](R) reader structure"]
impl crate::Readable for TSUTIMERADJUST_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tsutimeradjust::W`](W) writer structure"]
impl crate::Writable for TSUTIMERADJUST_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TSUTIMERADJUST to value 0"]
impl crate::Resettable for TSUTIMERADJUST_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
