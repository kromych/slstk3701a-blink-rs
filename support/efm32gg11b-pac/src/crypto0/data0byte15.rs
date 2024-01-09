#[doc = "Register `DATA0BYTE15` reader"]
pub type R = crate::R<DATA0BYTE15_SPEC>;
#[doc = "Register `DATA0BYTE15` writer"]
pub type W = crate::W<DATA0BYTE15_SPEC>;
#[doc = "Field `DATA0BYTE15` reader - Data 0 Byte 15 Access"]
pub type DATA0BYTE15_R = crate::FieldReader;
#[doc = "Field `DATA0BYTE15` writer - Data 0 Byte 15 Access"]
pub type DATA0BYTE15_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Data 0 Byte 15 Access"]
    #[inline(always)]
    pub fn data0byte15(&self) -> DATA0BYTE15_R {
        DATA0BYTE15_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Data 0 Byte 15 Access"]
    #[inline(always)]
    #[must_use]
    pub fn data0byte15(&mut self) -> DATA0BYTE15_W<DATA0BYTE15_SPEC> {
        DATA0BYTE15_W::new(self, 0)
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
#[doc = "DATA0 Register Byte 15 Access\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`data0byte15::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`data0byte15::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DATA0BYTE15_SPEC;
impl crate::RegisterSpec for DATA0BYTE15_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`data0byte15::R`](R) reader structure"]
impl crate::Readable for DATA0BYTE15_SPEC {}
#[doc = "`write(|w| ..)` method takes [`data0byte15::W`](W) writer structure"]
impl crate::Writable for DATA0BYTE15_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DATA0BYTE15 to value 0"]
impl crate::Resettable for DATA0BYTE15_SPEC {
    const RESET_VALUE: u32 = 0;
}
