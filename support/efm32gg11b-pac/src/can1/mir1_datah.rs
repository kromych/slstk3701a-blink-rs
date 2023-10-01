#[doc = "Register `MIR1_DATAH` reader"]
pub type R = crate::R<MIR1_DATAH_SPEC>;
#[doc = "Register `MIR1_DATAH` writer"]
pub type W = crate::W<MIR1_DATAH_SPEC>;
#[doc = "Field `DATA4` reader - Fifth Byte of CAN Data Frame"]
pub type DATA4_R = crate::FieldReader;
#[doc = "Field `DATA4` writer - Fifth Byte of CAN Data Frame"]
pub type DATA4_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `DATA5` reader - Sixth Byte of CAN Data Frame"]
pub type DATA5_R = crate::FieldReader;
#[doc = "Field `DATA5` writer - Sixth Byte of CAN Data Frame"]
pub type DATA5_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `DATA6` reader - Seventh Byte of CAN Data Frame"]
pub type DATA6_R = crate::FieldReader;
#[doc = "Field `DATA6` writer - Seventh Byte of CAN Data Frame"]
pub type DATA6_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
#[doc = "Field `DATA7` reader - Eight Byte of CAN Data Frame"]
pub type DATA7_R = crate::FieldReader;
#[doc = "Field `DATA7` writer - Eight Byte of CAN Data Frame"]
pub type DATA7_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Fifth Byte of CAN Data Frame"]
    #[inline(always)]
    pub fn data4(&self) -> DATA4_R {
        DATA4_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Sixth Byte of CAN Data Frame"]
    #[inline(always)]
    pub fn data5(&self) -> DATA5_R {
        DATA5_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Seventh Byte of CAN Data Frame"]
    #[inline(always)]
    pub fn data6(&self) -> DATA6_R {
        DATA6_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Eight Byte of CAN Data Frame"]
    #[inline(always)]
    pub fn data7(&self) -> DATA7_R {
        DATA7_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Fifth Byte of CAN Data Frame"]
    #[inline(always)]
    #[must_use]
    pub fn data4(&mut self) -> DATA4_W<MIR1_DATAH_SPEC, 0> {
        DATA4_W::new(self)
    }
    #[doc = "Bits 8:15 - Sixth Byte of CAN Data Frame"]
    #[inline(always)]
    #[must_use]
    pub fn data5(&mut self) -> DATA5_W<MIR1_DATAH_SPEC, 8> {
        DATA5_W::new(self)
    }
    #[doc = "Bits 16:23 - Seventh Byte of CAN Data Frame"]
    #[inline(always)]
    #[must_use]
    pub fn data6(&mut self) -> DATA6_W<MIR1_DATAH_SPEC, 16> {
        DATA6_W::new(self)
    }
    #[doc = "Bits 24:31 - Eight Byte of CAN Data Frame"]
    #[inline(always)]
    #[must_use]
    pub fn data7(&mut self) -> DATA7_W<MIR1_DATAH_SPEC, 24> {
        DATA7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Interface Data B Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mir1_datah::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mir1_datah::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MIR1_DATAH_SPEC;
impl crate::RegisterSpec for MIR1_DATAH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mir1_datah::R`](R) reader structure"]
impl crate::Readable for MIR1_DATAH_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mir1_datah::W`](W) writer structure"]
impl crate::Writable for MIR1_DATAH_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MIR1_DATAH to value 0"]
impl crate::Resettable for MIR1_DATAH_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
