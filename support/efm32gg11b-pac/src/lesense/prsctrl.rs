#[doc = "Register `PRSCTRL` reader"]
pub type R = crate::R<PRSCTRL_SPEC>;
#[doc = "Register `PRSCTRL` writer"]
pub type W = crate::W<PRSCTRL_SPEC>;
#[doc = "Field `DECCMPVAL` reader - Decoder State Compare Value"]
pub type DECCMPVAL_R = crate::FieldReader;
#[doc = "Field `DECCMPVAL` writer - Decoder State Compare Value"]
pub type DECCMPVAL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 5, O>;
#[doc = "Field `DECCMPMASK` reader - Decoder State Compare Value Mask"]
pub type DECCMPMASK_R = crate::FieldReader;
#[doc = "Field `DECCMPMASK` writer - Decoder State Compare Value Mask"]
pub type DECCMPMASK_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 5, O>;
#[doc = "Field `DECCMPEN` reader - Enable PRS Output DECCMP"]
pub type DECCMPEN_R = crate::BitReader;
#[doc = "Field `DECCMPEN` writer - Enable PRS Output DECCMP"]
pub type DECCMPEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bits 0:4 - Decoder State Compare Value"]
    #[inline(always)]
    pub fn deccmpval(&self) -> DECCMPVAL_R {
        DECCMPVAL_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:12 - Decoder State Compare Value Mask"]
    #[inline(always)]
    pub fn deccmpmask(&self) -> DECCMPMASK_R {
        DECCMPMASK_R::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bit 16 - Enable PRS Output DECCMP"]
    #[inline(always)]
    pub fn deccmpen(&self) -> DECCMPEN_R {
        DECCMPEN_R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - Decoder State Compare Value"]
    #[inline(always)]
    #[must_use]
    pub fn deccmpval(&mut self) -> DECCMPVAL_W<PRSCTRL_SPEC, 0> {
        DECCMPVAL_W::new(self)
    }
    #[doc = "Bits 8:12 - Decoder State Compare Value Mask"]
    #[inline(always)]
    #[must_use]
    pub fn deccmpmask(&mut self) -> DECCMPMASK_W<PRSCTRL_SPEC, 8> {
        DECCMPMASK_W::new(self)
    }
    #[doc = "Bit 16 - Enable PRS Output DECCMP"]
    #[inline(always)]
    #[must_use]
    pub fn deccmpen(&mut self) -> DECCMPEN_W<PRSCTRL_SPEC, 16> {
        DECCMPEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "PRS Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`prsctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`prsctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PRSCTRL_SPEC;
impl crate::RegisterSpec for PRSCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`prsctrl::R`](R) reader structure"]
impl crate::Readable for PRSCTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`prsctrl::W`](W) writer structure"]
impl crate::Writable for PRSCTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PRSCTRL to value 0"]
impl crate::Resettable for PRSCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
