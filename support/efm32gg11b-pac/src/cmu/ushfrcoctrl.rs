#[doc = "Register `USHFRCOCTRL` reader"]
pub type R = crate::R<USHFRCOCTRL_SPEC>;
#[doc = "Register `USHFRCOCTRL` writer"]
pub type W = crate::W<USHFRCOCTRL_SPEC>;
#[doc = "Field `TUNING` reader - USHFRCO Tuning Value"]
pub type TUNING_R = crate::FieldReader;
#[doc = "Field `TUNING` writer - USHFRCO Tuning Value"]
pub type TUNING_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 7, O>;
#[doc = "Field `FINETUNING` reader - USHFRCO Fine Tuning Value"]
pub type FINETUNING_R = crate::FieldReader;
#[doc = "Field `FINETUNING` writer - USHFRCO Fine Tuning Value"]
pub type FINETUNING_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 6, O>;
#[doc = "Field `FREQRANGE` reader - USHFRCO Frequency Range"]
pub type FREQRANGE_R = crate::FieldReader;
#[doc = "Field `FREQRANGE` writer - USHFRCO Frequency Range"]
pub type FREQRANGE_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 5, O>;
#[doc = "Field `CMPBIAS` reader - USHFRCO Comparator Bias Current"]
pub type CMPBIAS_R = crate::FieldReader;
#[doc = "Field `CMPBIAS` writer - USHFRCO Comparator Bias Current"]
pub type CMPBIAS_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `LDOHP` reader - USHFRCO LDO High Power Mode"]
pub type LDOHP_R = crate::BitReader;
#[doc = "Field `LDOHP` writer - USHFRCO LDO High Power Mode"]
pub type LDOHP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `CLKDIV` reader - Locally Divide USHFRCO Clock Output"]
pub type CLKDIV_R = crate::FieldReader<CLKDIV_A>;
#[doc = "Locally Divide USHFRCO Clock Output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CLKDIV_A {
    #[doc = "0: Divide by 1."]
    DIV1 = 0,
    #[doc = "1: Divide by 2."]
    DIV2 = 1,
    #[doc = "2: Divide by 4."]
    DIV4 = 2,
}
impl From<CLKDIV_A> for u8 {
    #[inline(always)]
    fn from(variant: CLKDIV_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CLKDIV_A {
    type Ux = u8;
}
impl CLKDIV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CLKDIV_A> {
        match self.bits {
            0 => Some(CLKDIV_A::DIV1),
            1 => Some(CLKDIV_A::DIV2),
            2 => Some(CLKDIV_A::DIV4),
            _ => None,
        }
    }
    #[doc = "Divide by 1."]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == CLKDIV_A::DIV1
    }
    #[doc = "Divide by 2."]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == CLKDIV_A::DIV2
    }
    #[doc = "Divide by 4."]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == CLKDIV_A::DIV4
    }
}
#[doc = "Field `CLKDIV` writer - Locally Divide USHFRCO Clock Output"]
pub type CLKDIV_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O, CLKDIV_A>;
impl<'a, REG, const O: u8> CLKDIV_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Divide by 1."]
    #[inline(always)]
    pub fn div1(self) -> &'a mut crate::W<REG> {
        self.variant(CLKDIV_A::DIV1)
    }
    #[doc = "Divide by 2."]
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(CLKDIV_A::DIV2)
    }
    #[doc = "Divide by 4."]
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(CLKDIV_A::DIV4)
    }
}
#[doc = "Field `FINETUNINGEN` reader - Enable Reference for Fine Tuning"]
pub type FINETUNINGEN_R = crate::BitReader;
#[doc = "Field `FINETUNINGEN` writer - Enable Reference for Fine Tuning"]
pub type FINETUNINGEN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `VREFTC` reader - USHFRCO Temperature Coefficient Trim on Comparator Reference"]
pub type VREFTC_R = crate::FieldReader;
#[doc = "Field `VREFTC` writer - USHFRCO Temperature Coefficient Trim on Comparator Reference"]
pub type VREFTC_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
impl R {
    #[doc = "Bits 0:6 - USHFRCO Tuning Value"]
    #[inline(always)]
    pub fn tuning(&self) -> TUNING_R {
        TUNING_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:13 - USHFRCO Fine Tuning Value"]
    #[inline(always)]
    pub fn finetuning(&self) -> FINETUNING_R {
        FINETUNING_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:20 - USHFRCO Frequency Range"]
    #[inline(always)]
    pub fn freqrange(&self) -> FREQRANGE_R {
        FREQRANGE_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 21:23 - USHFRCO Comparator Bias Current"]
    #[inline(always)]
    pub fn cmpbias(&self) -> CMPBIAS_R {
        CMPBIAS_R::new(((self.bits >> 21) & 7) as u8)
    }
    #[doc = "Bit 24 - USHFRCO LDO High Power Mode"]
    #[inline(always)]
    pub fn ldohp(&self) -> LDOHP_R {
        LDOHP_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 25:26 - Locally Divide USHFRCO Clock Output"]
    #[inline(always)]
    pub fn clkdiv(&self) -> CLKDIV_R {
        CLKDIV_R::new(((self.bits >> 25) & 3) as u8)
    }
    #[doc = "Bit 27 - Enable Reference for Fine Tuning"]
    #[inline(always)]
    pub fn finetuningen(&self) -> FINETUNINGEN_R {
        FINETUNINGEN_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bits 28:31 - USHFRCO Temperature Coefficient Trim on Comparator Reference"]
    #[inline(always)]
    pub fn vreftc(&self) -> VREFTC_R {
        VREFTC_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - USHFRCO Tuning Value"]
    #[inline(always)]
    #[must_use]
    pub fn tuning(&mut self) -> TUNING_W<USHFRCOCTRL_SPEC, 0> {
        TUNING_W::new(self)
    }
    #[doc = "Bits 8:13 - USHFRCO Fine Tuning Value"]
    #[inline(always)]
    #[must_use]
    pub fn finetuning(&mut self) -> FINETUNING_W<USHFRCOCTRL_SPEC, 8> {
        FINETUNING_W::new(self)
    }
    #[doc = "Bits 16:20 - USHFRCO Frequency Range"]
    #[inline(always)]
    #[must_use]
    pub fn freqrange(&mut self) -> FREQRANGE_W<USHFRCOCTRL_SPEC, 16> {
        FREQRANGE_W::new(self)
    }
    #[doc = "Bits 21:23 - USHFRCO Comparator Bias Current"]
    #[inline(always)]
    #[must_use]
    pub fn cmpbias(&mut self) -> CMPBIAS_W<USHFRCOCTRL_SPEC, 21> {
        CMPBIAS_W::new(self)
    }
    #[doc = "Bit 24 - USHFRCO LDO High Power Mode"]
    #[inline(always)]
    #[must_use]
    pub fn ldohp(&mut self) -> LDOHP_W<USHFRCOCTRL_SPEC, 24> {
        LDOHP_W::new(self)
    }
    #[doc = "Bits 25:26 - Locally Divide USHFRCO Clock Output"]
    #[inline(always)]
    #[must_use]
    pub fn clkdiv(&mut self) -> CLKDIV_W<USHFRCOCTRL_SPEC, 25> {
        CLKDIV_W::new(self)
    }
    #[doc = "Bit 27 - Enable Reference for Fine Tuning"]
    #[inline(always)]
    #[must_use]
    pub fn finetuningen(&mut self) -> FINETUNINGEN_W<USHFRCOCTRL_SPEC, 27> {
        FINETUNINGEN_W::new(self)
    }
    #[doc = "Bits 28:31 - USHFRCO Temperature Coefficient Trim on Comparator Reference"]
    #[inline(always)]
    #[must_use]
    pub fn vreftc(&mut self) -> VREFTC_W<USHFRCOCTRL_SPEC, 28> {
        VREFTC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "USHFRCO Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ushfrcoctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ushfrcoctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct USHFRCOCTRL_SPEC;
impl crate::RegisterSpec for USHFRCOCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ushfrcoctrl::R`](R) reader structure"]
impl crate::Readable for USHFRCOCTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ushfrcoctrl::W`](W) writer structure"]
impl crate::Writable for USHFRCOCTRL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets USHFRCOCTRL to value 0xb148_1f7f"]
impl crate::Resettable for USHFRCOCTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0xb148_1f7f;
}
