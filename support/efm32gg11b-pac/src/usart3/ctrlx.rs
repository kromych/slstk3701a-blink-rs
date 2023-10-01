#[doc = "Register `CTRLX` reader"]
pub type R = crate::R<CTRLX_SPEC>;
#[doc = "Register `CTRLX` writer"]
pub type W = crate::W<CTRLX_SPEC>;
#[doc = "Field `DBGHALT` reader - Debug Halt"]
pub type DBGHALT_R = crate::BitReader;
#[doc = "Field `DBGHALT` writer - Debug Halt"]
pub type DBGHALT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CTSINV` reader - CTS Pin Inversion"]
pub type CTSINV_R = crate::BitReader;
#[doc = "Field `CTSINV` writer - CTS Pin Inversion"]
pub type CTSINV_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CTSEN` reader - CTS Function Enabled"]
pub type CTSEN_R = crate::BitReader;
#[doc = "Field `CTSEN` writer - CTS Function Enabled"]
pub type CTSEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RTSINV` reader - RTS Pin Inversion"]
pub type RTSINV_R = crate::BitReader;
#[doc = "Field `RTSINV` writer - RTS Pin Inversion"]
pub type RTSINV_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - Debug Halt"]
    #[inline(always)]
    pub fn dbghalt(&self) -> DBGHALT_R {
        DBGHALT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CTS Pin Inversion"]
    #[inline(always)]
    pub fn ctsinv(&self) -> CTSINV_R {
        CTSINV_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - CTS Function Enabled"]
    #[inline(always)]
    pub fn ctsen(&self) -> CTSEN_R {
        CTSEN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - RTS Pin Inversion"]
    #[inline(always)]
    pub fn rtsinv(&self) -> RTSINV_R {
        RTSINV_R::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Debug Halt"]
    #[inline(always)]
    #[must_use]
    pub fn dbghalt(&mut self) -> DBGHALT_W<CTRLX_SPEC, 0> {
        DBGHALT_W::new(self)
    }
    #[doc = "Bit 1 - CTS Pin Inversion"]
    #[inline(always)]
    #[must_use]
    pub fn ctsinv(&mut self) -> CTSINV_W<CTRLX_SPEC, 1> {
        CTSINV_W::new(self)
    }
    #[doc = "Bit 2 - CTS Function Enabled"]
    #[inline(always)]
    #[must_use]
    pub fn ctsen(&mut self) -> CTSEN_W<CTRLX_SPEC, 2> {
        CTSEN_W::new(self)
    }
    #[doc = "Bit 3 - RTS Pin Inversion"]
    #[inline(always)]
    #[must_use]
    pub fn rtsinv(&mut self) -> RTSINV_W<CTRLX_SPEC, 3> {
        RTSINV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Control Register Extended\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctrlx::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctrlx::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTRLX_SPEC;
impl crate::RegisterSpec for CTRLX_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ctrlx::R`](R) reader structure"]
impl crate::Readable for CTRLX_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctrlx::W`](W) writer structure"]
impl crate::Writable for CTRLX_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTRLX to value 0"]
impl crate::Resettable for CTRLX_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
