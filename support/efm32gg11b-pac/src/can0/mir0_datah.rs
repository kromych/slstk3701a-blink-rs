#[doc = "Register `MIR0_DATAH` reader"]
pub type R = crate::R<MIR0_DATAH_SPEC>;
#[doc = "Register `MIR0_DATAH` writer"]
pub type W = crate::W<MIR0_DATAH_SPEC>;
#[doc = "Field `DATA4` reader - Fifth Byte of CAN Data Frame"]
pub type DATA4_R = crate::FieldReader;
#[doc = "Field `DATA4` writer - Fifth Byte of CAN Data Frame"]
pub type DATA4_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `DATA5` reader - Sixth Byte of CAN Data Frame"]
pub type DATA5_R = crate::FieldReader;
#[doc = "Field `DATA5` writer - Sixth Byte of CAN Data Frame"]
pub type DATA5_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `DATA6` reader - Seventh Byte of CAN Data Frame"]
pub type DATA6_R = crate::FieldReader;
#[doc = "Field `DATA6` writer - Seventh Byte of CAN Data Frame"]
pub type DATA6_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `DATA7` reader - Eight Byte of CAN Data Frame"]
pub type DATA7_R = crate::FieldReader;
#[doc = "Field `DATA7` writer - Eight Byte of CAN Data Frame"]
pub type DATA7_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
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
    pub fn data4(&mut self) -> DATA4_W<MIR0_DATAH_SPEC> {
        DATA4_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Sixth Byte of CAN Data Frame"]
    #[inline(always)]
    #[must_use]
    pub fn data5(&mut self) -> DATA5_W<MIR0_DATAH_SPEC> {
        DATA5_W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Seventh Byte of CAN Data Frame"]
    #[inline(always)]
    #[must_use]
    pub fn data6(&mut self) -> DATA6_W<MIR0_DATAH_SPEC> {
        DATA6_W::new(self, 16)
    }
    #[doc = "Bits 24:31 - Eight Byte of CAN Data Frame"]
    #[inline(always)]
    #[must_use]
    pub fn data7(&mut self) -> DATA7_W<MIR0_DATAH_SPEC> {
        DATA7_W::new(self, 24)
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
#[doc = "Interface Data B Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mir0_datah::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mir0_datah::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MIR0_DATAH_SPEC;
impl crate::RegisterSpec for MIR0_DATAH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mir0_datah::R`](R) reader structure"]
impl crate::Readable for MIR0_DATAH_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mir0_datah::W`](W) writer structure"]
impl crate::Writable for MIR0_DATAH_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MIR0_DATAH to value 0"]
impl crate::Resettable for MIR0_DATAH_SPEC {
    const RESET_VALUE: u32 = 0;
}
