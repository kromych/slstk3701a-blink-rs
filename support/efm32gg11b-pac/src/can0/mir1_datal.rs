#[doc = "Register `MIR1_DATAL` reader"]
pub type R = crate::R<MIR1_DATAL_SPEC>;
#[doc = "Register `MIR1_DATAL` writer"]
pub type W = crate::W<MIR1_DATAL_SPEC>;
#[doc = "Field `DATA0` reader - First Byte of CAN Data Frame"]
pub type DATA0_R = crate::FieldReader;
#[doc = "Field `DATA0` writer - First Byte of CAN Data Frame"]
pub type DATA0_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `DATA1` reader - Second Byte of CAN Data Frame"]
pub type DATA1_R = crate::FieldReader;
#[doc = "Field `DATA1` writer - Second Byte of CAN Data Frame"]
pub type DATA1_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `DATA2` reader - Third Byte of CAN Data Frame"]
pub type DATA2_R = crate::FieldReader;
#[doc = "Field `DATA2` writer - Third Byte of CAN Data Frame"]
pub type DATA2_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `DATA3` reader - Fourth Byte of CAN Data Frame"]
pub type DATA3_R = crate::FieldReader;
#[doc = "Field `DATA3` writer - Fourth Byte of CAN Data Frame"]
pub type DATA3_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - First Byte of CAN Data Frame"]
    #[inline(always)]
    pub fn data0(&self) -> DATA0_R {
        DATA0_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Second Byte of CAN Data Frame"]
    #[inline(always)]
    pub fn data1(&self) -> DATA1_R {
        DATA1_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Third Byte of CAN Data Frame"]
    #[inline(always)]
    pub fn data2(&self) -> DATA2_R {
        DATA2_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Fourth Byte of CAN Data Frame"]
    #[inline(always)]
    pub fn data3(&self) -> DATA3_R {
        DATA3_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - First Byte of CAN Data Frame"]
    #[inline(always)]
    #[must_use]
    pub fn data0(&mut self) -> DATA0_W<MIR1_DATAL_SPEC> {
        DATA0_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Second Byte of CAN Data Frame"]
    #[inline(always)]
    #[must_use]
    pub fn data1(&mut self) -> DATA1_W<MIR1_DATAL_SPEC> {
        DATA1_W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Third Byte of CAN Data Frame"]
    #[inline(always)]
    #[must_use]
    pub fn data2(&mut self) -> DATA2_W<MIR1_DATAL_SPEC> {
        DATA2_W::new(self, 16)
    }
    #[doc = "Bits 24:31 - Fourth Byte of CAN Data Frame"]
    #[inline(always)]
    #[must_use]
    pub fn data3(&mut self) -> DATA3_W<MIR1_DATAL_SPEC> {
        DATA3_W::new(self, 24)
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
#[doc = "Interface Data a Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mir1_datal::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mir1_datal::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MIR1_DATAL_SPEC;
impl crate::RegisterSpec for MIR1_DATAL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mir1_datal::R`](R) reader structure"]
impl crate::Readable for MIR1_DATAL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mir1_datal::W`](W) writer structure"]
impl crate::Writable for MIR1_DATAL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MIR1_DATAL to value 0"]
impl crate::Resettable for MIR1_DATAL_SPEC {
    const RESET_VALUE: u32 = 0;
}
